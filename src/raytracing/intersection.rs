use super::*;
use crate::{math::*, surfaces::Surface, textures::TextureCoordinate};

#[derive(Debug, Clone)]
pub struct RayIntersection<'a, S: Space<3>> {
  pub ray: Ray3<S>,
  pub surface: &'a dyn Surface,
  pub intersect_time: PositiveReal,
  pub intersect_point: Point3<S>,
  pub geometric_normal: UnitVector3<S>,
  pub shading_normal: UnitVector3<S>,
  pub tex_coords: TextureCoordinate
}

impl<'a, S: Space<3>> RayIntersection<'a, S> {
  pub fn cast_unsafe<T: Space<3>>(&self) -> RayIntersection<'a, T> {
    RayIntersection {
      ray: self.ray.cast_unsafe(),
      surface: self.surface,
      intersect_time: self.intersect_time,
      intersect_point: self.intersect_point.cast_unsafe(),
      geometric_normal: self.geometric_normal.cast_unsafe(),
      shading_normal: self.shading_normal.cast_unsafe(),
      tex_coords: self.tex_coords
    }
  }
}

pub type WorldRayIntersection<'a> = RayIntersection<'a, WorldSpace>;
