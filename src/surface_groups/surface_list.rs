use std::sync::Arc;

use serde::Deserialize;

use super::*;
use crate::{math::*, raytracing::*, sampling::Sampler, surfaces::Surface, BuildSettings};

#[derive(Debug, Deserialize)]
pub struct SurfaceListParameters {}

#[typetag::deserialize(name = "list")]
impl SurfaceGroupParameters for SurfaceListParameters {
  fn build_surface_group(
    &self,
    surfaces: Vec<Box<dyn Surface>>,
    _: BuildSettings
  ) -> Result<Arc<dyn SurfaceGroup>, Box<dyn std::error::Error>> {
    Ok(Arc::new(SurfaceList::build(surfaces)))
  }
}

#[derive(Debug)]
pub struct SurfaceList {
  surfaces: Vec<(Box<dyn Surface>, WorldBoundingBox)>,
  emitter_indices: Vec<usize>
}

impl SurfaceList {
  pub fn build(surfaces: Vec<Box<dyn Surface>>) -> Self {
    let bboxes: Vec<_> = surfaces.iter().map(|s| s.world_bounding_box()).collect();
    let emitter_indices = surfaces
      .iter()
      .enumerate()
      .filter_map(|(i, s)| s.material().is_emissive().then_some(i))
      .collect();

    Self { surfaces: surfaces.into_iter().zip(bboxes.into_iter()).collect(), emitter_indices }
  }
}

impl SurfaceGroup for SurfaceList {
  fn num_surfaces(&self) -> usize { self.surfaces.len() }

  fn intersect_world_ray(&self, mut ray: WorldRay) -> Option<WorldRayIntersection> {
    let mut closest = None;
    for (surface, bbox) in &self.surfaces {
      if bbox.ray_intersects_fast(&ray) {
        if let Some(hit) = surface.intersect_world_ray(ray.clone()) {
          ray.set_max_intersect_time(hit.intersect_time);
          closest = Some(hit);
        }
      }
    }

    closest
  }

  fn pdf(&self, point: &WorldPoint, direction: &WorldUnitVector) -> Real {
    self
      .emitter_indices
      .iter()
      .map(|i| self.surfaces[*i].0.intersecting_direction_pdf(point, direction))
      .sum::<Real>()
      / (self.emitter_indices.len() as Real)
  }

  fn sample_and_pdf(
    &self,
    point: &WorldPoint,
    sampler: &mut dyn Sampler
  ) -> (WorldUnitVector, Real) {
    let num_emitters = self.emitter_indices.len() as Real;
    let index = (sampler.next_non_one().into_inner() * num_emitters) as usize;
    let emitter = &self.surfaces[self.emitter_indices[index]].0;
    let direction = emitter.interesting_direction_sample(point, sampler).0;
    let pdf = self.pdf(point, &direction);
    (direction, pdf)
  }
}
