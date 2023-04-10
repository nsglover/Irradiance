use {
  super::*,
  crate::{color::Color, ray::WorldRay, samplers::Sampler, surface_groups::SurfaceGroup},
  serde::{Deserialize, Serialize}
};

#[derive(Debug, Deserialize, Serialize)]
pub struct NormalIntegratorParameters;

#[typetag::serde(name = "normals")]
impl IntegratorParameters for NormalIntegratorParameters {
  fn build_integrator(
    &self,
    surfaces: Box<dyn SurfaceGroup>
  ) -> Result<Box<dyn Integrator + Sync + Send>, Box<dyn std::error::Error>> {
    Ok(Box::new(NormalIntegrator { surfaces }))
  }
}

#[derive(Debug)]
pub struct NormalIntegrator {
  surfaces: Box<dyn SurfaceGroup>
}

impl Integrator for NormalIntegrator {
  fn estimate_radiance(&self, _: &mut dyn Sampler, ray: WorldRay) -> Color {
    if let Some(hit) = self.surfaces.intersect_world_ray(ray) {
      let n: nalgebra::Unit<_> = hit.shading_normal.into();
      Color::new(n.x.abs(), n.y.abs(), n.z.abs())
    } else {
      Color::black() // TODO: Return a background environment map
    }
  }
}

unsafe impl Sync for NormalIntegrator {}

unsafe impl Send for NormalIntegrator {}
