use {
  crate::{math::*, wrapper::*},
  derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign},
  nalgebra as na
};

#[derive(Add, AddAssign, Mul, MulAssign, Div, DivAssign)]
pub struct Color {
  inner: na::Vector3<Float>
}

impl Color {
  pub fn new(r: Float, g: Float, b: Float) -> Self { Self { inner: na::Vector3::new(r, g, b) } }

  pub fn black() -> Self { na::Vector3::zeros().into() }

  pub fn white() -> Self { na::Vector3::new(1.0, 1.0, 1.0).into() }

  pub fn red() -> Self { na::Vector3::new(1.0, 0.0, 0.0).into() }

  pub fn blue() -> Self { na::Vector3::new(0.0, 1.0, 0.0).into() }

  pub fn green() -> Self { na::Vector3::new(0.0, 0.0, 1.0).into() }

  pub fn cyan() -> Self { na::Vector3::new(0.0, 1.0, 1.0).into() }

  pub fn magenta() -> Self { na::Vector3::new(1.0, 1.0, 0.0).into() }

  pub fn yellow() -> Self { na::Vector3::new(1.0, 0.0, 1.0).into() }

  pub fn luminance(&self) -> Float {
    self.inner.x * 0.212671 + self.inner.y * 0.715160 + self.inner.z * 0.072169
  }
}

impl From<na::Vector3<Float>> for Color {
  fn from(raw: na::Vector3<Float>) -> Self { Self { inner: raw } }
}

impl Into<na::Vector3<Float>> for Color {
  fn into(self) -> na::Vector3<Float> { self.inner }
}

impl Wrapper<na::Vector3<Float>> for Color {
  fn raw(&self) -> &na::Vector3<Float> { &self.inner }

  // fn into_raw(self) -> na::Vector3<Float> { self.inner }

  // fn from_raw(raw: na::Vector3<Float>) -> Self { Self { inner: raw } }
}
