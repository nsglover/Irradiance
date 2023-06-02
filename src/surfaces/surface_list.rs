use std::{marker::PhantomData, sync::Arc};

use serde::Deserialize;

use super::*;
use crate::{
  materials::Material, math::*, raytracing::*, sampling::ContinuousRandomVariable, surfaces::Surface, BuildSettings
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
    lights: &std::collections::HashMap<String, Arc<dyn Light>>,
    materials: &std::collections::HashMap<String, Arc<dyn Material>>,
    meshes: &std::collections::HashMap<String, Mesh>,
    settings: BuildSettings
  ) -> Box<dyn Surface> {
    Box::new(SurfaceList::<BoxCheck>::build(
      self.surfaces.iter().map(|s| s.build_surface(lights, materials, meshes, settings)).collect()
    ))
  }

  fn has_light(&self) -> bool { self.surfaces.iter().any(|s| s.has_light()) }
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
    let bboxes: Vec<_> = surfaces.iter().map(|s| s.world_bounding_box().clone()).collect();
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

impl Surface for SurfaceList<BoxCheck> {
  fn intersect_world_ray(&self, ray: &mut WorldRay) -> Option<WorldSurfaceInterface> {
    let mut closest = None;
    for (surface, bbox) in &self.surfaces {
      if bbox.ray_intersects(&ray) {
        if let Some(hit) = surface.intersect_world_ray(ray) {
          ray.set_max_intersect_dist(hit.intersect_dist);
          closest = Some(hit);
        }
      }
    }

    closest
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

impl Surface for SurfaceList<NoBoxCheck> {
  fn intersect_world_ray(&self, ray: &mut WorldRay) -> Option<WorldSurfaceInterface> {
    let mut closest = None;
    for (surface, _) in &self.surfaces {
      if let Some(hit) = surface.intersect_world_ray(ray) {
        ray.set_max_intersect_dist(hit.intersect_dist);
        closest = Some(hit);
      }
    }

    closest
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
