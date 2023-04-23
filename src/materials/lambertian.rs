use serde::Deserialize;

use super::*;
use crate::{math::*, raytracing::*, samplers::*, textures::*};

#[derive(Debug, Deserialize)]
struct LambertianParameters {
  name: String,
  albedo: Box<dyn TextureParameters>
}

#[typetag::deserialize(name = "lambertian")]
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
  fn sample(&self, hit: &WorldRayIntersection, sampler: &mut dyn Sampler) -> MaterialSample {
    let random: WorldVector = uniform_random_on_unit_sphere(sampler).into();
    let normal: WorldVector = hit.shading_normal.into();
    let dir = (normal + random).normalize();

    let pdf = Real::max(0.0, dir.dot(&hit.shading_normal) / PI);
    let color = self.albedo.value(hit) * pdf;
    let scattered_ray = WorldRay::new(hit.intersect_point, dir);

    MaterialSample::diffuse(color, scattered_ray, pdf)
  }

  fn is_emissive(&self) -> bool { false }

  fn pdf(&self, hit: &WorldRayIntersection, sample: &WorldRay) -> Option<Real> {
    let pdf = Real::max(0.0, sample.dir().dot(&hit.shading_normal) / PI);
    (pdf > 0.0).then_some(pdf)
  }
}
