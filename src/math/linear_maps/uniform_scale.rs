use derive_more::Display;
use nalgebra as na;

use super::{MatrixTransform, Transform};
use crate::{
  common::Wrapper,
  math::{phantom::Phantom, *},
  raytracing::*
};

#[derive(Debug, Display)]
#[display(fmt = "{}", scale)]
pub struct UniformScaleTransform<In: Space<3>, Out: Space<3>> {
  scale: Real,
  inverse_scale: Real,
  _phantom_in: Phantom<In>,
  _phantom_out: Phantom<Out>
}

impl<In: Space<3>, Out: Space<3>> UniformScaleTransform<In, Out> {
  pub fn new(factor: PositiveReal) -> Self {
    Self {
      scale: factor.into_inner(),
      inverse_scale: 1.0 / factor,
      _phantom_in: Phantom::default(),
      _phantom_out: Phantom::default()
    }
  }
}

impl<In: Space<3>, Out: Space<3>> Transform<In, Out> for UniformScaleTransform<In, Out> {
  fn identity() -> Self
  where Self: Sized {
    Self::new(PositiveReal::ONE)
  }

  fn matrix(&self) -> MatrixTransform<In, Out> {
    MatrixTransform::from_raw(na::Matrix4::new_scaling(self.scale)).unwrap()
  }

  fn vector(&self, vector: &Vector3<In>) -> Vector3<Out> {
    Vector::from_raw(vector.inner * self.scale)
  }

  fn point(&self, point: &Point3<In>) -> Point3<Out> {
    Point::from_inner((point.inner.coords * self.scale).into())
  }

  fn direction(&self, dir: &UnitVector3<In>) -> UnitVector3<Out> { dir.cast_unchecked() }

  fn normal(&self, sn: &UnitVector3<In>) -> UnitVector3<Out> { sn.cast_unchecked() }

  fn ray(&self, ray: &Ray3<In>) -> Ray3<Out> {
    Ray3::new_with_time(
      ray.max_intersect_time() * PositiveReal::new_unchecked(self.scale),
      self.point(&ray.origin()),
      self.direction(&ray.dir())
    )
  }

  fn ray_intersect<'a>(&self, ray_intersection: &RayIntersection<In>) -> RayIntersection<Out> {
    let ray = &ray_intersection.ray;
    let transformed_ray = Ray3::new_with_time(
      ray.max_intersect_time() * PositiveReal::new_unchecked(self.scale),
      self.point(&ray.origin()),
      self.direction(&ray.dir())
    );

    RayIntersection {
      ray: transformed_ray,
      material: ray_intersection.material.clone(),
      intersect_time: ray_intersection.intersect_time * PositiveReal::new_unchecked(self.scale),
      intersect_point: self.point(&ray_intersection.intersect_point),
      geometric_normal: self.normal(&ray_intersection.geometric_normal),
      shading_normal: self.normal(&ray_intersection.shading_normal),
      tex_coords: ray_intersection.tex_coords
    }
  }

  fn inverse_vector(&self, vector: &Vector3<Out>) -> Vector3<In> {
    Vector::from_raw(vector.inner * self.inverse_scale)
  }

  fn inverse_point(&self, point: &Point3<Out>) -> Point3<In> {
    Point::from_inner((point.inner.coords * self.inverse_scale).into())
  }

  fn inverse_direction(&self, dir: &UnitVector3<Out>) -> UnitVector3<In> { dir.cast_unchecked() }

  fn inverse_normal(&self, sn: &UnitVector3<Out>) -> UnitVector3<In> { sn.cast_unchecked() }

  fn inverse_ray(&self, ray: &Ray3<Out>) -> Ray3<In> {
    Ray3::new_with_time(
      ray.max_intersect_time() * PositiveReal::new_unchecked(self.inverse_scale),
      self.inverse_point(&ray.origin()),
      self.inverse_direction(&ray.dir())
    )
  }

  fn inverse_ray_intersect<'a>(
    &self,
    ray_intersection: &RayIntersection<Out>
  ) -> RayIntersection<In> {
    let ray = &ray_intersection.ray;
    let transformed_ray = Ray3::new_with_time(
      ray.max_intersect_time() * PositiveReal::new_unchecked(self.inverse_scale),
      self.inverse_point(&ray.origin()),
      self.inverse_direction(&ray.dir())
    );

    RayIntersection {
      ray: transformed_ray,
      material: ray_intersection.material.clone(),
      intersect_time: ray_intersection.intersect_time
        * PositiveReal::new_unchecked(self.inverse_scale),
      intersect_point: self.inverse_point(&ray_intersection.intersect_point),
      geometric_normal: self.inverse_normal(&ray_intersection.geometric_normal),
      shading_normal: self.inverse_normal(&ray_intersection.shading_normal),
      tex_coords: ray_intersection.tex_coords
    }
  }
}

unsafe impl<In: Space<3>, Out: Space<3>> Sync for UniformScaleTransform<In, Out> {}

unsafe impl<In: Space<3>, Out: Space<3>> Send for UniformScaleTransform<In, Out> {}
