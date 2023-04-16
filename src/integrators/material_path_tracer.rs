use {
  super::*,
  crate::{light::*, materials::*, math::Float, raytracing::*, samplers::*, surface_groups::*},
  serde::Deserialize
};

// TODO: Add background to this
#[derive(Debug, Deserialize)]
struct PathTracerParameters {
  #[serde(alias = "average-path-length")]
  average_path_length: usize
}

#[typetag::deserialize(name = "material-path-tracer")]
impl IntegratorParameters for PathTracerParameters {
  fn build_integrator(
    &self,
    surfaces: Box<dyn SurfaceGroup>
  ) -> Result<Box<dyn Integrator + Sync + Send>, Box<dyn std::error::Error>> {
    Ok(Box::new(SimplePathTracer {
      surfaces,
      path_termination_probability: 1.0 / (self.average_path_length as Float),
      background: Color::black()
    }))
  }
}

#[derive(Debug)]
pub struct SimplePathTracer {
  surfaces: Box<dyn SurfaceGroup>,
  path_termination_probability: Float,
  background: Color
}

impl ParameterizedIntegrator for SimplePathTracer {
  fn estimate_radiance(
    &self,
    sampler: &mut dyn Sampler,
    maybe_ray: PathTerminator,
    integrator: &dyn Integrator
  ) -> Color {
    if let Some((ray, survival_probability, continuation)) = maybe_ray.into_ray(sampler) {
      if let Some(hit) = self.surfaces.intersect_world_ray(ray) {
        let sample = hit.material.sample(&hit, sampler);
        let mut radiance_emitted = sample.emission.unwrap_or(Color::black());

        if let Some((mut attenuation, scattered_ray, reflection_type)) = sample.reflection {
          attenuation /= survival_probability;
          let maybe_scattered_ray = continuation.into_terminator(scattered_ray);
          let mut radiance_in = integrator.radiance(sampler, maybe_scattered_ray);
          if let ReflectionType::Diffuse(pdf) = reflection_type {
            radiance_in /= pdf;
          }

          radiance_emitted += attenuation * radiance_in
        }

        radiance_emitted
      } else {
        self.background
      }
    } else {
      Color::black()
    }
  }

  fn initial_path_terminator(&self, ray: WorldRay) -> PathTerminator {
    PathTerminator::new(ray, self.path_termination_probability)
  }
}

unsafe impl Sync for SimplePathTracer {}

unsafe impl Send for SimplePathTracer {}
