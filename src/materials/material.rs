use {
  crate::{color::Color, math::*, ray::*, samplers::Sampler, surfaces::*},
  std::fmt::Debug
};

#[typetag::deserialize(tag = "type")]
pub trait MaterialParameters: Debug {
  fn name(&self) -> String;

  fn build_material(&self) -> Box<dyn Material>;
}

#[derive(Debug)]
pub struct MaterialSample {
  pub emission: Option<Color>,
  pub reflection: Option<(Color, WorldRay, ReflectionType)>
}

impl MaterialSample {
  pub fn nothing() -> Self { Self { emission: None, reflection: None } }

  pub fn emission(color: Color) -> Self { Self { emission: Some(color), reflection: None } }

  pub fn reflection(
    attenuation: Color,
    scattered_ray: WorldRay,
    reflection_type: ReflectionType
  ) -> Self {
    if let ReflectionType::Diffuse(pdf) = reflection_type {
      if pdf == 0.0 {
        return Self::nothing();
      }
    }

    Self { emission: None, reflection: Some((attenuation, scattered_ray, reflection_type)) }
  }
}

#[derive(Debug)]
pub enum ReflectionType {
  /// Specular
  Specular,

  // Diffuse(PDF)
  Diffuse(Float)
}

pub trait Material: Debug {
  fn sample(&self, hit: &WorldHitInfo, sampler: &mut dyn Sampler) -> MaterialSample;
}
