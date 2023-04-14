use {
  clap::Parser,
  std::{error::Error, fs::File, io::BufReader, time::Duration}
};

use renderer::Renderer;
mod camera;
mod color;
mod integrators;
mod materials;
mod math;
mod renderer;
mod samplers;
mod surface_groups;
mod surfaces;
mod textures;
mod wrapper;

// Project Features:
// TODO: Photon mapping

// Important Features:
// TODO: Dieletric and metal materials
// TODO: Stratified sampling
// TODO: Direct lighting MIS

// Minor Improvements:
// TODO: Stop cloning rays so much
// TODO: Better system for transform inverses
// TODO: Optimize transform system

// Side Features:
// TODO: Perlin noise
// TODO: Blend material (arbitrary number rather than just 2)
// TODO: Image loading and image texture interface
// TODO: Mesh loading and the triangle mesh surface
// TODO: Environment map

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = Some(""))]
struct Arguments {
  scene_file: String,

  #[arg(short = 'o', long)]
  image_file: Option<String>,

  #[arg(short = 'j', long = "threads", default_value_t = 1)]
  num_threads: u16,

  #[arg(long)]
  no_progress_bar: bool
}

fn duration_to_hms(time: &Duration) -> String {
  let total_seconds = time.as_secs();
  let s = total_seconds % 60;
  let m = (total_seconds / 60) % 60;
  let h = (total_seconds / 60) / 60;
  format!("{:0>2}:{:0>2}:{:0>2}", h, m, s)
}

fn main() -> Result<(), Box<dyn Error>> {
  let Arguments { scene_file, image_file, num_threads, no_progress_bar } = Arguments::parse();

  if num_threads == 0 {
    panic!("At least 1 thread is necessary to run the renderer!");
  }

  if !scene_file.ends_with(".json") {
    panic!("Scene file must have the .json file suffix!");
  }

  let default_file_name = scene_file.chars().take(scene_file.len() - ".json".len()).collect();
  let file_name = image_file.unwrap_or(default_file_name) + ".png";

  println!("Using scene file \"{scene_file}\".");
  println!("Image will be saved to \"{file_name}\".\n");

  println!("Building scene...");
  let build_time = std::time::Instant::now();
  let reader = BufReader::new(File::open(scene_file)?);
  let json = serde_json::from_reader(reader)?;
  let renderer = Renderer::build_from_json(json, num_threads, !no_progress_bar)?;
  println!("Done! Build Time: {}\n", duration_to_hms(&build_time.elapsed()));

  println!("Rendering scene...");
  let render_time = std::time::Instant::now();
  let image = renderer.render_scene();
  image.save(file_name)?;
  println!("Done! Render Time: {}", duration_to_hms(&render_time.elapsed()));

  Ok(())
}
