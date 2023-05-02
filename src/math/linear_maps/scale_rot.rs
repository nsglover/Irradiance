use derive_more::Display;

use super::{
  rotate::RotateTransform, uniform_scale::UniformScaleTransform, MatrixTransform, Transform
};
use crate::{
  math::*,
  raytracing::{Ray3, RayIntersection}
};

#[derive(Debug, Display)]
#[display(fmt = "{}, {}", scale, rotate)]
pub struct ScaleRotate<In: Space<3>, Middle: Space<3>, Out: Space<3>> {
  scale: UniformScaleTransform<In, Middle>,
  rotate: RotateTransform<Middle, Out>
}

impl<In: Space<3>, Middle: Space<3>, Out: Space<3>> ScaleRotate<In, Middle, Out> {
  pub fn new(
    scale: UniformScaleTransform<In, Middle>,
    rotate: RotateTransform<Middle, Out>
  ) -> Self {
    Self { scale, rotate }
  }
}

impl<In: Space<3>, Middle: Space<3>, Out: Space<3>> Transform<In, Out>
  for ScaleRotate<In, Middle, Out>
{
  fn identity() -> Self
  where Self: Sized {
    Self::new(UniformScaleTransform::identity(), RotateTransform::identity())
  }

  fn matrix(&self) -> MatrixTransform<In, Out> { self.rotate.matrix() * self.scale.matrix() }

  fn vector(&self, vector: &Vector3<In>) -> Vector3<Out> {
    self.rotate.vector(&self.scale.vector(vector))
  }

  fn point(&self, point: &Point3<In>) -> Point3<Out> { self.rotate.point(&self.scale.point(point)) }

  fn direction(&self, dir: &UnitVector3<In>) -> UnitVector3<Out> {
    self.rotate.direction(&self.scale.direction(dir))
  }

  fn normal(&self, sn: &UnitVector3<In>) -> UnitVector3<Out> {
    self.rotate.normal(&self.scale.normal(sn))
  }

  fn ray(&self, ray: &Ray3<In>) -> Ray3<Out> { self.rotate.ray(&self.scale.ray(ray)) }

  fn ray_intersect<'a>(&self, ray_intersection: &RayIntersection<In>) -> RayIntersection<Out> {
    self.rotate.ray_intersect(&self.scale.ray_intersect(ray_intersection))
  }

  fn inverse_vector(&self, vector: &Vector3<Out>) -> Vector3<In> {
    self.scale.inverse_vector(&self.rotate.inverse_vector(vector))
  }

  fn inverse_point(&self, point: &Point3<Out>) -> Point3<In> {
    self.scale.inverse_point(&self.rotate.inverse_point(point))
  }

  fn inverse_direction(&self, dir: &UnitVector3<Out>) -> UnitVector3<In> {
    self.scale.inverse_direction(&self.rotate.inverse_direction(dir))
  }

  fn inverse_normal(&self, sn: &UnitVector3<Out>) -> UnitVector3<In> {
    self.scale.inverse_normal(&self.rotate.inverse_normal(sn))
  }

  fn inverse_ray(&self, ray: &Ray3<Out>) -> Ray3<In> {
    self.scale.inverse_ray(&self.rotate.inverse_ray(ray))
  }

  fn inverse_ray_intersect<'a>(
    &self,
    ray_intersection: &RayIntersection<Out>
  ) -> RayIntersection<In> {
    self.scale.inverse_ray_intersect(&self.rotate.inverse_ray_intersect(ray_intersection))
  }
}

unsafe impl<In: Space<3>, Middle: Space<3>, Out: Space<3>> Sync for ScaleRotate<In, Middle, Out> {}

unsafe impl<In: Space<3>, Middle: Space<3>, Out: Space<3>> Send for ScaleRotate<In, Middle, Out> {}
