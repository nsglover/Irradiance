use crate::{
  light::Color,
  math::{WorldPoint, WorldUnitVector}
};

// TODO: Packed struct
#[derive(Debug)]
pub struct Photon {
  position: WorldPoint,
  direction: WorldUnitVector,
  power: Color
}

impl Photon {
  pub fn position(&self) -> &WorldPoint { &self.position }
}
