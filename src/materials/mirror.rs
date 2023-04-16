use {
  super::*,
  crate::{raytracing::*, samplers::*, textures::*},
  serde::Deserialize
};

#[derive(Debug, Deserialize)]
struct MirrorParameters {
  name: String,
  albedo: Box<dyn TextureParameters>
}

#[typetag::deserialize(name = "mirror")]
impl MaterialParameters for MirrorParameters {
  fn name(&self) -> String { self.name.clone() }

  fn build_material(&self) -> Box<dyn Material> {
    Box::new(Mirror { albedo: self.albedo.build_texture() })
  }
}

#[derive(Debug)]
pub struct Mirror {
  albedo: Box<dyn Texture>
}

impl Material for Mirror {
  fn sample(&self, hit: &WorldRayIntersection, _: &mut dyn Sampler) -> MaterialSample {
    let ray = Ray::new(hit.intersect_point, (-hit.ray.dir()).reflect_about(hit.shading_normal));
    MaterialSample::specular(self.albedo.value(hit), ray)
  }
}
