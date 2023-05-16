use std::{collections::HashMap, sync::Arc};

use serde::Deserialize;

use super::{bvh::*, *};
use crate::{
  light::Color, materials::Material, math::*, raytracing::*, sampling::ContinuousRandomVariable,
  textures::TextureCoordinate, BuildSettings
};

#[derive(Debug, Deserialize)]
pub struct TriangleMeshSurfaceParameters {
  transform: TransformParameters,
  mesh: String,
  material: String
}

#[typetag::deserialize(name = "mesh")]
impl SurfaceParameters for TriangleMeshSurfaceParameters {
  fn build_surface(
    &self,
    materials: &HashMap<String, Arc<dyn Material>>,
    meshes: &HashMap<String, Mesh>,
    settings: BuildSettings
  ) -> Box<dyn Surface> {
    let triangles = meshes
      .get(&self.mesh)
      .unwrap()
      .to_triangles(self.transform.clone().build_transform(), materials.get(&self.material).unwrap().clone())
      .into_iter()
      .map(|t| Box::new(t) as Box<dyn Surface>)
      .collect();

    Box::new(BoundingVolumeHierarchy::build(triangles, PartitionStrategy::SurfaceAreaHeuristic, 3, settings))
  }

  fn is_emissive(&self, materials: &HashMap<String, Arc<dyn Material>>) -> bool {
    materials.get(&self.material).unwrap().emit_random_variable().is_some()
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
  outer_normal: WorldUnitVector,
  bounding_box: WorldBoundingBox,
  material: Arc<dyn Material>
}

impl TriangleSurface {
  pub fn new(
    (p0, p1, p2): (WorldPoint, WorldPoint, WorldPoint),
    maybe_normals: Option<(WorldUnitVector, WorldUnitVector, WorldUnitVector)>,
    maybe_tex_coords: Option<(TextureCoordinate, TextureCoordinate, TextureCoordinate)>,
    material: Arc<dyn Material>
  ) -> Self {
    let mut bounding_box = WorldBoundingBox::default();
    bounding_box.enclose_point(&p0);
    bounding_box.enclose_point(&p1);
    bounding_box.enclose_point(&p2);

    let edge1 = p1 - p0;
    let edge2 = p2 - p0;
    let outer_normal = edge1.cross(&edge2).normalize();

    let (n0, n1, n2);
    if let Some(normals) = maybe_normals {
      (n0, n1, n2) = normals;
    } else {
      n0 = outer_normal;
      n1 = outer_normal;
      n2 = outer_normal;
    }

    let (t0, t1, t2);
    if let Some(tex_coords) = maybe_tex_coords {
      (t0, t1, t2) = tex_coords;
    } else {
      t0 = TextureCoordinate::from_array([0.0, 0.0]);
      t1 = TextureCoordinate::from_array([1.0, 0.0]);
      t2 = TextureCoordinate::from_array([0.0, 1.0]);
    }

    let v0 = (p0, n0, t0);
    let v1 = (p1, n1, t1);
    let v2 = (p2, n2, t2);

    Self { v0, v1, v2, edge1, edge2, outer_normal, bounding_box, material }
  }
}

impl Surface for TriangleSurface {
  fn intersect_world_ray(&self, ray: &mut WorldRay) -> Option<WorldSurfaceInterface> {
    let (p0, n0, t0) = self.v0;
    let (p1, n1, t1) = self.v1;
    let (p2, n2, t2) = self.v2;
    let dir = ray.dir().into_vector();

    let pvec = dir.cross(&self.edge2);
    let det = self.edge1.dot(&pvec);
    if det.abs() < 0.00001 {
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
    let barycentric_coords = [1.0 - (u + v), u, v];

    if let Some((t, _)) = ray.at_real(t) {
      let p = p0 * barycentric_coords[0] + (p1 * barycentric_coords[1]).into() + (p2 * barycentric_coords[2]).into();
      let sn = (n0 * barycentric_coords[0] + n1 * barycentric_coords[1] + n2 * barycentric_coords[2]).normalize();
      let uv = t0 * barycentric_coords[0] + t1 * barycentric_coords[1] + t2 * barycentric_coords[2];

      Some(SurfaceInterface {
        surface_point: SurfacePoint {
          point: p,
          geometric_normal: self.outer_normal,
          shading_normal: sn,
          tex_coord: uv
        },
        material: self.material.as_ref(),
        time: t
      })
    } else {
      None
    }
  }

  fn emitted_ray_random_variable(&self) -> &dyn ContinuousRandomVariable<Param = (), Sample = (WorldRay, Color)> {
    todo!()
  }

  fn world_bounding_box(&self) -> WorldBoundingBox { self.bounding_box.clone() }

  fn num_subsurfaces(&self) -> usize { 0 }
}
