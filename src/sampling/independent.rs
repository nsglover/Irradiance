use rand::{distributions, rngs::StdRng, Rng, SeedableRng};

use super::*;
use crate::math::*;

#[derive(Debug)]
pub struct IndependentSampler {
  rng: StdRng
}

impl IndependentSampler {
  pub fn new() -> Self { Self { rng: StdRng::from_seed(rand::thread_rng().gen()) } }
}

impl Sampler for IndependentSampler {
  fn next(&mut self) -> PositiveReal {
    PositiveReal::new_unchecked(self.rng.sample(distributions::Uniform::new_inclusive(0.0, 1.0)))
  }

  fn next_non_zero(&mut self) -> PositiveReal {
    PositiveReal::new_unchecked(self.rng.sample(distributions::OpenClosed01))
  }

  fn next_non_one(&mut self) -> PositiveReal { PositiveReal::new_unchecked(self.rng.sample(distributions::Standard)) }

  fn next_interior(&mut self) -> PositiveReal { PositiveReal::new_unchecked(self.rng.sample(distributions::Open01)) }

  fn random_in_closed_open(&mut self, min: Real, sup: Real) -> Real {
    self.rng.sample(distributions::Uniform::new(min, sup))
  }

  fn random_in_closed(&mut self, min: Real, max: Real) -> Real {
    self.rng.sample(distributions::Uniform::new_inclusive(min, max))
  }
}
