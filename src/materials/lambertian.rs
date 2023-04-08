use serde::{Deserialize, Serialize};

use {
  super::*,
  crate::{math::*, ray::*, samplers::*, surfaces::*, textures::*}
};

#[derive(Debug, Serialize, Deserialize)]
struct LambertianParameters {
  name: String,
  albedo: Box<dyn TextureParameters>
}

#[typetag::serde(name = "lambertian")]
impl MaterialParameters for LambertianParameters {
  fn name(&self) -> String { self.name.clone() }

  fn build_material(&self) -> Box<dyn Material> {
    Box::new(Lambertian { albedo: self.albedo.build_texture() })
  }
}

#[derive(Debug)]
pub struct Lambertian {
  albedo: Box<dyn Texture>
}

impl Material for Lambertian {
  fn sample(&self, hit: &WorldHitInfo, sampler: &mut dyn Sampler) -> MaterialSample {
    let scattered: WorldVector = uniform_random_on_unit_sphere(sampler).into();
    let normal: WorldVector = hit.shading_normal.into();
    let dir = (normal + scattered).normalize();

    let pdf = Float::max(0.0, dir.dot(&hit.shading_normal) / PI);
    let color = self.albedo.value(hit) * pdf;
    let scattered_ray = WorldRay::new(hit.hit_point, dir);

    MaterialSample::reflection(color, scattered_ray, ReflectionType::Diffuse(pdf))
  }
}
