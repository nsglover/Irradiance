// use {
//   super::*,
//   crate::{light::*, materials::*, raytracing::*, samplers::*, surface_groups::*},
//   serde::Deserialize
// };

// #[derive(Debug, Deserialize)]
// struct PathTracerParameters {
//   max_bounces: usize
// }

// #[typetag::deserialize(name = "debug-path-tracer")]
// impl IntegratorParameters for PathTracerParameters {
//   fn build_integrator(
//     &self,
//     surfaces: Rc<dyn SurfaceGroup>
//   ) -> Result<Box<dyn Integrator + Sync + Send>, Box<dyn std::error::Error>> {
//     Ok(Box::new(DebugPathTracer { max_bounces: self.max_bounces, surfaces }))
//   }
// }

// #[derive(Debug)]
// pub struct DebugPathTracer {
//   max_bounces: usize,
//   surfaces: Rc<dyn SurfaceGroup>
// }

// impl DebugPathTracer {
//   fn recursive_estimate(
//     &self,
//     sampler: &mut dyn Sampler,
//     ray: WorldRay,
//     remaining_bounces: usize,
//     pref_surface: Option<&dyn crate::surfaces::Surface>
//   ) -> Color {
//     if let Some(hit) = self.surfaces.intersect_world_ray(ray) {
//       if let Some(s) = pref_surface {
//         if hit.surface as *const _ == s as *const _ {
//           if hit.intersect_time < 10.0 {
//             println!(
//               "Self intersection at time {} on material {:.20}!!",
//               hit.intersect_time,
//               format!("{:?}", hit.material)
//             );
//           }
//         }
//       }

//       let sample = hit.material.sample(&hit, sampler);
//       let emitted = sample.emission.unwrap_or(Color::black());
//       if remaining_bounces == 0 {
//         return emitted;
//       }

//       if let Some((attenuation, scattered, reflection_type)) = sample.reflection {
//         let mut rec =
//           self.recursive_estimate(sampler, scattered, remaining_bounces - 1, Some(hit.surface));
//         if let ReflectionType::Diffuse(pdf) = reflection_type {
//           rec /= pdf;
//         }

//         emitted + attenuation * rec
//       } else {
//         emitted
//       }
//     } else {
//       Color::black()
//     }
//   }
// }

// impl Integrator for DebugPathTracer {
//   fn estimate_radiance(&self, sampler: &mut dyn Sampler, ray: WorldRay) -> Color {
//     self.recursive_estimate(sampler, ray, self.max_bounces, None)
//   }
// }

// unsafe impl Sync for DebugPathTracer {}

// unsafe impl Send for DebugPathTracer {}
