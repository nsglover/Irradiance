use std::{collections::HashMap, sync::Arc};

use nalgebra as na;
use serde::Deserialize;

use super::*;
use crate::{
  common::Wrapper, materials::Material, math::*, raytracing::*, textures::TextureCoordinate,
  BuildSettings
};

#[derive(Debug, Deserialize)]
pub struct QuadSurfaceParameters {
  transform: TransformParameters,
  material: String
}

#[typetag::deserialize(name = "quad")]
impl SurfaceParameters for QuadSurfaceParameters {
  fn build_surface(
    &self,
    materials: &HashMap<String, Arc<dyn Material>>,
    _: &HashMap<String, Mesh>,
    _: BuildSettings
  ) -> Box<dyn Surface> {
    Box::new(QuadSurface {
      transform: self.transform.clone().build_transform(),
      material: materials.get(&self.material).unwrap().clone(),
      normal: Vector3::from(na::vector![0.0, 0.0, 1.0]).normalize()
    })
  }

  fn is_emissive(&self, materials: &HashMap<String, Arc<dyn Material>>) -> bool {
    materials.get(&self.material).unwrap().is_emissive()
  }
}

#[derive(Debug, Clone, Copy)]
pub struct QuadSpace;

impl Space<3> for QuadSpace {}

#[derive(Debug)]
pub struct QuadSurface {
  transform: LocalToWorld<QuadSpace>,
  material: Arc<dyn Material>,
  normal: UnitVector3<<Self as TransformedSurface>::LocalSpace>
}

impl TransformedSurface for QuadSurface {
  type LocalSpace = QuadSpace;

  fn local_to_world(&self) -> &LocalToWorld<Self::LocalSpace> { &self.transform }

  fn intersect_ray(
    &self,
    ray: &mut Ray3<Self::LocalSpace>
  ) -> Option<(RayIntersection<Self::LocalSpace>, &dyn Material)> {
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
      let tex_coords = TextureCoordinate::from(na::vector![p.x + 0.5, p.y + 0.5]);
      Some((
        RayIntersection {
          intersect_direction: ray.dir(),
          intersect_time: t,
          intersect_point: p.into(),
          geometric_normal: self.normal,
          tex_coords
        },
        self.material.as_ref()
      ))
    })
  }

  fn local_bounding_box(&self) -> BoundingBox3<Self::LocalSpace> {
    BoundingBox3::new(na::point![-0.5, -0.5, 0.0].into(), na::point![0.5, 0.5, 0.0].into())
  }

  // fn intersecting_direction_sample(
  //   &self,
  //   point: &Point3<Self::LocalSpace>,
  //   sampler: &mut dyn Sampler
  // ) -> (UnitVector3<Self::LocalSpace>, Real) {
  //   let surface_point = na::point![sampler.next() - 0.5, sampler.next() - 0.5, 0.0];
  //   let (direction, dist) = (Point3::from(surface_point) - *point).normalize_with_norm();
  //   let cosine = direction.dot(&self.normal);

  //   // PDF is dist^2 / (cosine * area), and area is 1
  //   (direction, dist * dist / cosine)
  // }

  // fn intersecting_direction_pdf(
  //   &self,
  //   point: &Point3<Self::LocalSpace>,
  //   direction: &UnitVector3<Self::LocalSpace>
  // ) -> Real {
  //   if let Some(hit) = self.intersect_ray(&mut Ray3::new(*point, *direction)) {
  //     let cosine = direction.dot(&hit.0.geometric_normal);
  //     (hit.0.intersect_time * hit.0.intersect_time).into_inner() / cosine
  //   } else {
  //     0.0
  //   }
  // }
}
