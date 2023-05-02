use crate::{math::*, textures::TextureCoordinate};

#[derive(Debug, Clone)]
pub struct RayIntersection<S: Space<3>> {
  pub intersect_point: Point3<S>,
  pub intersect_direction: UnitVector3<S>,
  pub geometric_normal: UnitVector3<S>,
  // pub shading_normal: UnitVector3<S>,
  pub intersect_time: PositiveReal,
  pub tex_coords: TextureCoordinate
}

impl<S: Space<3>> RayIntersection<S> {
  pub fn cast_unchecked<T: Space<3>>(&self) -> RayIntersection<T> {
    RayIntersection {
      intersect_point: self.intersect_point.cast_unchecked(),
      intersect_direction: self.intersect_direction.cast_unchecked(),
      geometric_normal: self.geometric_normal.cast_unchecked(),
      // shading_normal: self.shading_normal.cast_unchecked(),
      intersect_time: self.intersect_time,
      tex_coords: self.tex_coords
    }
  }
}

pub type WorldRayIntersection = RayIntersection<WorldSpace>;
