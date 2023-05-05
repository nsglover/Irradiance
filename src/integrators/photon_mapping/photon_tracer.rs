use std::sync::Arc;

use serde::Deserialize;

use super::*;
use crate::{
  integrators::{Integrator, IntegratorParameters, PathTraceIntegrator},
  light::*,
  materials::ScatterRandomVariable,
  math::*,
  raytracing::*,
  sampling::Sampler,
  scene::Scene,
  BuildSettings
};

#[derive(Debug, Deserialize)]
pub struct PhotonTracerParameters {
  #[serde(alias = "total-photons")]
  num_map_photons: usize,

  #[serde(alias = "average-path-length")]
  average_path_length: usize,

  #[serde(alias = "initial-radius")]
  initial_radius: Real,

  #[serde(alias = "shrinking-factor")]
  alpha: Real,

  #[serde(alias = "iterations")]
  iterations: usize
}

#[typetag::deserialize(name = "progressive-photon-tracer")]
impl IntegratorParameters for PhotonTracerParameters {
  fn build_integrators(
    &self,
    scene: Scene,
    settings: BuildSettings
  ) -> Result<Vec<Box<dyn Integrator>>, Box<dyn std::error::Error>> {
    let scene = Arc::new(scene);
    let photon_map = Arc::new(PhotonMap::build(scene.clone(), self.num_map_photons, settings));
    let mut integrators = Vec::with_capacity(self.iterations);

    let mut radius = self.initial_radius;
    for i in 0..self.iterations {
      integrators.push(Box::new(PhotonTracer {
        photon_map: photon_map.clone(),
        radius: PositiveReal::new_unchecked(radius),
        kernel_area: PositiveReal::new_unchecked(PI * radius * radius),
        scene: scene.clone(),
        path_termination_probability: PositiveReal::new_unchecked(
          1.0 / (self.average_path_length as Real)
        ),
        background: Color::from_bytes([200, 230, 255]) / 4.0
      }) as Box<dyn Integrator>);

      let i = i as Real;
      radius = ((i + self.alpha) / (i + 1.0)).sqrt() * radius;
    }

    Ok(integrators)
  }
}

struct PhotonTracer {
  scene: Arc<Scene>,
  photon_map: Arc<PhotonMap>,
  radius: PositiveReal,
  kernel_area: PositiveReal,
  path_termination_probability: PositiveReal,
  background: Color
}

// fn epanechnikov_kernel(v: WorldVector, radius: PositiveReal) -> Real {
//   let t = v.norm() / radius;
//   let sqrt_five = 2.23606797749978969_f64 as Real;
//   if t < sqrt_five {
//     0.75 * (1.0 - 0.2 * t * t) / sqrt_five
//   } else {
//     0.0
//   }
// }

// fn cone_kernel(v: WorldVector, radius: PositiveReal) -> Real {
//   (1.0 - v.norm() / radius).max(0.0) / radius
// }

// impl PhotonTracer {
//   fn get_background(&self, ray: &WorldRay) -> Color {
//     let dir = ray.dir().into_vector();
//     let phi = dir[1].atan2(dir[0]);
//     let theta = dir[2].asin();
//     let u = (phi + PI) / (2.0 * PI);
//     let v = (theta + PI / 2.0) / PI;

//     let w = self.background.width();
//     let x = (u * (w as Real)).clamp(0.0, (w - 1) as Real) as u32;

//     let h = self.background.height();
//     let y = (v * (h as Real)).clamp(0.0, (h - 1) as Real) as u32;

//     Color::from_bytes(self.background.get_pixel(x, y).0)
//   }
// }

impl PathTraceIntegrator for PhotonTracer {
  fn initial_path_terminator(&self, ray: WorldRay) -> PathTerminator {
    PathTerminator::new(ray, self.path_termination_probability)
  }

  fn sample_scatter(
    &self,
    sampler: &mut dyn Sampler,
    ray: WorldRay
  ) -> Result<(Color, Color, WorldRay, Option<PositiveReal>), Color> {
    if let Some((mut hit, material)) = self.scene.intersect_world_ray(ray) {
      let radiance_emitted = material.emitted(&hit).unwrap_or(Color::black());
      if let Some(scatter_rv) = material.scatter_random_variable() {
        match scatter_rv {
          ScatterRandomVariable::Continuous(rv) => {
            if let Some((dir, pdf)) = rv.sample_with_pdf(&hit, sampler) {
              let ray = Ray::new(hit.intersect_point, dir);
              if let None = self.scene.intersect_world_ray(ray) {
                let bsdf = material.bsdf(&hit, &dir);
                return Err(radiance_emitted + self.background * bsdf / pdf.into_inner());
              }
            }

            let photons = self.photon_map.find_within_radius(&hit.intersect_point, self.radius);
            let out_dir = -hit.intersect_direction;
            let cos = out_dir.dot(&hit.geometric_normal).abs();
            if cos <= 0.0 {
              return Err(radiance_emitted);
            }

            let mut incoming_estimate = Color::black();
            for photon in photons {
              // TODO: This is incredibly stupid, RayIntersection and bsdf need to be redone
              hit.intersect_direction = photon.direction;
              let bsdf = material.bsdf(&hit, &out_dir) / cos;
              incoming_estimate += bsdf * photon.power;
            }

            let l = incoming_estimate.luminance();
            if l > 1000.0 {
              println!("{:?}", incoming_estimate);
            }

            return Err(radiance_emitted + incoming_estimate / self.kernel_area.into_inner());
          },
          ScatterRandomVariable::Discrete(rv) => {
            // If the BSDf is discrete, proceed as usual in standard path tracing
            if let Some(scattered_dir) = rv.sample(&hit, sampler) {
              return Ok((
                radiance_emitted,
                material.bsdf(&hit, &scattered_dir),
                Ray::new(hit.intersect_point, scattered_dir),
                None
              ));
            }
          }
        }
      }

      Err(radiance_emitted)
    } else {
      Err(self.background)
    }
  }
}

unsafe impl Sync for PhotonTracer {}

unsafe impl Send for PhotonTracer {}
