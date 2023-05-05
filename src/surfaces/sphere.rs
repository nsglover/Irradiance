use std::{collections::HashMap, sync::Arc};

use serde::Deserialize;

use super::*;
use crate::{
  common::Wrapper,
  light::Color,
  materials::Material,
  math::*,
  raytracing::*,
  sampling::{uniform_random_on_unit_sphere, ContinuousRandomVariable, Sampler},
  BuildSettings
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
    materials: &HashMap<String, Arc<dyn Material>>,
    _: &HashMap<String, Mesh>,
    _: BuildSettings
  ) -> Box<dyn Surface> {
    let t = self.transform.clone().build_transform();
    let r = PositiveReal::new_unchecked(t.vector(&Vector::from_array([1.0, 0.0, 0.0])).norm());
    Box::new(SphereSurface {
      transformed_radius: r,
      transformed_center: t.point(&Point::origin()),
      inverse_transformed_area: PositiveReal::new_unchecked(1.0 / (4.0 * PI * r * r)),
      transform: t,
      material: materials.get(&self.material).unwrap().clone()
    })
  }

  fn is_emissive(&self, materials: &HashMap<String, Arc<dyn Material>>) -> bool {
    materials.get(&self.material).unwrap().emit_random_variable().is_some()
  }
}

#[derive(Debug, Clone, Copy)]
pub struct SphereSpace;

impl Space<3> for SphereSpace {}

#[derive(Debug)]
pub struct SphereSurface {
  transform: LocalToWorld<SphereSpace>,
  transformed_radius: PositiveReal,
  inverse_transformed_area: PositiveReal,
  transformed_center: WorldPoint,
  material: Arc<dyn Material>
}

impl ContinuousRandomVariable<(), (WorldRay, Color)> for SphereSurface {
  fn sample_with_pdf(
    &self,
    _: &(),
    sampler: &mut dyn Sampler
  ) -> Option<((WorldRay, Color), PositiveReal)> {
    self
      .material
      .emit_random_variable()
      .map(|rv| {
        let normal = uniform_random_on_unit_sphere(sampler);
        let point = self.transformed_center + normal * self.transformed_radius.into_inner();

        rv.sample_with_pdf(&(point, normal), sampler).map(|((dir, light), pdf)| {
          (
            (Ray::new(point, dir), light * dir.dot(&normal).abs()),
            pdf * self.inverse_transformed_area
          )
        })
      })
      .flatten()
  }

  fn pdf(&self, _: &(), sample: &(WorldRay, Color)) -> Option<PositiveReal> {
    self
      .material
      .emit_random_variable()
      .map(|rv| {
        rv.pdf(
          &(sample.0.origin(), (sample.0.origin() - self.transformed_center).normalize()),
          &(sample.0.dir(), sample.1)
        )
        .map(|p| p * self.inverse_transformed_area)
      })
      .flatten()
  }
}

impl TransformedSurface for SphereSurface {
  type LocalSpace = SphereSpace;

  fn local_to_world(&self) -> &LocalToWorld<Self::LocalSpace> { &self.transform }

  fn intersect_ray(
    &self,
    ray: &mut Ray3<Self::LocalSpace>
  ) -> Option<(RayIntersection<Self::LocalSpace>, &dyn Material)> {
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

    Some((
      RayIntersection {
        intersect_direction: ray.dir(),
        intersect_time: t,
        intersect_point: Point::from_vector(normalized_p.into_vector()),
        geometric_normal: normalized_p,
        tex_coords: Vector::from_array([u, v])
      },
      self.material.as_ref()
    ))
  }

  fn emitted_ray_random_variable(&self) -> &dyn ContinuousRandomVariable<(), (WorldRay, Color)> {
    self
  }

  fn local_bounding_box(&self) -> BoundingBox3<Self::LocalSpace> {
    BoundingBox3::new(
      nalgebra::point![-1.0, -1.0, -1.0].into(),
      nalgebra::point![1.0, 1.0, 1.0].into()
    )
  }

  fn num_subsurfaces(&self) -> usize { 1 }

  // fn intersecting_direction_sample(
  //   &self,
  //   _point: &Point3<Self::LocalSpace>,
  //   _sampler: &mut dyn Sampler
  // ) -> (UnitVector3<Self::LocalSpace>, Real) {
  //   // ALREADY WRITTEN:
  //   // let radius = 1.0;
  //   // let (mut direction, distance) = Vector::from(point).normalize_with_norm();
  //   // direction = -direction; // Direction should point from the point to the origin
  //   // let dist2 = distance * distance;

  //   // SAMPLE:
  //   // float distance_squared = length2(direction);
  //   // ONBf onb;
  //   // onb.build_from_w(direction);
  //   // Vec3f ret = onb.toWorld(random_to_sphere(sample, radius, distance_squared));
  //   // if(pdf(o, ret) == 0)
  //   // cout << "sample: " << ret << "; " << pdf(o, ret) << endl;

  //   // PDF:
  //   // HitInfo hit;
  //   // if(this->intersect(Ray3f(o, v), hit)) {
  //   // 	Vec3f center = m_xform.point(Vec3f(0));
  //   // 	float radius2 = length2(m_xform.point(Vec3f(0, 0, m_radius)) - center);
  //   // 	float cos_theta_max = sqrt(1 - radius2 / length2(center - o));
  //   // 	float solid_angle = 2 * M_PI * (1 - cos_theta_max);
  //   // 	return  1 / solid_angle;
  //   // } else
  //   // 	return 0.000001f;

  //   // TODO: Move this to math module
  //   // float s = sample.x;
  //   // float t = sample.y;
  //   // float z = 1 + t * (sqrt(1 - radius * radius / distance_squared) - 1);
  //   // float phi = 2 * M_PI * s;
  //   // float x = cos(phi) * sqrt(1 - z * z);
  //   // float y = sin(phi) * sqrt(1 - z * z);
  //   // return {x, y, z};

  // }
}
