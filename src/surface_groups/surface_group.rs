use {
  crate::{
    math::{Float, WorldDirection, WorldPoint},
    raytracing::*,
    samplers::Sampler,
    surfaces::Surface
  },
  std::{error::Error, fmt::Debug, rc::Rc}
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

  fn sample(&self, point: &WorldPoint, sampler: &mut dyn Sampler) -> WorldDirection {
    self.sample_and_pdf(point, sampler).0
  }

  fn pdf(&self, point: &WorldPoint, direction: &WorldDirection) -> Float;

  fn sample_and_pdf(
    &self,
    point: &WorldPoint,
    sampler: &mut dyn Sampler
  ) -> (WorldDirection, Float);
}
