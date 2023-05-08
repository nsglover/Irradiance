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
mod sampling;
mod scene;
mod surfaces;
mod textures;

// Top Priority:
// TODO: Undo any last minute hacks for the project
// TODO: Any TODOs scattered throughout the code
// TODO: Does glass need PMF? Does glass need ni^2/no^2?
// TODO: BSDF SHOULD NOT RETURN COSINE TERM!!!
// TODO: BSDF SHOULD USE MATHEMATICAL CONVENTIONS FOR INPUT DIRECTION (i.e. negate it!)
// TODO: Split hit info into surface info and other stuff (like intersect time)
// TODO: Scale refracted radiance by ni^2/no^2 or whatever it is
// TODO: Add a debug mode which checks for NaNs and infinites and stuff like that?

// Major Features:
// TODO: Allow rays to carry more information
// TODO: Image loading and image texture implementation
// TODO: Use mesh surface normals and texcoords when available. Make sure to adjust for this in all
//       integrators; need to multiply by a ratio of dot products with geometric and shading normal
// TODO: Stratified sampling
// TODO: Direct-lighting MIS and mixture sampling
// TODO: Generalize MIS and mixture integrators to work with any number of arbitrary integrators
// TODO: Interfering mediums
// TODO: Even fancier integrators

// Major Optimizations:
// TODO: BVH should attempt to intersect the closer of its two children
// TODO: Avoid dynamically transformed surfaces at all costs; only do this for sufficiently
//       complex implicit surfaces which are invariant under linear transformation (not spheres)
// TODO: Investigate the performance disparity and fix it

// Minor Features:
// TODO: Perlin noise texture
// TODO: All the PA2 BSDFs
// TODO: Blend material (from any number of materials, which can be implemented using the original
//       one that just works for two.)
// TODO: Environment map

// Minor Code Improvements:
// TODO: Move camera into scene, make samples-per-pixel an integrator-specific thing
// TODO: Allow user to customize whether we use BVH or surface list for the emissive partition
// TODO: Better distinction between emissive and non-emissive things
// TODO: Better distinction between directions and surface normals
// TODO: All parameter structs should be consumed upon building their target

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
  let json = serde_json::from_reader(reader)?;
  let renderer = Renderer::build_from_json(json, BuildSettings { num_threads, use_progress_bar: !no_progress_bar })?;

  println!("Building complete! Time: {}\n", duration_to_hms(&build_time.elapsed()));

  // Render the scene
  println!("Rendering scene \"{scene_name}.json\"...");
  let render_time = std::time::Instant::now();
  let image = renderer.render_scene(RenderSettings {
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
