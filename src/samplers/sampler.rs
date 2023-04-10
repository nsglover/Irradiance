use {crate::math::*, std::fmt::Debug};

pub trait Sampler: Debug {
  fn next(&mut self) -> Float;

  fn next_non_zero(&mut self) -> Float;

  fn next_non_one(&mut self) -> Float;

  fn random_in_open_closed(&mut self, inf: Float, max: Float) -> Float {
    inf + (max - inf) * self.next_non_zero()
  }
}
