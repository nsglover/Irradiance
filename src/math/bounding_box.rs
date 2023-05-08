use nalgebra::{Const, ToTypenum};

use super::*;
use crate::{common::Wrapper, raytracing::*};

#[derive(Debug, Clone)]
pub struct BoundingBox<const D: usize, S: Space<D>>
where Const<D>: ToTypenum
{
  min: Point<D, S>,
  max: Point<D, S>
}

impl<const D: usize, S: Space<D>> BoundingBox<D, S>
where Const<D>: ToTypenum
{
  pub fn new(min: Point<D, S>, max: Point<D, S>) -> Self { Self { min, max } }

  pub fn is_empty(&self) -> bool {
    for i in 0..D {
      if self.min[i] > self.max[i] {
        return true;
      }
    }

    false
  }

  pub fn enclose_box(&mut self, other: &BoundingBox<D, S>) {
    *self = Self::new(self.min.inner.inf(&other.min.inner).into(), self.max.inner.sup(&other.max.inner).into());
  }

  pub fn enclose_point(&mut self, point: &Point<D, S>) {
    let p = &point.inner;
    *self = Self::new(self.min.inner.inf(p).into(), self.max.inner.sup(p).into());
  }

  pub fn center(&self) -> Point<D, S> { (self.min + self.max.into()) / 2.0 }

  pub fn diagonal(&self) -> Vector<D, S> { self.max - self.min }

  pub fn ray_intersects(&self, ray: &Ray<D, S>) -> bool {
    let (min_t, max_t) = ray.time_bounds();
    let (mut min_t, mut max_t) = (min_t.into_inner(), max_t.into_inner());
    let origin = ray.origin().into_inner();
    let dir = ray.dir().into_inner();

    for i in 0..D {
      let inv_d = 1.0 / dir[i];
      let mut t0 = (self.min.inner[i] - origin[i]) * inv_d;
      let mut t1 = (self.max.inner[i] - origin[i]) * inv_d;
      if inv_d < 0.0 {
        std::mem::swap(&mut t0, &mut t1);
      }

      min_t = if t0 > min_t { t0 } else { min_t };
      max_t = if t1 < max_t { t1 } else { max_t };

      if max_t < min_t {
        return false;
      }
    }

    true
  }

  pub fn min(&self) -> Point<D, S> { self.min }

  pub fn max(&self) -> Point<D, S> { self.max }
}

impl<S: Space<3>> BoundingBox<3, S> {
  pub fn ray_intersects_fast(&self, ray: &Ray<3, S>) -> bool {
    let (min_t, max_t) = ray.time_bounds();
    let (mut min_t, mut max_t) = (min_t.into_inner(), max_t.into_inner());
    let origin = ray.origin().into_inner();
    let dir = ray.dir().into_inner();

    // Macro for checking intersection along a given axis
    macro_rules! check_axis {
      ($a:ident) => {
        let inv_d = 1.0 / dir.$a;
        let mut t0 = (self.min.inner.$a - origin.$a) * inv_d;
        let mut t1 = (self.max.inner.$a - origin.$a) * inv_d;
        if inv_d < 0.0 {
          std::mem::swap(&mut t0, &mut t1);
        }

        min_t = if t0 > min_t { t0 } else { min_t };
        max_t = if t1 < max_t { t1 } else { max_t };

        if max_t < min_t {
          return false;
        }
      };
    }

    check_axis!(x);
    check_axis!(y);
    check_axis!(z);

    true
  }
}

impl<const D: usize, S: Space<D>> Default for BoundingBox<D, S>
where Const<D>: ToTypenum
{
  fn default() -> Self {
    Self {
      min: Point::from_raw(nalgebra::SVector::repeat(Real::MAX)),
      max: Point::from_raw(nalgebra::SVector::repeat(Real::MIN))
    }
  }
}

pub type BoundingBox3<S> = BoundingBox<3, S>;

impl<S: Space<3>> BoundingBox<3, S> {
  pub fn surface_area(&self) -> Real {
    if self.is_empty() {
      0.0
    } else {
      let extent = self.diagonal().into_inner();
      2.0 * (extent.x * extent.z + extent.x * extent.y + extent.y * extent.z)
    }
  }
}

pub type WorldBoundingBox = BoundingBox3<WorldSpace>;
