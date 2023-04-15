mod bvh;
mod surface_group;
mod surface_list;

pub use surface_group::*;

pub fn default_surface_group() -> Box<dyn SurfaceGroupParameters> {
  Box::new(bvh::BvhParameters {
    partition_strategy: bvh::PartitionStrategy::SurfaceAreaHeuristic,
    max_leaf_primitives: 2
  })
}
