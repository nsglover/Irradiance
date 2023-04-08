use {
  super::*,
  crate::{samplers::*, surfaces::*, textures::*}
};

#[derive(Debug)]
pub struct Dieletric<'a> {
  color: &'a dyn Texture
}

impl<'a> Material for Dieletric<'a> {
  fn sample(&self, _hit: &WorldHitInfo, _: &mut dyn Sampler) -> MaterialSample {
    // let _ = MaterialSample::reflection(
    //   self.color.value(hit),
    //   hit.hitting_ray.clone(),
    //   ReflectionType::Specular
    // );

    todo!()
  }
}
