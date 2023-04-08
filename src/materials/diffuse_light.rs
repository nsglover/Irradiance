use {
  super::*,
  crate::{math::Float, samplers::Sampler, surfaces::WorldHitInfo, textures::*},
  serde::{Deserialize, Serialize}
};

#[derive(Debug, Serialize, Deserialize)]
struct DiffuseLightParameters {
  name: String,
  emit: Box<dyn TextureParameters>,
  intensity: Float
}

#[typetag::serde(name = "diffuse light")]
impl MaterialParameters for DiffuseLightParameters {
  fn name(&self) -> String { self.name.clone() }

  fn build_material(&self) -> Box<dyn Material> {
    Box::new(DiffuseLight { light_color: self.emit.build_texture(), intensity: self.intensity })
  }
}

#[derive(Debug)]
pub struct DiffuseLight {
  light_color: Box<dyn Texture>,
  intensity: Float
}

impl Material for DiffuseLight {
  fn sample(&self, hit: &WorldHitInfo, _: &mut dyn Sampler) -> MaterialSample {
    MaterialSample::emission(self.light_color.value(hit) * self.intensity)
  }
}
