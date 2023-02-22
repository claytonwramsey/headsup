//! Algorithms for audio localization.

use nalgebra::{Const, OMatrix, OVector};

const SPEED_OF_SOUND: f64 = 343.260;

#[must_use]
pub fn compute_direction<const DIM: usize, const N_MICS: usize>(
    positions: &OMatrix<f64, Const<DIM>, Const<N_MICS>>,
    times: &OVector<f64, Const<N_MICS>>,
) -> OVector<f64, Const<DIM>> {
    let min_time_idx = (0..times.len())
        .min_by(|&i, &j| times[i].partial_cmp(&times[j]).unwrap())
        .unwrap();
    let min_time = times[min_time_idx];

    let adjusted_distances = SPEED_OF_SOUND * times.add_scalar(-min_time);

    let mut zeroed_positions = positions.clone_owned();

    for mut col in zeroed_positions.column_iter_mut() {
        col -= positions.column(min_time_idx);
    }

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

        println!("{:?}", compute_direction(&mic_points, &mic_times));
    }
}
