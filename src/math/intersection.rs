use crate::{materials::Material, math::*, surfaces::Surface, textures::TextureCoordinates};

#[derive(Debug, Clone)]
pub struct RayIntersection<'a, S: Space<3>> {
  pub ray: Ray3<S>,
  pub surface: &'a dyn Surface,
  pub material: &'a dyn Material,
  pub intersect_time: Float,
  pub intersect_point: Point3<S>,
  pub geometric_normal: Direction3<S>,
  pub shading_normal: Direction3<S>,
  pub tex_coords: TextureCoordinates
}

pub type WorldRayIntersection<'a> = RayIntersection<'a, WorldSpace>;
