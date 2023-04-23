use std::collections::HashMap;

use serde::Deserialize;

use super::*;
use crate::{
  common::Wrapper,
  materials::{Material, MaterialParameters},
  math::*,
  raytracing::*,
  samplers::Sampler
};

#[derive(Debug, Deserialize)]
pub struct SphereSurfaceParameters {
  transform: TransformParameters,
  material: String
}

#[typetag::deserialize(name = "sphere")]
impl SurfaceParameters for SphereSurfaceParameters {
  fn build_surface(
    &self,
    materials: &HashMap<String, Box<dyn MaterialParameters>>
  ) -> Box<dyn Surface> {
    Box::new(SphereSurface {
      transform: self.transform.clone().build_transform(),
      material: materials.get(&self.material).unwrap().build_material()
    })
  }
}

#[derive(Debug, Clone, Copy)]
pub struct SphereSpace;

impl Space<3> for SphereSpace {}

#[derive(Debug)]
pub struct SphereSurface {
  transform: LocalToWorld<SphereSpace>,
  material: Box<dyn Material>
}

impl TransformedSurface for SphereSurface {
  type LocalSpace = SphereSpace;

  fn local_to_world(&self) -> &LocalToWorld<Self::LocalSpace> { &self.transform }

  fn bounding_box(&self) -> BoundingBox3<Self::LocalSpace> {
    BoundingBox3::new(
      nalgebra::point![-1.0, -1.0, -1.0].into(),
      nalgebra::point![1.0, 1.0, 1.0].into()
    )
  }

  fn intersect_ray(
    &self,
    ray: Ray3<Self::LocalSpace>
  ) -> Option<RayIntersection<Self::LocalSpace>> {
    let origin_vec = Vector3::from(ray.origin());
    let b = 2.0 * Vector3::from(ray.dir()).dot(&origin_vec);
    let c = origin_vec.norm_squared() - 1.0;

    let discriminant = b * b - 4.0 * c;
    if discriminant < 0.0 {
      return None;
    }

    let q = -0.5 * (if b < 0.0 { b - discriminant.sqrt() } else { b + discriminant.sqrt() });
    let mut t1 = q;
    let mut t2 = c / q;
    if t1 > t2 {
      std::mem::swap(&mut t1, &mut t2);
    }

    let p;
    let t;
    if let Some((t1, p1)) = ray.at_real(t1) {
      t = t1;
      p = p1
    } else if let Some((t2, p2)) = ray.at_real(t2) {
      t = t2;
      p = p2
    } else {
      return None;
    }

    let normalized_p = Vector3::from(p).normalize();

    let phi = p.inner().y.atan2(p.inner().x);
    let theta = p.inner().z.asin();
    let u = (phi + PI) / (2.0 * PI);
    let v = (theta + PI / 2.0) / PI;

    Some(RayIntersection {
      ray,
      surface: self,
      intersect_time: t,
      intersect_point: Point::from_vector(normalized_p.into_vector()),
      geometric_normal: normalized_p,
      shading_normal: normalized_p,
      tex_coords: Vector::from_array([u, v])
    })
  }

  fn interesting_direction_sample(
    &self,
    _point: &Point3<Self::LocalSpace>,
    _sampler: &mut dyn Sampler
  ) -> (UnitVector3<Self::LocalSpace>, Real) {
    // ALREADY WRITTEN:
    // let radius = 1.0;
    // let (mut direction, distance) = Vector::from(point).normalize_with_norm();
    // direction = -direction; // Direction should point from the point to the origin
    // let dist2 = distance * distance;

    // SAMPLE:
    // float distance_squared = length2(direction);
    // ONBf onb;
    // onb.build_from_w(direction);
    // Vec3f ret = onb.toWorld(random_to_sphere(sample, radius, distance_squared));
    // if(pdf(o, ret) == 0)
    // cout << "sample: " << ret << "; " << pdf(o, ret) << endl;

    // PDF:
    // HitInfo hit;
    // if(this->intersect(Ray3f(o, v), hit)) {
    // 	Vec3f center = m_xform.point(Vec3f(0));
    // 	float radius2 = length2(m_xform.point(Vec3f(0, 0, m_radius)) - center);
    // 	float cos_theta_max = sqrt(1 - radius2 / length2(center - o));
    // 	float solid_angle = 2 * M_PI * (1 - cos_theta_max);
    // 	return  1 / solid_angle;
    // } else
    // 	return 0.000001f;

    // TODO: Move this to math module
    // float s = sample.x;
    // float t = sample.y;
    // float z = 1 + t * (sqrt(1 - radius * radius / distance_squared) - 1);
    // float phi = 2 * M_PI * s;
    // float x = cos(phi) * sqrt(1 - z * z);
    // float y = sin(phi) * sqrt(1 - z * z);
    // return {x, y, z};

    todo!()
  }

  fn intersecting_direction_pdf(
    &self,
    _point: &Point3<Self::LocalSpace>,
    _direction: &UnitVector3<Self::LocalSpace>
  ) -> Real {
    todo!()
  }

  fn material(&self) -> &dyn Material { self.material.as_ref() }
}
