//! Neville's interpolation algorithm.
//!

use crate::week02_neville::types::NevilleResult;

pub fn neville(points: &[(f64, f64)], x_target: f64) -> NevilleResult {
    if points.is_empty() {
        return NevilleResult {
            value: f64::NAN,
            pyramid: vec![],
            deltas: vec![],
        };
    }

    let n = points.len();
    let x_vals: Vec<f64> = points.iter().map(|(x, _)| *x).collect();

    // Build first column from y-values
    let mut pyramid: Vec<Vec<f64>> = Vec::with_capacity(n);
    let mut first_col = Vec::with_capacity(n);
    for &(_, y) in points {
        first_col.push(y);
    }
    pyramid.push(first_col);

    // Build remaining columns
    for k in 1..n {
        let prev = &pyramid[k - 1];
        let col_len = prev.len() - 1;
        let mut col = Vec::with_capacity(col_len);

        //Could caluclate delta here and drop out 
        for i in 0..col_len {
            //Apply Neville's formula here
            let x_i = x_vals[i];
            let x_j = x_vals[i + k];
            let denom = x_j - x_i;
            let val = ((x_target - x_i) * prev[i + 1] - (x_target - x_j) * prev[i]) / denom;
            col.push(val);
        }
        pyramid.push(col);
    }

    // Compute deltas
    let mut deltas = Vec::with_capacity(if n > 1 { n - 1 } else { 0 });
    for i in 0..pyramid.len() - 1 {
        deltas.push((pyramid[i + 1][0] - pyramid[i][0]).abs());
    }

    let value = pyramid.last().map(|col| col[0]).unwrap_or(f64::NAN);

    NevilleResult {
        value,
        pyramid,
        deltas,
    }
}