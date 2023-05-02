mod bvh;
mod mesh;
mod quad;
mod sphere;
mod surface;
mod surface_list;
mod triangle;

use std::{collections::HashMap, sync::Arc};

pub use mesh::*;
pub use surface::*;

use self::surface_list::SurfaceListParameters;
use crate::{materials::Material, BuildSettings};

pub fn default_grouping(
  surfaces: Vec<Box<dyn SurfaceParameters>>,
  materials: &HashMap<String, Arc<dyn Material>>,
  meshes: &HashMap<String, Mesh>,
  settings: BuildSettings
) -> Box<dyn Surface> {
  (SurfaceListParameters { surfaces }).build_surface(materials, meshes, settings)
}
