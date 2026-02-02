//! EG551T Practical Problems - Neville's Algorithm
//!
//! interpolate x = 0.51
//! x = [0.5, 0.7, 0.3, 0.9]
//! y = [0.58813, 0.72210, 0.39646, 0.89608]
//!
//! interpolate x = 0.15
//! x = [0.1, 0.2, 0.3, 0.4, 0.5]
//! y = [10.00000, 5.00000, 3.33333, 2.50000, 2.00000]

use crate::week02_neville::{neville, NevilleResult};

#[cfg(feature = "plotting")]
use crate::common::plot::{self, PlotConfig};

/// Reusable helper: build points from separate x/y slices, run Neville's, print results.
#[allow(dead_code)]
fn interpolate_and_print(x_points: &[f64], y_points: &[f64], x_target: f64) -> NevilleResult {
    let points: Vec<(f64, f64)> = x_points.iter().zip(y_points.iter()).map(|(&x, &y)| (x, y)).collect();
    let result = neville(&points, x_target);

    println!("  Interpolating at x = {}", x_target);
    println!("  x = {:?}", x_points);
    println!("  y = {:?}", y_points);
    println!("  Result: {:.6}", result.value);

    println!("  Pyramid:");
    for (k, col) in result.pyramid.iter().enumerate() {
        println!("    col {}: {:?}", k, col.iter().map(|v| format!("{:.6}", v)).collect::<Vec<_>>());
    }

    println!("  Deltas: {:?}", result.deltas.iter().map(|v| format!("{:.6e}", v)).collect::<Vec<_>>());
    println!();

    result
}

/// Run both datasets through Neville's algorithm and produce plots.
#[cfg(feature = "plotting")]
pub fn problem_neville_plots(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Dataset 1
    let x1 = [0.5, 0.7, 0.3, 0.9];
    let y1 = [0.58813, 0.72210, 0.39646, 0.89608];
    let xt1 = 0.51;

    println!("  Dataset 1:");
    let r1 = interpolate_and_print(&x1, &y1, xt1);

    // Dataset 2
    let x2 = [0.1, 0.2, 0.3, 0.4, 0.5];
    let y2 = [10.0, 5.0, 3.33333, 2.5, 2.0];
    let xt2 = 0.15;

    println!("  Dataset 2:");
    let r2 = interpolate_and_print(&x2, &y2, xt2);

    // Plot 1a: Dataset 1 with interpolated point
    {
        let mut xs: Vec<f64> = x1.to_vec();
        let mut ys: Vec<f64> = y1.to_vec();
        xs.push(xt1);
        ys.push(r1.value);
        // Sort by x so line_plot draws a proper curve
        let mut pairs: Vec<(f64, f64)> = xs.into_iter().zip(ys).collect();
        pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let xs: Vec<f64> = pairs.iter().map(|p| p.0).collect();
        let ys: Vec<f64> = pairs.iter().map(|p| p.1).collect();

        let config = PlotConfig {
            title: String::from("Dataset 1: Neville Interpolation"),
            x_label: String::from("x"),
            y_label: String::from("y"),
            ..PlotConfig::default()
        };
        plot::line_plot_with_markers(&format!("{}/neville_dataset1.png", dir), &xs, &ys, &[(xt1, r1.value)], &config)?;
    }

    // Plot 1b: Dataset 2 with interpolated point
    {
        let mut xs: Vec<f64> = x2.to_vec();
        let mut ys: Vec<f64> = y2.to_vec();
        xs.push(xt2);
        ys.push(r2.value);
        let mut pairs: Vec<(f64, f64)> = xs.into_iter().zip(ys).collect();
        pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let xs: Vec<f64> = pairs.iter().map(|p| p.0).collect();
        let ys: Vec<f64> = pairs.iter().map(|p| p.1).collect();

        let config = PlotConfig {
            title: String::from("Dataset 2: Neville Interpolation"),
            x_label: String::from("x"),
            y_label: String::from("y"),
            ..PlotConfig::default()
        };
        plot::line_plot_with_markers(&format!("{}/neville_dataset2.png", dir), &xs, &ys, &[(xt2, r2.value)], &config)?;
    }

    // Plot 2: Delta convergence for both datasets
    {
        let max_len = r1.deltas.len().max(r2.deltas.len());
        let x_idx: Vec<f64> = (1..=max_len).map(|i| i as f64).collect();

        // Pad shorter delta series with NaN (won't plot)
        let d1: Vec<f64> = (0..max_len)
            .map(|i| if i < r1.deltas.len() { r1.deltas[i] } else { f64::NAN })
            .collect();
        let d2: Vec<f64> = (0..max_len)
            .map(|i| if i < r2.deltas.len() { r2.deltas[i] } else { f64::NAN })
            .collect();

        let series: Vec<(&str, &[f64])> = vec![
            ("Dataset 1", d1.as_slice()),
            ("Dataset 2", d2.as_slice()),
        ];

        let config = PlotConfig {
            title: String::from("Neville Delta Convergence"),
            x_label: String::from("Iteration"),
            y_label: String::from("|delta|"),
            ..PlotConfig::default()
        };
        plot::multi_line_plot(&format!("{}/neville_deltas.png", dir), &x_idx, &series, &config)?;
    }

    Ok(())
}
