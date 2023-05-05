use std::sync::Arc;

use serde::Deserialize;
use tobj::LoadOptions;

use super::triangle::TriangleSurface;
use crate::{
  materials::Material,
  math::{LocalToWorld, Point3, Space, VectorLike, WorldPoint, WorldUnitVector},
  textures::TextureCoordinate
};

#[derive(Debug, Clone, Deserialize)]
pub struct MeshParameters {
  pub filename: String,
  pub name: String
}

impl MeshParameters {
  pub fn build_mesh(self) -> (String, Mesh) {
    println!("Loading mesh from \"{}\"...", self.filename);
    let mut raw_mesh = tobj::load_obj(
      self.filename,
      &LoadOptions {
        single_index: false,
        triangulate: true,
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

    // println!("{}", raw_mesh.indices.len());
    // println!("{}", raw_mesh.texcoord_indices.len());
    // println!("{}", raw_mesh.normal_indices.len());

    // let idx = raw_mesh.indices[2] as usize;
    // println!(
    //   "{}: {} {} {}",
    //   idx,
    //   raw_mesh.positions[3 * idx],
    //   raw_mesh.positions[3 * idx + 1],
    //   raw_mesh.positions[3 * idx + 2]
    // );

    // let idx = raw_mesh.texcoord_indices[2] as usize;
    // println!("{}: {} {} ", idx, raw_mesh.texcoords[2 * idx], raw_mesh.texcoords[2 * idx + 1]);

    // println!("{}", raw_mesh.positions.len());
    // println!("{}", raw_mesh.texcoords.len());
    // println!("{}", raw_mesh.normals.len());

    (
      self.name,
      Mesh {
        indices: raw_mesh.indices,
        positions: raw_mesh.positions,
        texcoord_indices: raw_mesh.texcoord_indices,
        texcoords: raw_mesh.texcoords
      }
    )
  }
}

pub struct Mesh {
  indices: Vec<u32>,
  positions: Vec<f64>,
  texcoord_indices: Vec<u32>,
  texcoords: Vec<f64>
}

#[derive(Debug, Clone, Copy)]
pub struct MeshSpace;

impl Space<3> for MeshSpace {}

impl Mesh {
  fn get_point(&self, index: usize, transform: &LocalToWorld<MeshSpace>) -> WorldPoint {
    let idx = self.indices[index] as usize;
    transform.point(&Point3::from_array([
      self.positions[idx * 3],
      self.positions[idx * 3 + 1],
      self.positions[idx * 3 + 2]
    ]))
  }

  fn get_normal(&self, _index: usize, _transform: &LocalToWorld<MeshSpace>) -> WorldUnitVector {
    // let idx = self.normal_indices[index] as usize;
    // transform.normal(&UnitVector3::from_array([
    //   self.normals[idx * 3],
    //   self.normals[idx * 3 + 1],
    //   self.normals[idx * 3 + 2]
    // ]))

    // TODO: Temporary garbage
    WorldUnitVector::from_array([1.0, 0.0, 0.0])
  }

  fn get_texcoords(&self, index: usize) -> TextureCoordinate {
    if self.texcoord_indices.len() == 0 {
      TextureCoordinate::zero()
    } else {
      let idx = self.texcoord_indices[index] as usize;
      TextureCoordinate::from_array([self.texcoords[idx * 2], self.texcoords[idx * 2 + 1]])
    }
  }

  pub fn to_triangles(
    &self,
    transform: LocalToWorld<MeshSpace>,
    material: Arc<dyn Material>
  ) -> Vec<TriangleSurface> {
    if self.indices.len() % 3 != 0 {
      panic!("Faces are not triangulated!")
    }

    let data: Vec<_> = (0..self.indices.len())
      .map(|i| {
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
