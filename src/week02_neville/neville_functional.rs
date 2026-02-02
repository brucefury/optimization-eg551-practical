//! Functional Style Neville's interpolation algorithm.(No early termination)
//!
//! This module implements the classical Neville's algorithm for polynomial
//! interpolation, building a full triangular pyramid of intermediate values.

use crate::week02_neville::types::NevilleResult;

/// Compute polynomial interpolation using Neville's algorithm.
pub fn neville_functional(points: &[(f64, f64)], x_target: f64) -> NevilleResult {
    if points.is_empty() {
        return NevilleResult {
            value: f64::NAN,
            pyramid: vec![],
            deltas: vec![],
        };
    }

    let x_vals: Vec<f64> = points.iter().map(|(x, _)| *x).collect();
    let first_column: Vec<f64> = points.iter().map(|(_, y)| *y).collect();

    let pyramid = build_pyramid(&x_vals, first_column, x_target);
    let deltas = compute_deltas(&pyramid);
    let value = pyramid.last().map(|col| col[0]).unwrap_or(f64::NAN);

    NevilleResult {
        value,
        pyramid,
        deltas,
    }
}

/// Build the triangular pyramid of intermediate values.
fn build_pyramid(x_vals: &[f64], first_column: Vec<f64>, x_target: f64) -> Vec<Vec<f64>> {
    let n = first_column.len();
    if n == 0 {
        return vec![];
    }

    (1..n).fold(vec![first_column], |mut pyramid, k| {
        let prev = &pyramid[k - 1];
        let next_column = compute_column(x_vals, prev, x_target, k);
        pyramid.push(next_column);
        pyramid
    })
}

/// Compute a single column of the pyramid.
fn compute_column(x_vals: &[f64], prev: &[f64], x_target: f64, k: usize) -> Vec<f64> {
    (0..prev.len() - 1)
        .map(|i| neville_formula(x_vals, prev, x_target, i, k))
        .collect()
}

/// Apply Neville's recursive formula for P_{i,i+k}.
fn neville_formula(x_vals: &[f64], prev: &[f64], x_target: f64, i: usize, k: usize) -> f64 {
    let x_i = x_vals[i];
    let x_j = x_vals[i + k];
    let denominator = x_j - x_i;

    let p_left = prev[i];      // P_{i, i+k-1}
    let p_right = prev[i + 1]; // P_{i+1, i+k}

    ((x_target - x_i) * p_right - (x_target - x_j) * p_left) / denominator
}

/// Compute deltas between successive column top values.
fn compute_deltas(pyramid: &[Vec<f64>]) -> Vec<f64> {
    if pyramid.len() < 2 {
        return vec![];
    }

    pyramid
        .windows(2)
        .map(|w| (w[1][0] - w[0][0]).abs())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_point() {
        let points = vec![(1.0, 5.0)];
        let result = neville_functional(&points, 2.0);

        assert!((result.value - 5.0).abs() < 1e-10);
        assert_eq!(result.pyramid.len(), 1);
        assert!(result.deltas.is_empty());
    }

    #[test]
    fn test_two_points_linear() {
        // Linear interpolation: y = 2x + 1
        let points = vec![(0.0, 1.0), (1.0, 3.0)];
        let result = neville_functional(&points, 0.5);

        // At x=0.5, y should be 2.0
        assert!((result.value - 2.0).abs() < 1e-10);
        assert_eq!(result.pyramid.len(), 2);
        assert_eq!(result.deltas.len(), 1);
    }

    #[test]
    fn test_quadratic_exact() {
        // y = x^2: points at x = -1, 0, 1
        let points = vec![(-1.0, 1.0), (0.0, 0.0), (1.0, 1.0)];
        let result = neville_functional(&points, 0.5);

        // At x=0.5, y should be 0.25
        assert!((result.value - 0.25).abs() < 1e-10);
        assert_eq!(result.pyramid.len(), 3);
    }

    #[test]
    fn test_cubic_exact() {
        // y = x^3: points at x = -1, 0, 1, 2
        let points = vec![(-1.0, -1.0), (0.0, 0.0), (1.0, 1.0), (2.0, 8.0)];
        let result = neville_functional(&points, 0.5);

        // At x=0.5, y should be 0.125
        assert!((result.value - 0.125).abs() < 1e-10);
        assert_eq!(result.pyramid.len(), 4);
    }

    #[test]
    fn test_pyramid_structure() {
        // 4 points should give pyramid with columns of length 4, 3, 2, 1
        let points = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 4.0), (3.0, 9.0)];
        let result = neville_functional(&points, 1.5);

        assert_eq!(result.pyramid.len(), 4);
        assert_eq!(result.pyramid[0].len(), 4);
        assert_eq!(result.pyramid[1].len(), 3);
        assert_eq!(result.pyramid[2].len(), 2);
        assert_eq!(result.pyramid[3].len(), 1);
    }

    #[test]
    fn test_deltas_decreasing_for_polynomial() {
        // For a polynomial with enough points, deltas should generally decrease
        let points: Vec<(f64, f64)> = (-5..=5)
            .map(|i| {
                let x = i as f64 * 0.5;
                (x, x * x)
            })
            .collect();
        let result = neville_functional(&points, 0.25);

        // Exact for quadratic with 3+ points, so later deltas should be small
        let last_delta = result.deltas.last().unwrap();
        assert!(*last_delta < 1e-10);
    }

    #[test]
    fn test_empty_points() {
        let points: Vec<(f64, f64)> = vec![];
        let result = neville_functional(&points, 1.0);

        assert!(result.value.is_nan());
        assert!(result.pyramid.is_empty());
        assert!(result.deltas.is_empty());
    }

    #[test]
    fn test_interpolate_at_known_point() {
        // When x_target equals one of the x values, should return exact y
        let points = vec![(0.0, 1.0), (1.0, 3.0), (2.0, 7.0)];
        let result = neville_functional(&points, 1.0);

        assert!((result.value - 3.0).abs() < 1e-10);
    }
}
