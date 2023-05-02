use std::sync::Arc;

use super::*;
use crate::{materials::Material, math::*, textures::TextureCoordinate};

#[derive(Debug, Clone)]
pub struct RayIntersection<S: Space<3>> {
  pub ray: Ray3<S>,
  pub material: Arc<dyn Material>,
  pub intersect_time: PositiveReal,
  pub intersect_point: Point3<S>,
  pub geometric_normal: UnitVector3<S>,
  pub shading_normal: UnitVector3<S>,
  pub tex_coords: TextureCoordinate
}

impl<S: Space<3>> RayIntersection<S> {
  pub fn cast_unchecked<T: Space<3>>(&self) -> RayIntersection<T> {
    RayIntersection {
      ray: self.ray.cast_unchecked(),
      material: self.material.clone(),
      intersect_time: self.intersect_time,
      intersect_point: self.intersect_point.cast_unchecked(),
      geometric_normal: self.geometric_normal.cast_unchecked(),
      shading_normal: self.shading_normal.cast_unchecked(),
      tex_coords: self.tex_coords
    }
  }
}

pub type WorldRayIntersection = RayIntersection<WorldSpace>;
