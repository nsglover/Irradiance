use std::{error::Error, fs::File, io::BufReader, time::Duration};

use clap::Parser;
use renderer::Renderer;

mod camera;
mod integrators;
mod lights;
mod materials;
mod math;
mod raytracing;
mod renderer;
mod sampling;
mod scene;
mod spectrum;
mod surfaces;
mod textures;

// Top Priority:
// TODO: Deal with BSDFs which aren't self-adjoint (i.e. dielectrics and anything using shading normals)
// TODO: Add a debug mode which checks for NaNs and infinites and stuff like that?
// TODO: Does glass need PMF?
// TODO: Properly handle shading normals in BSDF sampling

// Major Features:
// TODO: Allow rays to carry more information
// TODO: Stratified sampling
// TODO: Direct-lighting MIS and mixture sampling
// TODO: Interfering mediums
// TODO: Fourier materials
// TODO: Fancier integrators

// Major Optimizations:
// TODO: Add an embree BVH surface
// TODO: GPU acceleration

// Minor Features:
// TODO: Perlin noise texture
// TODO: All the PA2 materials
// TODO: Environment map

// Minor Code Improvements:
// TODO: Move camera into scene, make samples-per-pixel an integrator-specific thing
// TODO: Allow user to customize whether we use BVH or surface list for the emissive partition
// TODO: All parameter structs should be consumed upon building their target
// TODO: Put parsing in it's own place
// TODO: Make a progress bar wrapper
// TODO: No more mutating rays in intersect_world_ray

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

  #[arg(short = 'q', long = "quiet")]
  no_progress_bar: bool
}

fn duration_to_hms(time: &Duration) -> String {
  let total_seconds = time.as_secs_f64();
  let s = total_seconds % 60.0;
  let m = (total_seconds / 60.0) % 60.0;
  let h = (total_seconds / 60.0) / 60.0;
  format!("{:0>2}:{:0>2}:{:0>2}", h as u8, m as u8, s as u8)
}

#[derive(Debug)]
pub struct RenderSettings {
  num_threads: usize,
  subimage_dimensions: (u32, u32),
  use_progress_bar: bool
}

#[derive(Debug, Clone, Copy)]
pub struct BuildSettings {
  num_threads: usize,
  use_progress_bar: bool
}

fn main() -> Result<(), Box<dyn Error>> {
  let Arguments { scene_file, image_file, subimage_edge_length, num_threads, no_progress_bar } = Arguments::parse();

  if num_threads == 0 {
    panic!("At least 1 thread is necessary to run the renderer!");
  }

  if !scene_file.ends_with(".json") {
    panic!("Scene file must have the .json file suffix!");
  }

  let scene_name = scene_file.chars().take(scene_file.len() - ".json".len()).collect();

  // Build the scene
  println!("\nBuilding scene from \"{scene_file}\"...");
  let build_time = std::time::Instant::now();
  let reader = BufReader::new(File::open(scene_file)?);
  let params = serde_json::from_reader(reader)?;
  let renderer = Renderer::build(params, BuildSettings { num_threads, use_progress_bar: !no_progress_bar })?;

  println!("Building complete! Time: {}\n", duration_to_hms(&build_time.elapsed()));

  // Render the scene
  println!("Rendering scene \"{scene_name}.json\"...");
  let render_time = std::time::Instant::now();
  let image = renderer.render(RenderSettings {
    num_threads,
    subimage_dimensions: (subimage_edge_length, subimage_edge_length),
    use_progress_bar: !no_progress_bar
  });

  if no_progress_bar {
    println!("Rendering complete! Time: {}\n", duration_to_hms(&render_time.elapsed()));
  } else {
    println!("Rendering complete!\n");
  }

  // Save the rendered image
  let file_name = image_file.unwrap_or(scene_name) + ".png";
  println!("Saving image to \"{file_name}\"...");
  image.save(file_name)?;
  println!("Done!\n");

  Ok(())
}
