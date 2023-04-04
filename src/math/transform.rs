use {
  super::{phantom::*, *},
  crate::ray::*,
  nalgebra as na,
  std::ops
};

type Matrix<const D: usize> = na::SMatrix<Float, D, D>;

// TODO: Add another parameter here for transformation type, which will encode precisely how
// structures, particularly directions and surface normals, are affected. Replace the matrices
// below with this, which will allow us to optimize out a lot of the unnecessary normalize calls in
// place of fast_normalize calls.
#[derive(Clone)]
pub struct Transform<In: Space<3>, Out: Space<3>> {
  t: Matrix<4>,
  t_inv: Matrix<4>,
  det: Float,
  det_inv: Float,
  _phantom_in: Phantom<In>,
  _phantom_out: Phantom<Out>
}

impl<In: Space<3>, Out: Space<3>> Transform<In, Out> {
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
    let v = na::Vector3::from_homogeneous(self.t * vector.inner.to_homogeneous()).unwrap();
    Vector { inner: v, _phantom: vector._phantom.into_other() }
  }

  pub fn point(&self, point: &Point3<In>) -> Point3<Out> {
    let p = na::Point3::from_homogeneous(self.t * point.inner.to_homogeneous()).unwrap();
    Point { inner: p, _phantom: point._phantom.into_other() }
  }

  pub fn direction(&self, dir: &Direction3<In>) -> Direction3<Out> {
    let inner = dir.inner.into_inner();
    let d = na::Vector3::from_homogeneous(self.t * inner.to_homogeneous()).unwrap();
    Direction { inner: na::Unit::new_normalize(d), _phantom: dir._phantom.into_other() }
  }

  pub fn normal(&self, sn: &Normal3<In>) -> Normal3<Out> {
    let inner = sn.inner.into_inner();
    let v = na::Vector3::from_homogeneous(self.t_inv.transpose() * inner.to_homogeneous());
    Normal { inner: na::Unit::new_normalize(v.unwrap()), _phantom: sn._phantom.into_other() }
  }

  pub fn ray(&self, ray: &Ray3<In>) -> Ray3<Out> {
    Ray {
      time_bounds: ray.time_bounds,
      origin: self.point(&ray.origin),
      dir: self.direction(&ray.dir)
    }
  }

  pub fn inverse_vector(&self, vector: &Vector3<Out>) -> Vector3<In> {
    let v = na::Vector3::from_homogeneous(self.t_inv * vector.inner.to_homogeneous()).unwrap();
    Vector { inner: v, _phantom: vector._phantom.into_other() }
  }

  pub fn inverse_point(&self, point: &Point3<Out>) -> Point3<In> {
    let p = na::Point3::from_homogeneous(self.t_inv * point.inner.to_homogeneous()).unwrap();
    Point { inner: p, _phantom: point._phantom.into_other() }
  }

  pub fn inverse_direction(&self, dir: &Direction3<Out>) -> Direction3<In> {
    let inner = dir.inner.into_inner();
    let d = na::Vector3::from_homogeneous(self.t_inv * inner.to_homogeneous()).unwrap();
    Direction { inner: na::Unit::new_normalize(d), _phantom: dir._phantom.into_other() }
  }

  pub fn inverse_normal(&self, sn: &Normal3<Out>) -> Normal3<In> {
    let inner = sn.inner.into_inner();
    let v = na::Vector3::from_homogeneous(self.t.transpose() * inner.to_homogeneous());
    Normal { inner: na::Unit::new_normalize(v.unwrap()), _phantom: sn._phantom.into_other() }
  }

  pub fn inverse_ray(&self, ray: &Ray3<Out>) -> Ray3<In> {
    Ray {
      time_bounds: ray.time_bounds,
      origin: self.inverse_point(&ray.origin),
      dir: self.inverse_direction(&ray.dir)
    }
  }
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

impl<In: Space<3>, Out: Space<3>> ops::Mul<&Normal3<In>> for &Transform<In, Out> {
  type Output = Normal3<Out>;

  fn mul(self, rhs: &Normal3<In>) -> Self::Output { self.normal(rhs) }
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
