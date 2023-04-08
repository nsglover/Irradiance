use {
  super::*,
  crate::math::*,
  rand::{rngs::ThreadRng, *}
};

#[derive(Debug)]
pub struct IndependentSampler {
  rng: ThreadRng
}

impl IndependentSampler {
  pub fn new() -> Self { Self { rng: thread_rng() } }
}

impl Sampler for IndependentSampler {
  // fn next(&mut self) -> Float { (self.next_non_one() + self.next_non_zero()) / 2.0 }

  fn next(&mut self) -> Float { self.rng.sample(distributions::Uniform::new_inclusive(0.0, 1.0)) }

  fn next_non_zero(&mut self) -> Float { self.rng.sample(distributions::OpenClosed01) }

  fn next_non_one(&mut self) -> Float { self.rng.sample(distributions::Standard) }
}
