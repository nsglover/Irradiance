use std::fmt::Display;

use {
  super::{phantom::*, *},
  crate::raytracing::*,
  nalgebra as na,
  serde::Deserialize,
  std::ops
};

type Matrix<const D: usize> = na::SMatrix<Float, D, D>;
#[derive(Debug, Clone)]
pub struct Transform<In: Space<3>, Out: Space<3>> {
  t: Matrix<4>,
  t_inv: Matrix<4>,
  det: Float,
  det_inv: Float,
  _phantom_in: Phantom<In>,
  _phantom_out: Phantom<Out>
}

impl<In: Space<3>, Out: Space<3>> Transform<In, Out> {
  pub fn identity() -> Self {
    Self {
      t: Matrix::<4>::identity(),
      t_inv: Matrix::<4>::identity(),
      det: 1.0,
      det_inv: 1.0,
      _phantom_in: Phantom::default(),
      _phantom_out: Phantom::default()
    }
  }

  pub fn from_raw(t: Matrix<4>) -> Option<Self> {
    t.try_inverse().map(|t_inv| Self {
      t,
      t_inv,
      det: t.determinant(),
      det_inv: t_inv.determinant(),
      _phantom_in: Phantom::default(),
      _phantom_out: Phantom::default()
    })
  }

  pub fn into_inverse(self) -> Transform<Out, In> {
    Transform {
      t: self.t_inv,
      t_inv: self.t,
      det: self.det_inv,
      det_inv: self.det,
      _phantom_in: self._phantom_out,
      _phantom_out: self._phantom_in
    }
  }

  pub fn clone_inverse(&self) -> Transform<Out, In> {
    Transform {
      t: self.t_inv,
      t_inv: self.t,
      det: self.det_inv,
      det_inv: self.det,
      _phantom_in: self._phantom_out,
      _phantom_out: self._phantom_in
    }
  }

  pub fn determinant(&self) -> Float { self.det }

  pub fn inverse_determinant(&self) -> Float { self.det_inv }

  pub fn vector(&self, vector: &Vector3<In>) -> Vector3<Out> {
    (self.t * vector.inner.to_homogeneous()).xyz().into()
  }

  pub fn point(&self, point: &Point3<In>) -> Point3<Out> {
    na::Point3::from_homogeneous(self.t * point.inner.to_homogeneous()).unwrap().into()
  }

  pub fn direction(&self, dir: &Direction3<In>) -> Direction3<Out> {
    let inner = dir.inner.into_inner();
    let d = (self.t * inner.to_homogeneous()).xyz();
    na::Unit::new_normalize(d).into()
  }

  pub fn normal(&self, sn: &Direction3<In>) -> Direction3<Out> {
    let v = (self.t_inv.transpose() * sn.inner.into_inner().to_homogeneous()).xyz();
    na::Unit::new_normalize(v).into()
  }

  pub fn ray(&self, ray: &Ray3<In>) -> Ray3<Out> {
    let dir: Vector3<In> = ray.dir().inner.into_inner().into();
    let transformed_dir = self.vector(&dir);
    let time_dilation = transformed_dir.norm();

    Ray3::new_with_time(
      ray.max_intersect_time() * time_dilation,
      self.point(&ray.origin()),
      na::Unit::new_unchecked((transformed_dir / time_dilation).inner).into()
    )
  }

  pub fn ray_intersect<'a>(
    &self,
    ray_intersection: &RayIntersection<'a, In>
  ) -> RayIntersection<'a, Out> {
    let ray = &ray_intersection.ray;
    let dir: Vector3<In> = ray.dir().inner.into_inner().into();
    let transformed_dir = self.vector(&dir);
    let time_dilation = transformed_dir.norm();

    let transformed_ray = Ray3::new_with_time(
      ray.max_intersect_time() * time_dilation,
      self.point(&ray.origin()),
      na::Unit::new_unchecked((transformed_dir / time_dilation).inner).into()
    );

    RayIntersection {
      ray: transformed_ray,
      surface: ray_intersection.surface,
      material: ray_intersection.material,
      intersect_time: ray_intersection.intersect_time * time_dilation,
      intersect_point: self.point(&ray_intersection.intersect_point),
      geometric_normal: self.normal(&ray_intersection.geometric_normal),
      shading_normal: self.normal(&ray_intersection.shading_normal),
      tex_coords: ray_intersection.tex_coords
    }
  }

  pub fn inverse_vector(&self, vector: &Vector3<Out>) -> Vector3<In> {
    (self.t_inv * vector.inner.to_homogeneous()).xyz().into()
  }

  pub fn inverse_point(&self, point: &Point3<Out>) -> Point3<In> {
    na::Point3::from_homogeneous(self.t_inv * point.inner.to_homogeneous()).unwrap().into()
  }

  pub fn inverse_direction(&self, dir: &Direction3<Out>) -> Direction3<In> {
    let inner = dir.inner.into_inner();
    let d = (self.t_inv * inner.to_homogeneous()).xyz();
    na::Unit::new_normalize(d).into()
  }

  pub fn inverse_normal(&self, sn: &Direction3<Out>) -> Direction3<In> {
    let v = (self.t.transpose() * sn.inner.into_inner().to_homogeneous()).xyz();
    na::Unit::new_normalize(v).into()
  }

  pub fn inverse_ray(&self, ray: &Ray3<Out>) -> Ray3<In> {
    let dir: Vector3<Out> = ray.dir().inner.into_inner().into();
    let transformed_dir = self.inverse_vector(&dir);
    let time_dilation = transformed_dir.norm();

    Ray3::new_with_time(
      ray.max_intersect_time() * time_dilation,
      self.inverse_point(&ray.origin()),
      na::Unit::new_unchecked((transformed_dir / time_dilation).inner).into()
    )
  }

  pub fn inverse_ray_intersect<'a>(
    &self,
    ray_intersection: &RayIntersection<'a, Out>
  ) -> RayIntersection<'a, In> {
    let ray = &ray_intersection.ray;
    let dir: Vector3<Out> = ray.dir().inner.into_inner().into();
    let transformed_dir = self.inverse_vector(&dir);
    let time_dilation = transformed_dir.norm();

    let transformed_ray = Ray3::new_with_time(
      ray.max_intersect_time() * time_dilation,
      self.inverse_point(&ray.origin()),
      na::Unit::new_unchecked((transformed_dir / time_dilation).inner).into()
    );

    RayIntersection {
      ray: transformed_ray,
      surface: ray_intersection.surface,
      material: ray_intersection.material,
      intersect_time: ray_intersection.intersect_time * time_dilation,
      intersect_point: self.inverse_point(&ray_intersection.intersect_point),
      geometric_normal: self.inverse_normal(&ray_intersection.geometric_normal),
      shading_normal: self.inverse_normal(&ray_intersection.shading_normal),
      tex_coords: ray_intersection.tex_coords
    }
  }
}

