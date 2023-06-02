use crate::{raytracing::*, surfaces::Surface};

const NUM_PARTS: usize = 2;

pub struct Scene {
  /// The element surface_partition[0] is the non-emissive part of the scene, and likewise
  /// surface_partition[1] is the emissive part of the scene
  surface_partition: [Box<dyn Surface>; NUM_PARTS]
}

impl Scene {
  pub fn new(non_emissive_part: Box<dyn Surface>, emissive_part: Box<dyn Surface>) -> Self {
    Self { surface_partition: [non_emissive_part, emissive_part] }
  }

  pub fn intersect_world_ray(&self, mut ray: WorldRay) -> Option<WorldSurfaceInterface> {
    let mut closest = None;
    for p in 0..NUM_PARTS {
      if let Some(hit) = self.surface_partition[p].intersect_world_ray(&mut ray) {
        ray.set_max_intersect_dist(hit.intersect_dist);
        closest = Some(hit);
      }
    }

    closest
  }

  pub fn emissive_part(&self) -> &dyn Surface { self.surface_partition[1].as_ref() }
}

unsafe impl Send for Scene {}

unsafe impl Sync for Scene {}
