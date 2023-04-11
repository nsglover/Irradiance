use {
  super::phantom::Phantom,
  nalgebra::{Const, ToTypenum}
};

pub trait Space<const D: usize>: Clone + Copy + std::fmt::Debug
where Const<D>: ToTypenum
{
  fn dimension() -> usize { D }
}

pub trait OrthonormalSpace<const D: usize>: Space<D>
where Const<D>: ToTypenum
{
}

#[derive(Debug, Clone, Copy)]
pub struct EuclideanSpace<const D: usize> {
  _phantom: Phantom<Const<D>>
}

impl<const D: usize> Space<D> for EuclideanSpace<D> where Const<D>: ToTypenum {}

impl<const D: usize> OrthonormalSpace<D> for EuclideanSpace<D> where Const<D>: ToTypenum {}

pub type WorldSpace = EuclideanSpace<3>;
