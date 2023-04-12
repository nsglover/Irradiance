use {
  crate::{
    camera::*,
    color::Color,
    integrators::*,
    materials::MaterialParameters,
    math::*,
    samplers::*,
    surface_groups::{SurfaceGroupParameters, SurfaceListParameters},
    surfaces::SurfaceParameters
  },
  image::{DynamicImage, ImageBuffer, Rgb},
  serde::Deserialize,
  serde_json::Value,
  std::{
    error::Error,
    sync::{Arc, Mutex}
  },
  threadpool::ThreadPool
};

fn default_materials() -> Vec<Box<dyn MaterialParameters>> { Vec::new() }

fn default_surfaces() -> Vec<Box<dyn SurfaceParameters>> { Vec::new() }

fn default_surface_group() -> Box<dyn SurfaceGroupParameters> { Box::new(SurfaceListParameters {}) }

fn default_integrator() -> Box<dyn IntegratorParameters> { Box::new(NormalIntegratorParameters {}) }

#[derive(Debug, Deserialize)]
pub struct SceneParameters {
  #[serde(alias = "image_samples")]
  pub samples_per_pixel: usize,

  #[serde(alias = "camera")]
  pub camera_params: CameraParameters,

  #[serde(alias = "materials", default = "default_materials")]
  pub material_params: Vec<Box<dyn MaterialParameters>>,

  #[serde(alias = "surfaces", default = "default_surfaces")]
  pub surface_params: Vec<Box<dyn SurfaceParameters>>,

  #[serde(alias = "accelerator", default = "default_surface_group")]
  pub surface_group_params: Box<dyn SurfaceGroupParameters>,

  #[serde(alias = "integrator", default = "default_integrator")]
  pub integrator_params: Box<dyn IntegratorParameters>
}

pub fn parse_scene_params(json: Value) -> Result<SceneParameters, Box<dyn Error>> {
  Ok(serde_json::from_value(json)?)
}

#[derive(Debug)]
pub struct Renderer {
  samples_per_pixel: usize,
  camera: Camera,
  integrator: Box<dyn Integrator + Sync + Send>,
  num_threads: u16,
  use_progress_bar: bool
}

type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;

impl Renderer {
  pub fn build_from_json(
    json: serde_json::Value,
    num_threads: u16,
    use_progress_bar: bool
  ) -> Result<Renderer, Box<dyn Error>> {
    let SceneParameters {
      samples_per_pixel,
      camera_params,
      material_params,
      surface_params,
      surface_group_params,
      integrator_params
    } = parse_scene_params(json)?;

    let materials = material_params.into_iter().map(|p| (p.name(), p)).collect();
    let surfaces = surface_params.into_iter().map(|p| p.build_surface(&materials)).collect();
    let surface_group = surface_group_params.build_surface_group(surfaces)?;
    let integrator = integrator_params.build_integrator(surface_group)?;

    Ok(Self {
      samples_per_pixel,
      camera: camera_params.build_camera(),
      integrator,
      num_threads,
      use_progress_bar
    })
  }

