use {crate::math::*, nalgebra as na};

pub struct Ray<const D: usize, S: Space<D>>
where na::Const<D>: na::ToTypenum
{
  pub time_bounds: (Float, Float),
  pub origin: Point<D, S>,
  pub dir: Direction<D, S>
}

impl<const D: usize, S: Space<D>> Ray<D, S>
where na::Const<D>: na::ToTypenum
{
  pub fn at_unchecked(&self, t: Float) -> Point<D, S> { self.origin + self.dir * t }

  pub fn at(&self, t: Float) -> Option<Point<D, S>> {
    if self.time_bounds.0 <= t && t <= self.time_bounds.1 {
      Some(self.at_unchecked(t))
    } else {
      None
    }
  }
}

pub type Ray3<S> = Ray<3, S>;

pub type WorldRay = Ray3<WorldSpace>;
