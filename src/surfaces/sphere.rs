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
  textures::TextureCoordinate,
  BuildSettings
};

#[derive(Debug, Deserialize)]
pub struct SphereSurfaceParameters {
  center: [Real; 3],
  radius: Real,
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
    let radius = PositiveReal::new(self.radius).expect("Sphere radius must be positive");
    Box::new(SphereSurface {
      radius,
      radius_squared: radius * radius,
      center: Point::from_array(self.center),
      inverse_area: PositiveReal::new_unchecked(1.0 / (4.0 * PI * radius * radius)),
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
  radius: PositiveReal,
  radius_squared: PositiveReal,
  inverse_area: PositiveReal,
  center: WorldPoint,
  material: Arc<dyn Material>
}

impl ContinuousRandomVariable for SphereSurface {
  type Param = ();
  type Sample = (WorldRay, Color);

  fn sample_with_pdf(&self, _: &Self::Param, sampler: &mut dyn Sampler) -> Option<(Self::Sample, PositiveReal)> {
    self
      .material
      .emit_random_variable()
      .map(|rv| {
        let normal = uniform_random_on_unit_sphere(sampler);
        let point = self.center + normal * self.radius.into_inner();

        rv.sample_with_pdf(&(point, normal), sampler)
          .map(|((dir, light), pdf)| ((Ray::new(point, dir), light * dir.dot(&normal).abs()), pdf * self.inverse_area))
      })
      .flatten()
  }

  fn pdf(&self, _: &Self::Param, sample: &Self::Sample) -> Option<PositiveReal> {
    self
      .material
      .emit_random_variable()
      .map(|rv| {
        rv.pdf(&(sample.0.origin(), (sample.0.origin() - self.center).normalize()), &(sample.0.dir(), sample.1))
          .map(|p| p * self.inverse_area)
      })
      .flatten()
  }
}

impl Surface for SphereSurface {
  fn intersect_world_ray(&self, ray: &mut WorldRay) -> Option<WorldSurfaceInterface> {
    let o_minus_c = ray.origin() - self.center;
    let b = 2.0 * Vector3::from(ray.dir()).dot(&o_minus_c);
    let c = o_minus_c.norm_squared() - self.radius_squared;

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

    let normal = (p - self.center).normalize();

    let n = normal.inner();
    let phi = n.y.atan2(n.x);
    let theta = n.z.asin();
    let u = (phi + PI) * INV_PI / 2.0;
    let v = (theta + PI / 2.0) * INV_PI;

    Some(SurfaceInterface {
      surface_point: SurfacePoint {
        point: self.center + normal * self.radius.into_inner(),
        geometric_normal: normal,
        shading_normal: normal,
        tex_coord: TextureCoordinate::from_array([u, v])
      },
      material: self.material.as_ref(),
      time: t
    })
  }

  fn emitted_ray_random_variable(&self) -> &dyn ContinuousRandomVariable<Param = (), Sample = (WorldRay, Color)> {
    self
  }

  fn world_bounding_box(&self) -> WorldBoundingBox {
    let r = self.radius.into_inner();
    BoundingBox3::new(
      self.center + nalgebra::vector![-r, -r, -r].into(),
      self.center + nalgebra::vector![r, r, r].into()
    )
  }

  fn num_subsurfaces(&self) -> usize { 0 }

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
