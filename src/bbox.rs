use {
  crate::math::*,
  nalgebra::{Const, ToTypenum}
};

pub struct BBox<const D: usize, S: Space<D>>
where Const<D>: ToTypenum
{
  min: Point<D, S>,
  max: Point<D, S>
}

impl<const D: usize, S: Space<D>> BBox<D, S>
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

  pub fn enclose_box(&mut self, other: &BBox<D, S>) {
    self.min = self.min.inner.inf(&other.min.inner).into();
    self.max = self.max.inner.sup(&other.max.inner).into();
  }

  pub fn enclose_point(&mut self, point: &Point<D, S>) {
    self.min = self.min.inner.inf(&point.inner).into();
    self.max = self.max.inner.sup(&point.inner).into();
  }

  pub fn center(&self) -> Point<D, S> { (self.min + self.max.into()) / 2.0 }

  pub fn diagonal(&self) -> Vector<D, S> { self.max - self.min }

  pub fn ray_intersects(&self, ray: &Ray<D, S>) -> Option<Float> {
    let (mut min_t, mut max_t) = ray.time_bounds;
    let dir_vec: Vector<D, S> = ray.dir().into();

    for i in 0..D {
      if self.min[i] > self.max[i] {
        return None;
      }

      let inv_d = 1.0 / dir_vec[i];
      let mut t0 = (self.min[i] - ray.origin()[i]) * inv_d;
      let mut t1 = (self.max[i] - ray.origin()[i]) * inv_d;

      if inv_d < 0.0 {
        std::mem::swap(&mut t0, &mut t1);
      }

      min_t = if t0 > min_t { t0 } else { min_t };
      max_t = if t1 < max_t { t1 } else { max_t };

      if max_t < min_t {
        return None;
      }
    }

    Some(min_t)
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

pub type WorldBBox = BBox3<WorldSpace>;
