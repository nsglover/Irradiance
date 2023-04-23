use nalgebra as na;
use serde::Deserialize;

use super::{
  identity::IdentityTransform, rotate::RotateTransform, scale::ScaleTransform,
  translate::TranslateTransform, uniform_scale::UniformScaleTransform, *
};
use crate::math::{
  linear_maps::{
    rot_trans::RotateTranslate, scale_rot::ScaleRotate, scale_rot_trans::ScaleRotateTranslate,
    scale_trans::ScaleTranslate
  },
  PositiveReal, Real, Space, PI
};

fn array_to_vec(array: [Real; 3]) -> na::Vector3<Real> { na::vector![array[0], array[1], array[2]] }

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum SingleTransformParameters {
  Translate { translate: [Real; 3] },
  UniformScale { scale: Real },
  NonUniformScale { scale: [Real; 3] },
  AxisAngle { axis: [Real; 3], angle: Real },
  LookAt { from: [Real; 3], at: [Real; 3], up: [Real; 3] }
}

impl SingleTransformParameters {
  pub fn build_matrix(self) -> na::Matrix4<Real> {
    match self {
      SingleTransformParameters::Translate { translate } => {
        na::Matrix4::new_translation(&array_to_vec(translate))
      },
      SingleTransformParameters::UniformScale { scale } => na::Matrix4::new_scaling(scale),
      SingleTransformParameters::NonUniformScale { scale } => {
        na::Matrix4::new_nonuniform_scaling(&array_to_vec(scale))
      },
      SingleTransformParameters::AxisAngle { axis, angle } => na::Matrix4::from_axis_angle(
        &na::Unit::new_normalize(array_to_vec(axis)),
        angle * PI / 180.0
      ),
      SingleTransformParameters::LookAt { from, at, up } => na::Matrix4::look_at_rh(
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
  Single(SingleTransformParameters),
  Composed(Vec<SingleTransformParameters>)
}

impl TransformParameters {
  fn build_matrix_transform<In: Space<3> + 'static, Out: Space<3> + 'static>(
    params: Vec<SingleTransformParameters>
  ) -> Box<dyn Transform<In, Out>> {
    let mut matrix = na::Matrix4::identity();
    for m in params {
      matrix = m.build_matrix() * matrix;
    }

    if let Some(transform) = MatrixTransform::from_raw(matrix) {
      Box::new(transform)
    } else {
      panic!("Non-invertible transformation specified!")
    }
  }

  fn build_from_vec<In: Space<3> + 'static, Out: Space<3> + 'static>(
    params: Vec<SingleTransformParameters>
  ) -> Box<dyn Transform<In, Out>> {
    use SingleTransformParameters as P;

    match &params[..] {
      [] => Box::new(IdentityTransform {}),
      [m] => match m {
        P::Translate { translate } => {
          Box::new(TranslateTransform::new(translate[0], translate[1], translate[2]))
        },
        P::UniformScale { scale } => {
          if let Some(s) = PositiveReal::new(*scale) {
            Box::new(UniformScaleTransform::new(s))
          } else {
            Box::new(ScaleTransform::new(*scale, *scale, *scale))
          }
        },
        P::NonUniformScale { scale } => Box::new(ScaleTransform::new(scale[0], scale[1], scale[2])),
        P::AxisAngle { axis, angle } => {
          Box::new(RotateTransform::new(axis[0], axis[1], axis[2], *angle * PI / 180.0))
        },
        _ => Self::build_matrix_transform(params)
      },
      [m1, m2] => match (m1, m2) {
        (P::UniformScale { scale }, P::Translate { translate }) => {
          if let Some(s) = PositiveReal::new(*scale) {
            Box::new(ScaleTranslate::new(
              UniformScaleTransform::<In, Out>::new(s),
              TranslateTransform::new(translate[0], translate[1], translate[2])
            ))
          } else {
            Self::build_matrix_transform(params)
          }
        },
        (P::UniformScale { scale }, P::AxisAngle { axis, angle }) => {
          if let Some(s) = PositiveReal::new(*scale) {
            Box::new(ScaleRotate::new(
              UniformScaleTransform::<In, Out>::new(s),
              RotateTransform::new(axis[0], axis[1], axis[2], *angle * PI / 180.0)
            ))
          } else {
            Self::build_matrix_transform(params)
          }
        },
        (P::AxisAngle { axis, angle }, P::Translate { translate }) => {
          Box::new(RotateTranslate::new(
            RotateTransform::<In, Out>::new(axis[0], axis[1], axis[2], *angle * PI / 180.0),
            TranslateTransform::new(translate[0], translate[1], translate[2])
          ))
        },
        _ => Self::build_matrix_transform(params)
      },
      [P::UniformScale { scale }, P::AxisAngle { axis, angle }, P::Translate { translate }] => {
        if let Some(s) = PositiveReal::new(*scale) {
          Box::new(ScaleRotateTranslate::new(
            UniformScaleTransform::<In, Out>::new(s),
            RotateTransform::<Out, Out>::new(axis[0], axis[1], axis[2], *angle * PI / 180.0),
            TranslateTransform::new(translate[0], translate[1], translate[2])
          ))
        } else {
          Self::build_matrix_transform(params)
        }
      },
      _ => Self::build_matrix_transform(params)
    }
  }

  pub fn build_transform<In: Space<3> + 'static, Out: Space<3> + 'static>(
    self
  ) -> Box<dyn Transform<In, Out>> {
    match self {
      TransformParameters::Single(m) => Self::build_from_vec(vec![m]),
      TransformParameters::Composed(ms) => Self::build_from_vec(ms)
    }
  }
}
