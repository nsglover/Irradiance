use std::{fmt::Display, ops};

use nalgebra as na;

use super::super::{phantom::*, *};
use crate::raytracing::*;

type Matrix<const D: usize> = na::SMatrix<Real, D, D>;

#[derive(Debug, Clone)]
pub struct MatrixTransform<In: Space<3>, Out: Space<3>> {
  matrix: Matrix<4>,
  matrix_inv: Matrix<4>,
  det: Real,
  det_inv: Real,
  _phantom_in: Phantom<In>,
  _phantom_out: Phantom<Out>
}

impl<In: Space<3>, Out: Space<3>> MatrixTransform<In, Out> {
  pub fn from_raw(t: Matrix<4>) -> Option<Self> {
    t.try_inverse().map(|t_inv| Self {
      matrix: t,
      matrix_inv: t_inv,
      det: t.determinant(),
      det_inv: t_inv.determinant(),
      _phantom_in: Phantom::default(),
      _phantom_out: Phantom::default()
    })
  }
}

impl<In: Space<3>, Out: Space<3>> Display for MatrixTransform<In, Out> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", self.matrix) }
}

impl<In: Space<3>, Out: Space<3>> Transform<In, Out> for MatrixTransform<In, Out> {
  fn identity() -> Self {
    Self {
      matrix: Matrix::<4>::identity(),
      matrix_inv: Matrix::<4>::identity(),
      det: 1.0,
      det_inv: 1.0,
      _phantom_in: Phantom::default(),
      _phantom_out: Phantom::default()
    }
  }

  fn matrix(&self) -> &MatrixTransform<In, Out> { self }

  fn determinant(&self) -> Real { self.det }

  fn vector(&self, vector: &Vector3<In>) -> Vector3<Out> {
    (self.matrix * vector.inner.to_homogeneous()).xyz().into()
  }

  fn point(&self, point: &Point3<In>) -> Point3<Out> {
    na::Point3::from_homogeneous(self.matrix * point.inner.to_homogeneous()).unwrap().into()
  }

  fn direction(&self, dir: &UnitVector3<In>) -> UnitVector3<Out> {
    let inner = dir.inner.into_inner();
    let d = (self.matrix * inner.to_homogeneous()).xyz();
    na::Unit::new_normalize(d).into()
  }

  fn normal(&self, sn: &UnitVector3<In>) -> UnitVector3<Out> {
    let v = (self.matrix_inv.transpose() * sn.inner.into_inner().to_homogeneous()).xyz();
    na::Unit::new_normalize(v).into()
  }

  fn ray(&self, ray: &Ray3<In>) -> Ray3<Out> {
    let dir: Vector3<In> = ray.dir().inner.into_inner().into();
    let (transformed_dir, time_dilation) = self.vector(&dir).normalize_with_norm();

    Ray3::new_with_time(
      ray.max_intersect_time() * PositiveReal::new_unchecked(time_dilation),
      self.point(&ray.origin()),
      transformed_dir
    )
  }

  fn ray_intersect<'a>(
    &self,
    ray_intersection: &RayIntersection<'a, In>
  ) -> RayIntersection<'a, Out> {
    let ray = &ray_intersection.ray;
    let dir: Vector3<In> = ray.dir().inner.into_inner().into();
    let (transformed_dir, time_dilation) = self.vector(&dir).normalize_with_norm();

    let transformed_ray = Ray3::new_with_time(
      ray.max_intersect_time() * PositiveReal::new_unchecked(time_dilation),
      self.point(&ray.origin()),
      transformed_dir
    );

    RayIntersection {
      ray: transformed_ray,
      surface: ray_intersection.surface,
      intersect_time: ray_intersection.intersect_time * PositiveReal::new_unchecked(time_dilation),
      intersect_point: self.point(&ray_intersection.intersect_point),
      geometric_normal: self.normal(&ray_intersection.geometric_normal),
      shading_normal: self.normal(&ray_intersection.shading_normal),
      tex_coords: ray_intersection.tex_coords
    }
  }

  fn inverse_determinant(&self) -> Real { self.det_inv }

  fn inverse_vector(&self, vector: &Vector3<Out>) -> Vector3<In> {
    (self.matrix_inv * vector.inner.to_homogeneous()).xyz().into()
  }

  fn inverse_point(&self, point: &Point3<Out>) -> Point3<In> {
    na::Point3::from_homogeneous(self.matrix_inv * point.inner.to_homogeneous()).unwrap().into()
  }

  fn inverse_direction(&self, dir: &UnitVector3<Out>) -> UnitVector3<In> {
    let inner = dir.inner.into_inner();
    let d = (self.matrix_inv * inner.to_homogeneous()).xyz();
    na::Unit::new_normalize(d).into()
  }

  fn inverse_normal(&self, sn: &UnitVector3<Out>) -> UnitVector3<In> {
    let v = (self.matrix.transpose() * sn.inner.into_inner().to_homogeneous()).xyz();
    na::Unit::new_normalize(v).into()
  }

  fn inverse_ray(&self, ray: &Ray3<Out>) -> Ray3<In> {
    let dir: Vector3<Out> = ray.dir().inner.into_inner().into();
    let transformed_dir = self.inverse_vector(&dir);
    let time_dilation = transformed_dir.norm();

    Ray3::new_with_time(
      ray.max_intersect_time() * PositiveReal::new_unchecked(time_dilation),
      self.inverse_point(&ray.origin()),
      na::Unit::new_unchecked((transformed_dir / time_dilation).inner).into()
    )
  }

  fn inverse_ray_intersect<'a>(
    &self,
    ray_intersection: &RayIntersection<'a, Out>
  ) -> RayIntersection<'a, In> {
    let ray = &ray_intersection.ray;
    let dir: Vector3<Out> = ray.dir().inner.into_inner().into();
    let transformed_dir = self.inverse_vector(&dir);
    let time_dilation = transformed_dir.norm();

    let transformed_ray = Ray3::new_with_time(
      ray.max_intersect_time() * PositiveReal::new_unchecked(time_dilation),
      self.inverse_point(&ray.origin()),
      na::Unit::new_unchecked((transformed_dir / time_dilation).inner).into()
    );

    RayIntersection {
      ray: transformed_ray,
      surface: ray_intersection.surface,
      intersect_time: ray_intersection.intersect_time * PositiveReal::new_unchecked(time_dilation),
      intersect_point: self.inverse_point(&ray_intersection.intersect_point),
      geometric_normal: self.inverse_normal(&ray_intersection.geometric_normal),
      shading_normal: self.inverse_normal(&ray_intersection.shading_normal),
      tex_coords: ray_intersection.tex_coords
    }
  }
}

impl<In: Space<3>, Middle: Space<3>, Out: Space<3>> ops::Mul<MatrixTransform<In, Middle>>
  for MatrixTransform<Middle, Out>
{
  type Output = MatrixTransform<In, Out>;

  fn mul(self, rhs: MatrixTransform<In, Middle>) -> Self::Output {
    MatrixTransform {
      matrix: self.matrix * rhs.matrix,
      matrix_inv: rhs.matrix_inv * self.matrix_inv,
      det: self.det * rhs.det,
      det_inv: rhs.det_inv * self.det_inv,
      _phantom_in: rhs._phantom_in,
      _phantom_out: self._phantom_out
    }
  }
}

unsafe impl<In: Space<3>, Out: Space<3>> Sync for MatrixTransform<In, Out> {}

unsafe impl<In: Space<3>, Out: Space<3>> Send for MatrixTransform<In, Out> {}
