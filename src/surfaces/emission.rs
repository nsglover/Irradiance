use std::sync::Arc;

use super::Surface;
use crate::{
  light::Color,
  math::{PositiveReal, Real},
  raytracing::WorldRay,
  sampling::{ContinuousRandomVariable, Sampler}
};

#[derive(Debug)]
pub struct UniformChoiceEmittedRayRandomVariable {
  inverse_num_surfaces: PositiveReal,
  surfaces: Vec<Arc<dyn Surface>>
}

impl UniformChoiceEmittedRayRandomVariable {
  pub fn new(surfaces: Vec<Arc<dyn Surface>>) -> Self {
    Self {
      inverse_num_surfaces: PositiveReal::new_unchecked(1.0 / (surfaces.len() as Real)),
      surfaces: surfaces.into_iter().map(|s| (s as Arc<dyn Surface>)).collect()
    }
  }

  pub fn num_surfaces(&self) -> usize { self.surfaces.len() }
}

impl ContinuousRandomVariable for UniformChoiceEmittedRayRandomVariable {
  type Param = ();
  type Sample = (WorldRay, Color);

  fn sample(&self, _: &Self::Param, sampler: &mut dyn Sampler) -> Option<Self::Sample> {
    let index = sampler.random_in_closed_open(0.0, self.surfaces.len() as Real) as usize;
    let surface = &self.surfaces[index];
    surface.emitted_ray_random_variable().sample(&(), sampler)
  }

  fn sample_with_pdf(&self, _: &Self::Param, sampler: &mut dyn Sampler) -> Option<(Self::Sample, PositiveReal)> {
    if let Some(sample) = self.sample(&(), sampler) {
      if let Some(pdf) = self.pdf(&(), &sample) {
        return Some((sample, pdf));
      }
    }

    None
  }

  fn pdf(&self, _: &Self::Param, sample: &Self::Sample) -> Option<PositiveReal> {
    PositiveReal::new(
      self
        .surfaces
        .iter()
        .filter_map(|s| s.emitted_ray_random_variable().pdf(&(), sample).map(|p| p.into_inner()))
        .sum::<Real>()
        * self.inverse_num_surfaces
    )
  }
}
