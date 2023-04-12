use {
  crate::math::*,
  nalgebra::{Const, ToTypenum},
  std::fmt::Display
};

#[derive(Debug, Clone)]
pub struct Ray<const D: usize, S: Space<D>>
where Const<D>: ToTypenum
{
  pub time_bounds: (Float, Float),
  pub(super) origin: Point<D, S>,
  pub(super) dir: Direction<D, S>
}

impl<const D: usize, S: Space<D>> Ray<D, S>
where Const<D>: ToTypenum
{
  pub fn new(origin: Point<D, S>, dir: Direction<D, S>) -> Self {
    Self { time_bounds: (Float::EPSILON, Float::MAX), origin, dir }
  }

  pub fn at_unchecked(&self, t: Float) -> Point<D, S> { self.origin + self.dir * t }

  pub fn at(&self, t: Float) -> Option<Point<D, S>> {
    if self.time_bounds.0 <= t && t <= self.time_bounds.1 {
      Some(self.at_unchecked(t))
    } else {
      None
    }
  }

  pub fn origin(&self) -> Point<D, S> { self.origin }

  pub fn dir(&self) -> Direction<D, S> { self.dir }
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
