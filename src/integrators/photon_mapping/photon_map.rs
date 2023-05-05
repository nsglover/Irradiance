use std::{
  sync::{Arc, Mutex},
  thread,
  time::Duration
};

use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use kd_tree::KdTree3;

use super::*;
use crate::{
  common::Wrapper, duration_to_hms, light::Color, math::*, raytracing::*, sampling::*,
  scene::Scene, BuildSettings
};

pub struct PhotonMap {
  kd_tree: KdTree3<PackedPhoton>
}

impl PhotonMap {
  fn trace_photon(
    curr_photon: Photon,
    scene: &Scene,
    sampler: &mut dyn Sampler,
    initial_power: Color,
    photons: &mut Vec<PackedPhoton>
  ) {
    let point = curr_photon.position;
    let direction = curr_photon.direction;
    if let Some((hit, material)) = scene.intersect_world_ray(WorldRay::new(point, direction)) {
      if let Some(scatter_rv) = material.scatter_random_variable() {
        let mut new_photon;

        match scatter_rv {
          RandomVariable::Continuous(rv) => {
            // Store photon on diffuse surface
            photons.push(
              Photon { position: hit.intersect_point, direction, power: curr_photon.power }
                .into_packed()
            );

            // Sample the diffuse BSDF for the new photon's direction
            if let Some((scattered_dir, pdf)) = rv.sample_with_pdf(&hit, sampler) {
              new_photon = Photon {
                position: hit.intersect_point,
                direction: scattered_dir,
                power: curr_photon.power * material.bsdf(&hit, &scattered_dir) / pdf.into_inner()
              }
            } else {
              return;
            }
          },
          RandomVariable::Discrete(rv) => {
            // Note we do not store photons on specular surfaces!

            // Sample the specular BSDF for the new photon's direction
            if let Some(scattered_dir) = rv.sample(&hit, sampler) {
              new_photon = Photon {
                position: hit.intersect_point,
                direction: scattered_dir,
                power: curr_photon.power * material.bsdf(&hit, &scattered_dir)
              }
            } else {
              return;
            }
          }
        }

        // If the photon survives Russian roulette, then continue tracing it
        let survival_prob = (new_photon.power.luminance() / initial_power.luminance()).min(1.0);
        if sampler.next().into_inner() < survival_prob {
          new_photon.power /= survival_prob;
          Self::trace_photon(new_photon, scene, sampler, initial_power, photons)
        }
      }
    }
  }

  const BATCH_SIZE: usize = 32000;

  fn async_trace_photons(
    scene: Arc<Scene>,
    photons_lock: Arc<Mutex<(Vec<PackedPhoton>, usize)>>,
    num_photons: usize
  ) {
    let mut sampler = IndependentSampler::new();

    loop {
      // Initialize batch variables
      let mut num_emitted_photons = 0;
      let mut photon_batch = Vec::with_capacity(2 * Self::BATCH_SIZE);

      while photon_batch.len() < Self::BATCH_SIZE {
        // Attempt to emit a photon
        let rv = scene.as_ref().emissive_part().emitted_ray_random_variable();
        if let Some(((ray, power), pdf)) = rv.sample_with_pdf(&(), &mut sampler) {
          let initial_power = power / pdf.into_inner();
          let emitted_photon = Photon::from_ray(&ray, initial_power);
          num_emitted_photons += 1;

          Self::trace_photon(
            emitted_photon,
            &scene,
            &mut sampler,
            initial_power,
            &mut photon_batch
          );
        }
      }

      // Once the batch is finished, add the photons to the collective list
      let mut photons_and_emitted = photons_lock.as_ref().lock().unwrap();
      photons_and_emitted.0.append(&mut photon_batch);
      photons_and_emitted.1 += num_emitted_photons;

      // Finish execution if we've got enough photons
      if photons_and_emitted.0.len() > num_photons {
        break;
      }
    }
  }

