use std::sync::Arc;

use image::{io::Reader, ImageBuffer, Rgb};
use serde::Deserialize;

use super::{Texture, TextureCoordinate, TextureParameters};
use crate::{light::*, math::Real};

#[derive(Debug, Clone, Deserialize)]
pub struct ImageTextureParameters {
  filename: String
}

#[typetag::deserialize(name = "image")]
impl TextureParameters for ImageTextureParameters {
  fn build_texture(&self) -> Arc<dyn Texture> {
    Arc::new(ImageTexture {
      image: Reader::open(&self.filename).unwrap().decode().unwrap().into_rgb8()
    })
  }
}

#[derive(Debug)]
pub struct ImageTexture {
  image: ImageBuffer<Rgb<u8>, Vec<u8>>
}

impl ImageTexture {}

impl Texture for ImageTexture {
  fn value(&self, uv: &TextureCoordinate) -> Color {
    let w = self.image.width();
    let x = (uv[0] * (w as Real)).clamp(0.0, (w - 1) as Real) as u32;

    let h = self.image.height();
    let y = (uv[1] * (h as Real)).clamp(0.0, (h - 1) as Real) as u32;

    Color::from_bytes(self.image.get_pixel(x, y).0)
  }
}
