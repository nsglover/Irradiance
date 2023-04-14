// use {
//   super::*,
//   nalgebra::{Const, ToTypenum}
// };

pub type Float = f64;
pub const PI: Float = std::f64::consts::PI as Float;

// pub fn linear_interpolate<const D: usize, S: Space<D>>(
//   t: Float,
//   v0: Vector<D, S>,
//   v1: Vector<D, S>
// ) -> Vector<D, S>
// where
//   Const<D>: ToTypenum
// {
//   v0 * (1.0 - t) + v1 * t
// }
