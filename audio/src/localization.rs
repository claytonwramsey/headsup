//! Algorithms for audio localization.

use nalgebra::{Const, OMatrix, OVector}; 

const SPEED_OF_SOUND: f64 = 343.260;

#[must_use]
pub fn compute_direction<const DIM: usize, const N_MICS: usize>(
    positions: &OMatrix<f64, Const<DIM>, Const<N_MICS>>,
    times: &OVector<f64, Const<N_MICS>>,
) -> OVector<f64, Const<DIM>> {
    let mut adjusted_distances = times.clone_owned();

    let min_time_idx = (0..times.len())
        .min_by(|&i, &j| times[i].partial_cmp(&times[j]).unwrap())
        .unwrap();
    let min_time = times[min_time_idx];

    for i in 0..times.len() {
        adjusted_distances[i] = (times[i] - min_time) * SPEED_OF_SOUND;
    }

    let mut zeroed_positions = positions.clone_owned();

    for (i, elem) in zeroed_positions.iter_mut().enumerate() {
        let c = i % positions.nrows();
        *elem -= positions.column(min_time_idx)[c];
    }

    println!("{adjusted_distances:?}");
    println!("{:?}", zeroed_positions.transpose());

    // lstsq::lstsq(&zeroed_positions.transpose(), &adjusted_distances, 1e-3);
    let soln = -(zeroed_positions * zeroed_positions.transpose())
        .try_inverse()
        .unwrap()
        * zeroed_positions
        * adjusted_distances;

    soln
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::{SMatrix, SVector};

    #[test]
    fn it_works() {
        let mic_points = SMatrix::<f64, 2, 8>::from_row_slice(&[
            -0.09, -0.14, -0.105, -0.01, 0.09, 0.12, 0.085, 0.0, // x positions
            0.095, 0.015, -0.06, -0.115, -0.085, 0.0, 0.105, 0.16, // y positions
        ]);

        let mic_times = SVector::<f64, 8>::from_row_slice(&[
            0.0004, 0.0005, 0.0007, 0.0008, 0.0007, 0.0005, 0.0004, 0.0001,
        ]);

        compute_direction(&mic_points, &mic_times);
    }
}
