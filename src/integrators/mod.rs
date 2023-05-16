mod integrator;
mod material_path_tracer;
mod mixture_path_tracer;
mod normal_integrator;

pub use integrator::*;

pub fn default_integrator() -> Box<dyn IntegratorParameters> {
  Box::new(normal_integrator::NormalIntegratorParameters {})
}
