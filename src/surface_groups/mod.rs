// mod bvh;
mod surface_group;
mod surface_list;

pub use surface_group::*;

pub fn default_surface_group() -> Box<dyn SurfaceGroupParameters> {
  Box::new(surface_list::SurfaceListParameters {})
}
