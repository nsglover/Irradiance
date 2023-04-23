use std::rc::Rc;

use serde::Deserialize;

use super::*;
use crate::{light::*, raytracing::*, samplers::Sampler, surface_groups::SurfaceGroup};

#[derive(Debug, Deserialize)]
pub struct NormalIntegratorParameters;

#[typetag::deserialize(name = "normals")]
impl IntegratorParameters for NormalIntegratorParameters {
  fn build_integrator(
    &self,
    surfaces: Rc<dyn SurfaceGroup>
  ) -> Result<Box<dyn Integrator + Sync + Send>, Box<dyn std::error::Error>> {
    Ok(Box::new(NormalIntegrator { surfaces }))
  }
}

struct NormalIntegrator {
  surfaces: Rc<dyn SurfaceGroup>
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
