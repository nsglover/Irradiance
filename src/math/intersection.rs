use crate::{materials::Material, math::*, textures::TextureCoordinates};

#[derive(Debug, Clone)]
pub struct RayIntersection<'a, S: Space<3>> {
  pub ray: Ray3<S>,
  pub intersect_time: Float,
  pub intersect_point: Point3<S>,
  pub geom_normal: Direction3<S>,
  pub shading_normal: Direction3<S>,
  pub tex_coords: TextureCoordinates,
  pub material: &'a dyn Material
}

pub type WorldRayIntersection<'a> = RayIntersection<'a, WorldSpace>;
