use {
  crate::{color::Color, ray::WorldRay, samplers::Sampler, surface_groups::SurfaceGroup},
  std::{error::Error, fmt::Debug}
};

#[typetag::serde(tag = "type")]
pub trait IntegratorParameters: Debug {
  fn build_integrator(
    &self,
    surfaces: Box<dyn SurfaceGroup>
  ) -> Result<Box<dyn Integrator>, Box<dyn Error>>;
}

pub trait Integrator: Debug {
  fn estimate_radiance(&self, sampler: &mut dyn Sampler, ray: WorldRay) -> Color;
}