  fn async_integrate_subimage(
    thread_pool: &ThreadPool,
    integrator: &Arc<Box<dyn Integrator + Sync + Send>>,
    camera: &Arc<Camera>,
    samples_per_pixel: usize,
    image_lock: &Arc<Mutex<Image>>,
    subimage_bounds: ((u32, u32), (u32, u32))
  ) {
    // Copy the ARCs.
    let integrator = integrator.clone();
    let camera = camera.clone();
    let image_lock = image_lock.clone();

    // Send the render job to the thread pool.
    thread_pool.execute(move || {
      // Create a temporary image buffer to render into.
      let ((x_offset, y_offset), (width, height)) = subimage_bounds;
      let mut subimage = Image::new(width, height);

      // Precompute 1 / samples_per_pixel to save some time.
      let inv_spp = 1.0 / (samples_per_pixel as Float);

      // Build samplers for this subimage thread.
      let mut ray_sampler = IndependentSampler::new();
      let mut integrator_sampler = IndependentSampler::new();

      // For each pixel in the buffer, generate samples_per_pixel jittered rays and estimate the
      // incoming radiance along those rays.
      for x in 0..width {
        for y in 0..height {
          let mut light = Color::black();
          for _ in 0..samples_per_pixel {
            let ray_x = (x_offset + x) as Float + ray_sampler.next();
            let ray_y = (y_offset + y) as Float + ray_sampler.next();
            let ray = camera.sample_ray_through_pixel(&mut ray_sampler, ray_x, ray_y);
            light += integrator.estimate_radiance(&mut integrator_sampler, ray);
          }

          // Convert to sRGB, which is the color space expected by the image buffer
          let mut srgb = light * inv_spp;
          for c in srgb.inner.iter_mut() {
            if *c <= 0.0031308 {
              *c *= 12.92;
            } else {
              *c = (1.0 + 0.055) * (*c).powf(1.0 / 2.4) - 0.055;
            }
          }

          // Convert the sRGB pixel value into bytes and write to the temporary buffer.
          let bytes = srgb.to_bytes();
          subimage.put_pixel(x, y, image::Rgb([bytes[0], bytes[1], bytes[2]]));
        }
      }

      // Copy the temporary buffer into its place in the main image.
      let mut image = image_lock.as_ref().lock().unwrap();
      for x in 0..width {
        for y in 0..height {
          image.put_pixel(x_offset + x, y_offset + y, *subimage.get_pixel(x, y));
        }
      }
    });
  }

  pub fn render_scene(self) -> DynamicImage {
    // Create the image to which we will be rendering.
    let (width, height) = (self.camera.resolution().0, self.camera.resolution().1);
    let image = Image::new(width, height);

    // X and Y axes of image will be divided into ceil(sqrt(num_threads)) intervals.
    let num_intervals = (self.num_threads as Float).sqrt().ceil() as u32;

    // Compute the sizes of the intervals, potentially with remainders.
    let (x_interval_size, mut x_interval_rem) = (width / num_intervals, width % num_intervals);
    let (y_interval_size, mut y_interval_rem) = (height / num_intervals, height % num_intervals);

    // If the X intervals divide evenly, we can ignore the remainder; otherwise, we'll need another
    // interval of length equal to the remainder.
    let x_ints;
    if x_interval_rem == 0 {
      x_ints = num_intervals;
      x_interval_rem = x_interval_size;
    } else {
      x_ints = num_intervals + 1;
    }

    // If the Y intervals divide evenly, we can ignore the remainder; otherwise, we'll need another
    // interval of length equal to the remainder.
    let y_ints;
    if y_interval_rem == 0 {
      y_ints = num_intervals;
      y_interval_rem = y_interval_size;
    } else {
      y_ints = num_intervals + 1;
    }

    // Create thread pool and move all shared data into atomic reference counters.
    let thread_pool = ThreadPool::new(self.num_threads as usize);
    let integrator = Arc::new(self.integrator);
    let camera = Arc::new(self.camera);
    let image_lock = Arc::new(Mutex::new(image));

    // For each pair of intervals, spawn a thread to run the integrator on the subimage given by the
    // Cartesian product of those intervals.
    for x_interval in 0..x_ints {
      for y_interval in 0..y_ints {
        let sub_x = x_interval * x_interval_size;
        let sub_y = y_interval * y_interval_size;
        let sub_width = if x_interval == x_ints - 1 { x_interval_rem } else { x_interval_size };
        let sub_height = if y_interval == y_ints - 1 { y_interval_rem } else { y_interval_size };

        Self::async_integrate_subimage(
          &thread_pool,
          &integrator,
          &camera,
          self.samples_per_pixel,
          &image_lock,
          ((sub_x, sub_y), (sub_width, sub_height))
        )
      }
    }

    // Wait for the threads to finish and return the resulting image.
    thread_pool.join();
    Arc::into_inner(image_lock).unwrap().into_inner().unwrap().into()
  }
}
