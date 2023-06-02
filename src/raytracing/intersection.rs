use crate::{lights::Light, materials::Material, math::*, textures::TextureCoordinate};

#[derive(Debug, Clone)]
pub struct SurfacePoint<S: Space<3>> {
  pub point: Point3<S>,
  pub geometric_normal: UnitVector3<S>,
  pub shading_normal: UnitVector3<S>,
  pub tex_coord: TextureCoordinate
}

pub type WorldSurfacePoint = SurfacePoint<WorldSpace>;

#[derive(Debug, Clone)]
pub struct SurfaceInterface<'a, S: Space<3>> {
  pub surface_point: SurfacePoint<S>,
  pub light: &'a dyn Light,
  pub material: &'a dyn Material,
  pub intersect_dist: PositiveReal // TODO: Get this outta here
}

pub type WorldSurfaceInterface<'a> = SurfaceInterface<'a, WorldSpace>;
