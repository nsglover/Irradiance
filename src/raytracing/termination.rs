use super::*;
use crate::{
  math::{PositiveReal, Real},
  sampling::Sampler
};

pub struct PathContinuation {
  remaining_scatters: usize,
  survival_probability: Real
}

impl PathContinuation {
  pub fn into_terminator(mut self, scattered_ray: WorldRay) -> PathTerminator {
    self.remaining_scatters -= 1;
    PathTerminator { ray: scattered_ray, cont: self }
  }
}

const MAX_SCATTER_COUNT: usize = 16 * 1024;

/// The standard Russian roulette ray termination procedure
pub struct PathTerminator {
  ray: WorldRay,
  cont: PathContinuation
}

impl PathTerminator {
  pub fn new(ray: WorldRay, termination_probability: PositiveReal) -> Self {
    Self {
      ray,
      cont: PathContinuation {
        remaining_scatters: MAX_SCATTER_COUNT,
        survival_probability: 1.0 - termination_probability
      }
    }
  }

  pub fn into_ray(self, sampler: &mut dyn Sampler) -> Option<(WorldRay, Real, PathContinuation)> {
    if self.cont.remaining_scatters == 0 {
      None
    } else if sampler.next().into_inner() < self.cont.survival_probability {
      Some((self.ray, self.cont.survival_probability, self.cont))
    } else {
      None
    }
  }
}
