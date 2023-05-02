use std::{sync::Arc, time::Duration};

use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use rand::{distributions::Uniform, Rng};
use serde_derive::Deserialize;

use super::{surface_list::*, *};
use crate::{
  duration_to_hms, materials::Material, math::*, raytracing::*, surfaces::Surface, BuildSettings
};

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum PartitionStrategy {
  #[serde(alias = "sah")]
  SurfaceAreaHeuristic,

  #[serde(alias = "random")]
  Random
}

#[derive(Debug, Deserialize)]
pub struct BvhParameters {
  #[serde(alias = "strategy")]
  pub partition_strategy: PartitionStrategy,

  #[serde(alias = "max-leaf-prims")]
  pub max_leaf_primitives: usize,

  #[serde(alias = "sub-surfaces")]
  pub surfaces: Vec<Box<dyn SurfaceParameters>>
}

#[typetag::deserialize(name = "bvh")]
impl SurfaceParameters for BvhParameters {
  fn build_surface(
    &self,
    materials: &std::collections::HashMap<String, Arc<dyn Material>>,
    meshes: &std::collections::HashMap<String, Mesh>,
    settings: BuildSettings
  ) -> Box<dyn Surface> {
    Box::new(BoundingVolumeHierarchy::build(
      self.surfaces.iter().map(|s| s.build_surface(materials, meshes, settings)).collect(),
      self.partition_strategy,
      self.max_leaf_primitives,
      settings
    ))
  }

  fn is_emissive(&self, materials: &HashMap<String, Arc<dyn Material>>) -> bool {
    self.surfaces.iter().any(|s| s.is_emissive(materials))
  }
}

#[derive(Debug)]
enum BvhNodeType {
  Leaf(SurfaceList),
  Node(Option<Box<BvhNode>>, Option<Box<BvhNode>>)
}

#[derive(Debug)]
struct BvhNode {
  bounding_box: WorldBoundingBox,
  node_type: BvhNodeType
}

