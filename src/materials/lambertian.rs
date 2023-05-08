use std::sync::Arc;

use serde::Deserialize;

use super::*;
use crate::{light::Color, math::*, raytracing::*, sampling::*, textures::*};

#[derive(Debug, Deserialize)]
struct LambertianParameters {
  name: String,
  albedo: Box<dyn TextureParameters>,
  intensity: Option<Real>
}

#[typetag::deserialize(name = "lambertian")]
impl MaterialParameters for LambertianParameters {
  fn name(&self) -> String { self.name.clone() }

  fn build_material(&self) -> Arc<dyn Material> {
    Arc::new(Lambertian {
      albedo: self.albedo.build_texture(),
      scatter_random_var: ScatterRandomVariable::Continuous(Box::new(CosineWeightedHemisphere)),
      intensity: PositiveReal::new(self.intensity.unwrap_or(1.0)).unwrap()
    })
  }
}

#[derive(Debug)]
struct CosineWeightedHemisphere;

impl ContinuousRandomVariable<WorldRayIntersection, WorldUnitVector> for CosineWeightedHemisphere {
  fn sample_with_pdf(
    &self,
    hit: &WorldRayIntersection,
    sampler: &mut dyn Sampler
  ) -> Option<(WorldUnitVector, PositiveReal)> {
    let random: WorldVector = uniform_random_on_unit_sphere(sampler).into();
    let mut normal = hit.geometric_normal;
    if hit.geometric_normal.dot(&hit.intersect_direction) > 0.0 {
      normal = -normal;
    }

    let dir = (normal.into_vector() + random).normalize();
    self.pdf(hit, &dir).map(|pdf| (dir, pdf))
  }

  fn pdf(&self, hit: &WorldRayIntersection, sample: &WorldUnitVector) -> Option<PositiveReal> {
    PositiveReal::new((sample.dot(&hit.geometric_normal) / PI).abs())
  }
}

#[derive(Debug)]
pub struct Lambertian {
  albedo: Arc<dyn Texture>,
  scatter_random_var: ScatterRandomVariable,
  intensity: PositiveReal
}

impl Material for Lambertian {
  fn emitted(&self, _: &WorldRayIntersection) -> Option<Color> { None }

  fn bsdf(&self, hit: &WorldRayIntersection, sample: &WorldUnitVector) -> Color {
    self.albedo.value(&hit.tex_coords) * ((sample.dot(&hit.geometric_normal) / PI).abs() * self.intensity)
  }

  fn scatter_random_variable(&self) -> Option<&ScatterRandomVariable> { Some(&self.scatter_random_var) }

  fn emit_random_variable(
    &self
  ) -> Option<&dyn ContinuousRandomVariable<(WorldPoint, WorldUnitVector), (WorldUnitVector, Color)>> {
    None
  }
}
