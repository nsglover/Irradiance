use std::fmt::Debug;

use crate::{light::*, math::*, raytracing::WorldRayIntersection};

#[typetag::deserialize(tag = "type")]
pub trait TextureParameters: Debug {
  fn build_texture(&self) -> Box<dyn Texture>;
}

#[derive(Debug, Clone, Copy)]
pub struct TextureSpace;

impl Space<2> for TextureSpace {}

pub type TextureCoordinates = Vector<2, TextureSpace>;

pub trait Texture: Debug {
  fn value(&self, hit: &WorldRayIntersection) -> Color;
}
