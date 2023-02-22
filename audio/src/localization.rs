//! Algorithms for audio localization.

use nalgebra::{Const, OMatrix, OVector};

/// The speed of sound, in meters per second.
const SPEED_OF_SOUND: f64 = 343.260;

#[must_use]
/// Compute the source direction of an impulse based on the arrival times at a microphone.
///
/// # Generic parameters
///
/// - `DIM`: The dimension of the space (e.g. 2 for a 2-dimensional space).
///   `DIM` must be greater than zero.
/// - `N_MICS`: The number of microphones.
///   `N_MICS` must be greater than zero.
///
/// # Inputs
///
/// - `positions`: The positions of each microphone, measured in meters.
///   Each column of `positions` is the position vector of a microphone.
/// - `times`: The times at which each microphone received an impulse, measured in seconds.
///   The indices of `times` correspond to the columns of `positions`.
///
/// # Panics
///
/// This function will only panic in case of an internal error.
///
/// # Returns
///
/// This function will return `Some()` containing a vector pointing toward the source of the impulse
/// if it is possible to find a solution.
///
/// If no such solution exists, this function will return `None`.
pub fn compute_direction<const DIM: usize, const N_MICS: usize>(
    positions: &OMatrix<f64, Const<DIM>, Const<N_MICS>>,
    times: &OVector<f64, Const<N_MICS>>,
) -> Option<OVector<f64, Const<DIM>>> {
    assert!(N_MICS > 0);
    assert!(DIM > 0);

    let min_time_idx = (0..times.len())
        .min_by(|&i, &j| times[i].partial_cmp(&times[j]).unwrap())
        .unwrap();
    let min_time = times[min_time_idx];

    let adjusted_distances = SPEED_OF_SOUND * times.add_scalar(-min_time);

    let mut zeroed_positions = positions.clone_owned();

    for mut col in zeroed_positions.column_iter_mut() {
        col -= positions.column(min_time_idx);
    }

    let soln = -(zeroed_positions * zeroed_positions.transpose()).try_inverse()?
        * zeroed_positions
        * adjusted_distances;

    Some(soln)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::{SMatrix, SVector};

    #[test]
    fn handwritten_2d() {
        let mic_points = SMatrix::<f64, 2, 8>::from_row_slice(&[
            -0.09, -0.14, -0.105, -0.01, 0.09, 0.12, 0.085, 0.0, // x positions
            0.095, 0.015, -0.06, -0.115, -0.085, 0.0, 0.105, 0.16, // y positions
        ]);

        let mic_times = SVector::<f64, 8>::from_row_slice(&[
            0.0004, 0.0005, 0.0007, 0.0008, 0.0007, 0.0005, 0.0004, 0.0001,
        ]);

        let soln = compute_direction(&mic_points, &mic_times).unwrap();
        println!("{soln:?}");

        assert!(0.0 <= soln[0] && soln[0] <= 0.1);
        assert!(0.9 <= soln[1] && soln[1] <= 0.91);
    }
}
