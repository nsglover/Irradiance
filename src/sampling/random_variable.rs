use std::fmt::Debug;

use super::Sampler;
use crate::math::PositiveReal;

#[derive(Debug)]
pub enum RandomVariable<P, S> {
  Diffuse(Box<dyn ContinuousRandomVariable<Param = P, Sample = S>>),
  Specular(Box<dyn DiscreteRandomVariable<Param = P, Sample = S>>)
}

pub trait ContinuousRandomVariable: Debug {
  type Param;
  type Sample;

  fn sample(&self, param: &Self::Param, sampler: &mut dyn Sampler) -> Option<Self::Sample> {
    self.sample_with_pdf(param, sampler).map(|(sample, _)| sample)
  }

  fn sample_with_pdf(&self, param: &Self::Param, sampler: &mut dyn Sampler) -> Option<(Self::Sample, PositiveReal)>;

  fn pdf(&self, param: &Self::Param, sample: &Self::Sample) -> Option<PositiveReal>;
}

pub trait DiscreteRandomVariable: Debug {
  type Param;
  type Sample;

  fn sample(&self, param: &Self::Param, sampler: &mut dyn Sampler) -> Option<Self::Sample>;
}
