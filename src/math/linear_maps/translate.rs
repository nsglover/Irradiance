use derive_more::Display;
use nalgebra as na;

use super::{MatrixTransform, Transform};
use crate::{
  common::Wrapper,
  math::{phantom::Phantom, *},
  raytracing::*
};

#[derive(Debug, Display)]
#[display(fmt = "{}", translation)]
pub struct TranslateTransform<In: Space<3>, Out: Space<3>> {
  translation: na::Vector3<Real>,
  _phantom_in: Phantom<In>,
  _phantom_out: Phantom<Out>
}

impl<In: Space<3>, Out: Space<3>> TranslateTransform<In, Out> {
  pub fn new(x_offset: Real, y_offset: Real, z_offset: Real) -> Self {
    Self {
      translation: na::vector![x_offset, y_offset, z_offset],
      _phantom_in: Phantom::default(),
      _phantom_out: Phantom::default()
    }
  }
}

impl<In: Space<3>, Out: Space<3>> Transform<In, Out> for TranslateTransform<In, Out> {
  fn identity() -> Self
  where Self: Sized {
    Self::new(0.0, 0.0, 0.0)
  }

  fn matrix(&self) -> MatrixTransform<In, Out> {
    MatrixTransform::from_raw(na::Matrix4::new_translation(&self.translation)).unwrap()
  }

  fn vector(&self, vector: &Vector3<In>) -> Vector3<Out> { vector.cast_unsafe() }

  fn point(&self, point: &Point3<In>) -> Point3<Out> {
    Point::from_inner((point.inner.coords + self.translation).into())
  }

  fn direction(&self, dir: &UnitVector3<In>) -> UnitVector3<Out> { dir.cast_unsafe() }

  fn normal(&self, sn: &UnitVector3<In>) -> UnitVector3<Out> { sn.cast_unsafe() }

  fn ray(&self, ray: &Ray3<In>) -> Ray3<Out> {
    Ray3::new_with_time(
      ray.max_intersect_time(),
      self.point(&ray.origin()),
      self.direction(&ray.dir())
    )
  }

  fn ray_intersect<'a>(
    &self,
    ray_intersection: &RayIntersection<'a, In>
  ) -> RayIntersection<'a, Out> {
    RayIntersection {
      ray: Ray3::new_with_time(
        ray_intersection.ray.max_intersect_time(),
        self.point(&ray_intersection.ray.origin()),
        self.direction(&ray_intersection.ray.dir())
      ),
      surface: ray_intersection.surface,
      intersect_time: ray_intersection.intersect_time,
      intersect_point: self.point(&ray_intersection.intersect_point),
      geometric_normal: self.normal(&ray_intersection.geometric_normal),
      shading_normal: self.normal(&ray_intersection.shading_normal),
      tex_coords: ray_intersection.tex_coords
    }
  }

  fn inverse_vector(&self, vector: &Vector3<Out>) -> Vector3<In> { vector.cast_unsafe() }

  fn inverse_point(&self, point: &Point3<Out>) -> Point3<In> {
    Point::from_inner((point.inner.coords - self.translation).into())
  }

  fn inverse_direction(&self, dir: &UnitVector3<Out>) -> UnitVector3<In> { dir.cast_unsafe() }

  fn inverse_normal(&self, sn: &UnitVector3<Out>) -> UnitVector3<In> { sn.cast_unsafe() }

  fn inverse_ray(&self, ray: &Ray3<Out>) -> Ray3<In> {
    Ray3::new_with_time(
      ray.max_intersect_time(),
      self.inverse_point(&ray.origin()),
      self.inverse_direction(&ray.dir())
    )
  }

  fn inverse_ray_intersect<'a>(
    &self,
    ray_intersection: &RayIntersection<'a, Out>
  ) -> RayIntersection<'a, In> {
    RayIntersection {
      ray: Ray3::new_with_time(
        ray_intersection.ray.max_intersect_time(),
        self.inverse_point(&ray_intersection.ray.origin()),
        self.inverse_direction(&ray_intersection.ray.dir())
      ),
      surface: ray_intersection.surface,
      intersect_time: ray_intersection.intersect_time,
      intersect_point: self.inverse_point(&ray_intersection.intersect_point),
      geometric_normal: self.inverse_normal(&ray_intersection.geometric_normal),
      shading_normal: self.inverse_normal(&ray_intersection.shading_normal),
      tex_coords: ray_intersection.tex_coords
    }
  }
}

unsafe impl<In: Space<3>, Out: Space<3>> Sync for TranslateTransform<In, Out> {}

unsafe impl<In: Space<3>, Out: Space<3>> Send for TranslateTransform<In, Out> {}
