use core::fmt::Debug;
use std::fmt::Display;

use nalgebra::{Const, ToTypenum};

use super::super::{Real, Space, UnitVector, Vector};

pub trait VectorLike<const D: usize, S: Space<D>>: Debug + Display
where
  Const<D>: ToTypenum,
  Self: Sized
{
  fn from_raw(raw: nalgebra::SVector<Real, D>) -> Self;

  fn from_array(array: [Real; D]) -> Self { Self::from_vector(Vector::from_array(array)) }

  fn from_vector(vector: Vector<D, S>) -> Self;

  fn into_vector(self) -> Vector<D, S>;

  fn dot(&self, other: &Self) -> Real;

  fn normalize(&self) -> UnitVector<D, S>;

  fn normalize_fast(&self) -> UnitVector<D, S>;

  fn normalize_with_norm(&self) -> (UnitVector<D, S>, Real);

  fn norm(&self) -> Real;

  fn norm_squared(&self) -> Real;
}
