use std::sync::Arc;

use serde::Deserialize;

use super::*;
use crate::{
  light::Color,
  math::*,
  raytracing::*,
  sampling::{uniform_random_on_unit_sphere, ContinuousRandomVariable, Sampler},
  textures::*
};

#[derive(Debug, Deserialize)]
struct DiffuseLightParameters {
  name: String,
  emit: Box<dyn TextureParameters>,
  intensity: Real
}

#[typetag::deserialize(name = "diffuse light")]
impl MaterialParameters for DiffuseLightParameters {
  fn name(&self) -> String { self.name.clone() }

  fn build_material(&self) -> Arc<dyn Material> {
    Arc::new(DiffuseLight {
      light_color: self.emit.build_texture(),
      light_intensity: self.intensity
    })
  }
}

#[derive(Debug)]
pub struct DiffuseLight {
  light_color: Arc<dyn Texture>,
  light_intensity: Real
}

impl ContinuousRandomVariable<(WorldPoint, WorldUnitVector), (WorldUnitVector, Color)>
  for DiffuseLight
{
  fn sample_with_pdf(
    &self,
    (_, normal): &(WorldPoint, WorldUnitVector),
    sampler: &mut dyn Sampler
  ) -> Option<((WorldUnitVector, Color), PositiveReal)> {
    let random: WorldVector = uniform_random_on_unit_sphere(sampler).into();
    let dir = (normal.into_vector() + random).normalize();

    // TODO: Fix this asap; only works for constant textures
    PositiveReal::new(dir.dot(&normal) / PI).map(|pdf| {
      ((dir, self.light_color.value(&TextureCoordinate::zero()) * self.light_intensity), pdf)
    })
  }

  fn pdf(
    &self,
    (_, normal): &(WorldPoint, WorldUnitVector),
    sample: &(WorldUnitVector, Color)
  ) -> Option<PositiveReal> {
    PositiveReal::new(sample.0.dot(normal) / PI)
  }
}

impl Material for DiffuseLight {
  fn bsdf(&self, _: &WorldRayIntersection, _: &WorldUnitVector) -> Color { Color::black() }

  fn scatter_random_variable(&self) -> Option<&ScatterRandomVariable> { None }

  fn emitted(&self, hit: &WorldRayIntersection) -> Option<Color> {
    if hit.intersect_direction.dot(&hit.geometric_normal) > 0.0 {
      None
    } else {
      Some(self.light_color.value(&hit.tex_coords) * self.light_intensity)
    }
  }

  fn emit_random_variable(
    &self
  ) -> Option<&dyn ContinuousRandomVariable<(WorldPoint, WorldUnitVector), (WorldUnitVector, Color)>>
  {
    Some(self)
  }
}
