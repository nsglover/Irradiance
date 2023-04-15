use {
  super::*,
  crate::{color::Color, materials::*, math::*, samplers::*, surface_groups::*},
  serde::Deserialize
};

#[derive(Debug, Deserialize)]
struct PathTracerParameters {
  max_bounces: usize
}

#[typetag::deserialize(name = "material-path-tracer")]
impl IntegratorParameters for PathTracerParameters {
  fn build_integrator(
    &self,
    surfaces: Box<dyn SurfaceGroup>
  ) -> Result<Box<dyn Integrator + Sync + Send>, Box<dyn std::error::Error>> {
    Ok(Box::new(SimplePathTracer { max_bounces: self.max_bounces, surfaces }))
  }
}

#[derive(Debug)]
pub struct SimplePathTracer {
  max_bounces: usize,
  surfaces: Box<dyn SurfaceGroup>
}

impl SimplePathTracer {
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
      Color::black()
    }
  }
}

impl Integrator for SimplePathTracer {
  fn estimate_radiance(&self, sampler: &mut dyn Sampler, ray: WorldRay) -> Color {
    self.recursive_estimate(sampler, ray, self.max_bounces)
  }
}

unsafe impl Sync for SimplePathTracer {}

unsafe impl Send for SimplePathTracer {}
