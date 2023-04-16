use threadpool::Builder;

use {
  crate::{
    camera::*, integrators::*, light::*, materials::MaterialParameters, math::*, samplers::*,
    surface_groups::SurfaceGroupParameters, surfaces::SurfaceParameters, RenderSettings
  },
  image::{DynamicImage, ImageBuffer, Rgb},
  indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle},
  serde::Deserialize,
  serde_json::Value,
  std::{
    error::Error,
    sync::{Arc, Mutex},
    thread,
    time::Duration
  },
  threadpool::ThreadPool
};

#[derive(Debug, Deserialize)]
struct SceneParameters {
  #[serde(alias = "image_samples")]
  pub samples_per_pixel: usize,

  #[serde(alias = "camera")]
  pub camera_params: CameraParameters,

  #[serde(alias = "materials")]
  pub material_params: Vec<Box<dyn MaterialParameters>>,

  #[serde(alias = "surfaces")]
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
      surface_params,
      surface_group_params,
      integrator_params
    } = parse_scene_params(json)?;

    let materials = material_params.into_iter().map(|p| (p.name(), p)).collect();
    let surfaces = surface_params.into_iter().map(|p| p.build_surface(&materials)).collect();
    let surface_group = surface_group_params.build_surface_group(surfaces)?;
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

    // Create progress bar styles with nice spacing.
    let offset = usize::max(
      (num_subimages as f64).log10().ceil() as usize,
      ((subimg_width * subimg_height) as f64).log10().ceil() as usize
    );

    let main_bar_style = format!(
      "[ {{elapsed_precise}} / {{duration_precise}} ]: {{bar:50.green/red}} {{msg:>{offset}}}/{num_subimages} subimages"
    );

    let sub_bar_style = format!(
      "Subimage {{msg}}: {{bar:50.cyan/magenta}} {{pos:>{offset}}}/{{len:<{offset}}} pixels",
    );

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

    // Create a MultiProgressBar to manage each thread's progress bar.
    let mut maybe_progress_bars = settings.use_progress_bar.then(|| {
      let progress_bars = MultiProgress::with_draw_target(ProgressDrawTarget::stdout());
      let overall_progress_bar = ProgressBar::new((width * height) as u64);
      overall_progress_bar
        .set_style(ProgressStyle::with_template(&main_bar_style).unwrap().progress_chars("##-"));
      progress_bars.add(overall_progress_bar.clone());
      (progress_bars, Vec::new(), overall_progress_bar)
    });

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

    // Sort the subimages which will most likely to take the longest to render (i.e. the ones with
    // the with the most pixels) at the bottom.
    subimage_windows.sort_by(|(_, (w1, h1)), (_, (w2, h2))| (w1 * h1).cmp(&(w2 * h2)));

    // Send jobs to the threadpool and create progress bars.
    let style = ProgressStyle::with_template(&sub_bar_style).unwrap().progress_chars("##-");
    for subimage_window @ (_, (sub_w, sub_h)) in subimage_windows {
      // Create a progress bar for tracking each pixel rendered in this subimage.
      let maybe_progress_bar =
        maybe_progress_bars.as_mut().map(|(progress_bars, progress_bar_list, _)| {
          let progress_bar = ProgressBar::new((sub_w * sub_h) as u64);
          progress_bar.set_style(style.clone());
          progress_bars.add(progress_bar.clone());
          progress_bar_list.push(progress_bar.clone());
          progress_bar
        });

      // Send the render job to the threadpool
      Self::async_integrate_subimage(
        &thread_pool,
        &integrator,
        &camera,
        self.samples_per_pixel,
        &image_lock,
        subimage_window,
        maybe_progress_bar
      );
    }

    if let Some((_, progress_bar_list, overall_progress_bar)) = maybe_progress_bars {
      while thread_pool.queued_count() > 0 {
        thread::sleep(Duration::from_millis(100));
        let total_progress: u64 = progress_bar_list.iter().map(|p| p.position()).sum();
        overall_progress_bar.set_position(total_progress);
        thread::sleep(Duration::from_millis(10));
        overall_progress_bar.set_message(format!(
          "{}",
          num_subimages as usize - (thread_pool.queued_count() + thread_pool.active_count()),
        ))
      }

      overall_progress_bar.finish()
    };

    // Wait for the threads to finish and return the resulting image.
    thread_pool.join();
    Arc::into_inner(image_lock).unwrap().into_inner().unwrap().into()
  }

  fn async_integrate_subimage(
    thread_pool: &ThreadPool,
    integrator: &Arc<Box<dyn Integrator + Sync + Send>>,
    camera: &Arc<Camera>,
    samples_per_pixel: usize,
    image_lock: &Arc<Mutex<Image>>,
    ((sub_x, sub_y), (sub_w, sub_h)): ((u32, u32), (u32, u32)),
    maybe_progress_bar: Option<ProgressBar>
  ) {
    // Copy the ARCs.
    let integrator = integrator.clone();
    let camera = camera.clone();
    let image_lock = image_lock.clone();

    // Send the render job to the thread pool.
    thread_pool.execute(move || {
      // Create a temporary image buffer to render into.
      let mut subimage = Image::new(sub_w, sub_h);

      // Set the progress bar message to indicate that this thread has officially started
      if let Some(progress_bar) = maybe_progress_bar.as_ref() {
        progress_bar.set_message(format!("( {:<4}, {:>4} )", sub_x, sub_y));
      }

      // Precompute 1 / samples_per_pixel to save some time.
      let inv_spp = 1.0 / (samples_per_pixel as Float);

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
            let ray_x = (sub_x + x) as Float + ray_sampler.next();
            let ray_y = (sub_y + y) as Float + ray_sampler.next();
            let ray = camera.sample_ray_through_pixel(&mut ray_sampler, ray_x, ray_y);

            // Add the incoming radiance to our running average.
            light += integrator.incoming_radiance(&mut integrator_sampler, ray);
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

          // Increment the progress bar to count pixel (x, y) as done.
          if let Some(progress_bar) = maybe_progress_bar.as_ref() {
            progress_bar.inc(1)
          }
        }
      }

      // Copy the temporary buffer into its place in the main image.
      let mut image = image_lock.as_ref().lock().unwrap();
      for x in 0..sub_w {
        for y in 0..sub_h {
          image.put_pixel(sub_x + x, sub_y + y, *subimage.get_pixel(x, y));
        }
      }

      if let Some(progress_bar) = maybe_progress_bar {
        progress_bar.finish_and_clear()
      }
    })
  }
}
