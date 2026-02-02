//! EG551T Tutorial Sheet 1: Graphing Problems
//!
//! Ports the 4 plotting exercises from the PDF into Rust functions
//! using `common::plot` infrastructure.

#[cfg(feature = "plotting")]
use crate::common::{
    csv,
    plot::{self, PlotConfig},
};

/// Fixed table data from Q1-7.
pub const X_TABLE: [f64; 9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
pub const Y_TABLE: [f64; 9] = [3.0, 7.0, 13.0, 21.0, 31.0, 43.0, 57.0, 73.0, 91.0];

/// Generate `n` evenly spaced values in `[start, end]`.
#[cfg(any(feature = "plotting", test))]
fn linspace(start: f64, end: f64, n: usize) -> Vec<f64> {
    assert!(n >= 2, "linspace requires n >= 2");
    let step = (end - start) / (n - 1) as f64;
    (0..n).map(|i| start + i as f64 * step).collect()
}

/// Q1-7: Plot the table data and save to CSV.
#[cfg(feature = "plotting")]
pub fn problem_table_plot(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = PlotConfig {
        title: String::from("Tutorial Sheet 1: Table Data"),
        x_label: String::from("x"),
        y_label: String::from("y"),
        ..PlotConfig::default()
    };

    let plot_path = format!("{}/table_data.png", dir);
    plot::line_plot(&plot_path, &X_TABLE, &Y_TABLE, &config)?;

    // Save to CSV
    let csv_path = format!("{}/table_data.csv", dir);
    let rows: Vec<Vec<f64>> = X_TABLE
        .iter()
        .zip(Y_TABLE.iter())
        .map(|(&x, &y)| vec![x, y])
        .collect();
    csv::write_csv(&csv_path, &["x", "y"], &rows)?;

    Ok(())
}

/// Q8a: Plot sin(1/x) at three zoom levels.
#[cfg(feature = "plotting")]
pub fn problem_sin_reciprocal(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let ranges = [
        (-1.0, 1.0, "sin_reciprocal_wide.png"),
        (-0.1, 0.1, "sin_reciprocal_medium.png"),
        (-0.01, 0.01, "sin_reciprocal_narrow.png"),
    ];

    for (xmin, xmax, filename) in &ranges {
        let n = 2000;
        let x_data = linspace(*xmin, *xmax, n);
        let y_data: Vec<f64> = x_data
            .iter()
            .map(|&x| if x == 0.0 { 0.0 } else { (1.0 / x).sin() })
            .collect();

        let config = PlotConfig {
            title: format!("sin(1/x) over [{}, {}]", xmin, xmax),
            x_label: String::from("x"),
            y_label: String::from("sin(1/x)"),
            ..PlotConfig::default()
        };

        let path = format!("{}/{}", dir, filename);
        plot::line_plot(&path, &x_data, &y_data, &config)?;
    }

    Ok(())
}

/// Q8b: Plot |x|·|x-1|·|x+1| over [-1.5, 1.5].
#[cfg(feature = "plotting")]
pub fn problem_non_smooth(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let n = 1000;
    let x_data = linspace(-1.5, 1.5, n);
    let y_data: Vec<f64> = x_data
        .iter()
        .map(|&x| x.abs() * (x - 1.0).abs() * (x + 1.0).abs())
        .collect();

    let config = PlotConfig {
        title: String::from("|x|·|x-1|·|x+1|"),
        x_label: String::from("x"),
        y_label: String::from("y"),
        ..PlotConfig::default()
    };

    let path = format!("{}/non_smooth.png", dir);
    plot::line_plot(&path, &x_data, &y_data, &config)
}

/// Q8c: Piecewise discontinuous function over [-1.5, 2.5].
/// f(x) = 0 if x<0, 1 if 0≤x<1, 0.6 otherwise.
#[cfg(feature = "plotting")]
pub fn problem_piecewise(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let n = 1000;
    let x_data = linspace(-1.5, 2.5, n);
    let y_data: Vec<f64> = x_data
        .iter()
        .map(|&x| {
            if x < 0.0 {
                0.0
            } else if x < 1.0 {
                1.0
            } else {
                0.6
            }
        })
        .collect();

    let config = PlotConfig {
        title: String::from("Piecewise Discontinuous Function"),
        x_label: String::from("x"),
        y_label: String::from("f(x)"),
        ..PlotConfig::default()
    };

    let path = format!("{}/piecewise.png", dir);
    plot::line_plot(&path, &x_data, &y_data, &config)
}

/// Q8d: Plot 1/(x²-x) over [-1, 2] with y clamped to [-10, 10].
#[cfg(feature = "plotting")]
pub fn problem_rational(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let n = 2000;
    let x_data = linspace(-1.0, 2.0, n);
    let y_data: Vec<f64> = x_data
        .iter()
        .map(|&x| {
            let denom = x * x - x;
            if denom.abs() < 1e-12 {
                f64::NAN
            } else {
                (1.0 / denom).clamp(-10.0, 10.0)
            }
        })
        .collect();

    let config = PlotConfig {
        title: String::from("1/(x² - x)"),
        x_label: String::from("x"),
        y_label: String::from("y"),
        y_range: Some((-10.0, 10.0)),
        ..PlotConfig::default()
    };

    let path = format!("{}/rational.png", dir);
    plot::line_plot(&path, &x_data, &y_data, &config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linspace() {
        let v = linspace(0.0, 1.0, 5);
        assert_eq!(v.len(), 5);
        assert!((v[0] - 0.0).abs() < 1e-12);
        assert!((v[4] - 1.0).abs() < 1e-12);
        assert!((v[2] - 0.5).abs() < 1e-12);
    }

    #[test]
    fn test_table_data() {
        assert_eq!(X_TABLE.len(), 9);
        assert_eq!(Y_TABLE.len(), 9);
        assert_eq!(X_TABLE[0], 1.0);
        assert_eq!(Y_TABLE[8], 91.0);
    }
}
