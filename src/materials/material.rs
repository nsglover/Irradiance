use {
  crate::{light::*, math::*, raytracing::*, samplers::Sampler},
  std::fmt::Debug
};

#[typetag::deserialize(tag = "type")]
pub trait MaterialParameters: Debug {
  fn name(&self) -> String;

  fn build_material(&self) -> Box<dyn Material>;
}

pub struct MaterialSample {
  pub emission: Option<Color>,
  pub reflection: Option<(Color, WorldRay, ReflectionType)>
}

impl MaterialSample {
  pub fn nothing() -> Self { Self { emission: None, reflection: None } }

  pub fn emission(color: Color) -> Self { Self { emission: Some(color), reflection: None } }

  pub fn diffuse(attenuation: Color, scattered_ray: WorldRay, sample_pdf: Float) -> Self {
    if sample_pdf == 0.0 {
      Self::nothing()
    } else {
      Self {
        emission: None,
        reflection: Some((attenuation, scattered_ray, ReflectionType::Diffuse(sample_pdf)))
      }
    }
  }

  pub fn specular(attenuation: Color, scattered_ray: WorldRay) -> Self {
    Self {
      emission: None,
      reflection: Some((attenuation, scattered_ray, ReflectionType::Specular))
    }
  }

  pub fn scattered_ray(&self) -> Option<&WorldRay> {
    self.reflection.as_ref().map(|(_, ray, _)| ray)
  }
}

pub enum ReflectionType {
  /// Specular
  Specular,

  // Diffuse(PDF evaluated at the sampled reflection, PDF for for the entire random variable)
  Diffuse(Float)
}

pub trait Material: Debug {
  // fn bsdf(&self, hit: &WorldRayIntersection, scattered_dir: &WorldDirection) -> Color;

  fn sample(&self, hit: &WorldRayIntersection, sampler: &mut dyn Sampler) -> MaterialSample;

  fn pdf(&self, hit: &WorldRayIntersection, scattered_ray: &WorldRay) -> Option<Float>;

  fn is_emissive(&self) -> bool;
}
