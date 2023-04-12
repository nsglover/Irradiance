use {
  super::*,
  crate::{
    math::*,
    surfaces::{Surface, WorldHitInfo}
  },
  serde::{Deserialize, Serialize}
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfaceListParameters {}

#[typetag::deserialize(name = "list")]
impl SurfaceGroupParameters for SurfaceListParameters {
  fn build_surface_group(
    &self,
    surfaces: Vec<Box<dyn Surface>>
  ) -> Result<Box<dyn SurfaceGroup>, Box<dyn std::error::Error>> {
    Ok(Box::new(SurfaceList { surfaces }))
  }
}

#[derive(Debug)]
pub struct SurfaceList {
  surfaces: Vec<Box<dyn Surface>>
}

impl SurfaceGroup for SurfaceList {
  fn intersect_world_ray(&self, ray: &mut WorldRay) -> Option<WorldHitInfo> {
    let mut closest = None;
    for s in &self.surfaces {
      if let Some(hit) = s.intersect_world_ray(&ray) {
        ray.time_bounds.1 = hit.hit_time;
        closest = Some(hit);
      }
    }

    closest
  }
}
