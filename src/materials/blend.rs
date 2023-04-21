use {
  super::*,
  crate::{
    math::{scalar_linear_interpolate, Float},
    raytracing::*,
    samplers::*,
    textures::*
  },
  serde::Deserialize
};

#[derive(Debug, Deserialize)]
struct BlendParameters {
  name: String,

  #[serde(alias = "blend-factor")]
  blend_factor: Box<dyn TextureParameters>,

  #[serde(alias = "0")]
  mat0: Box<dyn MaterialParameters>,

  #[serde(alias = "1")]
  mat1: Box<dyn MaterialParameters>
}

#[typetag::deserialize(name = "blend")]
impl MaterialParameters for BlendParameters {
  fn name(&self) -> String { self.name.clone() }

  fn build_material(&self) -> Box<dyn Material> {
    Box::new(BlendMaterial {
      blend_factor: self.blend_factor.build_texture(),
      mat0: self.mat0.build_material(),
      mat1: self.mat1.build_material()
    })
  }
}

#[derive(Debug)]
pub struct BlendMaterial {
  blend_factor: Box<dyn Texture>,
  mat0: Box<dyn Material>,
  mat1: Box<dyn Material>
}

impl Material for BlendMaterial {
  fn sample(&self, hit: &WorldRayIntersection, sampler: &mut dyn Sampler) -> MaterialSample {
    let p = self.blend_factor.value(hit).luminance();
    let mut s = (if sampler.next() < p { &self.mat0 } else { &self.mat1 }).sample(hit, sampler);
    let pdf = scalar_linear_interpolate(
      p,
      self.mat0.pdf(hit, &s).unwrap_or(0.0),
      self.mat1.pdf(hit, &s).unwrap_or(0.0)
    );

    s.set_pdf(pdf);
    s
  }

  fn pdf(&self, hit: &WorldRayIntersection, sample: &MaterialSample) -> Option<Float> {
    let p = self.blend_factor.value(hit).luminance();
    let pdf = scalar_linear_interpolate(
      p,
      self.mat0.pdf(hit, sample).unwrap_or(0.0),
      self.mat1.pdf(hit, sample).unwrap_or(0.0)
    );

    (pdf > 0.0).then_some(pdf)
  }

  fn is_emissive(&self) -> bool { false }
}
