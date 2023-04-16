use {
  super::*,
  crate::{light::*, raytracing::*, samplers::Sampler, surface_groups::SurfaceGroup},
  serde::Deserialize
};

#[derive(Debug, Deserialize)]
pub struct NormalIntegratorParameters;

#[typetag::deserialize(name = "normals")]
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
  fn radiance(&self, sampler: &mut dyn Sampler, maybe_ray: PathTerminator) -> Color {
    if let Some((ray, _, _)) = maybe_ray.into_ray(sampler) {
      if let Some(hit) = self.surfaces.intersect_world_ray(ray) {
        let n: nalgebra::Unit<_> = hit.shading_normal.into();
        return Color::new(n.x.abs(), n.y.abs(), n.z.abs());
      }
    }

    Color::black()
  }

  fn initial_path_terminator(&self, ray: WorldRay) -> PathTerminator {
    PathTerminator::new(ray, 0.0)
  }
}

unsafe impl Sync for NormalIntegrator {}

unsafe impl Send for NormalIntegrator {}
