use serde::Deserialize;

use super::*;
use crate::{common::Wrapper, math::*, raytracing::*, samplers::*, textures::*};

#[derive(Debug, Deserialize)]
struct DieletricParameters {
  name: String,
  albedo: Box<dyn TextureParameters>,
  ior: Real
}

#[typetag::deserialize(name = "dielectric")]
impl MaterialParameters for DieletricParameters {
  fn name(&self) -> String { self.name.clone() }

  fn build_material(&self) -> Box<dyn Material> {
    Box::new(Dieletric { albedo: self.albedo.build_texture(), index_of_refraction: self.ior })
  }
}

#[derive(Debug)]
pub struct Dieletric {
  albedo: Box<dyn Texture>,
  index_of_refraction: Real
}

// TODO: Document this and move it to math module
fn refract<S: Space<3>>(
  d: UnitVector3<S>,
  normal: UnitVector3<S>,
  ior_in_out_ratio: Real
) -> Option<(UnitVector3<S>, Real)> {
  let dt = d.dot(&normal);
  let discriminant = 1.0 - ior_in_out_ratio * ior_in_out_ratio * (1.0 - dt * dt);
  (discriminant > 0.0).then(|| {
    let cos_theta_out = discriminant.sqrt();
    let refracted_dir = (Vector::from(d.inner().into_inner() - normal.inner().into_inner() * dt)
      * ior_in_out_ratio
      - normal * cos_theta_out)
      .normalize();
    (refracted_dir, cos_theta_out)
  })
}

impl Material for Dieletric {
  fn sample(&self, hit: &WorldRayIntersection, sampler: &mut dyn Sampler) -> MaterialSample {
    let dir = hit.ray.dir();

    // Ensure normal and IOR are correctly oriented (i.e. for whether ray is entering or exiting)
    let mut normal = hit.shading_normal;
    let mut eta_in = 1.0;
    let mut eta_out = self.index_of_refraction;
    if dir.dot(&hit.shading_normal) > 0.0 {
      normal = -normal;
      std::mem::swap(&mut eta_in, &mut eta_out);
    }

    let cos_theta_in = -dir.dot(&normal);
    let reflected = -dir.reflect_about(hit.shading_normal);
    let scattered_dir;
    if let Some((refracted, cos_theta_out)) = refract(dir, normal, eta_in / eta_out) {
      // Compute Fresnel coefficient (probability of reflection)
      let eta_out_cos_in = eta_out * cos_theta_in;
      let eta_in_cos_out = eta_in * cos_theta_out;
      let rho_parallel = (eta_out_cos_in - eta_in_cos_out) / (eta_out_cos_in + eta_in_cos_out);
      let eta_in_cos_in = eta_in * cos_theta_in;
      let eta_out_cos_out = eta_out * cos_theta_out;
      let rho_perp = (eta_in_cos_in - eta_out_cos_out) / (eta_in_cos_in + eta_out_cos_out);
      let reflect_probability = (rho_parallel * rho_parallel + rho_perp * rho_perp) / 2.0;

      // Refract or reflect based on the above probability
      let should_reflect = sampler.next().into_inner() < reflect_probability;
      scattered_dir = if should_reflect { reflected } else { refracted }
    } else {
      scattered_dir = reflected;
    }

    MaterialSample::specular(self.albedo.value(hit), Ray::new(hit.intersect_point, scattered_dir))
  }

  fn is_emissive(&self) -> bool { false }

  fn pdf(&self, _: &WorldRayIntersection, _: &WorldRay) -> Option<Real> { None }
}
