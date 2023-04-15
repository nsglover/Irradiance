use {
  crate::{color::Color, math::*, samplers::Sampler},
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

  pub fn diffuse(attenuation: Color, scattered_ray: WorldRay, pdf: Float) -> Self {
    if pdf == 0.0 {
      Self::nothing()
    } else {
      Self {
        emission: None,
        reflection: Some((attenuation, scattered_ray, ReflectionType::Diffuse(pdf)))
      }
    }
  }

  pub fn specular(attenuation: Color, scattered_ray: WorldRay) -> Self {
    Self {
      emission: None,
      reflection: Some((attenuation, scattered_ray, ReflectionType::Specular))
    }
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
  fn sample(&self, hit: &WorldRayIntersection, sampler: &mut dyn Sampler) -> MaterialSample;
}
