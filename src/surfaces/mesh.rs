use std::{collections::HashMap, sync::Arc};

use serde::Deserialize;
use tobj::LoadOptions;

use super::{
  bvh::{BoundingVolumeHierarchy, PartitionStrategy},
  triangle::TriangleSurface,
  Surface
};
use crate::{
  lights::{Light, NullLight},
  materials::{Material, NullMaterial},
  math::*,
  surfaces::SurfaceParameters,
  textures::TextureCoordinate,
  BuildSettings
};

#[derive(Debug, Clone, Deserialize)]
pub struct MeshParameters {
  pub filename: String,
  pub name: String
}

impl MeshParameters {
  pub fn build_mesh(self) -> (String, Mesh) {
    println!("Loading mesh from \"{}\"...", self.filename);
    let mut raw_meshes = tobj::load_obj(
      self.filename,
      &LoadOptions { single_index: false, triangulate: true, ignore_points: true, ignore_lines: true }
    )
    .expect("Mesh file not found!")
    .0;

    // TODO: Replace this panic and many others with proper error handling
    if raw_meshes.len() != 1 {
      panic!("Meshes with more than one model are not currently supported!")
    }

    let raw_mesh = raw_meshes.remove(0).mesh;
    if raw_mesh.indices.len() % 3 != 0 {
      panic!("Faces are not triangulated!")
    }

    let vertices = raw_mesh
      .positions
      .chunks_exact(3)
      .map(|chunk| {
        if let [x, y, z] = *chunk {
          Point::from_array([x as Real, y as Real, z as Real])
        } else {
          panic!("chunks_exact didn't work!");
        }
      })
      .collect();

    let vertex_normals = if raw_mesh.normal_indices.len() == 0 {
      None
    } else {
      Some((
        raw_mesh.normal_indices.into_iter().map(|i| i as usize).collect(),
        raw_mesh
          .normals
          .chunks_exact(3)
          .map(|chunk| {
            if let [x, y, z] = *chunk {
              UnitVector::from_array([x as Real, y as Real, z as Real])
            } else {
              panic!("chunks_exact didn't work!");
            }
          })
          .collect()
      ))
    };

    let vertex_tex_coords = if raw_mesh.texcoord_indices.len() == 0 {
      None
    } else {
      Some((
        raw_mesh.texcoord_indices.into_iter().map(|i| i as usize).collect(),
        raw_mesh
          .texcoords
          .chunks_exact(2)
          .map(|chunk| {
            if let [x, y] = *chunk {
              TextureCoordinate::from_array([x as Real, y as Real])
            } else {
              panic!("chunks_exact didn't work!");
            }
          })
          .collect()
      ))
    };

    (
      self.name,
      Mesh {
        indices: raw_mesh.indices.into_iter().map(|i| i as usize).collect(),
        vertices,
        vertex_normals,
        vertex_tex_coords
      }
    )
  }
}

#[derive(Debug, Clone, Copy)]
pub struct MeshSpace;

impl Space<3> for MeshSpace {}

pub struct Mesh {
  indices: Vec<usize>,
  vertices: Vec<Point3<MeshSpace>>,
  vertex_normals: Option<(Vec<usize>, Vec<UnitVector3<MeshSpace>>)>,
  vertex_tex_coords: Option<(Vec<usize>, Vec<TextureCoordinate>)>
}

impl Mesh {
  pub fn to_triangles(
    &self,
    transform: LocalToWorld<MeshSpace>,
    light: Arc<dyn Light>,
    material: Arc<dyn Material>
  ) -> Vec<TriangleSurface> {
    (0..self.indices.len())
      .collect::<Vec<_>>()
      .chunks_exact(3)
      .map(|vertex_indices| {
        if let [i0, i1, i2] = *vertex_indices {
          let vi0 = self.indices[i0];
          let vi1 = self.indices[i1];
          let vi2 = self.indices[i2];

          let vertices = (
            transform.point(&self.vertices[vi0]),
            transform.point(&self.vertices[vi1]),
            transform.point(&self.vertices[vi2])
          );

          let normals = self.vertex_normals.as_ref().map(|(normal_indices, normals)| {
            let ni0 = normal_indices[i0];
            let ni1 = normal_indices[i1];
            let ni2 = normal_indices[i2];
            (transform.normal(&normals[ni0]), transform.normal(&normals[ni1]), transform.normal(&normals[ni2]))
          });

          let tex_coords = self.vertex_tex_coords.as_ref().map(|(tex_coord_indices, tex_coords)| {
            let ti0 = tex_coord_indices[i0];
            let ti1 = tex_coord_indices[i1];
            let ti2 = tex_coord_indices[i2];
            (tex_coords[ti0], tex_coords[ti1], tex_coords[ti2])
          });

          TriangleSurface::new(light.clone(), material.clone(), vertices, normals, tex_coords)
        } else {
          panic!("chunks_exact didn't work!")
        }
      })
      .collect()
  }
}

#[derive(Debug, Deserialize)]
pub struct TriangleMeshSurfaceParameters {
  transform: TransformParameters,
  mesh: String,
  light: Option<String>,
  material: Option<String>
}

#[typetag::deserialize(name = "mesh")]
impl SurfaceParameters for TriangleMeshSurfaceParameters {
  fn build_surface(
    &self,

    lights: &HashMap<String, Arc<dyn Light>>,
    materials: &HashMap<String, Arc<dyn Material>>,
    meshes: &HashMap<String, Mesh>,
    settings: BuildSettings
  ) -> Box<dyn Surface> {
    let triangles = meshes
      .get(&self.mesh)
      .unwrap()
      .to_triangles(
        self.transform.clone().build_transform(),
        self.light.as_ref().map(|l| lights.get(l).unwrap().clone()).unwrap_or(Arc::new(NullLight::default())).clone(),
        self
          .material
          .as_ref()
          .map(|m| materials.get(m).unwrap().clone())
          .unwrap_or(Arc::new(NullMaterial::default()))
          .clone()
      )
      .into_iter()
      .map(|t| Box::new(t) as Box<dyn Surface>)
      .collect();

    Box::new(BoundingVolumeHierarchy::build(triangles, PartitionStrategy::SurfaceAreaHeuristic, 3, settings))
  }

  fn has_light(&self) -> bool { self.light.is_some() }
}
