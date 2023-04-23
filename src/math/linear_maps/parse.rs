use nalgebra as na;
use serde::Deserialize;

use super::*;
use crate::math::{Real, Space, PI};

fn array_to_vec(array: [Real; 3]) -> na::Vector3<Real> { na::vector![array[0], array[1], array[2]] }

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum MatrixParameters {
  Translate { translate: [Real; 3] },
  UniformScale { scale: Real },
  NonUniformScale { scale: [Real; 3] },
  AxisAngle { axis: [Real; 3], angle: Real },
  LookAt { from: [Real; 3], at: [Real; 3], up: [Real; 3] }
}

impl MatrixParameters {
  pub fn build_matrix(self) -> na::Matrix4<Real> {
    match self {
      MatrixParameters::Translate { translate } => {
        na::Matrix4::new_translation(&array_to_vec(translate))
      },
      MatrixParameters::UniformScale { scale } => na::Matrix4::new_scaling(scale),
      MatrixParameters::NonUniformScale { scale } => {
        na::Matrix4::new_nonuniform_scaling(&array_to_vec(scale))
      },
      MatrixParameters::AxisAngle { axis, angle } => na::Matrix4::from_axis_angle(
        &na::Unit::new_normalize(array_to_vec(axis)),
        angle * PI / 180.0
      ),
      MatrixParameters::LookAt { from, at, up } => na::Matrix4::look_at_rh(
        &array_to_vec(from).into(),
        &array_to_vec(at).into(),
        &array_to_vec(up)
      )
    }
  }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum TransformParameters {
  Single(MatrixParameters),
  Composed(Vec<MatrixParameters>)
}

impl TransformParameters {
  pub fn build_transform<In: Space<3> + 'static, Out: Space<3> + 'static>(
    self
  ) -> Box<dyn Transform<In, Out>> {
    let mut matrix = na::Matrix4::identity();
    match self {
      TransformParameters::Single(m) => matrix = m.build_matrix(),
      TransformParameters::Composed(ms) => {
        for m in ms {
          matrix = m.build_matrix() * matrix;
        }
      },
    }

    if let Some(transform) = MatrixTransform::from_raw(matrix) {
      Box::new(transform)
    } else {
      panic!("Non-invertible transformation specified!")
    }
  }
}
