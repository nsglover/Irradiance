use std::sync::Arc;

use serde_derive::Deserialize;

use super::{Light, LightParameters};
use crate::{
  math::*,
  raytracing::WorldSurfacePoint,
  sampling::{uniform_random_on_unit_sphere, ContinuousRandomVariable, Sampler},
  spectrum::Spectrum,
  textures::{Texture, TextureParameters}
};

#[derive(Debug, Deserialize)]
struct DiffuseLightParameters {
  name: String,
  emitted_radiance: Box<dyn TextureParameters>
}

#[typetag::deserialize(name = "diffuse light")]
impl LightParameters for DiffuseLightParameters {
  fn name(&self) -> String { self.name.clone() }

  fn build_light(&self) -> Arc<dyn Light> { Arc::new(DiffuseLight { emitted: self.emitted_radiance.build_texture() }) }
}

#[derive(Debug)]
pub struct DiffuseLight {
  emitted: Arc<dyn Texture>
}

impl Light for DiffuseLight {
  fn radiance_emitted(&self, emit_point: &WorldSurfacePoint, _: &WorldUnitVector) -> Spectrum {
    self.emitted.value(&emit_point.tex_coord)
  }

  fn random_emit_direction(
    &self
  ) -> &dyn ContinuousRandomVariable<Param = WorldSurfacePoint, Sample = WorldUnitVector> {
    self
  }
}

impl ContinuousRandomVariable for DiffuseLight {
  type Param = WorldSurfacePoint;
  type Sample = WorldUnitVector;

  fn sample(&self, param: &Self::Param, sampler: &mut dyn Sampler) -> Option<Self::Sample> {
    // Note that there's probably no good reason for this to be cosine-weighted other than the convenience (and
    // performance) of the add-and-normalize sampling method.
    let random: WorldVector = uniform_random_on_unit_sphere(sampler).into();
    Some((param.shading_normal.into_vector() + random).normalize())
  }

  fn pdf(&self, param: &Self::Param, sample: &Self::Sample) -> Option<PositiveReal> {
    PositiveReal::new(sample.dot(&param.shading_normal) / PI)
  }

  fn sample_with_pdf(&self, param: &Self::Param, sampler: &mut dyn Sampler) -> Option<(Self::Sample, PositiveReal)> {
    let maybe_sample = self.sample(param, sampler);
    maybe_sample.map(|sample| self.pdf(param, &sample).map(|pdf| (sample, pdf))).flatten()
  }
}
