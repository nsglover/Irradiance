use std::ops;

use nalgebra as na;
use nalgebra::{Const, ToTypenum};

use super::super::{phantom::Phantom, *};
use crate::common::Wrapper;

#[derive(Debug, Clone, Copy)]
pub struct UnitVector<const D: usize, S: Space<D>>
where Const<D>: ToTypenum
{
  pub(in crate::math) inner: na::Unit<na::SVector<Real, D>>,
  pub(super) _phantom: Phantom<S>
}

impl<const D: usize, S: Space<D>> UnitVector<D, S>
where Const<D>: ToTypenum
{
  pub fn reflect_about(&self, normal: UnitVector<D, S>) -> Self {
    let d = self.inner().into_inner();
    let n = normal.inner().into_inner();
    let mut r = na::Unit::new_unchecked(n * 2.0 * d.dot(&n) - d);
    r.renormalize_fast();
    r.into()
  }
}

impl<const D: usize, S: Space<D>> Wrapper<na::Unit<na::SVector<Real, D>>> for UnitVector<D, S>
where Const<D>: ToTypenum
{
  fn inner(&self) -> &na::Unit<na::SVector<Real, D>> { &self.inner }
}

impl<const D: usize, S: Space<D>> VectorLike<D, S> for UnitVector<D, S>
where Const<D>: ToTypenum
{
  fn from_raw(raw: na::SVector<Real, D>) -> Self { na::Unit::new_normalize(raw).into() }

  fn from_vector(vector: Vector<D, S>) -> Self { vector.normalize() }

  fn into_vector(self) -> Vector<D, S> { self.inner.into_inner().into() }

  fn dot(&self, other: &Self) -> Real { self.inner.dot(&other.inner) }

  fn normalize(&self) -> UnitVector<D, S> { *self }

  fn normalize_fast(&self) -> UnitVector<D, S> { *self }

  fn normalize_with_norm(&self) -> (UnitVector<D, S>, Real) { (*self, 1.0) }

  fn norm(&self) -> Real { 1.0 }

  fn norm_squared(&self) -> Real { 1.0 }
}

impl<const D: usize, S: Space<D>> std::fmt::Display for UnitVector<D, S>
where Const<D>: ToTypenum
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.inner.into_inner())
  }
}

impl<const D: usize, S: Space<D>> From<na::Unit<na::SVector<Real, D>>> for UnitVector<D, S>
where Const<D>: ToTypenum
{
  fn from(raw: na::Unit<na::SVector<Real, D>>) -> Self {
    Self { inner: raw, _phantom: Phantom::default() }
  }
}

impl<const D: usize, S: Space<D>> From<UnitVector<D, S>> for na::Unit<na::SVector<Real, D>>
where Const<D>: ToTypenum
{
  fn from(val: UnitVector<D, S>) -> Self { val.inner }
}

impl<const D: usize, S: Space<D>> ops::Mul<Real> for UnitVector<D, S>
where Const<D>: ToTypenum
{
  type Output = Vector<D, S>;

  fn mul(self, rhs: Real) -> Self::Output { Vector::from_raw(self.inner.into_inner() * rhs) }
}

impl<const D: usize, S: Space<D>> ops::Neg for UnitVector<D, S>
where Const<D>: ToTypenum
{
  type Output = Self;

  fn neg(self) -> Self::Output { (-self.inner).into() }
}

pub type UnitVector3<S> = UnitVector<3, S>;
pub type WorldUnitVector = UnitVector3<WorldSpace>;
