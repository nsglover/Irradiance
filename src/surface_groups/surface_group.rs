use {
  crate::{raytracing::*, surfaces::Surface},
  std::{error::Error, fmt::Debug}
};

#[typetag::deserialize(tag = "type")]
pub trait SurfaceGroupParameters: Debug {
  fn build_surface_group(
    &self,
    surfaces: Vec<Box<dyn Surface>>
  ) -> Result<Box<dyn SurfaceGroup>, Box<dyn Error>>;
}

pub trait SurfaceGroup: Debug {
  fn num_surfaces(&self) -> usize;

  fn intersect_world_ray(&self, ray: WorldRay) -> Option<WorldRayIntersection>;
}
