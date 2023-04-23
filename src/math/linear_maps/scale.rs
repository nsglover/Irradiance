use derive_more::Display;
use nalgebra as na;

use super::{MatrixTransform, Transform};
use crate::{
  common::Wrapper,
  math::{phantom::Phantom, *}
};

#[derive(Debug, Display)]
#[display(fmt = "{}", scale)]
pub struct ScaleTransform<In: Space<3>, Out: Space<3>> {
  scale: na::Vector3<Real>,
  inverse_scale: na::Vector3<Real>,
  _phantom_in: Phantom<In>,
  _phantom_out: Phantom<Out>
}

impl<In: Space<3>, Out: Space<3>> ScaleTransform<In, Out> {
  pub fn new(x_scale: Real, y_scale: Real, z_scale: Real) -> Self {
    if x_scale * y_scale * z_scale == 0.0 {
      panic!("Non-invertible scale!")
    }

    Self {
      scale: na::vector![x_scale, y_scale, z_scale],
      inverse_scale: na::vector![1.0 / x_scale, 1.0 / y_scale, 1.0 / z_scale],
      _phantom_in: Phantom::default(),
      _phantom_out: Phantom::default()
    }
  }
}

impl<In: Space<3>, Out: Space<3>> Transform<In, Out> for ScaleTransform<In, Out> {
  fn identity() -> Self
  where Self: Sized {
    Self::new(1.0, 1.0, 1.0)
  }

  fn matrix(&self) -> MatrixTransform<In, Out> {
    MatrixTransform::from_raw(na::Matrix4::new_nonuniform_scaling(&self.scale)).unwrap()
  }

  fn vector(&self, vector: &Vector3<In>) -> Vector3<Out> {
    Vector::from_raw(vector.inner.component_mul(&self.scale))
  }

  fn point(&self, point: &Point3<In>) -> Point3<Out> {
    Point::from_inner(point.inner.coords.component_mul(&self.scale).into())
  }

  fn direction(&self, dir: &UnitVector3<In>) -> UnitVector3<Out> {
    self.vector(&dir.into_vector()).normalize()
  }

  fn normal(&self, sn: &UnitVector3<In>) -> UnitVector3<Out> {
    UnitVector::from_raw(sn.into_inner().into_inner().component_mul(&self.inverse_scale))
  }

  fn inverse_vector(&self, vector: &Vector3<Out>) -> Vector3<In> {
    Vector::from_raw(vector.inner.component_mul(&self.inverse_scale))
  }

  fn inverse_point(&self, point: &Point3<Out>) -> Point3<In> {
    Point::from_inner(point.inner.coords.component_mul(&self.inverse_scale).into())
  }

  fn inverse_direction(&self, dir: &UnitVector3<Out>) -> UnitVector3<In> {
    self.inverse_vector(&dir.into_vector()).normalize()
  }

  fn inverse_normal(&self, sn: &UnitVector3<Out>) -> UnitVector3<In> {
    UnitVector::from_raw(sn.into_inner().into_inner().component_mul(&self.scale))
  }
}

unsafe impl<In: Space<3>, Out: Space<3>> Sync for ScaleTransform<In, Out> {}

unsafe impl<In: Space<3>, Out: Space<3>> Send for ScaleTransform<In, Out> {}
