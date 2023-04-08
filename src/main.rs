#![feature(never_type)]

use std::{error::Error, fs::File, io::BufReader};

use scene::Scene;
mod bbox;
mod camera;
mod color;
mod integrators;
mod materials;
mod math;
mod ray;
mod samplers;
mod scene;
mod surface_groups;
mod surfaces;
mod textures;
mod wrapper;

// TODO: BVH
// TODO: Threadpool
// TODO: Stratified sampling
// TODO: Direct lighting MIS
// TODO: Environment map

fn main() -> Result<(), Box<dyn Error>> {
  let path = "./scenes/constant_cbox.json";
  let reader = BufReader::new(File::open(path)?);

  let scene = Scene::build_from_json(serde_json::from_reader(reader)?)?;
  // println!("{scene:?}");

  let image = scene.integrate();
  image.save("./scenes/constant_cbox.png")?;

  Ok(())
}
