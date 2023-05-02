use std::sync::Arc;

use serde::Deserialize;

use super::{Texture, TextureParameters};
use crate::{light::*, raytracing::*};

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct ConstantTextureParameters {
  color: ColorParameters
}

#[typetag::deserialize(name = "constant")]
impl TextureParameters for ConstantTextureParameters {
  fn build_texture(&self) -> Arc<dyn Texture> {
    Arc::new(ConstantTexture::new(self.color.build_color()))
  }
}

#[derive(Debug)]
pub struct ConstantTexture {
  color: Color
}

impl ConstantTexture {
  pub fn new(color: Color) -> Self { Self { color } }
}

impl Texture for ConstantTexture {
  fn value(&self, _: &WorldRayIntersection) -> Color { self.color }
}
