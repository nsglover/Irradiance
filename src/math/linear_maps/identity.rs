use derive_more::Display;

use super::{MatrixTransform, Transform};
use crate::{math::*, raytracing::*};

#[derive(Debug, Display)]
pub struct IdentityTransform;

impl<In: Space<3>, Out: Space<3>> Transform<In, Out> for IdentityTransform {
  fn identity() -> Self
  where Self: Sized {
    Self {}
  }

  fn matrix(&self) -> MatrixTransform<In, Out> { MatrixTransform::identity() }

  fn vector(&self, vector: &Vector3<In>) -> Vector3<Out> { (*vector).cast_unchecked() }

  fn point(&self, point: &Point3<In>) -> Point3<Out> { (*point).cast_unchecked() }

  fn direction(&self, dir: &UnitVector3<In>) -> UnitVector3<Out> { (*dir).cast_unchecked() }

  fn normal(&self, sn: &UnitVector3<In>) -> UnitVector3<Out> { (*sn).cast_unchecked() }

  fn ray(&self, ray: &Ray3<In>) -> Ray3<Out> { ray.cast_unchecked() }

  fn ray_intersect<'a>(&self, ray_intersection: &RayIntersection<In>) -> RayIntersection<Out> {
    ray_intersection.cast_unchecked()
  }

  fn inverse_vector(&self, vector: &Vector3<Out>) -> Vector3<In> { (*vector).cast_unchecked() }

  fn inverse_point(&self, point: &Point3<Out>) -> Point3<In> { (*point).cast_unchecked() }

  fn inverse_direction(&self, dir: &UnitVector3<Out>) -> UnitVector3<In> { (*dir).cast_unchecked() }

  fn inverse_normal(&self, sn: &UnitVector3<Out>) -> UnitVector3<In> { (*sn).cast_unchecked() }

  fn inverse_ray(&self, ray: &Ray3<Out>) -> Ray3<In> { ray.cast_unchecked() }

  fn inverse_ray_intersect<'a>(
    &self,
    ray_intersection: &RayIntersection<Out>
  ) -> RayIntersection<In> {
    ray_intersection.cast_unchecked()
  }
}
