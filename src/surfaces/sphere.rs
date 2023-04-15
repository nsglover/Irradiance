use {
  super::*,
  crate::{
    materials::{Material, MaterialParameters},
    math::*
  },
  serde::Deserialize,
  std::collections::HashMap
};

#[derive(Debug, Deserialize)]
pub struct SphereSurfaceParameters {
  transform: TransformParameters,
  radius: Float,
  material: String
}

#[typetag::deserialize(name = "sphere")]
impl SurfaceParameters for SphereSurfaceParameters {
  fn build_surface(
    &self,
    materials: &HashMap<String, Box<dyn MaterialParameters>>
  ) -> Box<dyn Surface> {
    let scale_mat = nalgebra::Matrix4::new_scaling(self.radius);
    let scale: LocalToWorld<_> = Transform::from_raw(scale_mat).unwrap();

    Box::new(SphereSurface {
      transform: self.transform.clone().build_transform() * scale,
      material: materials.get(&self.material).unwrap().build_material()
    })
  }
}

#[derive(Debug, Clone, Copy)]
pub struct SphereSpace;

impl Space<3> for SphereSpace {}

#[derive(Debug)]
pub struct SphereSurface {
  transform: LocalToWorld<SphereSpace>,
  material: Box<dyn Material>
}

impl TransformedSurface for SphereSurface {
  type LocalSpace = SphereSpace;

  fn local_to_world(&self) -> &LocalToWorld<Self::LocalSpace> { &self.transform }

  fn bounding_box(&self) -> BBox3<Self::LocalSpace> {
    BBox3::new(nalgebra::point![-1.0, -1.0, -1.0].into(), nalgebra::point![1.0, 1.0, 1.0].into())
  }

  fn intersect_ray(
    &self,
    ray: Ray3<Self::LocalSpace>
  ) -> Option<RayIntersection<Self::LocalSpace>> {
    let origin_vec = Vector3::from(ray.origin());
    let b = 2.0 * Vector3::from(ray.dir()).dot(&origin_vec);
    let c = origin_vec.norm_squared() - 1.0;

    let discriminant = b * b - 4.0 * c;
    if discriminant < 0.0 {
      return None;
    }

    let q = -0.5 * (if b < 0.0 { b - discriminant.sqrt() } else { b + discriminant.sqrt() });
    let mut t1 = q;
    let mut t2 = c / q;
    if t1 > t2 {
      std::mem::swap(&mut t1, &mut t2);
    }

    let p;
    let t;
    if let Some(p1) = ray.at(t1) {
      t = t1;
      p = p1
    } else if let Some(p2) = ray.at(t2) {
      t = t2;
      p = p2
    } else {
      return None;
    }

    let normalized_p = Vector3::from(p).normalize();

    let phi = p.inner().y.atan2(p.inner().x);
    let theta = p.inner().z.asin();
    let u = (phi + PI) / (2.0 * PI);
    let v = (theta + PI / 2.0) / PI;

    Some(RayIntersection {
      ray,
      intersect_time: t,
      intersect_point: Vector3::from(normalized_p).into(),
      geom_normal: normalized_p,
      shading_normal: normalized_p,
      tex_coords: Vector::from_array([u, v]),
      material: self.material.as_ref()
    })
  }
}
