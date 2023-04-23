use derive_more::Display;

use super::{rotate::RotateTransform, translate::TranslateTransform, MatrixTransform, Transform};
use crate::{
  math::*,
  raytracing::{Ray3, RayIntersection}
};

#[derive(Debug, Display)]
#[display(fmt = "{}, {}", rotate, translate)]
pub struct RotateTranslate<In: Space<3>, Middle: Space<3>, Out: Space<3>> {
  rotate: RotateTransform<In, Middle>,
  translate: TranslateTransform<Middle, Out>
}

impl<In: Space<3>, Middle: Space<3>, Out: Space<3>> RotateTranslate<In, Middle, Out> {
  pub fn new(
    rotate: RotateTransform<In, Middle>,
    translate: TranslateTransform<Middle, Out>
  ) -> Self {
    Self { rotate, translate }
  }
}

impl<In: Space<3>, Middle: Space<3>, Out: Space<3>> Transform<In, Out>
  for RotateTranslate<In, Middle, Out>
{
  fn identity() -> Self
  where Self: Sized {
    Self::new(RotateTransform::identity(), TranslateTransform::identity())
  }

  fn matrix(&self) -> MatrixTransform<In, Out> { self.translate.matrix() * self.rotate.matrix() }

  fn vector(&self, vector: &Vector3<In>) -> Vector3<Out> {
    self.translate.vector(&self.rotate.vector(vector))
  }

  fn point(&self, point: &Point3<In>) -> Point3<Out> {
    self.translate.point(&self.rotate.point(point))
  }

  fn direction(&self, dir: &UnitVector3<In>) -> UnitVector3<Out> {
    self.translate.direction(&self.rotate.direction(dir))
  }

  fn normal(&self, sn: &UnitVector3<In>) -> UnitVector3<Out> {
    self.translate.normal(&self.rotate.normal(sn))
  }

  fn ray(&self, ray: &Ray3<In>) -> Ray3<Out> { self.translate.ray(&self.rotate.ray(ray)) }

  fn ray_intersect<'a>(
    &self,
    ray_intersection: &RayIntersection<'a, In>
  ) -> RayIntersection<'a, Out> {
    self.translate.ray_intersect(&self.rotate.ray_intersect(ray_intersection))
  }

  fn inverse_vector(&self, vector: &Vector3<Out>) -> Vector3<In> {
    self.rotate.inverse_vector(&self.translate.inverse_vector(vector))
  }

  fn inverse_point(&self, point: &Point3<Out>) -> Point3<In> {
    self.rotate.inverse_point(&self.translate.inverse_point(point))
  }

  fn inverse_direction(&self, dir: &UnitVector3<Out>) -> UnitVector3<In> {
    self.rotate.inverse_direction(&self.translate.inverse_direction(dir))
  }

  fn inverse_normal(&self, sn: &UnitVector3<Out>) -> UnitVector3<In> {
    self.rotate.inverse_normal(&self.translate.inverse_normal(sn))
  }

  fn inverse_ray(&self, ray: &Ray3<Out>) -> Ray3<In> {
    self.rotate.inverse_ray(&self.translate.inverse_ray(ray))
  }

  fn inverse_ray_intersect<'a>(
    &self,
    ray_intersection: &RayIntersection<'a, Out>
  ) -> RayIntersection<'a, In> {
    self.rotate.inverse_ray_intersect(&self.translate.inverse_ray_intersect(ray_intersection))
  }
}

unsafe impl<In: Space<3>, Middle: Space<3>, Out: Space<3>> Sync
  for RotateTranslate<In, Middle, Out>
{
}

unsafe impl<In: Space<3>, Middle: Space<3>, Out: Space<3>> Send
  for RotateTranslate<In, Middle, Out>
{
}
