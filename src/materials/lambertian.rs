use {
  super::*,
  crate::{math::*, samplers::*, textures::*},
  serde::Deserialize
};

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
  fn sample(
    &self,
    hit: &WorldRayIntersection,
    _: &WorldRay,
    sampler: &mut dyn Sampler
  ) -> MaterialSample {
    let random: WorldVector = uniform_random_on_unit_sphere(sampler).into();
    let normal: WorldVector = hit.shading_normal.into();
    let dir = (normal + random).normalize();

    let pdf = Float::max(0.0, dir.dot(&hit.shading_normal) / PI);
    let color = self.albedo.value(hit) * pdf;
    let scattered_ray = WorldRay::new(hit.intersect_point, dir);

    MaterialSample::reflection(color, scattered_ray, ReflectionType::Diffuse(pdf))
  }
}