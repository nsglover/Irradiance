use std::sync::Arc;

use serde::Deserialize;

use super::*;
use crate::{
  integrators::{Integrator, IntegratorParameters},
  light::*,
  raytracing::*,
  sampling::Sampler,
  surface_groups::SurfaceGroup,
  BuildSettings
};

#[derive(Debug, Deserialize)]
pub struct PhotonTracerParameters {
  #[serde(alias = "num-photons")]
  num_photons: usize
}

#[typetag::deserialize(name = "photon-tracer")]
impl IntegratorParameters for PhotonTracerParameters {
  fn build_integrator(
    &self,
    surfaces: Arc<dyn SurfaceGroup>,
    settings: BuildSettings
  ) -> Result<Box<dyn Integrator + Sync + Send>, Box<dyn std::error::Error>> {
    Ok(Box::new(PhotonTracer {
      photon_map: PhotonMap::build(surfaces.clone(), self.num_photons, settings),
      surfaces
    }))
  }
}

struct PhotonTracer {
  surfaces: Arc<dyn SurfaceGroup>,
  photon_map: PhotonMap
}

impl PhotonTracer {}

impl Integrator for PhotonTracer {
  fn radiance_estimate(&self, _: &mut dyn Sampler, ray: WorldRay) -> Color {
    if let Some(hit) = self.surfaces.intersect_world_ray(ray) {
      let n: nalgebra::Unit<_> = hit.shading_normal.into();
      Color::new(n.x.abs(), n.y.abs(), n.z.abs())
    } else {
      Color::black()
    }
  }
}

unsafe impl Sync for PhotonTracer {}

unsafe impl Send for PhotonTracer {}
