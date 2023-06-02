use std::{collections::HashMap, sync::Arc};

use serde::Deserialize;

use super::*;
use crate::{
  lights::{Light, NullLight},
  materials::{Material, NullMaterial},
  math::*,
  raytracing::*,
  sampling::ContinuousRandomVariable,
  textures::TextureCoordinate,
  BuildSettings
};

#[derive(Debug, Deserialize)]
pub struct SphereSurfaceParameters {
  center: [Real; 3],
  radius: Real,
  light: Option<String>,
  material: Option<String>
}

#[typetag::deserialize(name = "sphere")]
impl SurfaceParameters for SphereSurfaceParameters {
  fn build_surface(
    &self,
    lights: &HashMap<String, Arc<dyn Light>>,
    materials: &HashMap<String, Arc<dyn Material>>,
    _: &HashMap<String, Mesh>,
    _: BuildSettings
  ) -> Box<dyn Surface> {
    let r = self.radius;
    let radius = PositiveReal::new(r).expect("Sphere radius must be positive");
    let center = Point::from_array(self.center);
    Box::new(SphereSurface {
      light: self.material.as_ref().map(|m| lights.get(m).unwrap().clone()).unwrap_or(Arc::new(NullLight::default())),
      material: self
        .light
        .as_ref()
        .map(|l| materials.get(l).unwrap().clone())
        .unwrap_or(Arc::new(NullMaterial::default())),
      radius,
      radius_squared: radius * radius,
      inverse_area: PositiveReal::new_unchecked(1.0 / (4.0 * PI * radius * radius)),
      center,
      bounding_box: {
        BoundingBox3::new(center + nalgebra::vector![-r, -r, -r].into(), center + nalgebra::vector![r, r, r].into())
      }
    })
  }

  fn has_light(&self) -> bool { self.light.is_some() }
}

#[derive(Debug, Clone, Copy)]
pub struct SphereSpace;

impl Space<3> for SphereSpace {}

#[derive(Debug)]
pub struct SphereSurface {
  light: Arc<dyn Light>,
  material: Arc<dyn Material>,
  radius: PositiveReal,
  radius_squared: PositiveReal,
  inverse_area: PositiveReal,
  center: WorldPoint,
  bounding_box: WorldBoundingBox
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
      light: self.light.as_ref(),
      material: self.material.as_ref(),
      intersect_dist: t
    })
  }

  fn random_surface_interface(&self) -> &dyn ContinuousRandomVariable<Param = (), Sample = WorldSurfaceInterface> {
    todo!()
  }

  fn random_intersecting_direction(
    &self
  ) -> &dyn ContinuousRandomVariable<Param = WorldPoint, Sample = WorldUnitVector> {
    todo!()
  }

  fn world_bounding_box(&self) -> &WorldBoundingBox { &self.bounding_box }
}
