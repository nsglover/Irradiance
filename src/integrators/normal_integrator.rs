use serde::Deserialize;

use super::*;
use crate::{light::*, raytracing::*, sampling::Sampler, scene::Scene, BuildSettings};

#[derive(Debug, Deserialize)]
pub struct NormalIntegratorParameters;

#[typetag::deserialize(name = "normals")]
impl IntegratorParameters for NormalIntegratorParameters {
  fn build_integrator(
    &self,
    scene: Scene,
    _: BuildSettings
  ) -> Result<Box<dyn Integrator>, Box<dyn std::error::Error>> {
    Ok(Box::new(NormalIntegrator { scene }))
  }
}

struct NormalIntegrator {
  scene: Scene
}

impl Integrator for NormalIntegrator {
  fn radiance_estimate(&self, _: &mut dyn Sampler, ray: WorldRay) -> Color {
    if let Some((hit, _)) = self.scene.intersect_world_ray(ray) {
      let n: nalgebra::Unit<_> = hit.geometric_normal.into();
      Color::new(n.x.abs(), n.y.abs(), n.z.abs())
    } else {
      Color::black()
    }
  }
}

unsafe impl Sync for NormalIntegrator {}

unsafe impl Send for NormalIntegrator {}
