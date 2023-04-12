use crate::materials::MaterialParameters;

use {
  super::*,
  crate::{bbox::*, math::*},
  std::{collections::HashMap, fmt::Debug}
};

#[typetag::deserialize(tag = "type")]
pub trait SurfaceParameters: Debug {
  fn build_surface(
    &self,
    materials: &HashMap<String, Box<dyn MaterialParameters>>
  ) -> Box<dyn Surface>;
}

pub trait Surface: Debug {
  fn world_bounding_box(&self) -> WorldBBox;

  fn intersect_world_ray(&self, ray: &WorldRay) -> Option<WorldHitInfo>;
}

pub trait TransformedSurface {
  type LocalSpace: Space<3>;

  fn local_to_world(&self) -> &LocalToWorld<Self::LocalSpace>;

  fn bounding_box(&self) -> BBox3<Self::LocalSpace>;

  fn intersect_ray(&self, ray: &Ray3<Self::LocalSpace>) -> Option<HitInfo<Self::LocalSpace>>;
}

impl<T: TransformedSurface + Debug> Surface for T {
  fn world_bounding_box(&self) -> WorldBBox {
    todo!() // Transform the local bounding box and returning a bounding box of the resulting prism
  }

  fn intersect_world_ray(&self, ray: &WorldRay) -> Option<WorldHitInfo> {
    let maybe_local_hit = self.intersect_ray(&self.local_to_world().inverse_ray(ray));
    maybe_local_hit.map(|local_hit| local_hit.transform(self.local_to_world()))
  }
}
