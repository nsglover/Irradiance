use std::error::Error;

use serde::Deserialize;

use super::*;
use crate::{
  materials::ScatterRandomVariable, raytracing::*, sampling::Sampler, scene::Scene, spectrum::*, BuildSettings
};

#[derive(Debug, Deserialize)]
pub struct NormalIntegratorParameters;

#[typetag::deserialize(name = "normals")]
impl IntegratorParameters for NormalIntegratorParameters {
  fn build_integrator(&self, scene: Scene, _: BuildSettings) -> Result<Box<dyn Integrator>, Box<dyn Error>> {
    Ok(Box::new(NormalIntegrator { scene }))
  }
}

struct NormalIntegrator {
  scene: Scene
}

impl Integrator for NormalIntegrator {
  fn radiance_estimate(&self, sampler: &mut dyn Sampler, ray: WorldRay) -> Spectrum {
    let out_dir = -ray.dir();
    if let Some(hit) = self.scene.intersect_world_ray(ray) {
      let mut radiance_emitted = hit.light.radiance_emitted(&hit.surface_point, &out_dir);
      let param = (hit.surface_point, out_dir);

      match hit.material.random_bsdf_in_direction() {
        ScatterRandomVariable::Diffuse(rv) => {
          if let Some((in_dir, _)) = rv.sample_with_pdf(&param, sampler) {
            radiance_emitted += hit.material.bsdf_cos(&param.0, &in_dir, &out_dir);
          }
        },
        ScatterRandomVariable::Specular(rv) => {
          if let Some(in_dir) = rv.sample(&param, sampler) {
            radiance_emitted += hit.material.bsdf_cos(&param.0, &in_dir, &out_dir);
          }
        },
      }

      radiance_emitted
    } else {
      Spectrum::none()
    }
  }
}

unsafe impl Sync for NormalIntegrator {}

unsafe impl Send for NormalIntegrator {}
