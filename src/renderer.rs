use std::{
  error::Error,
  sync::{Arc, Mutex},
  thread,
  time::Duration
};

use image::{DynamicImage, ImageBuffer, Rgb};
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use serde::Deserialize;
use serde_json::Value;
use threadpool::{Builder, ThreadPool};

use crate::{
  camera::*,
  integrators::*,
  light::*,
  materials::MaterialParameters,
  math::*,
  samplers::*,
  surface_groups::SurfaceGroupParameters,
  surfaces::{MeshParameters, SurfaceParameters},
  RenderSettings
};

#[derive(Debug, Deserialize)]
struct SceneParameters {
  #[serde(alias = "samples-per-pixel")]
  pub samples_per_pixel: usize,

  #[serde(alias = "camera")]
  pub camera_params: CameraParameters,

  #[serde(alias = "materials", default)]
  pub material_params: Vec<Box<dyn MaterialParameters>>,

  #[serde(alias = "meshes", default)]
  pub mesh_params: Vec<MeshParameters>,

  #[serde(alias = "surfaces", default)]
  pub surface_params: Vec<Box<dyn SurfaceParameters>>,

  #[serde(alias = "accelerator", default = "crate::surface_groups::default_surface_group")]
  pub surface_group_params: Box<dyn SurfaceGroupParameters>,

  #[serde(alias = "integrator", default = "crate::integrators::default_integrator")]
  pub integrator_params: Box<dyn IntegratorParameters>
}

fn parse_scene_params(json: Value) -> Result<SceneParameters, Box<dyn Error>> {
  Ok(serde_json::from_value(json)?)
}

pub struct Renderer {
  samples_per_pixel: usize,
  camera: Camera,
  integrator: Box<dyn Integrator + Sync + Send>
}

type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;

impl Renderer {
  pub fn build_from_json(json: serde_json::Value) -> Result<Renderer, Box<dyn Error>> {
    // TODO: Add a progress bar for this building step
    let SceneParameters {
      samples_per_pixel,
      camera_params,
      material_params,
      mesh_params,
      surface_params,
      surface_group_params,
      integrator_params
    } = parse_scene_params(json)?;

    let materials = material_params.into_iter().map(|p| (p.name(), p.build_material())).collect();
    let meshes = mesh_params.into_iter().map(|p| p.build_mesh()).collect();
    let surface_group = surface_group_params.build_surface_group(
      surface_params.into_iter().flat_map(|p| p.build_surfaces(&materials, &meshes)).collect()
    )?;

    let integrator = integrator_params.build_integrator(surface_group)?;
    Ok(Self { samples_per_pixel, camera: camera_params.build_camera(), integrator })
  }

  pub fn render_scene(self, settings: RenderSettings) -> DynamicImage {
    // Create the image to which we will be rendering.
    let (width, height) = (self.camera.resolution().0, self.camera.resolution().1);
    let image = Image::new(width, height);

    // Compute the number of intervals that will be rendered concurrently.
    let (subimg_width, subimg_height) = settings.subimage_dimensions;
    let num_x_intervals = (width as f64 / (subimg_width as f64)).ceil() as u32;
    let num_y_intervals = (height as f64 / (subimg_height as f64)).ceil() as u32;
    let num_subimages = num_x_intervals * num_y_intervals;

    // Set up the thread pool with a larger stack size (due to many integrators being highly
    // recursive, terminated only by Russian roulette).
    let thread_pool = Builder::new()
      .num_threads(settings.num_threads)
      .thread_stack_size(16 * 1024 * 1024)
      .thread_name("pbr-project-subimage-renderer".to_owned())
      .build();

    // Move all shared data into atomic reference counters.
    let integrator = Arc::new(self.integrator);
    let camera = Arc::new(self.camera);
    let image_lock = Arc::new(Mutex::new(image));

    // For each pair of intervals, add a subimage window to a collection.
    let mut subimage_windows = Vec::new();
    for x_interval in 0..num_x_intervals {
      for y_interval in 0..num_y_intervals {
        // Compute the subimage offset.
        let sub_x = x_interval * subimg_width;
        let sub_y = y_interval * subimg_height;

        // Truncate the subimage size if necessary
        let sub_w = if sub_x + subimg_width <= width { subimg_width } else { width - sub_x };
        let sub_h = if sub_y + subimg_height <= height { subimg_height } else { height - sub_y };

        subimage_windows.push(((sub_x, sub_y), (sub_w, sub_h)));
      }
    }

    // Make the progress bar style
    let offset = (num_subimages as f64).log10().ceil() as usize;
    let main_bar_style = format!(
      "[ {{elapsed_precise}} / {{msg}} ]: {{bar:50.green/red}} {{pos:>{offset}}}/{num_subimages} subimages"
    );

    // If enabled, start up the progress bar
    let maybe_progress_bar = settings.use_progress_bar.then(|| {
      let progress_bar = ProgressBar::with_draw_target(
        Some(num_subimages as u64),
        ProgressDrawTarget::stdout_with_hz(24)
      );

      let style = ProgressStyle::with_template(&main_bar_style).unwrap().progress_chars("##-");
      progress_bar.set_style(style);
      progress_bar
    });

    // Send jobs to the threadpool.
    for subimage_window in subimage_windows {
      // Send the render job to the threadpool
      Self::async_integrate_subimage(
        &thread_pool,
        &integrator,
        &camera,
        self.samples_per_pixel,
        &image_lock,
        subimage_window
      );
    }

    // A helper to display durations
    fn duration_to_string(d: Duration) -> String {
      let seconds = d.as_secs() % 60;
      let minutes = (d.as_secs() / 60) % 60;
      let hours = (d.as_secs() / 60) / 60;
      format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds)
    }

