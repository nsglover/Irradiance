use std::sync::Arc;

use serde::Deserialize;

use super::*;
use crate::{
  light::*, raytracing::*, sampling::Sampler, surface_groups::SurfaceGroup, BuildSettings
};

#[derive(Debug, Deserialize)]
pub struct NormalIntegratorParameters;

#[typetag::deserialize(name = "normals")]
impl IntegratorParameters for NormalIntegratorParameters {
  fn build_integrator(
    &self,
    surfaces: Arc<dyn SurfaceGroup>,
    _: BuildSettings
  ) -> Result<Box<dyn Integrator + Sync + Send>, Box<dyn std::error::Error>> {
    Ok(Box::new(NormalIntegrator { surfaces }))
  }
}

struct NormalIntegrator {
  surfaces: Arc<dyn SurfaceGroup>
}

impl Integrator for NormalIntegrator {
  fn radiance_estimate(&self, _: &mut dyn Sampler, ray: WorldRay) -> Color {
    if let Some(hit) = self.surfaces.intersect_world_ray(ray) {
      let n: nalgebra::Unit<_> = hit.shading_normal.into();
      Color::new(n.x.abs(), n.y.abs(), n.z.abs())
    } else {
      Color::black()
    }
  }
}

unsafe impl Sync for NormalIntegrator {}

unsafe impl Send for NormalIntegrator {}
