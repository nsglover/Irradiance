use {
  super::{surface_list::*, *},
  crate::{math::*, surfaces::Surface},
  rand::{distributions::Uniform, Rng},
  serde_derive::Deserialize
};

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum PartitionStrategy {
  #[serde(alias = "sah")]
  SurfaceAreaHeuristic,

  #[serde(alias = "random")]
  Random
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct BvhParameters {
  #[serde(alias = "strategy")]
  pub partition_strategy: PartitionStrategy,

  #[serde(alias = "max-leaf-prims")]
  pub max_leaf_primitives: usize
}

#[typetag::deserialize(name = "bvh")]
impl SurfaceGroupParameters for BvhParameters {
  fn build_surface_group(
    &self,
    surfaces: Vec<Box<dyn Surface>>
  ) -> Result<Box<dyn SurfaceGroup>, Box<dyn std::error::Error>> {
    Ok(Box::new(BoundingVolumeHierarchy::build(surfaces, self)))
  }
}

#[derive(Debug)]
enum BvhNodeType {
  Leaf(SurfaceList),
  Node(Option<Box<BvhNode>>, Option<Box<BvhNode>>)
}

#[derive(Debug)]
struct BvhNode {
  bounding_box: WorldBBox,
  node_type: BvhNodeType
}

impl BvhNode {
  fn intersect(&self, mut ray: WorldRay) -> Option<WorldRayIntersection> {
    if self.bounding_box.ray_intersects(&ray) {
      match &self.node_type {
        BvhNodeType::Leaf(surface_list) => surface_list.intersect_world_ray(ray),
        BvhNodeType::Node(maybe_left, maybe_right) => {
          match (
            maybe_left.as_ref().and_then(|left| left.intersect(ray.clone())),
            maybe_right.as_ref().and_then(|right| right.intersect(ray.clone()))
          ) {
            (None, maybe_hit) | (maybe_hit, None) => {
              if let Some(hit) = maybe_hit.as_ref() {
                ray.set_max_intersect_time(hit.intersect_time);
              }

              maybe_hit
            },
            (Some(left_hit), Some(right_hit)) => {
              let left_time = left_hit.intersect_time;
              let right_time = right_hit.intersect_time;
              if left_time < right_time {
                ray.set_max_intersect_time(left_time);
                Some(left_hit)
              } else {
                ray.set_max_intersect_time(right_time);
                Some(right_hit)
              }
            }
          }
        }
      }
    } else {
      None
    }
  }
}

#[derive(Debug)]
pub struct BoundingVolumeHierarchy {
  num_surfaces: usize,
  root_node: BvhNode
}

impl BoundingVolumeHierarchy {
  fn build_node(mut surfaces: Vec<Box<dyn Surface>>, params: &BvhParameters) -> Option<BvhNode> {
    let num_surfaces = surfaces.len();

    let bounding_boxes: Vec<_> = surfaces.iter().map(|s| s.world_bounding_box()).collect();
    let bounding_box = bounding_boxes.iter().fold(WorldBBox::default(), |mut acc, bbox| {
      acc.enclose_box(bbox);
      acc
    });

    if num_surfaces <= params.max_leaf_primitives {
      if num_surfaces == 0 {
        None
      } else {
        Some(BvhNode { bounding_box, node_type: BvhNodeType::Leaf(SurfaceList::build(surfaces)) })
      }
    } else {
      let left: Option<BvhNode>;
      let right: Option<BvhNode>;
      match params.partition_strategy {
        PartitionStrategy::SurfaceAreaHeuristic => {
          // const NUM_BUCKETS: usize = 12;
          // let num_buckets = Self::NUM_BUCKETS.min(num_surfaces);
          todo!()
        },
        PartitionStrategy::Random => {
          let axis = rand::thread_rng().sample(Uniform::new(0, 3));
          let num_left = rand::thread_rng().sample(Uniform::new_inclusive(0, num_surfaces));

          surfaces.sort_by(|s1, s2| {
            s1.world_bounding_box().center().inner()[axis]
              .total_cmp(&(s2.world_bounding_box().center().inner()[axis]))
          });

          let mut surfaces_iter = surfaces.into_iter();

          let left_surfaces: Vec<_> = surfaces_iter.by_ref().take(num_left).collect();
          let right_surfaces: Vec<_> = surfaces_iter.collect();

          left = Self::build_node(left_surfaces, params);
          right = Self::build_node(right_surfaces, params);
        }
      };

      Some(BvhNode {
        bounding_box,
        node_type: BvhNodeType::Node(left.map(Box::new), right.map(Box::new))
      })
    }
  }

  fn build(surfaces: Vec<Box<dyn Surface>>, params: &BvhParameters) -> Self {
    Self { num_surfaces: surfaces.len(), root_node: Self::build_node(surfaces, params).unwrap() }
  }
}

impl SurfaceGroup for BoundingVolumeHierarchy {
  fn num_surfaces(&self) -> usize { self.num_surfaces }

  fn intersect_world_ray(&self, ray: WorldRay) -> Option<WorldRayIntersection> {
    self.root_node.intersect(ray)
  }
}
