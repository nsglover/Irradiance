use serde::Deserialize;

use super::*;
use crate::{
  light::*, materials::ScatterRandomVariable, raytracing::*, sampling::Sampler, scene::Scene, BuildSettings
};

#[derive(Debug, Deserialize)]
pub struct NormalIntegratorParameters;

#[typetag::deserialize(name = "normals")]
impl IntegratorParameters for NormalIntegratorParameters {
  fn build_integrators(
    &self,
    scene: Scene,
    _: BuildSettings
  ) -> Result<Vec<Box<dyn Integrator>>, Box<dyn std::error::Error>> {
    Ok(vec![Box::new(NormalIntegrator { scene })])
  }
}

struct NormalIntegrator {
  scene: Scene
}

impl Integrator for NormalIntegrator {
  fn radiance_estimate(&self, sampler: &mut dyn Sampler, ray: WorldRay) -> Color {
    let out_dir = -ray.dir();
    if let Some(hit) = self.scene.intersect_world_ray(ray) {
      let mut radiance_emitted = hit.material.emitted(&hit).unwrap_or(Color::black());
      if let Some(scatter_rv) = hit.material.scatter_random_variable() {
        let param = (hit.surface_point, out_dir);
        match scatter_rv {
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
      }

      radiance_emitted
    } else {
      Color::black()
    }
  }
}

unsafe impl Sync for NormalIntegrator {}

unsafe impl Send for NormalIntegrator {}
