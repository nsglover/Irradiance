use std::{error::Error, fmt::Debug, rc::Rc};

use crate::{
  math::{Real, WorldPoint, WorldUnitVector},
  raytracing::*,
  samplers::Sampler,
  surfaces::Surface
};

#[typetag::deserialize(tag = "type")]
pub trait SurfaceGroupParameters: Debug {
  fn build_surface_group(
    &self,
    surfaces: Vec<Box<dyn Surface>>
  ) -> Result<Rc<dyn SurfaceGroup>, Box<dyn Error>>;
}

pub trait SurfaceGroup: Debug {
  fn num_surfaces(&self) -> usize;

  fn intersect_world_ray(&self, ray: WorldRay) -> Option<WorldRayIntersection>;

  fn sample(&self, point: &WorldPoint, sampler: &mut dyn Sampler) -> WorldUnitVector {
    self.sample_and_pdf(point, sampler).0
  }

  fn pdf(&self, point: &WorldPoint, direction: &WorldUnitVector) -> Real;

  fn sample_and_pdf(
    &self,
    point: &WorldPoint,
    sampler: &mut dyn Sampler
  ) -> (WorldUnitVector, Real);
}
