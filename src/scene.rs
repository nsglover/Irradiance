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
  serde::Deserialize,
  serde_json::Value,
  std::error::Error
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

  #[serde(alias = "integrator", default = "default_integrator")]
  pub integrator_params: Box<dyn IntegratorParameters>,

  #[serde(alias = "accelerator", default = "default_surface_group")]
  pub surface_group_params: Box<dyn SurfaceGroupParameters>,

  #[serde(alias = "materials", default = "default_materials")]
  pub material_params: Vec<Box<dyn MaterialParameters>>,

  #[serde(alias = "surfaces", default = "default_surfaces")]
  pub surface_params: Vec<Box<dyn SurfaceParameters>>
}

pub fn parse_scene_params(json: Value) -> Result<SceneParameters, Box<dyn Error>> {
  Ok(serde_json::from_value(json)?)
}

#[derive(Debug)]
pub struct Scene<'a> {
  samples_per_pixel: usize,

  camera: Camera,

  integrator: Box<dyn Integrator + 'a>
}

impl<'a> Scene<'a> {
  pub fn build_from_json(json: serde_json::Value) -> Result<Scene<'a>, Box<dyn Error>> {
    let SceneParameters {
      samples_per_pixel,
      camera_params,
      integrator_params,
      surface_group_params,
      material_params,
      surface_params
    } = parse_scene_params(json)?;

    let materials = material_params.into_iter().map(|p| (p.name(), p)).collect();
    let surfaces = surface_params.into_iter().map(|p| p.build_surface(&materials)).collect();
    let surface_group = surface_group_params.build_surface_group(surfaces)?;
    let integrator = integrator_params.build_integrator(surface_group)?;

    Ok(Self { samples_per_pixel, camera: camera_params.build_camera(), integrator })
  }

  pub fn integrate(self) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let mut image = image::RgbImage::new(self.camera.resolution().0, self.camera.resolution().1);

    for x in 0..image.width() {
      for y in 0..image.height() {
        let mut ray_sampler = IndependentSampler::new();
        let mut surface_sampler = IndependentSampler::new();
        let mut light = Color::black();

        for _ in 0..self.samples_per_pixel {
          let ray_x = x as Float + ray_sampler.next();
          let ray_y = y as Float + ray_sampler.next();
          let ray = self.camera.generate_ray(ray_x, ray_y);
          light += self.integrator.estimate_radiance(&mut surface_sampler, ray);
        }

        light /= self.samples_per_pixel as Float;

        let bytes = light.to_bytes();
        image.put_pixel(x, y, image::Rgb([bytes[0], bytes[1], bytes[2]]));
      }
    }

    image
  }
}
