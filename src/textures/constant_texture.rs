use super::TextureParameters;

use {
  super::Texture,
  crate::color::{Color, ColorParameters},
  serde::{Deserialize, Serialize}
};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct ConstantTextureParameters {
  color: ColorParameters
}

#[typetag::serde(name = "constant")]
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
  fn value(&self, _: &crate::surfaces::WorldHitInfo) -> Color { self.color }
}
