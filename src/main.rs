use {
  clap::Parser,
  std::{error::Error, fs::File, io::BufReader}
};

use renderer::Renderer;
mod bbox;
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

// Important Features:
// TODO: Progress bars
// TODO: Privatize all the inner members of math structures
// TODO: BVH
// TODO: Dieletric and metal materials
// TODO: Stratified sampling
// TODO: Direct lighting MIS
// TODO: Transforms optimization
// TODO: Image loading and image texture interface
// TODO: Mesh loading and the triangle mesh surface
// TODO: Environment map

// Side Features:
// TODO: Perlin noise
// TODO: Blend material (arbitrary number rather than just 2)

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
  println!("Image will be saved to \"{file_name}\".");

  let mut time = std::time::Instant::now();

  let reader = BufReader::new(File::open(scene_file)?);
  let json = serde_json::from_reader(reader)?;
  let renderer = Renderer::build_from_json(json, num_threads, !no_progress_bar)?;

  println!("Build Time: {} Seconds", time.elapsed().as_secs_f32());
  time = std::time::Instant::now();

  let image = renderer.render_scene();
  image.save(file_name)?;

  println!("Render Time: {} Seconds", time.elapsed().as_secs_f32());

  Ok(())
}
