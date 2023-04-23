use std::ops;

use derive_more::From;
use nalgebra as na;
use nalgebra::{Const, ToTypenum};

use super::super::{phantom::Phantom, *};
use crate::common::Wrapper;

#[derive(Debug, Clone, Copy, From)]
pub struct Point<const D: usize, S: Space<D>>
where Const<D>: ToTypenum
{
  pub(in crate::math) inner: na::Point<Real, D>,
  _phantom: Phantom<S>
}

impl<const D: usize, S: Space<D>> Point<D, S>
where Const<D>: ToTypenum
{
  pub fn origin() -> Self { Self::from(na::Point::origin()) }
}

impl<const D: usize, S: Space<D>> From<na::Point<Real, D>> for Point<D, S>
where Const<D>: ToTypenum
{
  fn from(inner: na::Point<Real, D>) -> Self { Self { inner, _phantom: Phantom::default() } }
}

impl<const D: usize, S: Space<D>> From<Point<D, S>> for na::Point<Real, D>
where Const<D>: ToTypenum
{
  fn from(value: Point<D, S>) -> Self { value.inner }
}

impl<const D: usize, S: Space<D>> Wrapper<na::Point<Real, D>> for Point<D, S>
where Const<D>: ToTypenum
{
  fn from_inner(inner: na::Point<Real, D>) -> Self { Self { inner, _phantom: Phantom::default() } }

  fn into_inner(self) -> na::Point<Real, D> { self.inner }

  fn inner(&self) -> &na::Point<Real, D> { &self.inner }
}

impl<const D: usize, S: Space<D>> VectorLike<D, S> for Point<D, S>
where Const<D>: ToTypenum
{
  fn from_raw(raw: na::SVector<Real, D>) -> Self { Self::from_vector(raw.into()) }

  fn from_vector(vector: Vector<D, S>) -> Self { Self::from_inner(vector.inner.into()) }

  fn into_vector(self) -> Vector<D, S> { Vector::from_inner(self.inner.coords) }

  fn dot(&self, other: &Self) -> Real { self.inner.coords.dot(&other.inner.coords) }

  fn normalize(&self) -> UnitVector<D, S> { UnitVector::from_raw(self.inner.coords) }

  fn normalize_fast(&self) -> UnitVector<D, S> {
    let mut unit = na::Unit::new_unchecked(self.inner.coords);
    unit.renormalize_fast();
    unit.into()
  }

  fn normalize_with_norm(&self) -> (UnitVector<D, S>, Real) {
    let (unit, norm) = na::Unit::new_and_get(self.inner.coords);
    (unit.into(), norm)
  }

  fn norm(&self) -> Real { self.inner.coords.norm() }

  fn norm_squared(&self) -> Real { self.inner.coords.norm_squared() }
}

impl<const D: usize, S: Space<D>> std::fmt::Display for Point<D, S>
where Const<D>: ToTypenum
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write! {f, "{}", self.inner}
  }
}

impl<const D: usize, S: Space<D>> ops::Add<Vector<D, S>> for Point<D, S>
where Const<D>: ToTypenum
{
  type Output = Point<D, S>;

  fn add(self, rhs: Vector<D, S>) -> Self::Output {
    Self { inner: self.inner + rhs.inner, _phantom: self._phantom }
  }
}

impl<const D: usize, S: Space<D>> ops::Sub<Vector<D, S>> for Point<D, S>
where Const<D>: ToTypenum
{
  type Output = Point<D, S>;

  fn sub(self, rhs: Vector<D, S>) -> Self::Output {
    Self { inner: self.inner - rhs.inner, _phantom: self._phantom }
  }
}

impl<const D: usize, S: Space<D>> ops::Sub<Point<D, S>> for Point<D, S>
where Const<D>: ToTypenum
{
  type Output = Vector<D, S>;

  fn sub(self, rhs: Point<D, S>) -> Self::Output { Vector::from_raw(self.inner - rhs.inner) }
}

impl<const D: usize, S: Space<D>> ops::Mul<Real> for Point<D, S>
where Const<D>: ToTypenum
{
  type Output = Point<D, S>;

  fn mul(self, rhs: Real) -> Self::Output {
    Self { inner: self.inner * rhs, _phantom: self._phantom }
  }
}

impl<const D: usize, S: Space<D>> ops::Div<Real> for Point<D, S>
where Const<D>: ToTypenum
{
  type Output = Point<D, S>;

  fn div(self, rhs: Real) -> Self::Output {
    Self { inner: self.inner / rhs, _phantom: self._phantom }
  }
}

impl<const D: usize, S: Space<D>> ops::Index<usize> for Point<D, S>
where Const<D>: ToTypenum
{
  type Output = Real;

  fn index(&self, index: usize) -> &Self::Output { &self.inner[index] }
}

impl<const D: usize, S: Space<D>> ops::IndexMut<usize> for Point<D, S>
where Const<D>: ToTypenum
{
  fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.inner[index] }
}

pub type Point3<S> = Point<3, S>;
pub type WorldPoint = Point3<WorldSpace>;
