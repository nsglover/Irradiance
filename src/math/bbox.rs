use {
  super::*,
  nalgebra::{Const, ToTypenum}
};

#[derive(Debug)]
pub struct BBox<const D: usize, S: Space<D>>
where Const<D>: ToTypenum
{
  min: Point<D, S>,
  max: Point<D, S>
}

impl<const D: usize, S: Space<D>> BBox<D, S>
where Const<D>: ToTypenum
{
  pub fn new(min: Point<D, S>, max: Point<D, S>) -> Self {
    // This prevents zero-volume boxes
    // let offset = nalgebra::SVector::repeat(Float::EPSILON) * 8.0;
    let offset = nalgebra::SVector::repeat(Float::EPSILON) * 0.0;
    Self { min: (min.inner - offset).into(), max: (max.inner + offset).into() }
  }

  pub fn is_empty(&self) -> bool {
    for i in 0..D {
      if self.min[i] > self.max[i] {
        return true;
      }
    }

    false
  }

  pub fn enclose_box(&mut self, other: &BBox<D, S>) {
    *self = Self::new(
      self.min.inner.inf(&other.min.inner).into(),
      self.max.inner.sup(&other.max.inner).into()
    );
  }

  pub fn enclose_point(&mut self, point: &Point<D, S>) {
    let p = &point.inner;
    *self = Self::new(self.min.inner.inf(p).into(), self.max.inner.sup(p).into());
  }

  pub fn center(&self) -> Point<D, S> { (self.min + self.max.into()) / 2.0 }

  pub fn diagonal(&self) -> Vector<D, S> { self.max - self.min }

  pub fn ray_intersects(&self, ray: &Ray<D, S>) -> bool {
    let (mut min_t, mut max_t) = ray.time_bounds();
    for i in 0..D {
      let inv_d = 1.0 / ray.dir().inner()[i];
      let mut t0 = (self.min.inner[i] - ray.origin().inner()[i]) * inv_d;
      let mut t1 = (self.max.inner[i] - ray.origin().inner()[i]) * inv_d;
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

impl<const D: usize, S: Space<D>> Default for BBox<D, S>
where Const<D>: ToTypenum
{
  fn default() -> Self {
    Self {
      min: Vector::from(nalgebra::SVector::repeat(Float::MAX)).into(),
      max: Vector::from(nalgebra::SVector::repeat(Float::MIN)).into()
    }
  }
}

pub type BBox3<S> = BBox<3, S>;

impl<S: Space<3>> BBox<3, S> {
  pub fn surface_area(&self) -> Float {
    if self.is_empty() {
      0.0
    } else {
      let extent = self.diagonal().inner();
      2.0 * (extent.x * extent.z + extent.x * extent.y + extent.y * extent.z)
    }
  }
}

pub type WorldBBox = BBox3<WorldSpace>;
