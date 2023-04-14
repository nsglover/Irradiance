use {
  super::*,
  crate::{math::*, surfaces::Surface},
  serde::Deserialize
};

#[derive(Debug, Deserialize)]
pub struct SurfaceListParameters {}

#[typetag::deserialize(name = "list")]
impl SurfaceGroupParameters for SurfaceListParameters {
  fn build_surface_group(
    &self,
    surfaces: Vec<Box<dyn Surface>>
  ) -> Result<Box<dyn SurfaceGroup>, Box<dyn std::error::Error>> {
    Ok(Box::new(SurfaceList::build(surfaces)))
  }
}

#[derive(Debug)]
pub struct SurfaceList {
  surfaces: Vec<(Box<dyn Surface>, WorldBBox)>
}

impl SurfaceList {
  pub fn build(surfaces: Vec<Box<dyn Surface>>) -> Self {
    let bboxes: Vec<_> = surfaces.iter().map(|s| s.world_bounding_box()).collect();
    Self { surfaces: surfaces.into_iter().zip(bboxes.into_iter()).collect() }
  }
}

impl SurfaceGroup for SurfaceList {
  fn num_surfaces(&self) -> usize { self.surfaces.len() }

  fn intersect_world_ray(&self, mut ray: WorldRay) -> Option<WorldRayIntersection> {
    let mut closest = None;
    for (surface, bbox) in &self.surfaces {
      if bbox.ray_intersects(&ray) {
        if let Some(hit) = surface.intersect_world_ray(&ray) {
          ray.set_max_intersect_time(hit.intersect_time);
          closest = Some(hit);
        }
      }
    }

    closest
  }
}
