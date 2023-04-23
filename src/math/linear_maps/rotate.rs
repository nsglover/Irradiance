use derive_more::Display;
use nalgebra as na;

use super::{MatrixTransform, Transform};
use crate::{
  common::Wrapper,
  math::{phantom::Phantom, *},
  raytracing::{Ray3, RayIntersection}
};

#[derive(Debug, Display)]
#[display(fmt = "{}", quat)]
pub struct RotateTransform<In: Space<3>, Out: Space<3>> {
  quat: na::UnitQuaternion<Real>,
  _phantom_in: Phantom<In>,
  _phantom_out: Phantom<Out>
}

impl<In: Space<3>, Out: Space<3>> RotateTransform<In, Out> {
  pub fn new(axis_x: Real, axis_y: Real, axis_z: Real, angle: Real) -> Self {
    Self {
      quat: na::UnitQuaternion::from_axis_angle(
        &na::Unit::new_normalize(na::vector![axis_x, axis_y, axis_z]),
        angle
      ),
      _phantom_in: Phantom::default(),
      _phantom_out: Phantom::default()
    }
  }
}

impl<In: Space<3>, Out: Space<3>> Transform<In, Out> for RotateTransform<In, Out> {
  fn identity() -> Self
  where Self: Sized {
    Self::new(1.0, 0.0, 0.0, 0.0)
  }

  fn matrix(&self) -> MatrixTransform<In, Out> {
    MatrixTransform::from_raw(self.quat.to_homogeneous()).unwrap()
  }

  fn vector(&self, vector: &Vector3<In>) -> Vector3<Out> {
    Vector::from_raw(self.quat * vector.inner)
  }

  fn point(&self, point: &Point3<In>) -> Point3<Out> {
    Point::from_inner((self.quat * point.inner.coords).into())
  }

  fn direction(&self, dir: &UnitVector3<In>) -> UnitVector3<Out> {
    self.vector(&dir.into_vector()).normalize_fast()
  }

  fn normal(&self, sn: &UnitVector3<In>) -> UnitVector3<Out> {
    self.vector(&sn.into_vector()).normalize_fast()
  }

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

  fn inverse_vector(&self, vector: &Vector3<Out>) -> Vector3<In> {
    Vector::from_raw(self.quat.inverse_transform_vector(&vector.inner))
  }

  fn inverse_point(&self, point: &Point3<Out>) -> Point3<In> {
    Point::from_inner(self.quat.inverse_transform_point(&point.inner))
  }

  fn inverse_direction(&self, dir: &UnitVector3<Out>) -> UnitVector3<In> {
    self.inverse_vector(&dir.into_vector()).normalize_fast()
  }

  fn inverse_normal(&self, sn: &UnitVector3<Out>) -> UnitVector3<In> {
    self.inverse_vector(&sn.into_vector()).normalize_fast()
  }

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

unsafe impl<In: Space<3>, Out: Space<3>> Sync for RotateTransform<In, Out> {}

unsafe impl<In: Space<3>, Out: Space<3>> Send for RotateTransform<In, Out> {}
