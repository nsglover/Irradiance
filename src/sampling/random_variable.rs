use std::fmt::Debug;

use super::Sampler;
use crate::math::PositiveReal;

#[derive(Debug)]
pub enum RandomVariable<P, S> {
  Continuous(Box<dyn ContinuousRandomVariable<P, S>>),
  Discrete(Box<dyn DiscreteRandomVariable<P, S>>)
}

pub trait ContinuousRandomVariable<P, S>: Debug {
  fn sample(&self, param: &P, sampler: &mut dyn Sampler) -> Option<S> {
    self.sample_with_pdf(param, sampler).map(|(s, _)| s)
  }

  fn sample_with_pdf(&self, param: &P, sampler: &mut dyn Sampler) -> Option<(S, PositiveReal)>;

  fn pdf(&self, param: &P, sample: &S) -> Option<PositiveReal>;
}

pub trait DiscreteRandomVariable<P, S>: Debug {
  fn sample(&self, param: &P, sampler: &mut dyn Sampler) -> Option<S>;
}
