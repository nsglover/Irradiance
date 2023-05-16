use std::{collections::HashMap, fmt::Debug, sync::Arc};

use super::Mesh;
use crate::{
  light::Color, materials::Material, math::*, raytracing::*, sampling::ContinuousRandomVariable, BuildSettings
};

#[typetag::deserialize(tag = "type")]
pub trait SurfaceParameters: Debug {
  fn build_surface(
    &self,
    materials: &HashMap<String, Arc<dyn Material>>,
    meshes: &HashMap<String, Mesh>,
    settings: BuildSettings
  ) -> Box<dyn Surface>;

  fn is_emissive(&self, materials: &HashMap<String, Arc<dyn Material>>) -> bool;
}

pub trait Surface: Debug {
  fn intersect_world_ray(&self, ray: &mut WorldRay) -> Option<WorldSurfaceInterface>;

  fn emitted_ray_random_variable(&self) -> &dyn ContinuousRandomVariable<Param = (), Sample = (WorldRay, Color)>;

  fn world_bounding_box(&self) -> WorldBoundingBox;

  fn num_subsurfaces(&self) -> usize;
}
