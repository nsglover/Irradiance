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

  fn vector(&self, vector: &Vector3<In>) -> Vector3<Out> { (*vector).cast_unsafe() }

  fn point(&self, point: &Point3<In>) -> Point3<Out> { (*point).cast_unsafe() }

  fn direction(&self, dir: &UnitVector3<In>) -> UnitVector3<Out> { (*dir).cast_unsafe() }

  fn normal(&self, sn: &UnitVector3<In>) -> UnitVector3<Out> { (*sn).cast_unsafe() }

  fn ray(&self, ray: &Ray3<In>) -> Ray3<Out> { ray.cast_unsafe() }

  fn ray_intersect<'a>(
    &self,
    ray_intersection: &RayIntersection<'a, In>
  ) -> RayIntersection<'a, Out> {
    ray_intersection.cast_unsafe()
  }

  fn inverse_vector(&self, vector: &Vector3<Out>) -> Vector3<In> { (*vector).cast_unsafe() }

  fn inverse_point(&self, point: &Point3<Out>) -> Point3<In> { (*point).cast_unsafe() }

  fn inverse_direction(&self, dir: &UnitVector3<Out>) -> UnitVector3<In> { (*dir).cast_unsafe() }

  fn inverse_normal(&self, sn: &UnitVector3<Out>) -> UnitVector3<In> { (*sn).cast_unsafe() }

  fn inverse_ray(&self, ray: &Ray3<Out>) -> Ray3<In> { ray.cast_unsafe() }

  fn inverse_ray_intersect<'a>(
    &self,
    ray_intersection: &RayIntersection<'a, Out>
  ) -> RayIntersection<'a, In> {
    ray_intersection.cast_unsafe()
  }
}
