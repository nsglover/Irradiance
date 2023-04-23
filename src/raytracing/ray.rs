use std::fmt::Display;

use nalgebra::{Const, ToTypenum};

use crate::math::*;

#[derive(Debug, Clone)]
pub struct Ray<const D: usize, S: Space<D>>
where Const<D>: ToTypenum
{
  max_intersect_time: PositiveReal,
  origin: Point<D, S>,
  dir: UnitVector<D, S>
}

const MIN_INTERSECT_TIME: PositiveReal = PositiveReal::new_unchecked(0.00001);

impl<const D: usize, S: Space<D>> Ray<D, S>
where Const<D>: ToTypenum
{
  pub fn cast_unsafe<T: Space<D>>(&self) -> Ray<D, T> {
    Ray {
      max_intersect_time: self.max_intersect_time,
      origin: self.origin.cast_unsafe(),
      dir: self.dir.cast_unsafe()
    }
  }

  pub fn new(origin: Point<D, S>, dir: UnitVector<D, S>) -> Self {
    Self { max_intersect_time: PositiveReal::MAX, origin, dir }
  }

  pub fn new_with_time(max_time: PositiveReal, origin: Point<D, S>, dir: UnitVector<D, S>) -> Self {
    Self { max_intersect_time: max_time, origin, dir }
  }

  fn at_unchecked(&self, t: PositiveReal) -> Point<D, S> { self.origin + self.dir * t.into_inner() }

  pub fn at(&self, t: PositiveReal) -> Option<Point<D, S>> {
    if MIN_INTERSECT_TIME <= t && t <= self.max_intersect_time {
      Some(self.at_unchecked(t))
    } else {
      None
    }
  }

  pub fn at_real(&self, t: Real) -> Option<(PositiveReal, Point<D, S>)> {
    // We can use unchecked here because MIN_INTERSECT_TIME is a positive real
    let t = PositiveReal::new_unchecked(t);
    self.at(t).map(|p| (t, p))
  }

  pub fn origin(&self) -> Point<D, S> { self.origin }

  pub fn dir(&self) -> UnitVector<D, S> { self.dir }

  pub fn min_intersect_time(&self) -> PositiveReal { MIN_INTERSECT_TIME }

  pub fn max_intersect_time(&self) -> PositiveReal { self.max_intersect_time }

  pub fn time_bounds(&self) -> (PositiveReal, PositiveReal) {
    (self.min_intersect_time(), self.max_intersect_time())
  }

  pub fn set_max_intersect_time(&mut self, max_time: PositiveReal) {
    self.max_intersect_time = max_time;
  }
}

impl<const D: usize, S: Space<D>> Display for Ray<D, S>
where Const<D>: ToTypenum
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "O: {}, D: {}", self.origin, self.dir)
  }
}

pub type Ray3<S> = Ray<3, S>;

pub type WorldRay = Ray3<WorldSpace>;
