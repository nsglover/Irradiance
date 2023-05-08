use std::{collections::HashMap, sync::Arc};

use serde::Deserialize;

use super::*;
use crate::{
  light::Color,
  materials::Material,
  math::*,
  raytracing::*,
  sampling::{ContinuousRandomVariable, Sampler},
  surfaces::Surface,
  BuildSettings
};

#[derive(Debug, Deserialize)]
pub struct SurfaceListParameters {
  #[serde(alias = "sub-surfaces")]
  pub surfaces: Vec<Box<dyn SurfaceParameters>>
}

#[typetag::deserialize(name = "list")]
impl SurfaceParameters for SurfaceListParameters {
  fn build_surface(
    &self,
    materials: &std::collections::HashMap<String, Arc<dyn Material>>,
    meshes: &std::collections::HashMap<String, Mesh>,
    settings: BuildSettings
  ) -> Box<dyn Surface> {
    Box::new(SurfaceList::build(self.surfaces.iter().map(|s| s.build_surface(materials, meshes, settings)).collect()))
  }

  fn is_emissive(&self, materials: &HashMap<String, Arc<dyn Material>>) -> bool {
    self.surfaces.iter().any(|s| s.is_emissive(materials))
  }
}

#[derive(Debug)]
pub struct SurfaceList {
  inverse_num_surfaces: PositiveReal,
  surfaces: Vec<(Box<dyn Surface>, WorldBoundingBox)>,
  bounding_box: WorldBoundingBox
}

impl SurfaceList {
  pub fn build(surfaces: Vec<Box<dyn Surface>>) -> Self {
    let bboxes: Vec<_> = surfaces.iter().map(|s| s.world_bounding_box()).collect();
    let bounding_box = bboxes.iter().fold(WorldBoundingBox::default(), |mut acc, b| {
      acc.enclose_box(b);
      acc
    });

    Self {
      inverse_num_surfaces: PositiveReal::new_unchecked(1.0 / surfaces.len() as Real),
      surfaces: surfaces.into_iter().zip(bboxes.into_iter()).collect(),
      bounding_box
    }
  }
}

impl ContinuousRandomVariable<(), (WorldRay, Color)> for SurfaceList {
  fn sample(&self, param: &(), sampler: &mut dyn Sampler) -> Option<(WorldRay, Color)> {
    let index = sampler.random_in_closed_open(0.0, self.surfaces.len() as Real) as usize;
    let (surface, _) = &self.surfaces[index];
    surface.emitted_ray_random_variable().sample(param, sampler)
  }

  fn sample_with_pdf(&self, param: &(), sampler: &mut dyn Sampler) -> Option<((WorldRay, Color), PositiveReal)> {
    if let Some(sample) = self.sample(param, sampler) {
      if let Some(pdf) = self.pdf(param, &sample) {
        return Some((sample, pdf));
      }
    }

    None
  }

  fn pdf(&self, param: &(), sample: &(WorldRay, Color)) -> Option<PositiveReal> {
    PositiveReal::new(
      self
        .surfaces
        .iter()
        .filter_map(|(s, _)| s.emitted_ray_random_variable().pdf(param, sample).map(|p| p.into_inner()))
        .sum::<Real>()
        * self.inverse_num_surfaces
    )
  }
}

impl Surface for SurfaceList {
  fn intersect_world_ray(&self, ray: &mut WorldRay) -> Option<(WorldRayIntersection, &dyn Material)> {
    let mut closest = None;
    for (surface, bbox) in &self.surfaces {
      if bbox.ray_intersects_fast(&ray) {
        if let Some(hit) = surface.intersect_world_ray(ray) {
          ray.set_max_intersect_time(hit.0.intersect_time);
          closest = Some(hit);
        }
      }
    }

    closest
  }

  fn emitted_ray_random_variable(&self) -> &dyn ContinuousRandomVariable<(), (WorldRay, Color)> { self }

  fn world_bounding_box(&self) -> WorldBoundingBox { self.bounding_box.clone() }

  fn num_subsurfaces(&self) -> usize { self.surfaces.len() }

  // fn pdf(&self, point: &WorldPoint, direction: &WorldUnitVector) -> Real {
  //   self
  //     .emissive_indices
  //     .iter()
  //     .map(|i| self.surfaces[*i].0.intersecting_direction_pdf(point, direction))
  //     .sum::<Real>()
  //     / (self.emissive_indices.len() as Real)
  // }

  // fn sample_and_pdf(
  //   &self,
  //   point: &WorldPoint,
  //   sampler: &mut dyn Sampler
  // ) -> (WorldUnitVector, Real) {
  //   let num_emissives = self.emissive_indices.len() as Real;
  //   let index = (sampler.next_non_one().into_inner() * num_emissives) as usize;
  //   let emissive = &self.surfaces[self.emissive_indices[index]].0;
  //   let direction = emissive.interesting_direction_sample(point, sampler).0;
  //   let pdf = self.pdf(point, &direction);
  //   (direction, pdf)
  // }
}
