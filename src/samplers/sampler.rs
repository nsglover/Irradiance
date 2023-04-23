use std::fmt::Debug;

use crate::math::*;

pub trait Sampler: Debug {
  fn next(&mut self) -> PositiveReal;

  fn next_non_zero(&mut self) -> PositiveReal;

  fn next_non_one(&mut self) -> PositiveReal;

  fn next_interior(&mut self) -> PositiveReal;

  fn random_in_open_closed(&mut self, inf: Real, max: Real) -> Real {
    inf + self.next_non_zero().into_inner() * (max - inf)
  }

  fn random_in_closed_open(&mut self, min: Real, sup: Real) -> Real {
    min + self.next_non_one().into_inner() * (sup - min)
  }

  fn random_in_closed(&mut self, min: Real, max: Real) -> Real {
    min + self.next().into_inner() * (max - min)
  }

  fn random_in_open(&mut self, inf: Real, sup: Real) -> Real {
    inf + self.next_interior().into_inner() * (sup - inf)
  }
}
