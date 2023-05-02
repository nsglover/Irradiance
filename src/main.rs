use std::{error::Error, fs::File, io::BufReader, time::Duration};

use clap::Parser;
use renderer::Renderer;

mod camera;
mod common;
mod integrators;
mod light;
mod materials;
mod math;
mod raytracing;
mod renderer;
mod samplers;
mod surface_groups;
mod surfaces;
mod textures;

// For the semester:

// Project Features:
// TODO: Photon mapping
// TODO: Unbiased photon mapping
// TODO: If time permits, MIS with unbiased photon mapping and material path tracing
// TODO: README.md

// Important Features:
// TODO: Direct-lighting MIS and mixture sampling
// TODO: Stratified sampling
// TODO: Image loading and image texture

// For summer:

// Important Features:
// TODO: Overhaul the whole material sampling interface and also the PDF interface for surface
//       (perhaps use some sort of PositiveReal type?). Make a generalized random variable trait and
//       a BRDF class; materials are nothing more than a BRDF and a random variable, which aim to
//       to importance sample everything but the L_i term in the rendering equation.
// TODO: Investigate the performance disparity and fix it
// TODO: Generalize MIS and mixture integrators to work with any number of arbitrary integrators

// Minor Improvements:
// TODO: Stop cloning rays
// TODO: All parameter structs should be consumed upon building their target

// Side Features:
// TODO: Perlin noise
// TODO: Blend material (from any number of materials, which can be implemented using the original
//       one that just works for two.)
// TODO: Environment map

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = Some(""))]
struct Arguments {
  scene_file: String,

  #[arg(short = 'o', long)]
  image_file: Option<String>,

  #[arg(short = 's', long = "subimage-size")]
  subimage_edge_length: u32,

  #[arg(short = 'j', long = "threads", default_value_t = 1)]
  num_threads: usize,

  #[arg(short = 'q')]
  no_progress_bar: bool
}

fn duration_to_hms(time: &Duration) -> String {
  let total_seconds = time.as_secs();
  let s = total_seconds % 60;
  let m = (total_seconds / 60) % 60;
  let h = (total_seconds / 60) / 60;
  format!("{:0>2}:{:0>2}:{:0>2}", h, m, s)
}

#[derive(Debug)]
pub struct RenderSettings {
  num_threads: usize,
  subimage_dimensions: (u32, u32),
  use_progress_bar: bool
}

fn main() -> Result<(), Box<dyn Error>> {
  let Arguments { scene_file, image_file, subimage_edge_length, num_threads, no_progress_bar } =
    Arguments::parse();

  if num_threads == 0 {
    panic!("At least 1 thread is necessary to run the renderer!");
  }

  if !scene_file.ends_with(".json") {
    panic!("Scene file must have the .json file suffix!");
  }

  let scene_name = scene_file.chars().take(scene_file.len() - ".json".len()).collect();

  println!("Building scene from \"{scene_file}\"...");
  let build_time = std::time::Instant::now();
  let reader = BufReader::new(File::open(scene_file)?);
  let json = serde_json::from_reader(reader)?;
  let renderer = Renderer::build_from_json(json)?;
  println!("Building complete! Time: {}\n", duration_to_hms(&build_time.elapsed()));

  println!("Rendering scene \"{scene_name}.json\"...");
  let render_time = std::time::Instant::now();
  let image = renderer.render_scene(RenderSettings {
    num_threads,
    subimage_dimensions: (subimage_edge_length, subimage_edge_length),
    use_progress_bar: !no_progress_bar
  });

  let file_name = image_file.unwrap_or(scene_name) + ".png";
  println!("Saving image to \"{file_name}\"...\n");
  image.save(file_name)?;

  if no_progress_bar {
    println!("Rendering complete! Time: {}", duration_to_hms(&render_time.elapsed()));
  } else {
    println!("Rendering complete!");
  }

  Ok(())
}
