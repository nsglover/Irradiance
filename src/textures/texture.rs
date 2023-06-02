use std::{fmt::Debug, sync::Arc};

use crate::{math::*, spectrum::*};

#[typetag::deserialize(tag = "type")]
pub trait TextureParameters: Debug {
  fn build_texture(&self) -> Arc<dyn Texture>;
}

#[derive(Debug, Clone, Copy)]
pub struct TextureSpace;

impl Space<2> for TextureSpace {}

pub type TextureCoordinate = Vector<2, TextureSpace>;

pub trait Texture: Debug {
  fn value(&self, tex_coord: &TextureCoordinate) -> Spectrum;
}
