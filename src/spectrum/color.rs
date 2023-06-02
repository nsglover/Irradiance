use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};
use nalgebra as na;
use serde::Deserialize;

use crate::math::*;

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(untagged)]
pub enum ColorParameters {
  Array([Real; 3]),
  Single(Real)
}

impl ColorParameters {
  pub fn build_color(self) -> Spectrum {
    match self {
      ColorParameters::Array([r, g, b]) => Spectrum::new(r, g, b),
      ColorParameters::Single(c) => Spectrum::new(c, c, c)
    }
  }
}

#[derive(Debug, Clone, Copy, Add, AddAssign, Mul, MulAssign, Div, DivAssign)]
pub struct Spectrum {
  pub inner: na::Vector3<Real>
}

impl Spectrum {
  pub fn new(r: Real, g: Real, b: Real) -> Self { Self { inner: na::Vector3::new(r, g, b) } }

  pub fn none() -> Self { na::Vector3::new(0.0, 0.0, 0.0).into() }

  pub fn black() -> Self { Self::none() }

  pub fn white() -> Self { na::Vector3::new(1.0, 1.0, 1.0).into() }

  pub fn red() -> Self { na::Vector3::new(1.0, 0.0, 0.0).into() }

  pub fn green() -> Self { na::Vector3::new(0.0, 1.0, 0.0).into() }

  pub fn blue() -> Self { na::Vector3::new(0.0, 0.0, 1.0).into() }

  pub fn cyan() -> Self { na::Vector3::new(0.0, 1.0, 1.0).into() }

  pub fn magenta() -> Self { na::Vector3::new(1.0, 0.0, 1.0).into() }

  pub fn yellow() -> Self { na::Vector3::new(1.0, 1.0, 0.0).into() }

  pub fn r(&self) -> Real { self.inner.x }

  pub fn g(&self) -> Real { self.inner.y }

  pub fn b(&self) -> Real { self.inner.z }

  pub fn luminance(&self) -> Real { self.inner.x * 0.212671 + self.inner.y * 0.715160 + self.inner.z * 0.072169 }

  pub fn bytes(&self) -> na::Vector3<u8> {
    let r = na::clamp(self.inner.x * 255.0, 0.0, 255.0);
    let g = na::clamp(self.inner.y * 255.0, 0.0, 255.0);
    let b = na::clamp(self.inner.z * 255.0, 0.0, 255.0);

    na::Vector3::new(r as u8, g as u8, b as u8)
  }

  pub fn from_bytes(bytes: [u8; 3]) -> Self {
    let inv_255 = 1.0 / 255.0;
    Self { inner: na::vector![bytes[0] as Real * inv_255, bytes[1] as Real * inv_255, bytes[2] as Real * inv_255] }
  }
}

impl std::ops::Mul for Spectrum {
  type Output = Spectrum;

  fn mul(self, rhs: Self) -> Self::Output { self.inner.component_mul(&rhs.inner).into() }
}

impl std::ops::MulAssign for Spectrum {
  fn mul_assign(&mut self, rhs: Self) { self.inner.component_mul_assign(&rhs.inner) }
}

impl std::ops::Div for Spectrum {
  type Output = Spectrum;

  fn div(self, rhs: Self) -> Self::Output { self.inner.component_div(&rhs.inner).into() }
}

impl std::ops::DivAssign for Spectrum {
  fn div_assign(&mut self, rhs: Self) { self.inner.component_div_assign(&rhs.inner) }
}

impl From<na::Vector3<Real>> for Spectrum {
  fn from(raw: na::Vector3<Real>) -> Self { Self { inner: raw } }
}

impl From<Spectrum> for na::Vector3<Real> {
  fn from(val: Spectrum) -> Self { val.inner }
}
