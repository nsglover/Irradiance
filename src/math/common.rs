use {
  super::*,
  nalgebra::{ComplexField as na, Const, ToTypenum}
};

pub type Float = f64;
pub const PI: Float = std::f64::consts::PI as Float;

pub fn spherical_to_cartesian<S: Space<3>>(cos_phi: Float, cos_theta: Float) -> Direction3<S> {
  let sin_phi = na::sqrt(1.0 - cos_phi * cos_phi);
  let sin_theta = na::sqrt(1.0 - cos_theta * cos_theta);
  let x = cos_phi * sin_theta;
  let y = sin_phi * sin_theta;
  let z = cos_theta;

  let vec = nalgebra::vector![x, y, z];
  let mut unit = nalgebra::Unit::new_unchecked(vec);
  unit.renormalize_fast();

  unit.into()
}

pub fn linear_interpolate<const D: usize, S: Space<D>>(
  t: Float,
  v0: Vector<D, S>,
  v1: Vector<D, S>
) -> Vector<D, S>
where
  Const<D>: ToTypenum
{
  v0 * (1.0 - t) + v1 * t
}
