use {
  crate::math::*,
  nalgebra::{Const, ToTypenum},
  std::fmt::Display
};

#[derive(Debug, Clone)]
pub struct Ray<const D: usize, S: Space<D>>
where Const<D>: ToTypenum
{
  pub(super) max_intersect_time: Float,
  pub(super) origin: Point<D, S>,
  pub(super) dir: Direction<D, S>
}

const MIN_INTERSECT_TIME: Float = Float::EPSILON * 16.0;

impl<const D: usize, S: Space<D>> Ray<D, S>
where Const<D>: ToTypenum
{
  pub fn new(origin: Point<D, S>, dir: Direction<D, S>) -> Self {
    Self { max_intersect_time: Float::MAX, origin, dir }
  }

  fn at_unchecked(&self, t: Float) -> Point<D, S> { self.origin + self.dir * t }

  pub fn at(&self, t: Float) -> Option<Point<D, S>> {
    if MIN_INTERSECT_TIME <= t && t <= self.max_intersect_time {
      Some(self.at_unchecked(t))
    } else {
      None
    }
  }

  pub fn origin(&self) -> Point<D, S> { self.origin }

  pub fn dir(&self) -> Direction<D, S> { self.dir }

  pub fn time_bounds(&self) -> (Float, Float) { (MIN_INTERSECT_TIME, self.max_intersect_time) }

  pub fn set_max_intersect_time(&mut self, max_time: Float) { self.max_intersect_time = max_time; }
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
