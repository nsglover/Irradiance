use std::ops;

use derive_more::*;
use nalgebra as na;

use super::super::{phantom::*, *};
use crate::common::Wrapper;

#[derive(Debug, Clone, Copy, Mul, MulAssign, Div, DivAssign)]
pub struct Vector<const D: usize, S: Space<D>>
where na::Const<D>: na::ToTypenum
{
  pub(in crate::math) inner: na::SVector<Real, D>,
  _phantom: Phantom<S>
}

impl<const D: usize, S: Space<D>> Vector<D, S>
where na::Const<D>: na::ToTypenum
{
  pub fn zero() -> Self { na::SVector::<Real, D>::zeros().into() }

  pub fn from_array(array: [Real; D]) -> Self {
    let mut s = Self::zero();
    for i in 0..D {
      s[i] = array[i];
    }

    s
  }

  pub fn reflect_about(&self, normal: UnitVector<D, S>) -> Self { normal * 2.0 * self.dot(&normal.into()) - *self }
}

impl<S: Space<3>> Vector<3, S> {
  pub fn cross(&self, other: &Self) -> Self { Self::from_raw(self.inner.cross(&other.inner)) }
}

impl<const D: usize, S: Space<D>> VectorLike<D, S> for Vector<D, S>
where na::Const<D>: na::ToTypenum
{
  fn from_raw(raw: na::SVector<Real, D>) -> Self { Self { inner: raw, _phantom: Phantom::default() } }

  fn from_vector(vector: Vector<D, S>) -> Self { vector }

  fn into_vector(self) -> Vector<D, S> { self }

  fn dot(&self, other: &Self) -> Real { self.inner.dot(&other.inner) }

  fn normalize(&self) -> UnitVector<D, S> { UnitVector::from_raw(self.inner) }

  fn normalize_fast(&self) -> UnitVector<D, S> {
    let mut unit = na::Unit::new_unchecked(self.inner);
    unit.renormalize_fast();
    unit.into()
  }

  fn normalize_with_norm(&self) -> (UnitVector<D, S>, Real) {
    let (unit, norm) = na::Unit::new_and_get(self.inner);
    (unit.into(), norm)
  }

  fn norm(&self) -> Real { self.inner.norm() }

  fn norm_squared(&self) -> Real { self.inner.norm_squared() }
}

impl<const D: usize, S: Space<D>> std::ops::Add for Vector<D, S>
where na::Const<D>: na::ToTypenum
{
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output { Self { inner: self.inner + rhs.inner, _phantom: self._phantom } }
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

  fn sub(self, rhs: Self) -> Self::Output { Self { inner: self.inner - rhs.inner, _phantom: self._phantom } }
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

impl<const D: usize, S: Space<D>> From<na::SVector<Real, D>> for Vector<D, S>
where na::Const<D>: na::ToTypenum
{
  fn from(raw: na::SVector<Real, D>) -> Self { Self { inner: raw, _phantom: Phantom::default() } }
}

impl<const D: usize, S: Space<D>> From<Vector<D, S>> for na::SVector<Real, D>
where na::Const<D>: na::ToTypenum
{
  fn from(val: Vector<D, S>) -> Self { val.inner }
}

impl<const D: usize, S: Space<D>> Wrapper<na::SVector<Real, D>> for Vector<D, S>
where na::Const<D>: na::ToTypenum
{
  fn inner(&self) -> &na::SVector<Real, D> { &self.inner }
}

impl<const D: usize, S: Space<D>> From<Point<D, S>> for Vector<D, S>
where na::Const<D>: na::ToTypenum
{
  fn from(value: Point<D, S>) -> Self { value.inner.coords.into() }
}

impl<const D: usize, S: Space<D>> From<UnitVector<D, S>> for Vector<D, S>
where na::Const<D>: na::ToTypenum
{
  fn from(value: UnitVector<D, S>) -> Self { value.inner.into_inner().into() }
}

impl<const D: usize, S: Space<D>> ops::Index<usize> for Vector<D, S>
where na::Const<D>: na::ToTypenum
{
  type Output = Real;

  fn index(&self, index: usize) -> &Self::Output { &self.inner[index] }
}

impl<const D: usize, S: Space<D>> ops::IndexMut<usize> for Vector<D, S>
where na::Const<D>: na::ToTypenum
{
  fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.inner[index] }
}

pub type Vector2 = Vector<2, EuclideanSpace<2>>;
pub type Vector3<S> = Vector<3, S>;
pub type WorldVector = Vector3<WorldSpace>;
