use std::{fmt::Debug, sync::Arc};

use crate::{math::*, raytracing::*, sampling::RandomVariable, spectrum::*};

#[typetag::deserialize(tag = "type")]
pub trait MaterialParameters: Debug {
  fn name(&self) -> String;

  fn build_material(&self) -> Arc<dyn Material>;
}

pub type ScatterRandomVariable = RandomVariable<(WorldSurfacePoint, WorldUnitVector), WorldUnitVector>;

pub trait Material: Debug {
  fn bsdf(&self, point: &WorldSurfacePoint, in_dir: &WorldUnitVector, out_dir: &WorldUnitVector) -> Spectrum {
    self.bsdf_cos(point, in_dir, out_dir) / point.shading_normal.abs_dot(in_dir)
  }

  fn bsdf_cos(&self, point: &WorldSurfacePoint, in_dir: &WorldUnitVector, out_dir: &WorldUnitVector) -> Spectrum;

  fn random_bsdf_in_direction(&self) -> &ScatterRandomVariable;
}
