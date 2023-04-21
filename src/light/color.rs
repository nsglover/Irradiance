use {
  crate::{common::Wrapper, math::*},
  derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign},
  nalgebra as na,
  serde::Deserialize
};

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(untagged)]
pub enum ColorParameters {
  Array([Float; 3]),
  Single(Float)
}

impl ColorParameters {
  pub fn build_color(self) -> Color {
    match self {
      ColorParameters::Array([r, g, b]) => Color::new(r, g, b),
      ColorParameters::Single(c) => Color::new(c, c, c)
    }
  }
}

#[derive(Debug, Clone, Copy, Add, AddAssign, Mul, MulAssign, Div, DivAssign)]
pub struct Color {
  pub inner: na::Vector3<Float>
}

impl Color {
  pub fn new(r: Float, g: Float, b: Float) -> Self { Self { inner: na::Vector3::new(r, g, b) } }

  pub fn black() -> Self { na::Vector3::new(0.0, 0.0, 0.0).into() }

  pub fn white() -> Self { na::Vector3::new(1.0, 1.0, 1.0).into() }

  pub fn red() -> Self { na::Vector3::new(1.0, 0.0, 0.0).into() }

  pub fn green() -> Self { na::Vector3::new(0.0, 1.0, 0.0).into() }

  pub fn blue() -> Self { na::Vector3::new(0.0, 0.0, 1.0).into() }

  pub fn cyan() -> Self { na::Vector3::new(0.0, 1.0, 1.0).into() }

  pub fn magenta() -> Self { na::Vector3::new(1.0, 0.0, 1.0).into() }

  pub fn yellow() -> Self { na::Vector3::new(1.0, 1.0, 0.0).into() }

  pub fn r(&self) -> Float { self.inner.x }

  pub fn g(&self) -> Float { self.inner.y }

  pub fn b(&self) -> Float { self.inner.z }

  pub fn luminance(&self) -> Float {
    self.inner.x * 0.212671 + self.inner.y * 0.715160 + self.inner.z * 0.072169
  }

  pub fn bytes(&self) -> na::Vector3<u8> {
    let r = na::clamp(self.inner.x * 255.0, 0.0, 255.0);
    let g = na::clamp(self.inner.y * 255.0, 0.0, 255.0);
    let b = na::clamp(self.inner.z * 255.0, 0.0, 255.0);

    na::Vector3::new(r as u8, g as u8, b as u8)
  }
}

impl std::ops::Mul for Color {
  type Output = Color;

  fn mul(self, rhs: Self) -> Self::Output { self.inner.component_mul(&rhs.inner).into() }
}

impl std::ops::MulAssign for Color {
  fn mul_assign(&mut self, rhs: Self) { self.inner.component_mul_assign(&rhs.inner) }
}

impl From<na::Vector3<Float>> for Color {
  fn from(raw: na::Vector3<Float>) -> Self { Self { inner: raw } }
}

impl From<Color> for na::Vector3<Float> {
  fn from(val: Color) -> Self { val.inner }
}

impl Wrapper<na::Vector3<Float>> for Color {
  fn raw(&self) -> &na::Vector3<Float> { &self.inner }
}
