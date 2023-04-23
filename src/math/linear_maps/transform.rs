use std::fmt::{Debug, Display};

use super::super::*;
use crate::raytracing::*;

pub trait Transform<In: Space<3>, Out: Space<3>>: Debug + Display + Sync + Send {
  fn identity() -> Self
  where Self: Sized;

  fn matrix(&self) -> &MatrixTransform<In, Out>;

  fn determinant(&self) -> Real;

  fn vector(&self, vector: &Vector3<In>) -> Vector3<Out>;

  fn point(&self, point: &Point3<In>) -> Point3<Out>;

  fn direction(&self, dir: &UnitVector3<In>) -> UnitVector3<Out>;

  fn normal(&self, sn: &UnitVector3<In>) -> UnitVector3<Out>;

  fn ray(&self, ray: &Ray3<In>) -> Ray3<Out>;

  fn ray_intersect<'a>(
    &self,
    ray_intersection: &RayIntersection<'a, In>
  ) -> RayIntersection<'a, Out>;

  fn inverse_determinant(&self) -> Real;

  fn inverse_vector(&self, vector: &Vector3<Out>) -> Vector3<In>;

  fn inverse_point(&self, point: &Point3<Out>) -> Point3<In>;

  fn inverse_direction(&self, dir: &UnitVector3<Out>) -> UnitVector3<In>;

  fn inverse_normal(&self, sn: &UnitVector3<Out>) -> UnitVector3<In>;

  fn inverse_ray(&self, ray: &Ray3<Out>) -> Ray3<In>;

  fn inverse_ray_intersect<'a>(
    &self,
    ray_intersection: &RayIntersection<'a, Out>
  ) -> RayIntersection<'a, In>;
}

pub type LocalToWorld<S> = Box<dyn Transform<S, WorldSpace>>;
