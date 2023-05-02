use serde::Deserialize;

use crate::{
  common::Wrapper,
  math::*,
  raytracing::*,
  sampling::{uniform_random_in_unit_disc, Sampler}
};

fn default_resolution() -> (u32, u32) { (512, 512) }

fn default_field_of_view() -> Real { 90.0 }

fn default_focal_distance() -> Real { 1.0 }

fn default_aperture_radius() -> Real { 0.0 }

#[derive(Debug, Deserialize)]
pub struct CameraParameters {
  transform: TransformParameters,

  #[serde(default = "default_resolution")]
  resolution: (u32, u32),

  #[serde(alias = "vfov", default = "default_field_of_view")]
  field_of_view: Real,

  #[serde(alias = "fdist", default = "default_focal_distance")]
  focal_distance: Real,

  #[serde(alias = "aperture", default = "default_aperture_radius")]
  aperture_radius: Real
}

impl CameraParameters {
  pub fn build_camera(self) -> Camera {
    let fov = self.field_of_view * PI / 180.0;
    let resolution = self.resolution;
    let height = 2.0 * (fov / 2.0).tan() * self.focal_distance;
    let width = ((resolution.0 as Real) / (resolution.1 as Real)) * (height as Real);
    let image_plane_size = (width, height);

    Camera {
      resolution,
      image_plane_size,
      transform: self.transform.build_transform(),
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
  image_plane_size: (Real, Real),
  transform: Box<dyn Transform<WorldSpace, CameraSpace>>,
  focal_distance: Real,
  aperture_radius: Real
}

impl Camera {
  pub fn sample_ray_through_pixel(
    &self,
    sampler: &mut dyn Sampler,
    mut u: Real,
    mut v: Real
  ) -> WorldRay {
    u /= self.resolution.0 as Real;
    v /= self.resolution.1 as Real;

    let disc = (uniform_random_in_unit_disc(sampler) * self.aperture_radius).into_inner();
    let origin = Point::from(nalgebra::point![disc.x, disc.y, 0.0]);
    let dir = Point::from(nalgebra::point![
      (u - 0.5) * self.image_plane_size.0,
      (0.5 - v) * self.image_plane_size.1,
      -self.focal_distance
    ]) - origin;

    self.transform.inverse_ray(&Ray::new(origin, dir.normalize()))
  }

  pub fn resolution(&self) -> (u32, u32) { self.resolution }
}
