//! Week 1: Graphing Demos
//!
//! Basic plotting demonstrations using `common::plot`.
//!
//! This module provides simple examples of line plots, multi-line plots,
//! and bar charts to introduce the plotting infrastructure.

pub mod problems;

#[cfg(feature = "plotting")]
use crate::common::plot::{self, PlotConfig};

/// Generate a simple line plot of y = sin(x) over [0, 2π].
#[cfg(feature = "plotting")]
pub fn demo_line_plot(output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let n = 100;
    let x_data: Vec<f64> = (0..=n).map(|i| i as f64 * 2.0 * std::f64::consts::PI / n as f64).collect();
    let y_data: Vec<f64> = x_data.iter().map(|&x| x.sin()).collect();

    let config = PlotConfig {
        title: String::from("y = sin(x)"),
        x_label: String::from("x"),
        y_label: String::from("sin(x)"),
        ..PlotConfig::default()
    };

    plot::line_plot(output_path, &x_data, &y_data, &config)
}

/// Generate a multi-line plot comparing sin(x), cos(x), and sin(2x).
#[cfg(feature = "plotting")]
pub fn demo_multi_line_plot(output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let n = 100;
    let x_data: Vec<f64> = (0..=n).map(|i| i as f64 * 2.0 * std::f64::consts::PI / n as f64).collect();
    let sin_data: Vec<f64> = x_data.iter().map(|&x| x.sin()).collect();
    let cos_data: Vec<f64> = x_data.iter().map(|&x| x.cos()).collect();
    let sin2_data: Vec<f64> = x_data.iter().map(|&x| (2.0 * x).sin()).collect();

    let config = PlotConfig {
        title: String::from("Trigonometric Functions"),
        x_label: String::from("x"),
        y_label: String::from("y"),
        ..PlotConfig::default()
    };

    let series: Vec<(&str, &[f64])> = vec![
        ("sin(x)", &sin_data),
        ("cos(x)", &cos_data),
        ("sin(2x)", &sin2_data),
    ];

    plot::multi_line_plot(output_path, &x_data, &series, &config)
}

/// Generate a grouped bar chart comparing values across categories.
#[cfg(feature = "plotting")]
pub fn demo_bar_chart(output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let categories = vec!["A", "B", "C", "D"];
    let groups: Vec<(&str, &[f64])> = vec![
        ("Group 1", &[4.0, 7.0, 3.0, 8.0]),
        ("Group 2", &[6.0, 2.0, 9.0, 5.0]),
    ];

    let config = PlotConfig {
        title: String::from("Grouped Bar Chart Demo"),
        x_label: String::from("Category"),
        y_label: String::from("Value"),
        ..PlotConfig::default()
    };

    plot::grouped_bar_chart(output_path, &categories, &groups, &config)
}
