use crate::materials::MaterialParameters;

use {
  crate::math::*,
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

  fn intersect_world_ray(&self, ray: &WorldRay) -> Option<WorldRayIntersection>;
}

pub trait TransformedSurface {
  type LocalSpace: Space<3>;

  fn local_to_world(&self) -> &LocalToWorld<Self::LocalSpace>;

  fn bounding_box(&self) -> BBox3<Self::LocalSpace>;

  fn intersect_ray(
    &self,
    ray: &Ray3<Self::LocalSpace>
  ) -> Option<RayIntersection<Self::LocalSpace>>;
}

// TODO: Move this to transform class
impl<T: TransformedSurface + Debug> Surface for T {
  fn world_bounding_box(&self) -> WorldBBox {
    let bbox = self.bounding_box();
    let min = bbox.min().inner();
    let max = bbox.max().inner();
    if bbox.is_empty() {
      WorldBBox::new(min.into(), max.into())
    } else {
      let mut points: [Point3<T::LocalSpace>; 8] = [Point3::origin(); 8];
      points[0] = Point3::from(nalgebra::point![min.x, min.y, min.z]);
      points[1] = Point3::from(nalgebra::point![max.x, min.y, min.z]);
      points[2] = Point3::from(nalgebra::point![min.x, max.y, min.z]);
      points[3] = Point3::from(nalgebra::point![min.x, min.y, max.z]);
      points[4] = Point3::from(nalgebra::point![max.x, max.y, min.z]);
      points[5] = Point3::from(nalgebra::point![min.x, max.y, max.z]);
      points[6] = Point3::from(nalgebra::point![max.x, min.y, max.z]);
      points[7] = Point3::from(nalgebra::point![max.x, max.y, max.z]);

      let mut transformed_bbox = WorldBBox::default();
      for i in 0..8 {
        transformed_bbox.enclose_point(&(self.local_to_world() * &points[i]));
      }

      transformed_bbox
    }
  }

  fn intersect_world_ray(&self, ray: &WorldRay) -> Option<WorldRayIntersection> {
    let maybe_local_hit = self.intersect_ray(&self.local_to_world().inverse_ray(ray));
    maybe_local_hit.map(|local_hit| self.local_to_world().ray_intersect(&local_hit))
  }
}
