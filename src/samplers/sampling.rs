use {
  super::Sampler,
  crate::math::*,
  nalgebra::{ComplexField as na, Const, ToTypenum}
};

fn next_samples_map<const N: usize>(
  s: &mut dyn Sampler,
  map: fn(&mut dyn Sampler) -> Float
) -> [Float; N] {
  let mut arr = [0.0; N];
  for i in 0..N {
    arr[i] = map(s);
  }

  arr
}

fn next_vector_map<const D: usize, S: Space<D>>(
  s: &mut dyn Sampler,
  map: fn(&mut dyn Sampler) -> Float
) -> Vector<D, S>
where
  Const<D>: ToTypenum
{
  let mut vec = Vector::zero();

  for i in 0..D {
    vec[i] = map(s);
  }

  vec
}

pub fn next_samples<const N: usize>(s: &mut dyn Sampler) -> [Float; N] {
  next_samples_map(s, |t| t.next())
}

pub fn next_non_zero_samples<const N: usize>(s: &mut dyn Sampler) -> [Float; N] {
  next_samples_map(s, |t| t.next_non_zero())
}

pub fn next_non_one_samples<const N: usize>(s: &mut dyn Sampler) -> [Float; N] {
  next_samples_map(s, |t| t.next_non_one())
}

pub fn next_vector<const D: usize, S: Space<D>>(s: &mut dyn Sampler) -> Vector<D, S>
where Const<D>: ToTypenum {
  next_vector_map(s, |t| t.next())
}

pub fn next_non_zero_vector<const D: usize, S: Space<D>>(s: &mut dyn Sampler) -> Vector<D, S>
where Const<D>: ToTypenum {
  next_vector_map(s, |t| t.next_non_zero())
}

pub fn next_non_one_vector<const D: usize, S: Space<D>>(s: &mut dyn Sampler) -> Vector<D, S>
where Const<D>: ToTypenum {
  next_vector_map(s, |t| t.next_non_one())
}

fn random_linear_interpolate<const D: usize, S: Space<D>>(
  s: &mut dyn Sampler,
  v0: Vector<D, S>,
  v1: Vector<D, S>
) -> Vector<D, S>
where
  Const<D>: ToTypenum
{
  linear_interpolate(s.next(), v0, v1)
}

fn random_in_range_open_closed(s: &mut dyn Sampler, inf: Float, max: Float) -> Float {
  inf + (max - inf) * s.next_non_zero()
}

pub fn uniform_random_in_unit_disc(s: &mut dyn Sampler) -> Vector2 {
  let r = na::sqrt(s.next());
  let theta = random_in_range_open_closed(s, 0.0, 2.0 * PI);
  (r * nalgebra::vector![na::cos(theta), na::sin(theta)]).into()
}

pub fn uniform_random_on_unit_sphere<S: Space<3>>(s: &mut dyn Sampler) -> Direction3<S> {
  spherical_to_cartesian(
    na::cos(s.next_non_one() * 2.0 * (std::f64::consts::PI as Float)),
    2.0 * s.next_non_one() - 1.0
  )
}

pub fn uniform_random_on_unit_hemisphere<S: Space<3>>(s: &mut dyn Sampler) -> Direction3<S> {
  spherical_to_cartesian(
    na::cos(s.next_non_one() * 2.0 * (std::f64::consts::PI as Float)),
    s.next_non_one()
  )
}

pub fn cosine_random_on_unit_hemisphere<S: Space<3>>(s: &mut dyn Sampler) -> Direction3<S> {
  spherical_to_cartesian(
    na::cos(s.next_non_one() * 2.0 * (std::f64::consts::PI as Float)),
    na::sqrt(s.next_non_one())
  )
}

pub fn cosine_power_random_on_unit_hemisphere<S: Space<3>>(
  s: &mut dyn Sampler,
  p: Float
) -> Direction3<S> {
  spherical_to_cartesian(
    na::cos(s.next_non_one() * 2.0 * (std::f64::consts::PI as Float)),
    na::powf(s.next_non_one(), 1.0 / (p + 1.0))
  )
}