impl<In: Space<3>, Out: Space<3>> Display for Transform<In, Out> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.t) }
}

impl<In: Space<3>, Middle: Space<3>, Out: Space<3>> ops::Mul<Transform<In, Middle>>
  for Transform<Middle, Out>
{
  type Output = Transform<In, Out>;

  fn mul(self, rhs: Transform<In, Middle>) -> Self::Output {
    Transform {
      t: self.t * rhs.t,
      t_inv: rhs.t_inv * self.t_inv,
      det: self.det * rhs.det,
      det_inv: rhs.det_inv * self.det_inv,
      _phantom_in: rhs._phantom_in,
      _phantom_out: self._phantom_out
    }
  }
}

impl<In: Space<3>, Out: Space<3>> ops::Mul<&Point3<In>> for &Transform<In, Out> {
  type Output = Point3<Out>;

  fn mul(self, rhs: &Point3<In>) -> Self::Output { self.point(rhs) }
}

impl<In: Space<3>, Out: Space<3>> ops::Mul<&Vector3<In>> for &Transform<In, Out> {
  type Output = Vector3<Out>;

  fn mul(self, rhs: &Vector3<In>) -> Self::Output { self.vector(rhs) }
}

impl<In: Space<3>, Out: Space<3>> ops::Mul<&Direction3<In>> for &Transform<In, Out> {
  type Output = Direction3<Out>;

  fn mul(self, rhs: &Direction3<In>) -> Self::Output { self.direction(rhs) }
}

impl<In: Space<3>, Out: Space<3>> ops::Mul<&Ray3<In>> for &Transform<In, Out> {
  type Output = Ray3<Out>;

  fn mul(self, rhs: &Ray3<In>) -> Self::Output { self.ray(rhs) }
}

impl<In: Space<3>, Out: Space<3>> ops::MulAssign for Transform<In, Out> {
  fn mul_assign(&mut self, rhs: Self) {
    self.t = self.t * rhs.t;
    self.t_inv = rhs.t_inv * self.t_inv;
    self.det = self.det * rhs.det;
    self.det_inv = rhs.det_inv * self.det_inv;
  }
}

pub type LocalToWorld<S> = Transform<S, WorldSpace>;

fn array_to_vec(array: [Float; 3]) -> na::Vector3<Float> {
  na::vector![array[0], array[1], array[2]]
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum MatrixParameters {
  Translate { translate: [Float; 3] },
  UniformScale { scale: Float },
  NonUniformScale { scale: [Float; 3] },
  AxisAngle { axis: [Float; 3], angle: Float },
  LookAt { from: [Float; 3], at: [Float; 3], up: [Float; 3] }
}

impl MatrixParameters {
  pub fn build_matrix(self) -> na::Matrix4<Float> {
    match self {
      MatrixParameters::Translate { translate } => {
        na::Matrix4::new_translation(&array_to_vec(translate))
      },
      MatrixParameters::UniformScale { scale } => na::Matrix4::new_scaling(scale),
      MatrixParameters::NonUniformScale { scale } => {
        na::Matrix4::new_nonuniform_scaling(&array_to_vec(scale))
      },
      MatrixParameters::AxisAngle { axis, angle } => na::Matrix4::from_axis_angle(
        &na::Unit::new_normalize(array_to_vec(axis)),
        angle * PI / 180.0
      ),
      MatrixParameters::LookAt { from, at, up } => na::Matrix4::look_at_rh(
        &array_to_vec(from).into(),
        &array_to_vec(at).into(),
        &array_to_vec(up)
      )
    }
  }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum TransformParameters {
  Single(MatrixParameters),
  Composed(Vec<MatrixParameters>)
}

impl TransformParameters {
  pub fn build_transform<In: Space<3>, Out: Space<3>>(self) -> Transform<In, Out> {
    let mut matrix = na::Matrix4::identity();
    match self {
      TransformParameters::Single(m) => matrix = m.build_matrix(),
      TransformParameters::Composed(ms) => {
        for m in ms {
          matrix = m.build_matrix() * matrix;
        }
      },
    }

    if let Some(transform) = Transform::from_raw(matrix) {
      transform
    } else {
      panic!("Non-invertible transformation specified!")
    }
  }
}
