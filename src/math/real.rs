use std::ops;

use derive_more::{Add, AddAssign, Into};

pub type Real = f64;

pub const PI: Real = std::f64::consts::PI as Real;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Add, AddAssign, Into)]
pub struct PositiveReal {
  inner: Real
}

impl PositiveReal {
  pub const ONE: Self = Self::new_unchecked(1.0);

  pub const MAX: Self = Self::new_unchecked(Real::MAX);

  pub fn new(real: Real) -> Option<Self> { (real > 0.0).then_some(Self { inner: real }) }

  pub const fn new_unchecked(real: Real) -> Self { Self { inner: real } }

  pub fn inner(&self) -> &Real { &self.inner }

  pub fn into_inner(self) -> Real { self.inner }
}

impl ops::Add<Real> for PositiveReal {
  type Output = Real;

  fn add(self, rhs: Real) -> Self::Output { self.inner + rhs }
}

impl ops::AddAssign<Real> for PositiveReal {
  fn add_assign(&mut self, rhs: Real) { self.inner += rhs }
}

impl ops::Sub for PositiveReal {
  type Output = Real;

  fn sub(self, rhs: Self) -> Self::Output { self.inner - rhs.inner }
}

impl ops::Sub<Real> for PositiveReal {
  type Output = Real;

  fn sub(self, rhs: Real) -> Self::Output { self.inner - rhs }
}

impl ops::Sub<PositiveReal> for Real {
  type Output = Real;

  fn sub(self, rhs: PositiveReal) -> Self::Output { self - rhs.inner }
}

impl ops::SubAssign<PositiveReal> for Real {
  fn sub_assign(&mut self, rhs: PositiveReal) { *self -= rhs.inner }
}

impl ops::Mul for PositiveReal {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output { Self { inner: self.inner * rhs.inner } }
}

impl ops::MulAssign for PositiveReal {
  fn mul_assign(&mut self, rhs: Self) { self.inner *= rhs.inner }
}

impl ops::Mul<PositiveReal> for Real {
  type Output = Self;

  fn mul(self, rhs: PositiveReal) -> Self::Output { self * rhs.inner }
}

impl ops::MulAssign<PositiveReal> for Real {
  fn mul_assign(&mut self, rhs: PositiveReal) { *self *= rhs.inner }
}

impl ops::Div for PositiveReal {
  type Output = Self;

  fn div(self, rhs: Self) -> Self::Output { Self { inner: self.inner / rhs.inner } }
}

impl ops::DivAssign for PositiveReal {
  fn div_assign(&mut self, rhs: Self) { self.inner /= rhs.inner }
}

impl ops::Div<PositiveReal> for Real {
  type Output = Real;

  fn div(self, rhs: PositiveReal) -> Self::Output { self / rhs.inner }
}
