use super::Light;
use crate::{
  math::*,
  raytracing::WorldSurfacePoint,
  sampling::{ContinuousRandomVariable, Sampler},
  spectrum::Spectrum
};

#[derive(Debug)]
pub struct NullLight;

impl Default for NullLight {
  fn default() -> Self { Self }
}

impl Light for NullLight {
  fn radiance_emitted(&self, _: &WorldSurfacePoint, _: &WorldUnitVector) -> Spectrum { Spectrum::none() }

  fn random_emit_direction(
    &self
  ) -> &dyn ContinuousRandomVariable<Param = WorldSurfacePoint, Sample = WorldUnitVector> {
    self
  }
}

impl ContinuousRandomVariable for NullLight {
  type Param = WorldSurfacePoint;
  type Sample = WorldUnitVector;

  fn sample(&self, _: &Self::Param, _: &mut dyn Sampler) -> Option<Self::Sample> { None }

  fn sample_with_pdf(&self, _: &Self::Param, _: &mut dyn Sampler) -> Option<(Self::Sample, PositiveReal)> { None }

  fn pdf(&self, _: &Self::Param, _: &Self::Sample) -> Option<PositiveReal> { None }
}
