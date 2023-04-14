use {
  super::{phantom::*, *},
  crate::wrapper::*,
  derive_more::*,
  nalgebra as na,
  std::ops
};

/* #region Point */

#[derive(Debug, Clone, Copy)]
pub struct Point<const D: usize, S: Space<D>>
where na::Const<D>: na::ToTypenum
{
  pub(super) inner: na::Point<Float, D>,
  _phantom: Phantom<S>
}

impl<const D: usize, S: Space<D>> Point<D, S>
where na::Const<D>: na::ToTypenum
{
  pub fn origin() -> Self { From::from(na::Point::origin()) }

  pub fn inner(&self) -> na::Point<Float, D> { self.inner }
}

impl<const D: usize, S: Space<D>> std::fmt::Display for Point<D, S>
where na::Const<D>: na::ToTypenum
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write! {f, "{}", self.inner}
  }
}

impl<const D: usize, S: Space<D>> From<Vector<D, S>> for Point<D, S>
where na::Const<D>: na::ToTypenum
{
  fn from(v: Vector<D, S>) -> Self { Self { inner: v.inner.into(), _phantom: v._phantom } }
}

impl<const D: usize, S: Space<D>> From<na::Point<Float, D>> for Point<D, S>
where na::Const<D>: na::ToTypenum
{
  fn from(raw: na::Point<Float, D>) -> Self { Self { inner: raw, _phantom: Phantom::default() } }
}

impl<const D: usize, S: Space<D>> From<Point<D, S>> for na::Point<Float, D>
where na::Const<D>: na::ToTypenum
{
  fn from(val: Point<D, S>) -> Self { val.inner }
}

impl<const D: usize, S: Space<D>> Wrapper<na::Point<Float, D>> for Point<D, S>
where na::Const<D>: na::ToTypenum
{
  fn raw(&self) -> &na::Point<Float, D> { &self.inner }
}

impl<const D: usize, S: Space<D>> ops::Add<Vector<D, S>> for Point<D, S>
where na::Const<D>: na::ToTypenum
{
  type Output = Point<D, S>;

  fn add(self, rhs: Vector<D, S>) -> Self::Output {
    Self { inner: self.inner + rhs.inner, _phantom: self._phantom }
  }
}

impl<const D: usize, S: Space<D>> ops::Sub<Vector<D, S>> for Point<D, S>
where na::Const<D>: na::ToTypenum
{
  type Output = Point<D, S>;

  fn sub(self, rhs: Vector<D, S>) -> Self::Output {
    Self { inner: self.inner - rhs.inner, _phantom: self._phantom }
  }
}

impl<const D: usize, S: Space<D>> ops::Sub<Point<D, S>> for Point<D, S>
where na::Const<D>: na::ToTypenum
{
  type Output = Vector<D, S>;

  fn sub(self, rhs: Point<D, S>) -> Self::Output {
    Vector { inner: self.inner - rhs.inner, _phantom: self._phantom }
  }
}

impl<const D: usize, S: Space<D>> ops::Mul<Float> for Point<D, S>
where na::Const<D>: na::ToTypenum
{
  type Output = Point<D, S>;

  fn mul(self, rhs: Float) -> Self::Output {
    Self { inner: self.inner * rhs, _phantom: self._phantom }
  }
}

impl<const D: usize, S: Space<D>> ops::Div<Float> for Point<D, S>
where na::Const<D>: na::ToTypenum
{
  type Output = Point<D, S>;

  fn div(self, rhs: Float) -> Self::Output {
    Self { inner: self.inner / rhs, _phantom: self._phantom }
  }
}

impl<const D: usize, S: Space<D>> ops::Index<usize> for Point<D, S>
where na::Const<D>: na::ToTypenum
{
  type Output = Float;

  fn index(&self, index: usize) -> &Self::Output { &self.inner[index] }
}

impl<const D: usize, S: Space<D>> ops::IndexMut<usize> for Point<D, S>
where na::Const<D>: na::ToTypenum
{
  fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.inner[index] }
}

/* #endregion */

/* #region Vector */

#[derive(Debug, Clone, Copy, Mul, MulAssign, Div, DivAssign)]
pub struct Vector<const D: usize, S: Space<D>>
where na::Const<D>: na::ToTypenum
{
  pub(super) inner: na::SVector<Float, D>,
  _phantom: Phantom<S>
}

impl<const D: usize, S: Space<D>> Vector<D, S>
where na::Const<D>: na::ToTypenum
{
  pub fn from_array(array: [Float; D]) -> Self {
    let mut s = Self::zero();
    for i in 0..D {
      s[i] = array[i];
    }

    s
  }

  pub fn zero() -> Self { na::SVector::<Float, D>::zeros().into() }

  pub fn normalize(&self) -> Direction<D, S> {
    Direction { inner: na::Unit::new_normalize(self.inner), _phantom: self._phantom }
  }

  pub fn norm(&self) -> Float { self.inner.norm() }

  pub fn norm_squared(&self) -> Float { self.inner.norm_squared() }

  pub fn dot(&self, other: &Self) -> Float { self.inner.dot(&other.inner) }

  pub fn inner(&self) -> na::SVector<Float, D> { self.inner }
}

