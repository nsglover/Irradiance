use std::sync::Arc;

use serde::Deserialize;

use super::*;
use crate::{
  light::*,
  materials::ScatterRandomVariable,
  math::{PositiveReal, Real},
  raytracing::*,
  sampling::*,
  surface_groups::*,
  BuildSettings
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
    surfaces: Arc<dyn SurfaceGroup>,
    _: BuildSettings
  ) -> Result<Box<dyn Integrator + Sync + Send>, Box<dyn std::error::Error>> {
    Ok(Box::new(MaterialPathTracer {
      surfaces,
      path_termination_probability: 1.0 / (self.average_path_length as Real),
      background: Color::black()
    }))
  }
}

pub struct MaterialPathTracer {
  pub(super) surfaces: Arc<dyn SurfaceGroup>,
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
  ) -> Result<(Color, Color, WorldRay, Option<PositiveReal>), Color> {
    if let Some(hit) = self.surfaces.intersect_world_ray(ray) {
      let radiance_emitted = hit.material.emitted(&hit);
      if let Some(scatter_rv) = hit.material.scatter_random_variable() {
        match scatter_rv {
          ScatterRandomVariable::Diffuse(rv) => {
            if let Some((sample, pdf)) = rv.sample_with_pdf(&hit, sampler) {
              return Ok((
                radiance_emitted,
                hit.material.bsdf(&hit, &sample),
                Ray::new(hit.intersect_point, sample),
                Some(pdf)
              ));
            }
          },
          ScatterRandomVariable::Specular(rv) => {
            if let Some(sample) = rv.sample(&hit, sampler) {
              return Ok((
                radiance_emitted,
                hit.material.bsdf(&hit, &sample),
                Ray::new(hit.intersect_point, sample),
                None
              ));
            }
          },
        }
      }

      Err(radiance_emitted)
    } else {
      Err(self.background)
    }
  }
}

unsafe impl Sync for MaterialPathTracer {}

unsafe impl Send for MaterialPathTracer {}
