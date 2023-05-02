use std::{collections::HashMap, fmt::Debug, sync::Arc};

use super::Mesh;
use crate::{common::Wrapper, materials::Material, math::*, raytracing::*, BuildSettings};

#[typetag::deserialize(tag = "type")]
pub trait SurfaceParameters: Debug {
  fn build_surface(
    &self,
    materials: &HashMap<String, Arc<dyn Material>>,
    meshes: &HashMap<String, Mesh>,
    settings: BuildSettings
  ) -> Box<dyn Surface>;

  fn is_emissive(&self, materials: &HashMap<String, Arc<dyn Material>>) -> bool;
}

pub trait Surface: Debug {
  fn intersect_world_ray(
    &self,
    ray: &mut WorldRay
  ) -> Option<(WorldRayIntersection, &dyn Material)>;

  fn world_bounding_box(&self) -> WorldBoundingBox;
}

pub trait TransformedSurface {
  type LocalSpace: Space<3>;

  fn local_to_world(&self) -> &LocalToWorld<Self::LocalSpace>;

  fn intersect_ray(
    &self,
    ray: &mut Ray3<Self::LocalSpace>
  ) -> Option<(RayIntersection<Self::LocalSpace>, &dyn Material)>;

  fn local_bounding_box(&self) -> BoundingBox3<Self::LocalSpace>;
}

// TODO: Move this to transform class
impl<T: TransformedSurface + Debug> Surface for T {
  fn intersect_world_ray(
    &self,
    ray: &mut WorldRay
  ) -> Option<(WorldRayIntersection, &dyn Material)> {
    let tr = self.local_to_world();
    let mut local_ray = tr.inverse_ray(&ray);
    self.intersect_ray(&mut local_ray).map(|(local_hit, mat)| (tr.ray_intersect(&local_hit), mat))
  }

  fn world_bounding_box(&self) -> WorldBoundingBox {
    let bbox = self.local_bounding_box();
    let min = bbox.min().into_inner();
    let max = bbox.max().into_inner();
    if bbox.is_empty() {
      WorldBoundingBox::new(min.into(), max.into())
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

      let mut transformed_bbox = WorldBoundingBox::default();
      for point in points {
        transformed_bbox.enclose_point(&(self.local_to_world().point(&point)));
      }

      transformed_bbox
    }
  }

  // fn intersecting_direction_sample(
  //   &self,
  //   point: &WorldPoint,
  //   sampler: &mut dyn Sampler
  // ) -> (WorldUnitVector, Real) {
  //   let tr = self.local_to_world();
  //   let (dir, pdf) = self.intersecting_direction_sample(&tr.inverse_point(point), sampler);
  //   (tr.direction(&dir), pdf)
  // }

  // fn intersecting_direction_pdf(&self, point: &WorldPoint, direction: &WorldUnitVector) -> Real {
  //   let tr = self.local_to_world();
  //   self.intersecting_direction_pdf(&tr.inverse_point(point), &tr.inverse_direction(direction))
  // }
}
