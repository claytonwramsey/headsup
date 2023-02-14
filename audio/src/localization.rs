//! Algorithms for audio localization.

/// A length in meters.
type Meters = f32;

/// A duration in seconds.
type Seconds = f32;

/// The speed of sound in meters per second.
const SPEED_OF_SOUND: f32 = 343.0;

#[derive(Clone, Copy, Debug, PartialEq)]
/// A point in `DIM`-dimensional space.
struct Point<const DIM: usize>([Meters; DIM]);

#[derive(Clone, Copy, Debug, PartialEq)]
/// A point-localized event in time.
pub struct Event<const DIM: usize> {
    /// The location where the event occurred.
    location: Point<DIM>,
    /// The time when the event occurred.
    time: Seconds,
}

#[allow(clippy::cast_precision_loss, dead_code)]
/// Compute the source of a gunshot based on a set of microphone events.
///
/// # Inputs
///
/// - `mic_events`: A list of events corresponding to impulse receipts on the microphones.
/// - `n_iters`: The number of iterations to run the solver for.
/// - `err_tolerance`: The maximum allowable error for a converged solution to the source of the
///     shot.
/// - `step_scale`: A hyperparameter for the scaling of gradient descent steps.
///
/// # Returns
///
/// In the `Ok()` variant, this function will return the triple `(event, iters, err)`.
///
/// # Errors
///
/// This function will return an `Err` if it is unable to find a solution with err below
/// `err_tolerance` in under `n_iters` steps.
///
/// # Panics
///
/// This function will panic if `DIM` is 0.
pub fn source_of_shot<const DIM: usize>(
    mic_events: &[Event<DIM>],
    n_iters: usize,
    err_tolerance: f32,
    step_scale: f32,
) -> Result<(Event<DIM>, usize, f32), ()> {
    // initial guess: the gunshot happened at the point where the first impulse was received
    // and at the time when the impulse arrived
    let mut origin_estimate = *mic_events
        .iter()
        .min_by(|e1, e2| e1.time.total_cmp(&e2.time))
        .unwrap();
    for iter_id in 0..n_iters {
        let mut mse = 0.0;
        let mut forward_results = Vec::new();

        for event in mic_events {
            let distance = distance(&event.location, &origin_estimate.location);
            let prediction = origin_estimate.time + distance / SPEED_OF_SOUND;
            let residual = prediction - event.time;
            mse += residual.powi(2);
            forward_results.push((residual, distance));
        }

        mse /= mic_events.len() as f32;
        println!("iter {iter_id}, prediction {origin_estimate:?}, mse {mse} vs {err_tolerance}");

        if mse < err_tolerance {
            // converged!
            return Ok((origin_estimate, iter_id, mse));
        }

        // perform one step of gradient descent
        for ((residual, distance), event) in forward_results.into_iter().zip(mic_events.iter()) {
            // contribution from this microphone event in space
            for dim_id in 0..DIM {
                origin_estimate.location.0[dim_id] -= step_scale
                    * 2.0
                    * residual
                    * (origin_estimate.location.0[dim_id] - event.location.0[dim_id])
                    / (SPEED_OF_SOUND * (distance + 1e-4) * mic_events.len() as f32);
            }
            // contribution from this microphone event in time
            origin_estimate.time -= step_scale * 2.0 * residual / mic_events.len() as f32;
        }
    }

    Err(())
}

/// Compute the distance between two points.
fn distance<const DIM: usize>(p1: &Point<DIM>, p2: &Point<DIM>) -> Meters {
    p1.0.iter()
        .zip(p2.0.iter())
        .map(|(&a, &b)| (a - b).powi(2))
        .sum::<Meters>()
        .sqrt()
}

#[cfg(test)]
mod tests {
    use std::f32::consts::{PI, TAU};

    use super::*;

    #[allow(clippy::cast_precision_loss)]
    fn source_helper<const DIM: usize>(
        seed: u64,
        n_iters: usize,
        n_mics: usize,
        fit_tolerance: f32,
        step_scale: f32,
        angular_error_tolerance: f32,
    ) {
        /// The radius of the helmet.
        const HELMET_RADIUS: Meters = 10.0;

        assert!(DIM >= 2);

        fastrand::seed(seed);

        // randomly generate source of shot
        let mut true_source = Point([0.0; DIM]);
        let true_time = 0.0f32;
        for component in true_source.0.iter_mut() {
            *component = 100.0 * fastrand::f32() - 50.0;
        }
        println!("true source position: {true_source:?}");

        // distribute microphones evenly about circle
        let mut events = Vec::new();
        for mic_id in 0..n_mics {
            let mut mic_point = Point([0.0; DIM]);
            let mic_angle = TAU * mic_id as f32 / n_mics as f32;
            mic_point.0[0] = HELMET_RADIUS * mic_angle.cos();
            mic_point.0[1] = HELMET_RADIUS * mic_angle.sin();
            events.push(Event {
                location: mic_point,
                time: distance(&mic_point, &true_source) / SPEED_OF_SOUND + true_time,
            });
            println!("{:?}", events[events.len() - 1]);
        }

        let (event, iters_used, train_err) =
            source_of_shot(&events, n_iters, fit_tolerance, step_scale).unwrap();

        assert!(iters_used < n_iters);
        assert!(train_err < fit_tolerance);

        println!("predicted event {event:?}");

        let true_angle = f32::atan2(true_source.0[0], true_source.0[1]);
        let predicted_angle = f32::atan2(event.location.0[0], event.location.0[1]);

        let angular_error = (true_angle - predicted_angle + TAU + PI) % TAU - PI;
        println!("angular error {angular_error}");
        assert!(angular_error.abs() < angular_error_tolerance);
    }

    #[test]
    #[should_panic]
    #[allow(unused_must_use)]
    fn nodim_panics() {
        source_of_shot::<0>(&[], 0, 0.0, 0.0);
    }

    #[test]
    fn two_dimensional() {
        source_helper::<2>(1234, 30, 8, 1e-9, 2.0, 0.1);
    }
}
