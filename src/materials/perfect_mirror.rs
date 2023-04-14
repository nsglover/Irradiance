use {
  super::*,
  crate::{math::*, samplers::*, textures::*},
  serde::Deserialize
};

#[derive(Debug, Deserialize)]
struct MirrorParameters {
  name: String,
  albedo: Box<dyn TextureParameters>
}

#[typetag::deserialize(name = "dielectric")]
impl MaterialParameters for MirrorParameters {
  fn name(&self) -> String { self.name.clone() }

  fn build_material(&self) -> Box<dyn Material> {
    Box::new(PerfectMirror { albedo: self.albedo.build_texture() })
  }
}

#[derive(Debug)]
pub struct PerfectMirror {
  albedo: Box<dyn Texture>
}

impl Material for PerfectMirror {
  fn sample(
    &self,
    hit: &WorldRayIntersection,
    ray: &WorldRay,
    sampler: &mut dyn Sampler
  ) -> MaterialSample {
    todo!()
  }
}
