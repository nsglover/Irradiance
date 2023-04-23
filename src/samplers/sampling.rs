use super::Sampler;
use crate::math::*;
// use nalgebra::{Const, ToTypenum};

// fn next_samples_map<const N: usize>(
//   s: &mut dyn Sampler,
//   map: fn(&mut dyn Sampler) -> Float
// ) -> [Float; N] {
//   let mut arr = [0.0; N];
//   for i in arr.iter_mut() {
//     *i = map(s);
//   }

//   arr
// }

// fn next_vector_map<const D: usize, S: Space<D>>(
//   s: &mut dyn Sampler,
//   map: fn(&mut dyn Sampler) -> Float
// ) -> Vector<D, S>
// where
//   Const<D>: ToTypenum
// {
//   let mut vec = nalgebra::SVector::<Float, D>::zeros();
//   for i in vec.iter_mut() {
//     *i = map(s);
//   }

//   vec.into()
// }

// pub fn next_samples<const N: usize>(s: &mut dyn Sampler) -> [Float; N] {
//   next_samples_map(s, |t| t.next())
// }

// pub fn next_non_zero_samples<const N: usize>(s: &mut dyn Sampler) -> [Float; N] {
//   next_samples_map(s, |t| t.next_non_zero())
// }

// pub fn next_non_one_samples<const N: usize>(s: &mut dyn Sampler) -> [Float; N] {
//   next_samples_map(s, |t| t.next_non_one())
// }

// pub fn next_vector<const D: usize, S: Space<D>>(s: &mut dyn Sampler) -> Vector<D, S>
// where Const<D>: ToTypenum {
//   next_vector_map(s, |t| t.next())
// }

// pub fn next_non_zero_vector<const D: usize, S: Space<D>>(s: &mut dyn Sampler) -> Vector<D, S>
// where Const<D>: ToTypenum {
//   next_vector_map(s, |t| t.next_non_zero())
// }

// pub fn next_non_one_vector<const D: usize, S: Space<D>>(s: &mut dyn Sampler) -> Vector<D, S>
// where Const<D>: ToTypenum {
//   next_vector_map(s, |t| t.next_non_one())
// }

// fn random_linear_interpolate<const D: usize, S: Space<D>>(
//   s: &mut dyn Sampler,
//   v0: Vector<D, S>,
//   v1: Vector<D, S>
// ) -> Vector<D, S>
// where
//   Const<D>: ToTypenum
// {
//   linear_interpolate(s.next(), v0, v1)
// }

pub fn spherical_to_cartesian<S: Space<3>>(phi: Real, cos_theta: Real) -> UnitVector3<S> {
  let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
  let x = phi.cos() * sin_theta;
  let y = -phi.sin() * sin_theta;
  let z = cos_theta;

  let vec = nalgebra::vector![x, y, z];
  let mut unit = nalgebra::Unit::new_unchecked(vec);
  unit.renormalize_fast();

  unit.into()
}

pub fn uniform_random_in_unit_disc(s: &mut dyn Sampler) -> Vector2 {
  let theta = s.random_in_closed_open(0.0, 2.0 * PI);
  (s.next().into_inner().sqrt() * nalgebra::vector![theta.cos(), theta.sin()]).into()
}

pub fn uniform_random_on_unit_sphere<S: Space<3>>(s: &mut dyn Sampler) -> UnitVector3<S> {
  spherical_to_cartesian(s.random_in_closed_open(0.0, 2.0 * PI), s.random_in_closed(-1.0, 1.0))
}

// pub fn uniform_random_on_unit_hemisphere<S: Space<3>>(s: &mut dyn Sampler) -> Direction3<S> {
//   spherical_to_cartesian(s.random_in_closed_open(0.0, 2.0 * PI), s.next())
// }

// pub fn cosine_random_on_unit_hemisphere<S: Space<3>>(s: &mut dyn Sampler) -> Direction3<S> {
//   spherical_to_cartesian(s.random_in_closed_open(0.0, 2.0 * PI), s.next().sqrt())
// }

// pub fn cosine_power_random_on_unit_hemisphere<S: Space<3>>(
//   s: &mut dyn Sampler,
//   p: Float
// ) -> Direction3<S> {
//   spherical_to_cartesian(s.random_in_closed_open(0.0, 2.0 * PI), s.next().powf(1.0 / (p + 1.0)))
// }
