use std::error::Error;

use serde::Deserialize;

use super::*;
use crate::{
  materials::ScatterRandomVariable,
  math::{PositiveReal, Real},
  raytracing::*,
  sampling::*,
  scene::Scene,
  spectrum::*,
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
  fn build_integrator(&self, scene: Scene, _: BuildSettings) -> Result<Box<dyn Integrator>, Box<dyn Error>> {
    Ok(Box::new(MaterialPathTracer {
      scene,
      path_termination_probability: PositiveReal::new_unchecked(1.0 / (self.average_path_length as Real)),
      background: Spectrum::none()
    }))
  }
}

pub struct MaterialPathTracer {
  scene: Scene,
  path_termination_probability: PositiveReal,
  background: Spectrum
}

impl PathTraceIntegrator for MaterialPathTracer {
  fn initial_path_terminator(&self, ray: WorldRay) -> PathTerminator {
    PathTerminator::new(ray, self.path_termination_probability)
  }

  fn sample_scatter(
    &self,
    sampler: &mut dyn Sampler,
    ray: WorldRay
  ) -> Result<(Spectrum, Spectrum, WorldRay, Option<PositiveReal>), Spectrum> {
    let out_dir = -ray.dir();
    if let Some(hit) = self.scene.intersect_world_ray(ray) {
      let radiance_emitted = hit.light.radiance_emitted(&hit.surface_point, &out_dir);
      let param = (hit.surface_point, out_dir);

      match hit.material.random_bsdf_in_direction() {
        ScatterRandomVariable::Diffuse(rv) => {
          if let Some((in_dir, pdf)) = rv.sample_with_pdf(&param, sampler) {
            return Ok((
              radiance_emitted,
              hit.material.bsdf_cos(&param.0, &in_dir, &out_dir),
              Ray::new(param.0.point, in_dir),
              Some(pdf)
            ));
          }
        },
        ScatterRandomVariable::Specular(rv) => {
          if let Some(in_dir) = rv.sample(&param, sampler) {
            return Ok((
              radiance_emitted,
              hit.material.bsdf_cos(&param.0, &in_dir, &out_dir),
              Ray::new(param.0.point, in_dir),
              None
            ));
          }
        },
      }

      Err(radiance_emitted)
    } else {
      Err(self.background)
    }
  }
}

unsafe impl Sync for MaterialPathTracer {}

unsafe impl Send for MaterialPathTracer {}
