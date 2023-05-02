use std::sync::Arc;

use serde::Deserialize;

use super::*;
use crate::{light::Color, math::*, raytracing::*, sampling::*, textures::*};

#[derive(Debug, Deserialize)]
struct LambertianParameters {
  name: String,
  albedo: Box<dyn TextureParameters>
}

#[typetag::deserialize(name = "lambertian")]
impl MaterialParameters for LambertianParameters {
  fn name(&self) -> String { self.name.clone() }

  fn build_material(&self) -> Arc<dyn Material> {
    Arc::new(Lambertian {
      albedo: self.albedo.build_texture(),
      scatter_random_var: ScatterRandomVariable::Continuous(Box::new(CosineWeightedHemisphere))
    })
  }
}

#[derive(Debug)]
struct CosineWeightedHemisphere;

impl ContinuousRandomVariable<WorldRayIntersection, WorldUnitVector> for CosineWeightedHemisphere {
  fn sample_with_pdf(
    &self,
    param: &WorldRayIntersection,
    sampler: &mut dyn Sampler
  ) -> Option<(WorldUnitVector, PositiveReal)> {
    if param.geometric_normal.dot(&param.intersect_direction) > 0.0 {
      None
    } else {
      let random: WorldVector = uniform_random_on_unit_sphere(sampler).into();
      let normal: WorldVector = param.geometric_normal.into();
      let dir = (normal + random).normalize();
      PositiveReal::new(dir.dot(&param.geometric_normal) / PI).map(|pdf| (dir, pdf))
    }
  }

  fn pdf(&self, param: &WorldRayIntersection, sample: &WorldUnitVector) -> Option<PositiveReal> {
    let pdf = sample.dot(&param.geometric_normal) / PI;
    PositiveReal::new(pdf)
  }
}

#[derive(Debug)]
pub struct Lambertian {
  albedo: Arc<dyn Texture>,
  scatter_random_var: ScatterRandomVariable
}

impl Material for Lambertian {
  fn emitted(&self, _: &WorldRayIntersection) -> Option<Color> { None }

  fn bsdf(&self, hit: &WorldRayIntersection, sample: &WorldUnitVector) -> Color {
    self.albedo.value(hit) * Real::max(0.0, sample.dot(&hit.geometric_normal) / PI)
  }

  fn scatter_random_variable(&self) -> Option<&ScatterRandomVariable> {
    Some(&self.scatter_random_var)
  }

  fn is_emissive(&self) -> bool { false }
}
