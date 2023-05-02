use std::time::Duration;

use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use kd_tree::KdTree3;

use super::*;
use crate::{duration_to_hms, scene::Scene, BuildSettings};

pub struct PhotonMap {
  kd_tree: KdTree3<Photon>
}

impl PhotonMap {
  fn trace_photons(
    scene: &Scene,
    num_photons: usize,
    maybe_progress_bar: Option<ProgressBar>
  ) -> Vec<Photon> {
    let photons = Vec::new();

    photons
  }

  pub fn build(scene: &Scene, num_photons: usize, settings: BuildSettings) -> Self {
    println!("Tracing {} photons...", num_photons);

    // If enabled, start up the progress bar
    let maybe_progress_bar = settings.use_progress_bar.then(|| {
      let bar_style = "[ {elapsed_precise} / {msg} ]: {bar:50.cyan/magenta} ".to_string()
        + &format!("{{pos:>{}}}/{{len}} photons", (num_photons as f64).log10().ceil() as usize);

      let progress_bar = ProgressBar::with_draw_target(
        Some(num_photons as u64),
        ProgressDrawTarget::stdout_with_hz(24)
      );

      let style = ProgressStyle::with_template(&bar_style).unwrap().progress_chars("##-");
      progress_bar.set_style(style);
      progress_bar.set_message(duration_to_hms(&Duration::from_nanos(0)));
      progress_bar
    });

    let photons = Self::trace_photons(scene, num_photons, maybe_progress_bar.clone());
    if let Some(progress_bar) = maybe_progress_bar {
      progress_bar.finish();
    }

    let gigabytes = (num_photons * std::mem::size_of::<Photon>()) as f64 / 1_073_741_824.0;
    println!("Building photon map with {} photons ({:.1} gigabytes)...", num_photons, gigabytes);
    Self {
      kd_tree: KdTree3::build_by_key(photons, |photon, k| {
        ordered_float::OrderedFloat(photon.position()[k])
      })
    }
  }
}
