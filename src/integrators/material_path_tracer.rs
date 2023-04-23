use std::rc::Rc;

use serde::Deserialize;

use super::*;
use crate::{
  light::*, materials::ReflectionType, math::Real, raytracing::*, samplers::*, surface_groups::*
};

// TODO: Add background to this
#[derive(Debug, Deserialize)]
struct Parameters {
  #[serde(alias = "average-path-length")]
  average_path_length: usize
}

#[typetag::deserialize(name = "material-path-tracer")]
impl IntegratorParameters for Parameters {
  fn build_integrator(
    &self,
    surfaces: Rc<dyn SurfaceGroup>
  ) -> Result<Box<dyn Integrator + Sync + Send>, Box<dyn std::error::Error>> {
    Ok(Box::new(MaterialPathTracer {
      surfaces,
      path_termination_probability: 1.0 / (self.average_path_length as Real),
      background: Color::black()
    }))
  }
}

pub struct MaterialPathTracer {
  pub(super) surfaces: Rc<dyn SurfaceGroup>,
  pub(super) path_termination_probability: Real,
  pub(super) background: Color
}

impl PathTraceIntegrator for MaterialPathTracer {
  fn initial_path_terminator(&self, ray: WorldRay) -> PathTerminator {
    PathTerminator::new(ray, self.path_termination_probability)
  }

  fn sample_scatter(
    &self,
    sampler: &mut dyn Sampler,
    ray: WorldRay
  ) -> Result<(Color, Color, WorldRay, ReflectionType), Color> {
    if let Some(hit) = self.surfaces.intersect_world_ray(ray) {
      let sample = hit.surface.material().sample(&hit, sampler);
      let radiance_emitted = sample.emission.unwrap_or(Color::black());

      if let Some((attenuation, scattered_ray, reflection_type)) = sample.reflection {
        Ok((radiance_emitted, attenuation, scattered_ray, reflection_type))
      } else {
        Err(radiance_emitted)
      }
    } else {
      Err(self.background)
    }
  }
}

unsafe impl Sync for MaterialPathTracer {}

unsafe impl Send for MaterialPathTracer {}
