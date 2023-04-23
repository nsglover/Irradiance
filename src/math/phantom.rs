use std::{marker::PhantomData, ops};

use super::Real;

#[derive(Clone, Copy)]
pub struct Phantom<S> {
  _phantom: PhantomData<S>
}

impl<S> std::fmt::Debug for Phantom<S> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "!") }
}

impl<S> Phantom<S> {
  pub fn into_other<T>(self) -> Phantom<T> { Phantom::default() }
}

impl<S> Default for Phantom<S> {
  fn default() -> Self { Self { _phantom: Default::default() } }
}

impl<S> ops::Add for Phantom<S> {
  type Output = Self;

  fn add(self, _: Self) -> Self::Output { self }
}

impl<S> ops::Sub for Phantom<S> {
  type Output = Self;

  fn sub(self, _: Self) -> Self::Output { self }
}

impl<S> ops::Mul for Phantom<S> {
  type Output = Self;

  fn mul(self, _: Self) -> Self::Output { self }
}

impl<S> ops::Div for Phantom<S> {
  type Output = Self;

  fn div(self, _: Self) -> Self::Output { self }
}

impl<S> ops::Mul<Real> for Phantom<S> {
  type Output = Self;

  fn mul(self, _: Real) -> Self::Output { self }
}

impl<S> ops::Div<Real> for Phantom<S> {
  type Output = Self;

  fn div(self, _: Real) -> Self::Output { self }
}

impl<S> ops::AddAssign for Phantom<S> {
  fn add_assign(&mut self, _: Self) {}
}

impl<S> ops::SubAssign for Phantom<S> {
  fn sub_assign(&mut self, _: Self) {}
}

impl<S> ops::MulAssign for Phantom<S> {
  fn mul_assign(&mut self, _: Self) {}
}

impl<S> ops::DivAssign for Phantom<S> {
  fn div_assign(&mut self, _: Self) {}
}

impl<S> ops::MulAssign<Real> for Phantom<S> {
  fn mul_assign(&mut self, _: Real) {}
}

impl<S> ops::DivAssign<Real> for Phantom<S> {
  fn div_assign(&mut self, _: Real) {}
}
