use std::{collections::HashMap, sync::Arc};

use serde::Deserialize;

use super::*;
use crate::{
  materials::Material, math::*, raytracing::*, sampling::Sampler, textures::TextureCoordinate
};

#[derive(Debug, Deserialize)]
pub struct TriangleSurfaceParameters {
  transform: TransformParameters,
  mesh: String,
  material: String
}

#[typetag::deserialize(name = "mesh")]
impl SurfaceParameters for TriangleSurfaceParameters {
  fn build_surfaces(
    &self,
    materials: &HashMap<String, Arc<dyn Material>>,
    meshes: &HashMap<String, Mesh>
  ) -> Vec<Box<dyn Surface>> {
    meshes
      .get(&self.mesh)
      .unwrap()
      .to_triangles(
        self.transform.clone().build_transform(),
        materials.get(&self.material).unwrap().clone()
      )
      .into_iter()
      .map(|t| Box::new(t) as Box<dyn Surface>)
      .collect()
  }
}

type VertexInfo = (WorldPoint, WorldUnitVector, TextureCoordinate);

#[derive(Debug)]
pub struct TriangleSurface {
  v0: VertexInfo,
  v1: VertexInfo,
  v2: VertexInfo,
  edge1: WorldVector,
  edge2: WorldVector,
  true_normal: WorldUnitVector,
  bounding_box: WorldBoundingBox,
  material: Arc<dyn Material>
}

impl TriangleSurface {
  pub fn new(
    v0 @ (p0, _, _): VertexInfo,
    v1 @ (p1, _, _): VertexInfo,
    v2 @ (p2, _, _): VertexInfo,
    material: Arc<dyn Material>
  ) -> Self {
    let mut bounding_box = WorldBoundingBox::default();
    bounding_box.enclose_point(&p0);
    bounding_box.enclose_point(&p1);
    bounding_box.enclose_point(&p2);

    let edge1 = p1 - p0;
    let edge2 = p2 - p0;

    Self {
      v0,
      v1,
      v2,
      edge1,
      edge2,
      true_normal: edge1.cross(&edge2).normalize(),
      bounding_box,
      material
    }
  }
}

impl Surface for TriangleSurface {
  fn world_bounding_box(&self) -> WorldBoundingBox { self.bounding_box.clone() }

  fn intersect_world_ray(&self, ray: WorldRay) -> Option<WorldRayIntersection> {
    let (p0, _, t0) = self.v0;
    let (p1, _, t1) = self.v1;
    let (p2, _, t2) = self.v2;
    let dir = ray.dir().into_vector();

    let pvec = dir.cross(&self.edge2);
    let det = self.edge1.dot(&pvec);
    if det.abs() < 1e-8 {
      return None;
    }

    let inv_det = 1.0 / det;
    let tvec = ray.origin() - p0;
    let u = tvec.dot(&pvec) * inv_det;
    if u < 0.0 || u > 1.0 {
      return None;
    }

    let qvec = tvec.cross(&self.edge1);
    let v = dir.dot(&qvec) * inv_det;
    if v < 0.0 || u + v > 1.0 {
      return None;
    }

    let t = self.edge2.dot(&qvec) * inv_det;
    let bary = [1.0 - (u + v), u, v];

    if let Some((t, _)) = ray.at_real(t) {
      let mut geometric_normal = self.true_normal;
      // TODO: Shading normals look strange; seems to be a mesh loading issue?
      // let mut sn = (n0 * bary[0] + n1 * bary[1] + n2 * bary[2]).normalize();
      if ray.dir().dot(&geometric_normal) > 0.0 {
        // sn = -sn;
        geometric_normal = -geometric_normal;
      }

      Some(WorldRayIntersection {
        ray,
        material: self.material.clone(),
        intersect_time: t,
        intersect_point: p0 * bary[0] + (p1 * bary[1]).into() + (p2 * bary[2]).into(),
        geometric_normal,
        shading_normal: geometric_normal,
        tex_coords: t0 * bary[0] + t1 * bary[1] + t2 * bary[2]
      })
    } else {
      None
    }
  }

  fn interesting_direction_sample(
    &self,
    _point: &WorldPoint,
    _sampler: &mut dyn Sampler
  ) -> (WorldUnitVector, Real) {
    todo!()
  }

  fn intersecting_direction_pdf(&self, _point: &WorldPoint, _direction: &WorldUnitVector) -> Real {
    todo!()
  }

  fn material(&self) -> &dyn Material { self.material.as_ref() }
}
