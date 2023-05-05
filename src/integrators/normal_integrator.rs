use serde::Deserialize;

use super::*;
use crate::{
  light::*, materials::ScatterRandomVariable, raytracing::*, sampling::Sampler, scene::Scene,
  BuildSettings
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
    if let Some((hit, mat)) = self.scene.intersect_world_ray(ray) {
      let mut radiance_emitted = mat.emitted(&hit).unwrap_or(Color::black());
      if let Some(scatter_rv) = mat.scatter_random_variable() {
        match scatter_rv {
          ScatterRandomVariable::Continuous(rv) => {
            if let Some((sample, _)) = rv.sample_with_pdf(&hit, sampler) {
              radiance_emitted += mat.bsdf(&hit, &sample);
            }
          },
          ScatterRandomVariable::Discrete(rv) => {
            if let Some(sample) = rv.sample(&hit, sampler) {
              radiance_emitted += mat.bsdf(&hit, &sample);
            }
          },
        }
      }

      radiance_emitted / 2.0
    } else {
      Color::black()
    }
  }
}

unsafe impl Sync for NormalIntegrator {}

unsafe impl Send for NormalIntegrator {}
