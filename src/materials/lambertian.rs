use std::sync::Arc;

use serde::Deserialize;

use super::*;
use crate::{math::*, raytracing::*, sampling::*, spectrum::Spectrum, textures::*};

#[derive(Debug, Deserialize)]
struct LambertianParameters {
  name: String,
  albedo: Box<dyn TextureParameters>
}

#[typetag::deserialize(name = "lambertian")]
impl MaterialParameters for LambertianParameters {
  fn name(&self) -> String { self.name.clone() }

  fn build_material(&self) -> Arc<dyn Material> {
    Arc::new(Lambertian {
      albedo: self.albedo.build_texture(),
      scatter_random_var: ScatterRandomVariable::Diffuse(Box::new(CosineWeightedHemisphere))
    })
  }
}

#[derive(Debug)]
struct CosineWeightedHemisphere;

impl ContinuousRandomVariable for CosineWeightedHemisphere {
  type Param = (WorldSurfacePoint, WorldUnitVector);
  type Sample = WorldUnitVector;

  fn sample_with_pdf(
    &self,
    p @ (hit, out_dir): &Self::Param,
    sampler: &mut dyn Sampler
  ) -> Option<(Self::Sample, PositiveReal)> {
    let random: WorldVector = uniform_random_on_unit_sphere(sampler).into();
    let mut normal = hit.shading_normal;
    if normal.dot(out_dir) < 0.0 {
      normal = -normal;
    }

    let dir = (normal.into_vector() + random).normalize();
    self.pdf(p, &dir).map(|pdf| (dir, pdf))
  }

  fn pdf(&self, (hit, _): &Self::Param, sample: &Self::Sample) -> Option<PositiveReal> {
    PositiveReal::new((sample.dot(&hit.shading_normal) * INV_PI).abs())
  }
}

#[derive(Debug)]
pub struct Lambertian {
  albedo: Arc<dyn Texture>,
  scatter_random_var: ScatterRandomVariable
}

impl Material for Lambertian {
  fn bsdf(&self, hit: &WorldSurfacePoint, _: &WorldUnitVector, _: &WorldUnitVector) -> Spectrum {
    self.albedo.value(&hit.tex_coord) * INV_PI
  }

  fn bsdf_cos(&self, hit: &WorldSurfacePoint, in_dir: &WorldUnitVector, _: &WorldUnitVector) -> Spectrum {
    self.albedo.value(&hit.tex_coord) * in_dir.abs_dot(&hit.shading_normal) * INV_PI
  }

  fn random_bsdf_in_direction(&self) -> &ScatterRandomVariable { &self.scatter_random_var }
}
