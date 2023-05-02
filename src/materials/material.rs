use std::{fmt::Debug, sync::Arc};

use crate::{light::*, math::*, raytracing::*, sampling::RandomVariable};

#[typetag::deserialize(tag = "type")]
pub trait MaterialParameters: Debug {
  fn name(&self) -> String;

  fn build_material(&self) -> Arc<dyn Material>;
}

pub type ScatterRandomVariable = RandomVariable<WorldRayIntersection, WorldUnitVector>;

pub trait Material: Debug {
  fn emitted(&self, hit: &WorldRayIntersection) -> Option<Color>;

  fn bsdf(&self, hit: &WorldRayIntersection, scattered_dir: &WorldUnitVector) -> Color;

  fn scatter_random_variable(&self) -> Option<&ScatterRandomVariable>;

  fn is_emissive(&self) -> bool;
}
