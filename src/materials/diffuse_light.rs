use {
  super::*,
  crate::{math::*, samplers::Sampler, surfaces::WorldHitInfo, textures::*},
  serde::Deserialize
};

#[derive(Debug, Deserialize)]
struct DiffuseLightParameters {
  name: String,
  emit: Box<dyn TextureParameters>,
  intensity: Float
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
  light_intensity: Float
}

impl Material for DiffuseLight {
  fn sample(&self, hit: &WorldHitInfo, ray: &WorldRay, _: &mut dyn Sampler) -> MaterialSample {
    if ray.dir().dot(&hit.shading_normal) > 0.0 {
      MaterialSample::nothing()
    } else {
      MaterialSample::emission(self.light_color.value(hit) * self.light_intensity)
    }
  }
}
