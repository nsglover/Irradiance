use std::sync::Arc;

use serde::Deserialize;

use super::*;
use crate::{
  math::{PositiveReal, WorldUnitVector},
  raytracing::*,
  sampling::*,
  spectrum::Spectrum
};

#[derive(Debug, Deserialize)]
struct NullMaterialParameters {
  name: String
}

#[typetag::deserialize(name = "mirror")]
impl MaterialParameters for NullMaterialParameters {
  fn name(&self) -> String { self.name.clone() }

  fn build_material(&self) -> Arc<dyn Material> { Arc::new(NullMaterial::default()) }
}

#[derive(Debug)]
struct NullRandomVariable;

#[derive(Debug)]
pub struct NullMaterial {
  scatter_random_var: ScatterRandomVariable
}

impl Default for NullMaterial {
  fn default() -> Self { Self { scatter_random_var: ScatterRandomVariable::Diffuse(Box::new(NullRandomVariable)) } }
}

impl Material for NullMaterial {
  fn bsdf_cos(&self, _: &WorldSurfacePoint, _: &WorldUnitVector, _: &WorldUnitVector) -> Spectrum { Spectrum::none() }

  fn random_bsdf_in_direction(&self) -> &ScatterRandomVariable { &self.scatter_random_var }
}

impl ContinuousRandomVariable for NullRandomVariable {
  type Param = (WorldSurfacePoint, WorldUnitVector);
  type Sample = WorldUnitVector;

  fn sample(&self, _: &Self::Param, _: &mut dyn Sampler) -> Option<Self::Sample> { None }

  fn sample_with_pdf(&self, _: &Self::Param, _: &mut dyn Sampler) -> Option<(Self::Sample, PositiveReal)> { None }

  fn pdf(&self, _: &Self::Param, _: &Self::Sample) -> Option<PositiveReal> { None }
}
