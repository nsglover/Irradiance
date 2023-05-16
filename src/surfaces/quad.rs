use std::{collections::HashMap, sync::Arc};

use serde::Deserialize;

use super::{
  surface_list::{NoBoxCheck, SurfaceList},
  triangle::TriangleSurface,
  *
};
use crate::{materials::Material, math::*, textures::TextureCoordinate, BuildSettings};

#[derive(Debug, Deserialize)]
pub struct QuadSurfaceParameters {
  transform: TransformParameters,
  material: String
}

#[typetag::deserialize(name = "quad")]
impl SurfaceParameters for QuadSurfaceParameters {
  fn build_surface(
    &self,
    materials: &HashMap<String, Arc<dyn Material>>,
    _: &HashMap<String, Mesh>,
    _: BuildSettings
  ) -> Box<dyn Surface> {
    let transform: LocalToWorld<WorldSpace> = self.transform.clone().build_transform();
    let normal = transform.normal(&UnitVector3::from_array([0.0, 0.0, 1.0]));
    let material = materials.get(&self.material).unwrap().clone();
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
      Box::new(TriangleSurface::new((p00, p10, p11), normals, Some((t00, t10, t11)), material.clone())),
      Box::new(TriangleSurface::new((p00, p11, p01), normals, Some((t00, t11, t01)), material)),
    ]))
  }

  fn is_emissive(&self, materials: &HashMap<String, Arc<dyn Material>>) -> bool {
    materials.get(&self.material).unwrap().emit_random_variable().is_some()
  }
}
