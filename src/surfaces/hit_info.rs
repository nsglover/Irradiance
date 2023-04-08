use crate::{materials::Material, math::*, textures::TextureCoordinates};

#[derive(Debug, Clone)]
pub struct HitInfo<'a, S: Space<3>> {
  pub hit_time: Float,
  pub hit_point: Point3<S>,
  pub geom_normal: Direction3<S>,
  pub shading_normal: Direction3<S>,
  pub tex_coords: TextureCoordinates,
  pub material: &'a dyn Material
}

impl<'a, S: Space<3>> HitInfo<'a, S> {
  pub fn transform<T: Space<3>>(self, tr: &Transform<S, T>) -> HitInfo<'a, T> {
    HitInfo {
      hit_time: self.hit_time,
      hit_point: tr * &self.hit_point,
      geom_normal: tr.normal(&self.geom_normal),
      shading_normal: tr.normal(&self.shading_normal),
      tex_coords: self.tex_coords,
      material: self.material
    }
  }
}

pub type WorldHitInfo<'a> = HitInfo<'a, WorldSpace>;
