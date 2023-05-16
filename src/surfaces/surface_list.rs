use std::{collections::HashMap, marker::PhantomData, sync::Arc};

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
    Box::new(SurfaceList::<BoxCheck>::build(
      self.surfaces.iter().map(|s| s.build_surface(materials, meshes, settings)).collect()
    ))
  }

  fn is_emissive(&self, materials: &HashMap<String, Arc<dyn Material>>) -> bool {
    self.surfaces.iter().any(|s| s.is_emissive(materials))
  }
}

#[derive(Debug)]
pub struct BoxCheck;

#[derive(Debug)]
pub struct NoBoxCheck;

#[derive(Debug)]
pub struct SurfaceList<Variant> {
  inverse_num_surfaces: PositiveReal,
  surfaces: Vec<(Box<dyn Surface>, WorldBoundingBox)>,
  bounding_box: WorldBoundingBox,
  _phantom: PhantomData<Variant>
}

impl<T> SurfaceList<T> {
  pub fn build(surfaces: Vec<Box<dyn Surface>>) -> Self {
    let bboxes: Vec<_> = surfaces.iter().map(|s| s.world_bounding_box()).collect();
    let bounding_box = bboxes.iter().fold(WorldBoundingBox::default(), |mut acc, b| {
      acc.enclose_box(b);
      acc
    });

    Self {
      inverse_num_surfaces: PositiveReal::new_unchecked(1.0 / surfaces.len() as Real),
      surfaces: surfaces.into_iter().zip(bboxes.into_iter()).collect(),
      bounding_box,
      _phantom: PhantomData::default()
    }
  }
}

impl<T: std::fmt::Debug> ContinuousRandomVariable for SurfaceList<T> {
  type Param = ();
  type Sample = (WorldRay, Color);

  fn sample(&self, _: &Self::Param, sampler: &mut dyn Sampler) -> Option<Self::Sample> {
    let index = sampler.random_in_closed_open(0.0, self.surfaces.len() as Real) as usize;
    let (surface, _) = &self.surfaces[index];
    surface.emitted_ray_random_variable().sample(&(), sampler)
  }

  fn sample_with_pdf(&self, _: &Self::Param, sampler: &mut dyn Sampler) -> Option<(Self::Sample, PositiveReal)> {
    if let Some(sample) = self.sample(&(), sampler) {
      if let Some(pdf) = self.pdf(&(), &sample) {
        return Some((sample, pdf));
      }
    }

    None
  }

  fn pdf(&self, param: &Self::Param, sample: &Self::Sample) -> Option<PositiveReal> {
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

impl Surface for SurfaceList<BoxCheck> {
  fn intersect_world_ray(&self, ray: &mut WorldRay) -> Option<WorldSurfaceInterface> {
    let mut closest = None;
    for (surface, bbox) in &self.surfaces {
      if bbox.ray_intersects(&ray) {
        if let Some(hit) = surface.intersect_world_ray(ray) {
          ray.set_max_intersect_time(hit.time);
          closest = Some(hit);
        }
      }
    }

    closest
  }

  fn emitted_ray_random_variable(&self) -> &dyn ContinuousRandomVariable<Param = (), Sample = (WorldRay, Color)> {
    self
  }

  fn world_bounding_box(&self) -> WorldBoundingBox { self.bounding_box.clone() }

  fn num_subsurfaces(&self) -> usize { self.surfaces.len() }
}

// TODO: This whole Box/NoBox Check thing is kinda hacky
impl Surface for SurfaceList<NoBoxCheck> {
  fn intersect_world_ray(&self, ray: &mut WorldRay) -> Option<WorldSurfaceInterface> {
    let mut closest = None;
    for (surface, _) in &self.surfaces {
      if let Some(hit) = surface.intersect_world_ray(ray) {
        ray.set_max_intersect_time(hit.time);
        closest = Some(hit);
      }
    }

    closest
  }

  fn emitted_ray_random_variable(&self) -> &dyn ContinuousRandomVariable<Param = (), Sample = (WorldRay, Color)> {
    self
  }

  fn world_bounding_box(&self) -> WorldBoundingBox { self.bounding_box.clone() }

  fn num_subsurfaces(&self) -> usize { self.surfaces.len() }
}
