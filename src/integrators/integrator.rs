use std::{error::Error, fmt::Debug};

use crate::{light::*, math::PositiveReal, raytracing::*, sampling::Sampler, scene::Scene, BuildSettings};

#[typetag::deserialize(tag = "type")]
pub trait IntegratorParameters: Debug {
  fn build_integrators(
    &self,
    scene: Scene,
    settings: BuildSettings
  ) -> Result<Vec<Box<dyn Integrator>>, Box<dyn Error>>;
}

#[typetag::deserialize(tag = "type")]
pub trait PathTraceIntegratorParameters: IntegratorParameters {}

pub trait Integrator: Send + Sync {
  fn radiance_estimate(&self, sampler: &mut dyn Sampler, ray: WorldRay) -> Color;
}

pub trait PathTraceIntegrator {
  fn initial_path_terminator(&self, ray: WorldRay) -> PathTerminator;

  /// Returns Ok((emitted, attenuation, scattered_ray, maybe_pdf)) or Err(final_estimate)
  fn sample_scatter(
    &self,
    sampler: &mut dyn Sampler,
    ray: WorldRay
  ) -> Result<(Color, Color, WorldRay, Option<PositiveReal>), Color>;
}

impl<T: PathTraceIntegrator + Send + Sync> Integrator for T {
  fn radiance_estimate(&self, sampler: &mut dyn Sampler, ray: WorldRay) -> Color {
    let mut terminator = self.initial_path_terminator(ray);
    let mut total_path_attenuation = Color::white();
    let mut radiance = Color::black();

    while let Some((ray, survival_probability, cont)) = terminator.into_ray(sampler) {
      match self.sample_scatter(sampler, ray) {
        Ok((emitted, attenuation, scattered_ray, maybe_pdf)) => {
          radiance += total_path_attenuation * emitted;
          total_path_attenuation *= attenuation / survival_probability;
          if let Some(sample_pdf) = maybe_pdf {
            total_path_attenuation /= sample_pdf.into_inner();
          }

          terminator = cont.into_terminator(scattered_ray);
        },
        Err(final_radiance) => {
          radiance += total_path_attenuation * final_radiance;
          break;
        }
      }
    }

    radiance
  }
}