impl BvhNode {
  fn intersect(&self, ray: &mut WorldRay) -> Option<(WorldRayIntersection, &dyn Material)> {
    if self.bounding_box.ray_intersects_fast(&ray) {
      match &self.node_type {
        BvhNodeType::Leaf(surface_list) => surface_list.intersect_world_ray(ray),
        BvhNodeType::Node(maybe_left, maybe_right) => {
          match (
            maybe_left.as_ref().and_then(|left| left.intersect(ray)),
            maybe_right.as_ref().and_then(|right| right.intersect(ray))
          ) {
            (None, maybe_hit) | (maybe_hit, None) => {
              if let Some((hit, _)) = maybe_hit.as_ref() {
                ray.set_max_intersect_time(hit.intersect_time);
              }

              maybe_hit
            },
            (Some(left_hit), Some(right_hit)) => {
              let hit = if left_hit.0.intersect_time < right_hit.0.intersect_time {
                left_hit
              } else {
                right_hit
              };

              ray.set_max_intersect_time(hit.0.intersect_time);
              Some(hit)
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
  root_node: BvhNode
}

impl BoundingVolumeHierarchy {
  fn build_node(
    mut surfaces: Vec<Box<dyn Surface>>,
    partition_strategy: PartitionStrategy,
    max_leaf_primitives: usize,
    maybe_progress_bar: Option<ProgressBar>
  ) -> Option<BvhNode> {
    let num_surfaces = surfaces.len();
    let bounding_boxes: Vec<_> = surfaces.iter().map(|s| s.world_bounding_box()).collect();
    let bounding_box = bounding_boxes.iter().fold(WorldBoundingBox::default(), |mut acc, bbox| {
      acc.enclose_box(bbox);
      acc
    });

    let mut left_surfaces: Vec<Box<dyn Surface>>;
    let mut right_surfaces: Vec<Box<dyn Surface>>;
    if num_surfaces <= max_leaf_primitives {
      if num_surfaces == 0 {
        None
      } else {
        if let Some(progress_bar) = maybe_progress_bar {
          progress_bar.inc(num_surfaces as u64);

          let elapsed = progress_bar.elapsed().as_secs_f64();
          let ratio = (progress_bar.length().unwrap() as f64) / (progress_bar.position() as f64);

          let projected = Duration::from_secs_f64(elapsed * ratio);
          progress_bar.set_message(duration_to_hms(&projected));
        }

        Some(BvhNode { bounding_box, node_type: BvhNodeType::Leaf(SurfaceList::build(surfaces)) })
      }
    } else {
      let left: Option<BvhNode>;
      let right: Option<BvhNode>;
      match partition_strategy {
        PartitionStrategy::SurfaceAreaHeuristic => {
          const NUM_BUCKETS: usize = 12;
          let num_buckets = NUM_BUCKETS.min(num_surfaces);
          let mut buckets = vec![(WorldBoundingBox::default(), 0usize); num_buckets];

          let mut split_axis = None;
          let mut split_boundary = 0.0;
          let mut split_cost = bounding_box.surface_area() * (num_surfaces as Real);

          for axis in 0..3 {
            let extent = bounding_box.diagonal();
            if extent[axis] < 0.00001 {
              continue;
            }

            let bucket_width = extent[axis] / (num_buckets as Real);
            for bucket in buckets.iter_mut() {
              bucket.0 = WorldBoundingBox::default();
              bucket.1 = 0;
            }

            for surface in &surfaces {
              let surface_bbox = surface.world_bounding_box();
              let dist = (surface_bbox.center()[axis] - bounding_box.min()[axis]) / bucket_width;
              let bucket_index = (dist as usize).clamp(0, num_buckets - 1);
              buckets[bucket_index].0.enclose_box(&surface_bbox);
              buckets[bucket_index].1 += 1;
            }

            for idx in 0..num_buckets {
              let mut num_left = 0;
              let mut left_box = WorldBoundingBox::default();
              for i in 0..idx {
                left_box.enclose_box(&buckets[i].0);
                num_left += buckets[i].1;
              }

              let mut num_right = 0;
              let mut right_box = WorldBoundingBox::default();
              for i in idx..num_buckets {
                right_box.enclose_box(&buckets[i].0);
                num_right += buckets[i].1;
              }

              let cost = (num_left as Real) * left_box.surface_area()
                + (num_right as Real) * right_box.surface_area();
              let boundary = bounding_box.min()[axis] + (idx as Real) * bucket_width;

              if cost < split_cost {
                split_axis = Some(axis);
                split_boundary = boundary;
                split_cost = cost;
              }
            }
          }

          let axis = split_axis.unwrap_or(0);
          (left_surfaces, right_surfaces) = surfaces
            .into_iter()
            .partition(|s| s.world_bounding_box().center()[axis] < split_boundary);

          if left_surfaces.len() == 0 {
            let mut right_iter = right_surfaces.into_iter();
            left_surfaces = right_iter.by_ref().take(num_surfaces / 2).collect();
            right_surfaces = right_iter.collect();
          } else if right_surfaces.len() == 0 {
            let mut left_iter = left_surfaces.into_iter();
            right_surfaces = left_iter.by_ref().take(num_surfaces / 2).collect();
            left_surfaces = left_iter.collect();
          }
        },
        PartitionStrategy::Random => {
          let axis = rand::thread_rng().sample(Uniform::new(0, 3));
          let num_left = num_surfaces / 2;

          surfaces.sort_by(|s1, s2| {
            s1.world_bounding_box().center()[axis]
              .total_cmp(&(s2.world_bounding_box().center()[axis]))
          });

          let mut surfaces_iter = surfaces.into_iter();

          left_surfaces = surfaces_iter.by_ref().take(num_left).collect();
          right_surfaces = surfaces_iter.collect();
        }
      };

      left = Self::build_node(
        left_surfaces,
        partition_strategy,
        max_leaf_primitives,
        maybe_progress_bar.clone()
      );

      right = Self::build_node(
        right_surfaces,
        partition_strategy,
        max_leaf_primitives,
        maybe_progress_bar
      );

      Some(BvhNode {
        bounding_box,
        node_type: BvhNodeType::Node(left.map(Box::new), right.map(Box::new))
      })
    }
  }

  pub fn build(
    surfaces: Vec<Box<dyn Surface>>,
    partition_strategy: PartitionStrategy,
    max_leaf_primitives: usize,
    settings: BuildSettings
  ) -> Self {
    println!("Building BVH on {} surfaces...", surfaces.len());

    // If enabled, start up the progress bar
    let maybe_progress_bar = settings.use_progress_bar.then(|| {
      let bar_style = "[ {elapsed_precise} / {msg} ]: {bar:50.blue/yellow} ".to_string()
        + &format!("{{pos:>{}}}/{{len}} surfaces", (surfaces.len() as f64).log10().ceil() as usize);

      let progress_bar = ProgressBar::with_draw_target(
        Some(surfaces.len() as u64),
        ProgressDrawTarget::stdout_with_hz(24)
      );

      let style = ProgressStyle::with_template(&bar_style).unwrap().progress_chars("##-");
      progress_bar.set_style(style);
      progress_bar.set_message(duration_to_hms(&Duration::from_nanos(0)));
      progress_bar
    });

    let s = Self {
      root_node: Self::build_node(
        surfaces,
        partition_strategy,
        max_leaf_primitives,
        maybe_progress_bar.clone()
      )
      .unwrap()
    };

    if let Some(progress_bar) = maybe_progress_bar {
      progress_bar.finish();
    }

    s
  }
}

impl Surface for BoundingVolumeHierarchy {
  fn intersect_world_ray(
    &self,
    ray: &mut WorldRay
  ) -> Option<(WorldRayIntersection, &dyn Material)> {
    self.root_node.intersect(ray)
  }

  fn world_bounding_box(&self) -> WorldBoundingBox { self.root_node.bounding_box.clone() }
}
