use {
  super::*,
  crate::{
    materials::{Material, MaterialParameters},
    math::*,
    textures::TextureCoordinates
  },
  nalgebra as na,
  serde::Deserialize,
  std::collections::HashMap
};

#[derive(Debug, Deserialize)]
pub struct QuadSurfaceParameters {
  transform: TransformParameters,
  size: [Float; 2],
  material: String
}

#[typetag::deserialize(name = "quad")]
impl SurfaceParameters for QuadSurfaceParameters {
  fn build_surface(
    &self,
    materials: &HashMap<String, Box<dyn MaterialParameters>>
  ) -> Box<dyn Surface> {
    let scale: LocalToWorld<QuadSpace> =
      Transform::from_raw(na::Matrix4::new_nonuniform_scaling(&na::vector![
        self.size[0],
        self.size[1],
        1.0
      ]))
      .unwrap();

    Box::new(QuadSurface {
      transform: self.transform.clone().build_transform() * scale,
      material: materials.get(&self.material).unwrap().build_material()
    })
  }
}

#[derive(Debug, Clone, Copy)]
pub struct QuadSpace;

impl Space<3> for QuadSpace {}

#[derive(Debug)]
pub struct QuadSurface {
  transform: LocalToWorld<QuadSpace>,
  material: Box<dyn Material>
}

impl TransformedSurface for QuadSurface {
  type LocalSpace = QuadSpace;

  fn local_to_world(&self) -> &LocalToWorld<Self::LocalSpace> { &self.transform }

  fn bounding_box(&self) -> BBox3<Self::LocalSpace> {
    BBox3::new(na::point![-0.5, -0.5, 0.0].into(), na::point![0.5, 0.5, 0.0].into())
  }

  fn intersect_ray(
    &self,
    ray: &Ray3<Self::LocalSpace>
  ) -> Option<RayIntersection<Self::LocalSpace>> {
    if ray.dir().inner().z == 0.0 {
      return None;
    }

    let t = -ray.origin().inner().z / ray.dir().inner().z;
    ray.at(t).and_then(|point| {
      let mut p = point.inner();
      p.z = 0.0;
      if 0.5 < p.x || -0.5 > p.x || 0.5 < p.y || -0.5 > p.y {
        return None;
      }

      let normal = Vector3::from(na::vector![0.0, 0.0, 1.0]).normalize();
      let tex_coords = TextureCoordinates::from(na::vector![p.x + 0.5, p.y + 0.5]);

      Some(RayIntersection {
        intersecting_ray: ray.clone(),
        intersect_time: t,
        intersect_point: p.into(),
        geom_normal: normal,
        shading_normal: normal,
        tex_coords,
        material: self.material.as_ref()
      })
    })
  }
}
