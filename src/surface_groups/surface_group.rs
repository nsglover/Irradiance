use {
  crate::{
    ray::WorldRay,
    surfaces::{Surface, WorldHitInfo}
  },
  std::{error::Error, fmt::Debug}
};

#[typetag::serde(tag = "type")]
pub trait SurfaceGroupParameters: Debug {
  fn build_surface_group(
    &self,
    surfaces: Vec<Box<dyn Surface>>
  ) -> Result<Box<dyn SurfaceGroup>, Box<dyn Error>>;
}

pub trait SurfaceGroup: Debug {
  fn intersect_world_ray(&self, ray: WorldRay) -> Option<WorldHitInfo>;
}