  fn trace_photons(
    scene: Arc<Scene>,
    num_photons: usize,
    settings: BuildSettings
  ) -> Vec<PackedPhoton> {
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

    // Create a mutex for the photon array and the number of emitted photons
    let photons_lock = Arc::new(Mutex::new((Vec::with_capacity(num_photons), 0)));

    // Start threads to trace photons, periodically pushing their results to the vector
    let mut threads = Vec::with_capacity(settings.num_threads);
    for _ in 0..settings.num_threads {
      let scene = scene.clone();
      let photons = photons_lock.clone();
      threads.push(
        thread::Builder::new()
          .stack_size(1024 * 1024 * 1024)
          .spawn(move || Self::async_trace_photons(scene, photons, num_photons))
          .unwrap()
      );
    }

    // Manually update the progress bar as the threads run.
    if let Some(progress_bar) = &maybe_progress_bar {
      loop {
        let photons_and_emitted = photons_lock.as_ref().lock().unwrap();
        let num_complete = photons_and_emitted.0.len();
        drop(photons_and_emitted);

        progress_bar.set_position(num_complete as u64);

        let elapsed = progress_bar.elapsed().as_secs_f64();
        let ratio = if num_complete == 0 {
          num_photons as f64
        } else {
          (num_photons as f64) / (num_complete as f64)
        };

        let projected = Duration::from_secs_f64(elapsed * ratio);
        progress_bar.set_message(duration_to_hms(&projected));

        thread::sleep(Duration::from_millis(250));
        if num_complete >= num_photons {
          break;
        }
      }
    }

    // Wait for the threads to finish and mark the overall progress bar as finished.
    for t in threads {
      t.join().unwrap();
    }

    if let Some(progress_bar) = maybe_progress_bar {
      progress_bar.set_message(duration_to_hms(&progress_bar.elapsed()));
      progress_bar.finish();
    }

    // Divide all powers by the number of emitted photons
    println!("Finalizing photon powers...");
    let (mut photons, num_emitted) =
      Arc::try_unwrap(photons_lock).unwrap().into_inner().unwrap().into();
    let inverse_num_emitted = 1.0 / num_emitted as f32;
    for p in photons.iter_mut() {
      for c in p.power.iter_mut() {
        *c *= inverse_num_emitted;
      }
    }

    photons
  }

  pub fn build(scene: Arc<Scene>, num_photons: usize, settings: BuildSettings) -> Self {
    println!("Tracing photons...");
    let photons = Self::trace_photons(scene, num_photons, settings);

    let photon_size = std::mem::size_of::<PackedPhoton>();
    let gigabytes = (photons.len() * photon_size) as f64 / 1_073_741_824.0;
    println!(
      "Building photon map with {} {photon_size}-byte photons ({:.1} gigabytes total)...",
      photons.len(),
      gigabytes
    );

    Self {
      kd_tree: KdTree3::par_build_by_key(photons, |photon, k| {
        ordered_float::OrderedFloat(photon.position[k])
      })
    }
  }

  // /// Returns the nearest photons to point in the photon map, as well as the maximum squared
  // /// distance between point and a nearest photon.
  // pub fn find_nearest_photons(
  //   &self,
  //   point: &WorldPoint,
  //   num_photons: usize
  // ) -> (Vec<(Photon, PositiveReal)>, PositiveReal) {
  //   let mut nearest_items =
  //     self.kd_tree.nearests(&Into::<[f32; 3]>::into(point.into_inner().cast()), num_photons);
  //   let mut photons = Vec::with_capacity(nearest_items.capacity() - 1);

  //   let mut max_squared_distance = 0.0;
  //   let mut max_index = 0;
  //   for (i, item_and_squared_distance) in nearest_items.iter_mut().enumerate() {
  //     if item_and_squared_distance.squared_distance > max_squared_distance {
  //       max_squared_distance = item_and_squared_distance.squared_distance;
  //       max_index = i;
  //     }
  //   }

  //   for (i, item_and_squared_distance) in nearest_items.into_iter().enumerate() {
  //     if i == max_index {
  //       continue;
  //     } else {
  //       photons.push((
  //         Photon::from_packed(item_and_squared_distance.item.clone()),
  //         PositiveReal::new_unchecked(item_and_squared_distance.squared_distance as Real)
  //       ));
  //     }
  //   }

  //   (photons, PositiveReal::new_unchecked(max_squared_distance as Real))
  // }

  /// Returns the nearest photons to point in the photon map, as well as the maximum squared
  /// distance between point and a nearest photon.
  pub fn find_within_radius(&self, point: &WorldPoint, radius: PositiveReal) -> Vec<Photon> {
    let nearest_photons = self.kd_tree.within_radius(
      &Into::<[f32; 3]>::into(point.into_inner().cast()),
      radius.into_inner() as f32
    );

    nearest_photons.into_iter().map(|p| Photon::from_packed(p.clone())).collect()
  }
}
