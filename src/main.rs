#![feature(never_type)]

use nalgebra::vector;

mod color;
mod math;
mod ray;
mod wrapper;

use math::*;

fn main() {
  let mut vec: WorldVector = vector![1.0, 2.0, 3.0].into();
  vec *= 3.0;
  println!("{}", vec);
}
