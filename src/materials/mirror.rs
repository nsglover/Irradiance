use std::sync::Arc;

use serde::Deserialize;

use super::*;
use crate::{
  light::Color,
  math::{WorldPoint, WorldUnitVector},
  raytracing::*,
  sampling::*,
  textures::*
};

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
      scatter_random_var: ScatterRandomVariable::Specular(Box::new(ReflectRandomVariable))
    })
  }
}

#[derive(Debug)]
struct ReflectRandomVariable;

impl DiscreteRandomVariable for ReflectRandomVariable {
  type Param = (WorldSurfacePoint, WorldUnitVector);
  type Sample = WorldUnitVector;

  fn sample(&self, (hit, out_dir): &Self::Param, _: &mut dyn Sampler) -> Option<WorldUnitVector> {
    Some(out_dir.reflect_about(hit.geometric_normal))
  }
}

#[derive(Debug)]
pub struct Mirror {
  albedo: Arc<dyn Texture>,
  scatter_random_var: ScatterRandomVariable
}

impl Material for Mirror {
  fn emitted(&self, _: &WorldSurfaceInterface) -> Option<Color> { None }

  fn bsdf_cos(&self, hit: &WorldSurfacePoint, _: &WorldUnitVector, _: &WorldUnitVector) -> Color {
    self.albedo.value(&hit.tex_coord)
  }

  fn scatter_random_variable(&self) -> Option<&ScatterRandomVariable> { Some(&self.scatter_random_var) }

  fn emit_random_variable(
    &self
  ) -> Option<&dyn ContinuousRandomVariable<Param = (WorldPoint, WorldUnitVector), Sample = (WorldUnitVector, Color)>>
  {
    None
  }
}
