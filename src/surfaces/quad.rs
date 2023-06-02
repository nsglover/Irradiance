use std::{collections::HashMap, sync::Arc};

use serde::Deserialize;

use super::{
  surface_list::{NoBoxCheck, SurfaceList},
  triangle::TriangleSurface,
  *
};
use crate::{
  lights::NullLight,
  materials::{Material, NullMaterial},
  math::*,
  textures::TextureCoordinate,
  BuildSettings
};

#[derive(Debug, Deserialize)]
pub struct QuadSurfaceParameters {
  transform: TransformParameters,
  light: Option<String>,
  material: Option<String>
}

#[typetag::deserialize(name = "quad")]
impl SurfaceParameters for QuadSurfaceParameters {
  fn build_surface(
    &self,
    lights: &HashMap<String, Arc<dyn Light>>,
    materials: &HashMap<String, Arc<dyn Material>>,
    _: &HashMap<String, Mesh>,
    _: BuildSettings
  ) -> Box<dyn Surface> {
    let transform: LocalToWorld<WorldSpace> = self.transform.clone().build_transform();
    let normal = transform.normal(&UnitVector3::from_array([0.0, 0.0, 1.0]));
    let mat =
      self.material.as_ref().map(|m| materials.get(m).unwrap().clone()).unwrap_or(Arc::new(NullMaterial::default()));
    let light = self.light.as_ref().map(|l| lights.get(l).unwrap().clone()).unwrap_or(Arc::new(NullLight::default()));
    let normals = Some((normal, normal, normal));

    let p00 = transform.point(&Point3::from_array([-0.5, -0.5, 0.0]));
    let p10 = transform.point(&Point3::from_array([0.5, -0.5, 0.0]));
    let p11 = transform.point(&Point3::from_array([0.5, 0.5, 0.0]));
    let p01 = transform.point(&Point3::from_array([-0.5, 0.5, 0.0]));

    let t00 = TextureCoordinate::from_array([0.0, 0.0]);
    let t10 = TextureCoordinate::from_array([1.0, 0.0]);
    let t11 = TextureCoordinate::from_array([1.0, 1.0]);
    let t01 = TextureCoordinate::from_array([0.0, 1.0]);

    Box::new(SurfaceList::<NoBoxCheck>::build(vec![
      Box::new(TriangleSurface::new(light.clone(), mat.clone(), (p00, p10, p11), normals, Some((t00, t10, t11)))),
      Box::new(TriangleSurface::new(light, mat, (p00, p11, p01), normals, Some((t00, t11, t01)))),
    ]))
  }

  fn has_light(&self) -> bool { self.light.is_some() }
}
