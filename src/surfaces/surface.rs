use std::{collections::HashMap, fmt::Debug, sync::Arc};

use super::Mesh;
use crate::{
  lights::Light, materials::Material, math::*, raytracing::*, sampling::ContinuousRandomVariable, BuildSettings
};

#[typetag::deserialize(tag = "type")]
pub trait SurfaceParameters: Debug {
  fn build_surface(
    &self,
    lights: &HashMap<String, Arc<dyn Light>>,
    materials: &HashMap<String, Arc<dyn Material>>,
    meshes: &HashMap<String, Mesh>,
    settings: BuildSettings
  ) -> Box<dyn Surface>;

  fn has_light(&self) -> bool;
}

pub trait Surface: Debug {
  fn intersect_world_ray(&self, ray: &mut WorldRay) -> Option<WorldSurfaceInterface>;

  fn random_surface_interface(&self) -> &dyn ContinuousRandomVariable<Param = (), Sample = WorldSurfaceInterface>;

  fn random_intersecting_direction(
    &self
  ) -> &dyn ContinuousRandomVariable<Param = WorldPoint, Sample = WorldUnitVector>;

  fn world_bounding_box(&self) -> &WorldBoundingBox;
}
