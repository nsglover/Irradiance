use serde::Deserialize;

use super::*;
use crate::{math::*, raytracing::*, samplers::Sampler, textures::*};

#[derive(Debug, Deserialize)]
struct DiffuseLightParameters {
  name: String,
  emit: Box<dyn TextureParameters>,
  intensity: Real
}

#[typetag::deserialize(name = "diffuse light")]
impl MaterialParameters for DiffuseLightParameters {
  fn name(&self) -> String { self.name.clone() }

  fn build_material(&self) -> Box<dyn Material> {
    Box::new(DiffuseLight {
      light_color: self.emit.build_texture(),
      light_intensity: self.intensity
    })
  }
}

#[derive(Debug)]
pub struct DiffuseLight {
  light_color: Box<dyn Texture>,
  light_intensity: Real
}

impl Material for DiffuseLight {
  fn sample(&self, hit: &WorldRayIntersection, _: &mut dyn Sampler) -> MaterialSample {
    if hit.ray.dir().dot(&hit.shading_normal) > 0.0 {
      MaterialSample::nothing()
    } else {
      MaterialSample::emission(self.light_color.value(hit) * self.light_intensity)
    }
  }

  fn is_emissive(&self) -> bool { true }

  fn pdf(&self, _: &WorldRayIntersection, _: &WorldRay) -> Option<Real> { None }
}
