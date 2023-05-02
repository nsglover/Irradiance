mod debug_path_tracer;
mod direct_light;
mod integrator;
mod material_path_tracer;
mod normal_integrator;
mod photon_mapping;
// mod mixture_path_tracer;

pub use integrator::*;

pub fn default_integrator() -> Box<dyn IntegratorParameters> {
  Box::new(normal_integrator::NormalIntegratorParameters {})
}
