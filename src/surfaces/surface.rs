use {
  crate::{
    materials::{Material, MaterialParameters},
    math::*,
    raytracing::*,
    samplers::Sampler
  },
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
  fn world_bounding_box(&self) -> WorldBoundingBox;

  fn intersect_world_ray(&self, ray: &WorldRay) -> Option<WorldRayIntersection>;

  fn interesting_direction_sample(
    &self,
    point: &WorldPoint,
    sampler: &mut dyn Sampler
  ) -> (WorldDirection, Float);

  fn intersecting_direction_pdf(&self, point: &WorldPoint, direction: &WorldDirection) -> Float;

  fn material(&self) -> &dyn Material;
}

pub trait TransformedSurface {
  type LocalSpace: Space<3>;

  fn local_to_world(&self) -> &LocalToWorld<Self::LocalSpace>;

  fn bounding_box(&self) -> BoundingBox3<Self::LocalSpace>;

  fn intersect_ray(&self, ray: Ray3<Self::LocalSpace>)
    -> Option<RayIntersection<Self::LocalSpace>>;

  fn interesting_direction_sample(
    &self,
    point: &Point3<Self::LocalSpace>,
    sampler: &mut dyn Sampler
  ) -> (Direction3<Self::LocalSpace>, Float);

  fn intersecting_direction_pdf(
    &self,
    point: &Point3<Self::LocalSpace>,
    direction: &Direction3<Self::LocalSpace>
  ) -> Float;

  fn material(&self) -> &dyn Material;
}

// TODO: Move this to transform class
impl<T: TransformedSurface + Debug> Surface for T {
  fn world_bounding_box(&self) -> WorldBoundingBox {
    let bbox = self.bounding_box();
    let min = bbox.min().inner();
    let max = bbox.max().inner();
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
        transformed_bbox.enclose_point(&(self.local_to_world() * &point));
      }

      transformed_bbox
    }
  }

  fn intersect_world_ray(&self, ray: &WorldRay) -> Option<WorldRayIntersection> {
    let tr = self.local_to_world();
    self.intersect_ray(tr.inverse_ray(ray)).map(|local_hit| tr.ray_intersect(&local_hit))
  }

  fn interesting_direction_sample(
    &self,
    point: &WorldPoint,
    sampler: &mut dyn Sampler
  ) -> (WorldDirection, Float) {
    let tr = self.local_to_world();
    let (dir, pdf) = self.interesting_direction_sample(&tr.inverse_point(point), sampler);
    (tr.direction(&dir), pdf)
  }

  fn intersecting_direction_pdf(&self, point: &WorldPoint, direction: &WorldDirection) -> Float {
    let tr = self.local_to_world();
    self.intersecting_direction_pdf(&tr.inverse_point(point), &tr.inverse_direction(direction))
  }

  fn material(&self) -> &dyn Material { self.material() }
}
