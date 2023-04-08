use {
  super::phantom::Phantom,
  nalgebra::{Const, ToTypenum}
};

pub trait Space<const D: usize>: Clone + Copy + std::fmt::Debug
where Const<D>: ToTypenum
{
  fn dimension() -> usize { D }

  // fn origin() -> Point<D, EuclideanSpace<D>>;

  // fn basis() -> [Vector<D, EuclideanSpace<D>>; D];
}

pub trait OrthonormalSpace<const D: usize>: Space<D>
where Const<D>: ToTypenum
{
  // fn orthonormal_basis() -> [Direction<D, EuclideanSpace<D>>; D];
}

#[derive(Debug, Clone, Copy)]
pub struct EuclideanSpace<const D: usize> {
  _phantom: Phantom<Const<D>>
}

impl<const D: usize> Space<D> for EuclideanSpace<D>
where Const<D>: ToTypenum
{
  // fn origin() -> Point<D, EuclideanSpace<D>> { Vector::zero().into() }

  // // TODO: It would be nice if this simply returned a statically computed constant
  // fn basis() -> [Vector<D, EuclideanSpace<D>>; D] {
  //   let mut arr = [Vector::zero(); D];

  //   for i in 0..D {
  //     arr[i] = SVector::<Float, D>::ith_axis(i).into_inner().into();
  //   }

  //   arr
  // }
}

impl<const D: usize> OrthonormalSpace<D> for EuclideanSpace<D>
where Const<D>: ToTypenum
{
  // // TODO: It would be nice if this simply returned a statically computed constant
  // fn orthonormal_basis() -> [Direction<D, EuclideanSpace<D>>; D] {
  //   let mut arr = [SVector::<Float, D>::ith_axis(0).into(); D];

  //   for i in 0..D {
  //     arr[i] = SVector::<Float, D>::ith_axis(i).into();
  //   }

  //   arr
  // }
}

pub type WorldSpace = EuclideanSpace<3>;
