// use {
//   super::*,
//   nalgebra::{Const, ToTypenum}
// };

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

// pub fn scalar_linear_interpolate(t: Float, v0: Float, v1: Float) -> Float {
//   v0 * (1.0 - t) + v1 * t
// }