impl<const D: usize, S: Space<D>> std::ops::Add for Vector<D, S>
where na::Const<D>: na::ToTypenum
{
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self { inner: self.inner + rhs.inner, _phantom: self._phantom }
  }
}

impl<const D: usize, S: Space<D>> std::ops::AddAssign for Vector<D, S>
where na::Const<D>: na::ToTypenum
{
  fn add_assign(&mut self, rhs: Self) { self.inner += rhs.inner }
}

impl<const D: usize, S: Space<D>> std::ops::Sub for Vector<D, S>
where na::Const<D>: na::ToTypenum
{
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self { inner: self.inner - rhs.inner, _phantom: self._phantom }
  }
}

impl<const D: usize, S: Space<D>> std::ops::SubAssign for Vector<D, S>
where na::Const<D>: na::ToTypenum
{
  fn sub_assign(&mut self, rhs: Self) { self.inner -= rhs.inner }
}

impl<const D: usize, S: Space<D>> std::fmt::Display for Vector<D, S>
where na::Const<D>: na::ToTypenum
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write! {f, "{}", self.inner}
  }
}

impl<const D: usize, S: Space<D>> From<na::SVector<Float, D>> for Vector<D, S>
where na::Const<D>: na::ToTypenum
{
  fn from(raw: na::SVector<Float, D>) -> Self { Self { inner: raw, _phantom: Phantom::default() } }
}

impl<const D: usize, S: Space<D>> From<Vector<D, S>> for na::SVector<Float, D>
where na::Const<D>: na::ToTypenum
{
  fn from(val: Vector<D, S>) -> Self { val.inner }
}

impl<const D: usize, S: Space<D>> Wrapper<na::SVector<Float, D>> for Vector<D, S>
where na::Const<D>: na::ToTypenum
{
  fn raw(&self) -> &na::SVector<Float, D> { &self.inner }
}

impl<const D: usize, S: Space<D>> From<Point<D, S>> for Vector<D, S>
where na::Const<D>: na::ToTypenum
{
  fn from(value: Point<D, S>) -> Self { value.inner.coords.into() }
}

impl<const D: usize, S: Space<D>> From<Direction<D, S>> for Vector<D, S>
where na::Const<D>: na::ToTypenum
{
  fn from(value: Direction<D, S>) -> Self { value.inner.into_inner().into() }
}

impl<const D: usize, S: Space<D>> ops::Index<usize> for Vector<D, S>
where na::Const<D>: na::ToTypenum
{
  type Output = Float;

  fn index(&self, index: usize) -> &Self::Output { &self.inner[index] }
}

impl<const D: usize, S: Space<D>> ops::IndexMut<usize> for Vector<D, S>
where na::Const<D>: na::ToTypenum
{
  fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.inner[index] }
}

/* #endregion */

/* #region Direction */

#[derive(Debug, Clone, Copy)]
pub struct Direction<const D: usize, S: Space<D>>
where na::Const<D>: na::ToTypenum
{
  pub(super) inner: na::Unit<na::SVector<Float, D>>,
  _phantom: Phantom<S>
}

impl<const D: usize, S: Space<D>> Direction<D, S>
where na::Const<D>: na::ToTypenum
{
  pub fn dot(&self, other: &Self) -> Float { self.inner.dot(&other.inner) }

  pub fn inner(&self) -> na::Unit<na::SVector<Float, D>> { self.inner }
}

impl<const D: usize, S: Space<D>> std::fmt::Display for Direction<D, S>
where na::Const<D>: na::ToTypenum
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.inner.into_inner())
  }
}

impl<const D: usize, S: Space<D>> From<na::Unit<na::SVector<Float, D>>> for Direction<D, S>
where na::Const<D>: na::ToTypenum
{
  fn from(raw: na::Unit<na::SVector<Float, D>>) -> Self {
    Self { inner: raw, _phantom: Phantom::default() }
  }
}

impl<const D: usize, S: Space<D>> From<Direction<D, S>> for na::Unit<na::SVector<Float, D>>
where na::Const<D>: na::ToTypenum
{
  fn from(val: Direction<D, S>) -> Self { val.inner }
}

impl<const D: usize, S: Space<D>> Wrapper<na::Unit<na::SVector<Float, D>>> for Direction<D, S>
where na::Const<D>: na::ToTypenum
{
  fn raw(&self) -> &na::Unit<na::SVector<Float, D>> { &self.inner }
}

impl<const D: usize, S: Space<D>> ops::Mul<Float> for Direction<D, S>
where na::Const<D>: na::ToTypenum
{
  type Output = Vector<D, S>;

  fn mul(self, rhs: Float) -> Self::Output {
    Vector { inner: self.inner.into_inner() * rhs, _phantom: self._phantom }
  }
}

/* #endregion */

pub type Vector2 = Vector<2, EuclideanSpace<2>>;

pub type Point3<S> = Point<3, S>;

pub type Vector3<S> = Vector<3, S>;

pub type Direction3<S> = Direction<3, S>;

/* #region World Space Structures */

// pub type WorldPoint = Point3<WorldSpace>;

pub type WorldVector = Vector3<WorldSpace>;

// pub type WorldDirection = Direction3<WorldSpace>;

/* #endregion */
