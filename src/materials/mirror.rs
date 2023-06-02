use std::sync::Arc;

use serde::Deserialize;

use super::*;
use crate::{math::WorldUnitVector, raytracing::*, sampling::*, spectrum::Spectrum, textures::*};

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
    Some(out_dir.reflect_about(hit.shading_normal))
  }
}

#[derive(Debug)]
pub struct Mirror {
  albedo: Arc<dyn Texture>,
  scatter_random_var: ScatterRandomVariable
}

impl Material for Mirror {
  fn bsdf_cos(&self, hit: &WorldSurfacePoint, _: &WorldUnitVector, _: &WorldUnitVector) -> Spectrum {
    self.albedo.value(&hit.tex_coord)
  }

  fn random_bsdf_in_direction(&self) -> &ScatterRandomVariable { &self.scatter_random_var }
}
