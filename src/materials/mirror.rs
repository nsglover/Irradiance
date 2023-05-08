use std::sync::Arc;

use serde::Deserialize;

use super::*;
use crate::{light::Color, math::WorldUnitVector, raytracing::*, sampling::*, textures::*};

#[derive(Debug, Deserialize)]
struct MirrorParameters {
  name: String,
  albedo: Box<dyn TextureParameters>
}

#[typetag::deserialize(name = "mirror")]
impl MaterialParameters for MirrorParameters {
  fn name(&self) -> String { self.name.clone() }

  fn build_material(&self) -> Arc<dyn Material> {
    Arc::new(Mirror {
      albedo: self.albedo.build_texture(),
      scatter_random_var: ScatterRandomVariable::Discrete(Box::new(ReflectRandomVariable))
    })
  }
}

#[derive(Debug)]
struct ReflectRandomVariable;

impl DiscreteRandomVariable<WorldRayIntersection, WorldUnitVector> for ReflectRandomVariable {
  fn sample(&self, param: &WorldRayIntersection, _: &mut dyn Sampler) -> Option<WorldUnitVector> {
    Some((-param.intersect_direction).reflect_about(param.geometric_normal))
  }
}

#[derive(Debug)]
pub struct Mirror {
  albedo: Arc<dyn Texture>,
  scatter_random_var: ScatterRandomVariable
}

impl Material for Mirror {
  fn emitted(&self, _: &WorldRayIntersection) -> Option<Color> { None }

  fn bsdf(&self, hit: &WorldRayIntersection, _: &WorldUnitVector) -> Color { self.albedo.value(&hit.tex_coords) }

  fn scatter_random_variable(&self) -> Option<&ScatterRandomVariable> { Some(&self.scatter_random_var) }

  fn emit_random_variable(
    &self
  ) -> Option<&dyn ContinuousRandomVariable<(crate::math::WorldPoint, WorldUnitVector), (WorldUnitVector, Color)>> {
    None
  }
}
