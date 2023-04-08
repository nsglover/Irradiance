use {
  crate::{color::*, math::*, surfaces::WorldHitInfo},
  std::fmt::Debug
};

#[typetag::serde(tag = "type")]
pub trait TextureParameters: Debug {
  fn build_texture(&self) -> Box<dyn Texture>;
}

#[derive(Debug, Clone, Copy)]
pub struct TextureSpace;

impl Space<2> for TextureSpace {}

pub type TextureCoordinates = Vector<2, TextureSpace>;

pub trait Texture: Debug {
  fn value(&self, hit: &WorldHitInfo) -> Color;
}
