use super::*;
use crate::{math::*, surfaces::Surface, textures::TextureCoordinates};

#[derive(Debug, Clone)]
pub struct RayIntersection<'a, S: Space<3>> {
  pub ray: Ray3<S>,
  pub surface: &'a dyn Surface,
  pub intersect_time: PositiveReal,
  pub intersect_point: Point3<S>,
  pub geometric_normal: UnitVector3<S>,
  pub shading_normal: UnitVector3<S>,
  pub tex_coords: TextureCoordinates
}

pub type WorldRayIntersection<'a> = RayIntersection<'a, WorldSpace>;
