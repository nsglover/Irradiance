mod bvh;
mod mesh;
mod quad;
mod sphere;
mod surface;
mod surface_list;
mod triangle;

pub use mesh::*;
pub use surface::*;

use self::surface_list::SurfaceListParameters;
use crate::{lights::Light, materials::Material, BuildSettings};

pub fn default_grouping(
  surfaces: Vec<Box<dyn SurfaceParameters>>,
  lights: &std::collections::HashMap<String, std::sync::Arc<dyn Light>>,
  materials: &std::collections::HashMap<String, std::sync::Arc<dyn Material>>,
  meshes: &std::collections::HashMap<String, Mesh>,
  settings: BuildSettings
) -> Box<dyn Surface> {
  (SurfaceListParameters { surfaces }).build_surface(lights, materials, meshes, settings)
}
