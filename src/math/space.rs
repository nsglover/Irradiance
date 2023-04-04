use super::phantom::Phantom;

use {super::*, nalgebra as na};

pub trait Space<const D: usize>: Clone + Copy + std::fmt::Debug
where na::Const<D>: na::ToTypenum
{
  fn dimension(&self) -> usize { D }

  fn origin() -> Point<D, EuclideanSpace<D>>;

  fn basis() -> [Vector<D, EuclideanSpace<D>>; D];
}

pub trait OrthonormalSpace<const D: usize>: Space<D>
where na::Const<D>: na::ToTypenum
{
  fn orthonormal_basis() -> [Direction<D, EuclideanSpace<D>>; D];
}

#[derive(Clone, Copy)]
pub struct EuclideanSpace<const D: usize> {
  _phantom: Phantom<na::Const<D>>
}

impl<const D: usize> std::fmt::Debug for EuclideanSpace<D> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "WorldSpace<D>") }
}

impl<const D: usize> Space<D> for EuclideanSpace<D>
where na::Const<D>: na::ToTypenum
{
  fn origin() -> Point<D, EuclideanSpace<D>> { Vector::zero().into() }

  // TODO: It would be nice if this simply returned a statically computed constant
  fn basis() -> [Vector<D, EuclideanSpace<D>>; D] {
    let mut arr = [Vector::zero(); D];

    for i in 0..D {
      arr[i] = na::SVector::<Float, D>::ith_axis(i).into_inner().into();
    }

    arr
  }
}

impl<const D: usize> OrthonormalSpace<D> for EuclideanSpace<D>
where na::Const<D>: na::ToTypenum
{
  // TODO: It would be nice if this simply returned a statically computed constant
  fn orthonormal_basis() -> [Direction<D, EuclideanSpace<D>>; D] {
    let mut arr = [na::SVector::<Float, D>::ith_axis(0).into(); D];

    for i in 0..D {
      arr[i] = na::SVector::<Float, D>::ith_axis(i).into();
    }

    arr
  }
}

pub type WorldSpace = EuclideanSpace<3>;
