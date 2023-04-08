use {
  super::*,
  crate::{color::Color, materials::*, ray::*, samplers::*, surface_groups::*}
};

#[derive(Debug)]
pub struct PathTracerMats<'a> {
  max_bounces: usize,
  surfaces: &'a dyn SurfaceGroup
}

impl PathTracerMats<'_> {
  // TODO: Convert to a while loop for efficiency
  fn recursive_estimate(
    &self,
    sampler: &mut dyn Sampler,
    ray: WorldRay,
    remaining_bounces: usize
  ) -> Color {
    if let Some(hit) = self.surfaces.intersect_world_ray(ray) {
      let sample = hit.material.sample(&hit, sampler);
      let emitted = sample.emission.unwrap_or(Color::black());
      if remaining_bounces == 0 {
        return emitted;
      }

      if let Some((attenuation, scattered, reflection_type)) = sample.reflection {
        let mut rec = self.recursive_estimate(sampler, scattered, remaining_bounces - 1);
        if let ReflectionType::Diffuse(pdf) = reflection_type {
          rec /= pdf;
        }

        emitted + attenuation * rec
      } else {
        emitted
      }
    } else {
      Color::black() // TODO: Return a background environment map
    }
  }
}

impl Integrator for PathTracerMats<'_> {
  fn estimate_radiance(&self, sampler: &mut dyn Sampler, ray: WorldRay) -> Color {
    self.recursive_estimate(sampler, ray, self.max_bounces)
  }
}
