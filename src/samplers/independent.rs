use {
  super::*,
  crate::math::*,
  rand::{distributions, rngs::StdRng, Rng, SeedableRng}
};

#[derive(Debug)]
pub struct IndependentSampler {
  rng: StdRng
}

impl IndependentSampler {
  pub fn new() -> Self { Self { rng: StdRng::from_seed(rand::thread_rng().gen()) } }
}

impl Sampler for IndependentSampler {
  fn next(&mut self) -> Float { self.rng.sample(distributions::Uniform::new_inclusive(0.0, 1.0)) }

  fn next_non_zero(&mut self) -> Float { self.rng.sample(distributions::OpenClosed01) }

  fn next_non_one(&mut self) -> Float { self.rng.sample(distributions::Standard) }
}
