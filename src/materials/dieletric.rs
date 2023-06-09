use std::{ops::Neg, sync::Arc};

use serde::Deserialize;

use super::*;
use crate::{math::*, raytracing::*, sampling::*, spectrum::Spectrum, textures::*};

#[derive(Debug, Deserialize)]
struct DieletricParameters {
  name: String,
  albedo: Box<dyn TextureParameters>,
  ior: Real
}

#[typetag::deserialize(name = "dielectric")]
impl MaterialParameters for DieletricParameters {
  fn name(&self) -> String { self.name.clone() }

  fn build_material(&self) -> Arc<dyn Material> {
    Arc::new(Dieletric {
      albedo: self.albedo.build_texture(),
      scatter_random_var: ScatterRandomVariable::Specular(Box::new(RefractRandomVariable {
        index_of_refraction: self.ior
      }))
    })
  }
}

#[derive(Debug)]
struct RefractRandomVariable {
  index_of_refraction: Real
}

impl DiscreteRandomVariable for RefractRandomVariable {
  type Param = (WorldSurfacePoint, WorldUnitVector);
  type Sample = WorldUnitVector;

  fn sample(
    &self,
    (hit, out_dir): &(WorldSurfacePoint, WorldUnitVector),
    sampler: &mut dyn Sampler
  ) -> Option<WorldUnitVector> {
    // Ensure normal and IOR are correctly oriented (i.e. for whether ray is entering or exiting)
    let mut normal = hit.shading_normal;
    let mut eta_in = 1.0;
    let mut eta_out = self.index_of_refraction;
    if out_dir.dot(&normal) < 0.0 {
      normal = -normal;
      std::mem::swap(&mut eta_in, &mut eta_out);
    }

    let cos_theta_in = out_dir.dot(&normal);
    let reflected = out_dir.reflect_about(normal);
    let scattered_dir;
    if let Some((refracted, cos_theta_out)) = refract(out_dir, normal, eta_in / eta_out) {
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

    Some(scattered_dir)
  }
}

#[derive(Debug)]
pub struct Dieletric {
  albedo: Arc<dyn Texture>,
  scatter_random_var: ScatterRandomVariable
}

fn refract<S: Space<3>>(
  d: &UnitVector3<S>,
  normal: UnitVector3<S>,
  ior_in_out_ratio: Real
) -> Option<(UnitVector3<S>, Real)> {
  let d = d.neg();
  let dt = d.dot(&normal);
  let discriminant = 1.0 - ior_in_out_ratio * ior_in_out_ratio * (1.0 - dt * dt);
  (discriminant > 0.0).then(|| {
    let cos_theta_out = discriminant.sqrt();
    let refracted_dir = (Vector::from(d.inner().into_inner() - normal.inner().into_inner() * dt) * ior_in_out_ratio
      - normal * cos_theta_out)
      .normalize();
    (refracted_dir, cos_theta_out)
  })
}

impl Material for Dieletric {
  fn bsdf_cos(&self, hit: &WorldSurfacePoint, _: &WorldUnitVector, _: &WorldUnitVector) -> Spectrum {
    self.albedo.value(&hit.tex_coord)
  }

  fn random_bsdf_in_direction(&self) -> &ScatterRandomVariable { &self.scatter_random_var }
}
