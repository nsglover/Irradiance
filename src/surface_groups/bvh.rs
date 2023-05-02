use std::{fmt::Display, sync::Arc, time::Duration};

use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use rand::{distributions::Uniform, Rng};
use serde_derive::Deserialize;

use super::{surface_list::*, *};
use crate::{duration_to_hms, math::*, raytracing::*, surfaces::Surface, BuildSettings};

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
    surfaces: Vec<Box<dyn Surface>>,
    settings: BuildSettings
  ) -> Result<Arc<dyn SurfaceGroup>, Box<dyn std::error::Error>> {
    Ok(Arc::new(BoundingVolumeHierarchy::build(surfaces, self, settings)))
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
  fn intersect(&self, mut ray: WorldRay) -> Option<WorldRayIntersection> {
    if self.bounding_box.ray_intersects_fast(&ray) {
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
              let hit = if left_hit.intersect_time < right_hit.intersect_time {
                left_hit
              } else {
                right_hit
              };

              ray.set_max_intersect_time(hit.intersect_time);
              Some(hit)
            }
          }
        }
      }
    } else {
      None
    }
  }

  fn fmt_recursive(&self, f: &mut std::fmt::Formatter, prefix: String) -> std::fmt::Result {
    writeln!(f, "{}Box: {}, {}", prefix, self.bounding_box.min(), self.bounding_box.max())?;
    match &self.node_type {
      BvhNodeType::Leaf(s) => writeln!(f, "{}Leaf({})", prefix, s.num_surfaces()),
      BvhNodeType::Node(maybe_left, maybe_right) => {
        writeln!(f, "{}Node:", prefix)?;
        writeln!(f, "{}LEFT:", prefix)?;
        if let Some(left) = maybe_left {
          left.fmt_recursive(f, prefix.clone() + " - ")?;
        }

        writeln!(f, "\n{}RIGHT:", prefix)?;
        if let Some(right) = maybe_right {
          right.fmt_recursive(f, prefix + " - ")?;
        }

        Ok(())
      }
    }
  }
}

impl Display for BvhNode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.fmt_recursive(f, String::default())
  }
}

#[derive(Debug)]
pub struct BoundingVolumeHierarchy {
  num_surfaces: usize,
  root_node: BvhNode
}

impl BoundingVolumeHierarchy {
  fn build_node(
    mut surfaces: Vec<Box<dyn Surface>>,
    params: &BvhParameters,
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
    if num_surfaces <= params.max_leaf_primitives {
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
      match params.partition_strategy {
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

      left = Self::build_node(left_surfaces, params, maybe_progress_bar.clone());
      right = Self::build_node(right_surfaces, params, maybe_progress_bar);

      Some(BvhNode {
        bounding_box,
        node_type: BvhNodeType::Node(left.map(Box::new), right.map(Box::new))
      })
    }
  }

  fn build(
    surfaces: Vec<Box<dyn Surface>>,
    params: &BvhParameters,
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
      num_surfaces: surfaces.len(),
      root_node: Self::build_node(surfaces, params, maybe_progress_bar.clone()).unwrap()
    };

    if let Some(progress_bar) = maybe_progress_bar {
      progress_bar.finish();
    }

    s
  }
}

impl SurfaceGroup for BoundingVolumeHierarchy {
  fn num_surfaces(&self) -> usize { self.num_surfaces }

  fn intersect_world_ray(&self, ray: WorldRay) -> Option<WorldRayIntersection> {
    self.root_node.intersect(ray)
  }

  fn pdf(&self, _point: &WorldPoint, _direction: &WorldUnitVector) -> Real { todo!() }

  fn sample_and_pdf(
    &self,
    _point: &WorldPoint,
    _sampler: &mut dyn crate::sampling::Sampler
  ) -> (WorldUnitVector, Real) {
    todo!()
  }
}
