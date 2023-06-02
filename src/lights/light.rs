use std::{fmt::Debug, sync::Arc};

use crate::{
  math::WorldUnitVector, raytracing::WorldSurfacePoint, sampling::ContinuousRandomVariable, spectrum::Spectrum
};

#[typetag::deserialize(tag = "type")]
pub trait LightParameters: Debug {
  fn name(&self) -> String;

  fn build_light(&self) -> Arc<dyn Light>;
}

pub trait Light: Debug {
  fn radiance_emitted(&self, emit_point: &WorldSurfacePoint, emit_dir: &WorldUnitVector) -> Spectrum;

  fn random_emit_direction(&self)
    -> &dyn ContinuousRandomVariable<Param = WorldSurfacePoint, Sample = WorldUnitVector>;
}