    // Manually update the progress bar as the threads run.
    if let Some(progress_bar) = &maybe_progress_bar {
      loop {
        let num_incomplete = thread_pool.queued_count() + thread_pool.active_count();
        let num_complete = num_subimages as usize - num_incomplete;
        progress_bar.set_position(num_complete as u64);

        let elapsed = progress_bar.elapsed().as_secs_f64();
        let ratio = if num_complete == 0 {
          num_subimages as f64
        } else {
          (num_subimages as f64) / (num_complete as f64)
        };

        let projected = Duration::from_secs_f64(elapsed * ratio);
        progress_bar.set_message(duration_to_string(projected));

        thread::sleep(Duration::from_millis(250));
        if num_incomplete == 0 {
          break;
        }
      }
    }

    // Wait for the threads to finish and mark the overall progress bar as finished.
    thread_pool.join();
    if let Some(progress_bar) = maybe_progress_bar {
      progress_bar.set_message(duration_to_string(progress_bar.elapsed()));
      progress_bar.finish();
    }

    // Return the resulting image.
    Arc::into_inner(image_lock).unwrap().into_inner().unwrap().into()
  }

  fn async_integrate_subimage(
    thread_pool: &ThreadPool,
    integrator: &Arc<Box<dyn Integrator + Sync + Send>>,
    camera: &Arc<Camera>,
    samples_per_pixel: usize,
    image_lock: &Arc<Mutex<Image>>,
    ((sub_x, sub_y), (sub_w, sub_h)): ((u32, u32), (u32, u32))
  ) {
    // Copy the ARCs.
    let integrator = integrator.clone();
    let camera = camera.clone();
    let image_lock = image_lock.clone();

    // Send the render job to the thread pool.
    thread_pool.execute(move || {
      // Create a temporary image buffer to render into.
      let mut subimage = Image::new(sub_w, sub_h);

      // Precompute 1 / samples_per_pixel to save some time.
      let inv_spp = 1.0 / (samples_per_pixel as Real);

      // Build samplers for this subimage thread.
      let mut ray_sampler = IndependentSampler::new();
      let mut integrator_sampler = IndependentSampler::new();

      // For each pixel in the buffer, generate samples_per_pixel jittered rays and estimate the
      // incoming radiance along those rays.
      for x in 0..sub_w {
        for y in 0..sub_h {
          let mut light = Color::black();
          for _ in 0..samples_per_pixel {
            // Generate a slightly jittered ray through pixel (x, y).
            let ray_x = ray_sampler.next() + (sub_x + x) as Real;
            let ray_y = ray_sampler.next() + (sub_y + y) as Real;
            let ray = camera.sample_ray_through_pixel(&mut ray_sampler, ray_x, ray_y);

            // Add the incoming radiance to our running average.
            light += integrator.radiance_estimate(&mut integrator_sampler, ray);
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
          let bytes = srgb.bytes();
          subimage.put_pixel(x, y, image::Rgb([bytes[0], bytes[1], bytes[2]]));
        }
      }

      // Copy the temporary buffer into its place in the main image.
      let mut image = image_lock.as_ref().lock().unwrap();
      for x in 0..sub_w {
        for y in 0..sub_h {
          image.put_pixel(sub_x + x, sub_y + y, *subimage.get_pixel(x, y));
        }
      }
    })
  }
}
