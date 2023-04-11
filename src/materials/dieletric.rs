use {
  super::*,
  crate::{samplers::*, surfaces::*, textures::*}
};

#[derive(Debug)]
pub struct Dieletric<'a> {
  color: &'a dyn Texture
}

impl<'a> Material for Dieletric<'a> {
  fn sample(&self, hit: &WorldHitInfo, _: &mut dyn Sampler) -> MaterialSample { todo!() }
}
