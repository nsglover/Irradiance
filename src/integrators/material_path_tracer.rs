use serde::Deserialize;

use super::*;
use crate::{
  light::*,
  materials::ScatterRandomVariable,
  math::{PositiveReal, Real},
  raytracing::*,
  sampling::*,
  scene::Scene,
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
  fn build_integrators(
    &self,
    scene: Scene,
    _: BuildSettings
  ) -> Result<Vec<Box<dyn Integrator>>, Box<dyn std::error::Error>> {
    Ok(vec![Box::new(MaterialPathTracer {
      scene,
      path_termination_probability: PositiveReal::new_unchecked(
        1.0 / (self.average_path_length as Real)
      ),
      background: Color::black()
    })])
  }
}

pub struct MaterialPathTracer {
  scene: Scene,
  path_termination_probability: PositiveReal,
  background: Color
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
    if let Some((hit, material)) = self.scene.intersect_world_ray(ray) {
      let radiance_emitted = material.emitted(&hit).unwrap_or(Color::black());
      if let Some(scatter_rv) = material.scatter_random_variable() {
        match scatter_rv {
          ScatterRandomVariable::Continuous(rv) => {
            if let Some((sample, pdf)) = rv.sample_with_pdf(&hit, sampler) {
              return Ok((
                radiance_emitted,
                material.bsdf(&hit, &sample),
                Ray::new(hit.intersect_point, sample),
                Some(pdf)
              ));
            }
          },
          ScatterRandomVariable::Discrete(rv) => {
            if let Some(sample) = rv.sample(&hit, sampler) {
              return Ok((
                radiance_emitted,
                material.bsdf(&hit, &sample),
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
