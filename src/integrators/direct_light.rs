// use std::sync::Arc;

// use serde::Deserialize;

// use super::*;
// use crate::{
//   light::*, math::PositiveReal, raytracing::*, sampling::*, surface_groups::*, BuildSettings
// };

// // TODO: Add background to this
// #[derive(Debug, Deserialize)]
// struct Parameters {}

// #[typetag::deserialize(name = "direct-light")]
// impl IntegratorParameters for Parameters {
//   fn build_integrator(
//     &self,
//     surfaces: Arc<dyn SurfaceGroup>,
//     _: BuildSettings
//   ) -> Result<Box<dyn Integrator + Sync + Send>, Box<dyn std::error::Error>> {
//     Ok(Box::new(DirectLightPathTracer { surfaces, background: Color::black() }))
//   }
// }

// pub struct DirectLightPathTracer {
//   surfaces: Arc<dyn SurfaceGroup>,
//   background: Color
// }

// impl PathTraceIntegrator for DirectLightPathTracer {
//   fn initial_path_terminator(&self, ray: WorldRay) -> PathTerminator {
//     PathTerminator::new(ray, 0.0)
//   }

//   fn sample_scatter(
//     &self,
//     sampler: &mut dyn Sampler,
//     ray: WorldRay
//   ) -> Result<(Color, Color, WorldRay, Option<PositiveReal>), Color> {
//     if let Some(hit) = self.surfaces.intersect_world_ray(ray) {
//       let sample = hit.material.sample(&hit, sampler);
//       let radiance_emitted = sample.emission.unwrap_or(Color::black());
//       Err(radiance_emitted)
//     } else {
//       Err(self.background)
//     }
//   }
// }

// unsafe impl Sync for DirectLightPathTracer {}

// unsafe impl Send for DirectLightPathTracer {}
