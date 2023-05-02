use std::{collections::HashMap, rc::Rc};

use nalgebra as na;
use serde::Deserialize;

use super::*;
use crate::{
  common::Wrapper, materials::Material, math::*, raytracing::*, samplers::Sampler,
  textures::TextureCoordinate
};

#[derive(Debug, Deserialize)]
pub struct QuadSurfaceParameters {
  transform: TransformParameters,
  material: String
}

#[typetag::deserialize(name = "quad")]
impl SurfaceParameters for QuadSurfaceParameters {
  fn build_surfaces(
    &self,
    materials: &HashMap<String, Rc<dyn Material>>,
    _: &HashMap<String, Mesh>
  ) -> Vec<Box<dyn Surface>> {
    vec![Box::new(QuadSurface {
      transform: self.transform.clone().build_transform(),
      material: materials.get(&self.material).unwrap().clone(),
      normal: Vector3::from(na::vector![0.0, 0.0, 1.0]).normalize()
    })]
  }
}

#[derive(Debug, Clone, Copy)]
pub struct QuadSpace;

impl Space<3> for QuadSpace {}

#[derive(Debug)]
pub struct QuadSurface {
  transform: LocalToWorld<QuadSpace>,
  material: Rc<dyn Material>,
  normal: UnitVector3<<Self as TransformedSurface>::LocalSpace>
}

impl TransformedSurface for QuadSurface {
  type LocalSpace = QuadSpace;

  fn local_to_world(&self) -> &LocalToWorld<Self::LocalSpace> { &self.transform }

  fn bounding_box(&self) -> BoundingBox3<Self::LocalSpace> {
    BoundingBox3::new(na::point![-0.5, -0.5, 0.0].into(), na::point![0.5, 0.5, 0.0].into())
  }

  fn intersect_ray(
    &self,
    ray: Ray3<Self::LocalSpace>
  ) -> Option<RayIntersection<Self::LocalSpace>> {
    if ray.dir().inner().z == 0.0 {
      return None;
    }

    let t = -ray.origin().inner().z / ray.dir().inner().z;
    ray.at_real(t).and_then(|(t, point)| {
      let mut p = point.into_inner();
      if 0.5 < p.x || -0.5 > p.x || 0.5 < p.y || -0.5 > p.y {
        return None;
      }

      p.z = 0.0;
      let mut normal = self.normal;
      if ray.dir().dot(&normal) > 0.0 {
        normal = -normal;
      }

      let tex_coords = TextureCoordinate::from(na::vector![p.x + 0.5, p.y + 0.5]);
      Some(RayIntersection {
        ray,
        surface: self,
        intersect_time: t,
        intersect_point: p.into(),
        geometric_normal: normal,
        shading_normal: normal,
        tex_coords
      })
    })
  }

  fn interesting_direction_sample(
    &self,
    point: &Point3<Self::LocalSpace>,
    sampler: &mut dyn Sampler
  ) -> (UnitVector3<Self::LocalSpace>, Real) {
    let surface_point = na::point![sampler.next() - 0.5, sampler.next() - 0.5, 0.0];
    let (direction, dist) = (Point3::from(surface_point) - *point).normalize_with_norm();
    let cosine = direction.dot(&self.normal);

    // PDF is dist^2 / (cosine * area), and area is 1
    (direction, dist * dist / cosine)
  }

  fn intersecting_direction_pdf(
    &self,
    point: &Point3<Self::LocalSpace>,
    direction: &UnitVector3<Self::LocalSpace>
  ) -> Real {
    if let Some(hit) = self.intersect_ray(Ray3::new(*point, *direction)) {
      let cosine = direction.dot(&hit.geometric_normal);
      (hit.intersect_time * hit.intersect_time).into_inner() / cosine
    } else {
      0.0
    }
  }

  fn material(&self) -> &dyn Material { self.material.as_ref() }
}
