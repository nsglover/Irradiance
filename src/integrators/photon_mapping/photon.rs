use std::fmt::Display;

use kd_tree::KdPoint;

use crate::{
  light::Color,
  math::{Real, VectorLike, WorldPoint, WorldUnitVector, WorldVector},
  raytracing::WorldRay
};

#[derive(Debug)]
pub struct Photon {
  pub position: WorldPoint,
  pub direction: WorldUnitVector,
  pub power: Color
}

// TODO: This isn't really that packed; can do better
#[derive(Debug, Clone)]
pub struct PackedPhoton {
  pub position: [f32; 3],
  direction: [f32; 3],
  pub power: [f32; 3]
}

impl Photon {
  pub fn from_ray(ray: &WorldRay, power: Color) -> Self {
    Self { position: ray.origin(), direction: ray.dir(), power }
  }

  pub fn into_packed(self) -> PackedPhoton {
    let p = self.position;
    let d = self.direction.into_vector();
    let c = self.power;
    PackedPhoton {
      position: [p[0] as f32, p[1] as f32, p[2] as f32],
      direction: [d[0] as f32, d[1] as f32, d[2] as f32],
      power: [c.r() as f32, c.g() as f32, c.b() as f32]
    }
  }

  pub fn from_packed(packed: PackedPhoton) -> Self {
    let p = packed.position;
    let d = packed.direction;
    let c = packed.power;
    Self {
      position: WorldPoint::from_array([p[0] as Real, p[1] as Real, p[2] as Real]),
      direction: WorldVector::from_array([d[0] as Real, d[1] as Real, d[2] as Real])
        .normalize_fast(),
      power: Color::new(c[0] as Real, c[1] as Real, c[2] as Real)
    }
  }
}

impl KdPoint for PackedPhoton {
  type Scalar = f32;

  type Dim = typenum::U3;

  fn at(&self, i: usize) -> Self::Scalar { self.position[i] }
}

impl Display for Photon {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "(p: {}, d: {}, power: {:?})", self.position, self.direction, self.power)
  }
}
