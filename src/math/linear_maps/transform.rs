use std::fmt::{Debug, Display};

use super::super::*;
use crate::raytracing::*;

pub trait Transform<In: Space<3>, Out: Space<3>>: Debug + Display + Sync + Send {
  fn identity() -> Self
  where Self: Sized;

  fn matrix(&self) -> MatrixTransform<In, Out>;

  fn vector(&self, vector: &Vector3<In>) -> Vector3<Out>;

  fn point(&self, point: &Point3<In>) -> Point3<Out>;

  fn direction(&self, dir: &UnitVector3<In>) -> UnitVector3<Out>;

  fn normal(&self, sn: &UnitVector3<In>) -> UnitVector3<Out>;

  fn ray(&self, ray: &Ray3<In>) -> Ray3<Out> {
    let dir: Vector3<In> = ray.dir().inner.into_inner().into();
    let (transformed_dir, time_dilation) = self.vector(&dir).normalize_with_norm();

    Ray3::new_with_time(
      ray.max_intersect_time() * PositiveReal::new_unchecked(time_dilation),
      self.point(&ray.origin()),
      transformed_dir
    )
  }

  fn ray_intersect<'a>(&self, ray_intersection: &RayIntersection<In>) -> RayIntersection<Out> {
    let dir: Vector3<In> = ray_intersection.intersect_direction.inner.into_inner().into();
    let (transformed_dir, time_dilation) = self.vector(&dir).normalize_with_norm();

    RayIntersection {
      intersect_direction: transformed_dir,
      intersect_time: ray_intersection.intersect_time * PositiveReal::new_unchecked(time_dilation),
      intersect_point: self.point(&ray_intersection.intersect_point),
      geometric_normal: self.normal(&ray_intersection.geometric_normal),
      tex_coords: ray_intersection.tex_coords
    }
  }

  fn inverse_vector(&self, vector: &Vector3<Out>) -> Vector3<In>;

  fn inverse_point(&self, point: &Point3<Out>) -> Point3<In>;

  fn inverse_direction(&self, dir: &UnitVector3<Out>) -> UnitVector3<In>;

  fn inverse_normal(&self, sn: &UnitVector3<Out>) -> UnitVector3<In>;

  fn inverse_ray(&self, ray: &Ray3<Out>) -> Ray3<In> {
    let dir: Vector3<Out> = ray.dir().inner.into_inner().into();
    let (transformed_dir, time_dilation) = self.inverse_vector(&dir).normalize_with_norm();

    Ray3::new_with_time(
      ray.max_intersect_time() * PositiveReal::new_unchecked(time_dilation),
      self.inverse_point(&ray.origin()),
      transformed_dir
    )
  }

  fn inverse_ray_intersect<'a>(
    &self,
    ray_intersection: &RayIntersection<Out>
  ) -> RayIntersection<In> {
    let dir: Vector3<Out> = ray_intersection.intersect_direction.inner.into_inner().into();
    let (transformed_dir, time_dilation) = self.inverse_vector(&dir).normalize_with_norm();

    RayIntersection {
      intersect_direction: transformed_dir,
      intersect_time: ray_intersection.intersect_time * PositiveReal::new_unchecked(time_dilation),
      intersect_point: self.inverse_point(&ray_intersection.intersect_point),
      geometric_normal: self.inverse_normal(&ray_intersection.geometric_normal),
      tex_coords: ray_intersection.tex_coords
    }
  }
}

pub type LocalToWorld<S> = Box<dyn Transform<S, WorldSpace>>;
