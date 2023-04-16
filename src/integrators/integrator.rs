use {
  crate::{light::*, raytracing::*, samplers::Sampler, surface_groups::SurfaceGroup},
  std::{error::Error, fmt::Debug}
};

#[typetag::deserialize(tag = "type")]
pub trait IntegratorParameters: Debug {
  fn build_integrator(
    &self,
    surfaces: Box<dyn SurfaceGroup>
  ) -> Result<Box<dyn Integrator + Sync + Send>, Box<dyn Error>>;
}

pub trait Integrator {
  fn radiance(&self, sampler: &mut dyn Sampler, maybe_ray: PathTerminator) -> Color;

  fn incoming_radiance(&self, sampler: &mut dyn Sampler, ray: WorldRay) -> Color {
    self.radiance(sampler, self.initial_path_terminator(ray))
  }

  fn initial_path_terminator(&self, ray: WorldRay) -> PathTerminator;
}

pub trait ParameterizedIntegrator {
  fn initial_path_terminator(&self, ray: WorldRay) -> PathTerminator;

  fn estimate_radiance(
    &self,
    sampler: &mut dyn Sampler,
    maybe_ray: PathTerminator,
    integrator: &dyn Integrator
  ) -> Color;
}

impl<T: ParameterizedIntegrator> Integrator for T {
  fn radiance(&self, sampler: &mut dyn Sampler, maybe_ray: PathTerminator) -> Color {
    self.estimate_radiance(sampler, maybe_ray, self)
  }

  fn initial_path_terminator(&self, ray: WorldRay) -> PathTerminator {
    self.initial_path_terminator(ray)
  }
}
