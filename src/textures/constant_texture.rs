use super::TextureParameters;

use {
  super::Texture,
  crate::{
    color::{Color, ColorParameters},
    math::*
  },
  serde::Deserialize
};

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct ConstantTextureParameters {
  color: ColorParameters
}

#[typetag::deserialize(name = "constant")]
impl TextureParameters for ConstantTextureParameters {
  fn build_texture(&self) -> Box<dyn Texture> {
    Box::new(ConstantTexture { color: self.color.build_color() })
  }
}

#[derive(Debug)]
pub struct ConstantTexture {
  color: Color
}

impl Texture for ConstantTexture {
  fn value(&self, _: &WorldRayIntersection) -> Color { self.color }
}
