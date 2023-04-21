use {
  super::*,
  crate::{math::*, raytracing::*, samplers::Sampler, surfaces::Surface},
  serde::Deserialize,
  std::rc::Rc
};

#[derive(Debug, Deserialize)]
pub struct SurfaceListParameters {}

#[typetag::deserialize(name = "list")]
impl SurfaceGroupParameters for SurfaceListParameters {
  fn build_surface_group(
    &self,
    surfaces: Vec<Box<dyn Surface>>
  ) -> Result<Rc<dyn SurfaceGroup>, Box<dyn std::error::Error>> {
    Ok(Rc::new(SurfaceList::build(surfaces)))
  }
}

#[derive(Debug)]
pub struct SurfaceList {
  surfaces: Vec<(Box<dyn Surface>, WorldBoundingBox)>,
  emitter_indices: Vec<usize>
}

impl SurfaceList {
  pub fn build(surfaces: Vec<Box<dyn Surface>>) -> Self {
    let bboxes: Vec<_> = surfaces.iter().map(|s| s.world_bounding_box()).collect();
    let emitter_indices = surfaces
      .iter()
      .enumerate()
      .filter_map(|(i, s)| s.material().is_emissive().then_some(i))
      .collect();

    Self { surfaces: surfaces.into_iter().zip(bboxes.into_iter()).collect(), emitter_indices }
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

  fn pdf(&self, point: &WorldPoint, direction: &WorldDirection) -> Float {
    self
      .emitter_indices
      .iter()
      .map(|i| self.surfaces[*i].0.intersecting_direction_pdf(point, direction))
      .sum::<Float>()
      / (self.emitter_indices.len() as Float)
  }

  fn sample_and_pdf(
    &self,
    point: &WorldPoint,
    sampler: &mut dyn Sampler
  ) -> (WorldDirection, Float) {
    let index = (sampler.next_non_one() * (self.emitter_indices.len() as Float)) as usize;
    let emitter = &self.surfaces[self.emitter_indices[index]].0;
    let direction = emitter.interesting_direction_sample(point, sampler).0;
    let pdf = self.pdf(point, &direction);
    (direction, pdf)
  }
}
