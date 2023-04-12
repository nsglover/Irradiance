use {crate::math::*, std::fmt::Debug};

pub trait Sampler: Debug {
  fn next(&mut self) -> Float;

  fn next_non_zero(&mut self) -> Float;

  fn next_non_one(&mut self) -> Float;

  fn next_interior(&mut self) -> Float;

  fn random_in_open_closed(&mut self, inf: Float, max: Float) -> Float {
    inf + (max - inf) * self.next_non_zero()
  }

  fn random_in_closed_open(&mut self, min: Float, sup: Float) -> Float {
    min + (sup - min) * self.next_non_one()
  }

  fn random_in_closed(&mut self, min: Float, max: Float) -> Float {
    min + (max - min) * self.next()
  }

  fn random_in_open(&mut self, inf: Float, sup: Float) -> Float {
    inf + (sup - inf) * self.next_interior()
  }
}
