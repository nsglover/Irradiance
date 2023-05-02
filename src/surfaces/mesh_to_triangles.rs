use std::sync::Arc;

use serde::Deserialize;
use tobj::LoadOptions;

use super::triangle::TriangleSurface;
use crate::{
  materials::Material,
  math::{LocalToWorld, Point3, Space, UnitVector3, VectorLike, WorldPoint, WorldUnitVector},
  textures::TextureCoordinate
};

#[derive(Debug, Clone, Deserialize)]
pub struct MeshParameters {
  pub filename: String,
  pub name: String
}

impl MeshParameters {
  pub fn build_mesh(self) -> (String, Mesh) {
    println!("Loading mesh from \"{}\"", self.filename);
    let mut raw_mesh = tobj::load_obj(
      self.filename,
      &LoadOptions {
        single_index: false,
        triangulate: false,
        ignore_points: true,
        ignore_lines: true,
        reorder_data: false
      }
    )
    .expect("Mesh file not found!")
    .0;

    if raw_mesh.len() != 1 {
      panic!("Meshes with more than one model are not currently supporsed!")
    }

    let raw_mesh = raw_mesh.remove(0).mesh;
    (
      self.name,
      Mesh {
        indices: raw_mesh.indices,
        positions: raw_mesh.positions,
        normal_indices: raw_mesh.normal_indices,
        normals: raw_mesh.normals,
        texcoord_indices: raw_mesh.texcoord_indices,
        texcoords: raw_mesh.texcoords
      }
    )
  }
}

pub struct Mesh {
  indices: Vec<u32>,
  positions: Vec<f64>,
  normal_indices: Vec<u32>,
  normals: Vec<f64>,
  texcoord_indices: Vec<u32>,
  texcoords: Vec<f64>
}

#[derive(Debug, Clone, Copy)]
pub struct MeshSpace;

impl Space<3> for MeshSpace {}

impl Mesh {
  fn get_point(&self, index: usize, transform: &LocalToWorld<MeshSpace>) -> WorldPoint {
    transform.point(&Point3::from_array([
      self.positions[index * 3],
      self.positions[index * 3 + 1],
      self.positions[index * 3 + 2]
    ]))
  }

  fn get_normal(&self, index: usize, transform: &LocalToWorld<MeshSpace>) -> WorldUnitVector {
    let idx = self.normal_indices[index] as usize;
    transform.normal(&UnitVector3::from_array([
      self.normals[idx * 3],
      self.normals[idx * 3 + 1],
      self.normals[idx * 3 + 2]
    ]))
  }

  fn get_texcoords(&self, index: usize) -> TextureCoordinate {
    let idx = self.texcoord_indices[index] as usize;
    TextureCoordinate::from_array([self.texcoords[idx * 2], self.texcoords[idx * 2 + 1]])
  }

  pub fn to_triangles(
    &self,
    transform: LocalToWorld<MeshSpace>,
    material: Arc<dyn Material>
  ) -> Vec<TriangleSurface> {
    if self.indices.len() % 3 != 0 {
      panic!("Faces are not triangulated!")
    }

    let data: Vec<_> = self
      .indices
      .iter()
      .map(|i| {
        let i = *i as usize;
        (self.get_point(i, &transform), self.get_normal(i, &transform), self.get_texcoords(i))
      })
      .collect();

    data
      .chunks_exact(3)
      .map(|triple| {
        if let [d0, d1, d2] = triple {
          TriangleSurface::new(*d0, *d1, *d2, material.clone())
        } else {
          panic!("chunks_exact didn't work!")
        }
      })
      .collect()
  }
}
