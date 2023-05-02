// use std::sync::Arc;

// use serde::Deserialize;

// use super::*;
// use crate::{
//   integrators::IntegratorParameters, light::Color, materials::ReflectionType, math::Float,
//   raytracing::*, samplers::Sampler, surface_groups::SurfaceGroup
// };

// // TODO: Add background to this
// #[derive(Debug, Deserialize)]
// struct Parameters {
//   #[serde(alias = "average-path-length")]
//   average_path_length: usize
// }

// #[typetag::deserialize(name = "mixture-path-tracer")]
// impl IntegratorParameters for Parameters {
//   fn build_integrator(
//     &self,
//     surfaces: Arc<dyn SurfaceGroup>
//   ) -> Result<Box<dyn Integrator>, Box<dyn std::error::Error>> {
//     Ok(Box::new(MixturePathTracer {
//       surfaces,
//       path_termination_probability: 1.0 / (self.average_path_length as Float),
//       background: Color::black()
//     }))
//   }
// }

// pub struct MixturePathTracer {
//   surfaces: Arc<dyn SurfaceGroup>,
//   path_termination_probability: Float,
//   background: Color
// }

// impl PathTraceIntegrator for MixturePathTracer {
//   fn initial_path_terminator(&self, ray: WorldRay) -> PathTerminator {
//     PathTerminator::new(ray, self.path_termination_probability)
//   }

//   fn sample_scatter(
//     &self,
//     sampler: &mut dyn Sampler,
//     ray: WorldRay
//   ) -> Result<(Color, Color, WorldRay, ReflectionType), Color> {
//     if let Some(hit) = self.surfaces.intersect_world_ray(ray) {
//       let sample = hit.surface.material().sample(&hit, sampler);
//       let radiance_emitted = sample.emission.unwrap_or(Color::black());

//       if let Some((attenuation, scattered_ray, reflection_type)) = sample.reflection {
//         if let ReflectionType::Diffuse(..) = reflection_type {
//           let ray_dir;
//           let pdf;
//           if sampler.next() < 0.5 {
//             let (light_dir, light_pdf) =
//               self.surfaces.sample_and_pdf(&hit.intersect_point, sampler);
//             ray_dir = light_dir;
//           }
//         } else {
//           return Ok((radiance_emitted, attenuation, scattered_ray, reflection_type));
//         }
//       } else {
//         Err(radiance_emitted)
//       }
//     } else {
//       Err(self.background)
//     }
//   }
// }

// unsafe impl Sync for MixturePathTracer {}

// unsafe impl Send for MixturePathTracer {}
