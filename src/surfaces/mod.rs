mod bvh;
mod emission;
mod mesh;
mod quad;
mod sphere;
mod surface;
mod surface_list;
mod triangle;

pub use mesh::*;
pub use surface::*;

use self::surface_list::SurfaceListParameters;
use crate::{materials::Material, BuildSettings};

pub fn default_grouping(
  surfaces: Vec<Box<dyn SurfaceParameters>>,
  materials: &std::collections::HashMap<String, std::sync::Arc<dyn Material>>,
  meshes: &std::collections::HashMap<String, Mesh>,
  settings: BuildSettings
) -> Box<dyn Surface> {
  (SurfaceListParameters { surfaces }).build_surface(materials, meshes, settings)
}
