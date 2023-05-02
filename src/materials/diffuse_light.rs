use std::sync::Arc;

use serde::Deserialize;

use super::*;
use crate::{light::Color, math::*, raytracing::*, textures::*};

#[derive(Debug, Deserialize)]
struct DiffuseLightParameters {
  name: String,
  emit: Box<dyn TextureParameters>,
  intensity: Real
}

#[typetag::deserialize(name = "diffuse light")]
impl MaterialParameters for DiffuseLightParameters {
  fn name(&self) -> String { self.name.clone() }

  fn build_material(&self) -> Arc<dyn Material> {
    Arc::new(DiffuseLight {
      light_color: self.emit.build_texture(),
      light_intensity: self.intensity
    })
  }
}

#[derive(Debug)]
pub struct DiffuseLight {
  light_color: Arc<dyn Texture>,
  light_intensity: Real
}

impl Material for DiffuseLight {
  fn bsdf(&self, _: &WorldRayIntersection, _: &WorldUnitVector) -> Color { Color::black() }

  fn scatter_random_variable(&self) -> Option<&ScatterRandomVariable> { None }

  fn is_emissive(&self) -> bool { true }

  fn emitted(&self, hit: &WorldRayIntersection) -> Option<Color> {
    if hit.intersect_direction.dot(&hit.geometric_normal) > 0.0 {
      None
    } else {
      Some(self.light_color.value(hit) * self.light_intensity)
    }
  }
}
