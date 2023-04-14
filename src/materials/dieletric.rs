use {
  super::*,
  crate::{math::*, samplers::*, textures::*},
  serde::Deserialize
};

#[derive(Debug, Deserialize)]
struct DieletricParameters {
  name: String,
  albedo: Box<dyn TextureParameters>
}

#[typetag::deserialize(name = "dielectric")]
impl MaterialParameters for DieletricParameters {
  fn name(&self) -> String { self.name.clone() }

  fn build_material(&self) -> Box<dyn Material> {
    Box::new(Dieletric { albedo: self.albedo.build_texture() })
  }
}

#[derive(Debug)]
pub struct Dieletric {
  albedo: Box<dyn Texture>
}

impl Material for Dieletric {
  fn sample(
    &self,
    hit: &WorldRayIntersection,
    ray: &WorldRay,
    sampler: &mut dyn Sampler
  ) -> MaterialSample {
    todo!()
  }
}
