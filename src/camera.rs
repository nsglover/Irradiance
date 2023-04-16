use {
  crate::{
    math::*,
    raytracing::*,
    samplers::{uniform_random_in_unit_disc, Sampler}
  },
  serde::Deserialize
};

fn default_resolution() -> (u32, u32) { (512, 512) }

fn default_field_of_view() -> Float { 90.0 }

fn default_focal_distance() -> Float { 1.0 }

fn default_aperture_radius() -> Float { 0.0 }

#[derive(Debug, Deserialize)]
pub struct CameraParameters {
  transform: TransformParameters,

  #[serde(default = "default_resolution")]
  resolution: (u32, u32),

  #[serde(alias = "vfov", default = "default_field_of_view")]
  field_of_view: Float,

  #[serde(alias = "fdist", default = "default_focal_distance")]
  focal_distance: Float,

  #[serde(alias = "aperture", default = "default_aperture_radius")]
  aperture_radius: Float
}

impl CameraParameters {
  pub fn build_camera(self) -> Camera {
    let fov = self.field_of_view * PI / 180.0;
    let resolution = self.resolution;
    let height = 2.0 * (fov / 2.0).tan() * self.focal_distance;
    let width = ((resolution.0 as Float) / (resolution.1 as Float)) * (height as Float);
    let image_plane_size = (width, height);

    Camera {
      resolution,
      image_plane_size,
      transform: self.transform.build_transform().into_inverse(),
      focal_distance: self.focal_distance,
      aperture_radius: self.aperture_radius
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct CameraSpace;

impl Space<3> for CameraSpace {}

#[derive(Debug)]
pub struct Camera {
  resolution: (u32, u32),
  image_plane_size: (Float, Float),
  transform: LocalToWorld<CameraSpace>,
  focal_distance: Float,
  aperture_radius: Float
}

impl Camera {
  pub fn sample_ray_through_pixel(
    &self,
    sampler: &mut dyn Sampler,
    mut u: Float,
    mut v: Float
  ) -> WorldRay {
    u /= self.resolution.0 as Float;
    v /= self.resolution.1 as Float;

    let disc = uniform_random_in_unit_disc(sampler) * self.aperture_radius;
    let origin = Point::from(nalgebra::point![disc.inner().x, disc.inner().y, 0.0]);
    let dir = Point::from(nalgebra::point![
      (u - 0.5) * self.image_plane_size.0,
      (0.5 - v) * self.image_plane_size.1,
      -self.focal_distance
    ]) - origin;

    self.transform.ray(&Ray::new(origin, dir.normalize()))
  }

  pub fn resolution(&self) -> (u32, u32) { self.resolution }
}
