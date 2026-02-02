//! Plotting utilities for visualizing algorithm results.
//!
//! Enable the `plotting` feature to use these utilities:
//! ```toml
//! optimization-eg551-practical = { version = "0.1", features = ["plotting"] }
//! ```

#[cfg(feature = "plotting")]
use plotters::prelude::*;

/// Configuration for a 2D line plot.
pub struct PlotConfig {
    pub title: String,
    pub x_label: String,
    pub y_label: String,
    pub width: u32,
    pub height: u32,
    pub y_log_scale: bool,
    pub show_legend: bool,
    /// Custom x-axis range (None = auto-compute from data)
    pub x_range: Option<(f64, f64)>,
    /// Custom y-axis range (None = auto-compute from data)
    pub y_range: Option<(f64, f64)>,
    /// Fraction for axis padding (default 0.05)
    pub margin_fraction: f64,
    /// Line thickness (default 3)
    pub stroke_width: u32,
}

impl Default for PlotConfig {
    fn default() -> Self {
        Self {
            title: String::from("Plot"),
            x_label: String::from("x"),
            y_label: String::from("y"),
            width: 800,
            height: 600,
            y_log_scale: false,
            show_legend: true,
            x_range: None,
            y_range: None,
            margin_fraction: 0.05,
            stroke_width: 3,
        }
    }
}

impl PlotConfig {
    /// Set a custom x-axis range.
    pub fn with_x_range(mut self, min: f64, max: f64) -> Self {
        self.x_range = Some((min, max));
        self
    }

    /// Set a custom y-axis range.
    pub fn with_y_range(mut self, min: f64, max: f64) -> Self {
        self.y_range = Some((min, max));
        self
    }

    /// Set the margin fraction for axis padding.
    pub fn with_margin_fraction(mut self, fraction: f64) -> Self {
        self.margin_fraction = fraction;
        self
    }

    /// Set the stroke width for lines.
    pub fn with_stroke_width(mut self, width: u32) -> Self {
        self.stroke_width = width;
        self
    }
}

/// Creates a simple 2D line plot (requires `plotting` feature).
#[cfg(feature = "plotting")]
pub fn line_plot(
    path: &str,
    x_data: &[f64],
    y_data: &[f64],
    config: &PlotConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(path, (config.width, config.height)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_range = config
        .x_range
        .map(|(min, max)| min..max)
        .unwrap_or_else(|| find_range(x_data, config.margin_fraction));
    let y_range = config
        .y_range
        .map(|(min, max)| min..max)
        .unwrap_or_else(|| find_range(y_data, config.margin_fraction));

    let mut chart = ChartBuilder::on(&root)
        .caption(&config.title, ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(x_range, y_range)?;

    chart
        .configure_mesh()
        .x_desc(&config.x_label)
        .y_desc(&config.y_label)
        .draw()?;

    let points: Vec<(f64, f64)> = x_data.iter().zip(y_data.iter()).map(|(&x, &y)| (x, y)).collect();

    chart.draw_series(LineSeries::new(points, BLUE.stroke_width(config.stroke_width)))?;

    root.present()?;
    Ok(())
}

/// Creates a 2D line plot with red circle markers at specified highlight points (requires `plotting` feature).
#[cfg(feature = "plotting")]
pub fn line_plot_with_markers(
    path: &str,
    x_data: &[f64],
    y_data: &[f64],
    markers: &[(f64, f64)],
    config: &PlotConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(path, (config.width, config.height)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_range = config
        .x_range
        .map(|(min, max)| min..max)
        .unwrap_or_else(|| find_range(x_data, config.margin_fraction));
    let y_range = config
        .y_range
        .map(|(min, max)| min..max)
        .unwrap_or_else(|| find_range(y_data, config.margin_fraction));

    let mut chart = ChartBuilder::on(&root)
        .caption(&config.title, ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(x_range, y_range)?;

    chart
        .configure_mesh()
        .x_desc(&config.x_label)
        .y_desc(&config.y_label)
        .draw()?;

    let points: Vec<(f64, f64)> = x_data.iter().zip(y_data.iter()).map(|(&x, &y)| (x, y)).collect();
    chart.draw_series(LineSeries::new(points, BLUE.stroke_width(config.stroke_width)))?;

    chart.draw_series(
        markers.iter().map(|&(x, y)| Circle::new((x, y), 8, RED.filled())),
    )?;

    root.present()?;
    Ok(())
}

#[cfg(feature = "plotting")]
fn find_range(data: &[f64], margin_fraction: f64) -> std::ops::Range<f64> {
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let margin = (max - min) * margin_fraction;
    (min - margin)..(max + margin)
}

/// Returns a consistent color for series by index.
#[cfg(feature = "plotting")]
pub fn series_color(index: usize) -> RGBColor {
    const COLORS: [(u8, u8, u8); 8] = [
        (31, 119, 180),   // blue
        (255, 127, 14),   // orange
        (44, 160, 44),    // green
        (214, 39, 40),    // red
        (148, 103, 189),  // purple
        (140, 86, 75),    // brown
        (227, 119, 194),  // pink
        (127, 127, 127),  // gray
    ];
    let (r, g, b) = COLORS[index % COLORS.len()];
    RGBColor(r, g, b)
}

/// Computes y-axis range for linear scale.
#[cfg(feature = "plotting")]
fn find_range_linear(data: &[f64], margin_fraction: f64) -> std::ops::Range<f64> {
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let margin = (max - min) * margin_fraction;
    (min - margin).max(0.0)..(max + margin)
}

/// Computes y-axis range for log scale (filters to positive values).
#[cfg(feature = "plotting")]
fn find_range_log(data: &[f64]) -> std::ops::Range<f64> {
    let positive: Vec<f64> = data.iter().cloned().filter(|&v| v > 0.0).collect();
    if positive.is_empty() {
        return 1e-10..1.0;
    }

    let min = positive.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = positive.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let log_min = min.log10().floor();
    let log_max = max.log10().ceil();
    10f64.powf(log_min)..10f64.powf(log_max)
}

/// Creates a multi-line plot with multiple series (requires `plotting` feature).
#[cfg(feature = "plotting")]
pub fn multi_line_plot(
    path: &str,
    x_data: &[f64],
    series: &[(&str, &[f64])],
    config: &PlotConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    if config.y_log_scale {
        multi_line_plot_log(path, x_data, series, config)
    } else {
        multi_line_plot_linear(path, x_data, series, config)
    }
}

#[cfg(feature = "plotting")]
fn multi_line_plot_linear(
    path: &str,
    x_data: &[f64],
    series: &[(&str, &[f64])],
    config: &PlotConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(path, (config.width, config.height)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_range = config
        .x_range
        .map(|(min, max)| min..max)
        .unwrap_or_else(|| find_range(x_data, config.margin_fraction));
    let all_y: Vec<f64> = series.iter().flat_map(|(_, ys)| ys.iter().cloned()).collect();
    let y_range = config
        .y_range
        .map(|(min, max)| min..max)
        .unwrap_or_else(|| find_range_linear(&all_y, config.margin_fraction));

    let mut chart = ChartBuilder::on(&root)
        .caption(&config.title, ("sans-serif", 24))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(x_range, y_range)?;

    chart
        .configure_mesh()
        .x_desc(&config.x_label)
        .y_desc(&config.y_label)
        .draw()?;

    let stroke = config.stroke_width;
    for (idx, (label, y_data)) in series.iter().enumerate() {
        let color = series_color(idx);
        let points: Vec<(f64, f64)> = x_data
            .iter()
            .zip(y_data.iter())
            .map(|(&x, &y)| (x, y))
            .collect();

        let line_series = LineSeries::new(points, color.stroke_width(stroke));

        if config.show_legend {
            chart
                .draw_series(line_series)?
                .label(*label)
                .legend(move |(x, y)| {
                    PathElement::new(vec![(x, y), (x + 20, y)], color.stroke_width(stroke))
                });
        } else {
            chart.draw_series(line_series)?;
        }
    }

    if config.show_legend {
        chart.configure_series_labels().border_style(BLACK).draw()?;
    }

    root.present()?;
    Ok(())
}

#[cfg(feature = "plotting")]
fn multi_line_plot_log(
    path: &str,
    x_data: &[f64],
    series: &[(&str, &[f64])],
    config: &PlotConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(path, (config.width, config.height)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_range = config
        .x_range
        .map(|(min, max)| min..max)
        .unwrap_or_else(|| find_range(x_data, config.margin_fraction));
    let all_y: Vec<f64> = series.iter().flat_map(|(_, ys)| ys.iter().cloned()).collect();
    let y_range = config
        .y_range
        .map(|(min, max)| min..max)
        .unwrap_or_else(|| find_range_log(&all_y));

    let mut chart = ChartBuilder::on(&root)
        .caption(&config.title, ("sans-serif", 24))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(80)
        .build_cartesian_2d(x_range, y_range.log_scale())?;

    chart
        .configure_mesh()
        .x_desc(&config.x_label)
        .y_desc(&config.y_label)
        .draw()?;

    let stroke = config.stroke_width;
    for (idx, (label, y_data)) in series.iter().enumerate() {
        let color = series_color(idx);
        let points: Vec<(f64, f64)> = x_data
            .iter()
            .zip(y_data.iter())
            .filter(|(_, &y)| y > 0.0)
            .map(|(&x, &y)| (x, y))
            .collect();

        let line_series = LineSeries::new(points, color.stroke_width(stroke));

        if config.show_legend {
            chart
                .draw_series(line_series)?
                .label(*label)
                .legend(move |(x, y)| {
                    PathElement::new(vec![(x, y), (x + 20, y)], color.stroke_width(stroke))
                });
        } else {
            chart.draw_series(line_series)?;
        }
    }

    if config.show_legend {
        chart.configure_series_labels().border_style(BLACK).draw()?;
    }

    root.present()?;
    Ok(())
}

/// Creates a grouped bar chart (requires `plotting` feature).
#[cfg(feature = "plotting")]
pub fn grouped_bar_chart(
    path: &str,
    categories: &[&str],
    groups: &[(&str, &[f64])],
    config: &PlotConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    if config.y_log_scale {
        grouped_bar_chart_log(path, categories, groups, config)
    } else {
        grouped_bar_chart_linear(path, categories, groups, config)
    }
}

#[cfg(feature = "plotting")]
fn grouped_bar_chart_linear(
    path: &str,
    categories: &[&str],
    groups: &[(&str, &[f64])],
    config: &PlotConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(path, (config.width, config.height)).into_drawing_area();
    root.fill(&WHITE)?;

    let n_categories = categories.len();
    let n_groups = groups.len();
    let bar_width = 0.8 / n_groups as f64;

    let all_y: Vec<f64> = groups.iter().flat_map(|(_, vals)| vals.iter().cloned()).collect();
    let y_range = config
        .y_range
        .map(|(min, max)| min..max)
        .unwrap_or_else(|| find_range_linear(&all_y, config.margin_fraction));

    let mut chart = ChartBuilder::on(&root)
        .caption(&config.title, ("sans-serif", 24))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(-0.5..(n_categories as f64 - 0.5), y_range)?;

    let cats: Vec<String> = categories.iter().map(|s| s.to_string()).collect();

    chart
        .configure_mesh()
        .x_desc(&config.x_label)
        .y_desc(&config.y_label)
        .x_labels(n_categories)
        .x_label_formatter(&|x| {
            let idx = x.round() as i32;
            if idx >= 0 && (idx as usize) < cats.len() {
                cats[idx as usize].clone()
            } else {
                String::new()
            }
        })
        .draw()?;

    for (group_idx, (label, values)) in groups.iter().enumerate() {
        let color = series_color(group_idx);
        let offset = (group_idx as f64 - (n_groups as f64 - 1.0) / 2.0) * bar_width;

        let bars: Vec<_> = values
            .iter()
            .enumerate()
            .map(|(cat_idx, &val)| {
                let x = cat_idx as f64 + offset;
                Rectangle::new(
                    [(x - bar_width / 2.0, 0.0), (x + bar_width / 2.0, val)],
                    color.filled(),
                )
            })
            .collect();

        if config.show_legend {
            chart.draw_series(bars)?.label(*label).legend(move |(x, y)| {
                Rectangle::new([(x, y - 5), (x + 20, y + 5)], color.filled())
            });
        } else {
            chart.draw_series(bars)?;
        }
    }

    if config.show_legend {
        chart.configure_series_labels().border_style(BLACK).draw()?;
    }

    root.present()?;
    Ok(())
}

#[cfg(feature = "plotting")]
fn grouped_bar_chart_log(
    path: &str,
    categories: &[&str],
    groups: &[(&str, &[f64])],
    config: &PlotConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(path, (config.width, config.height)).into_drawing_area();
    root.fill(&WHITE)?;

    let n_categories = categories.len();
    let n_groups = groups.len();
    let bar_width = 0.8 / n_groups as f64;

    let all_y: Vec<f64> = groups.iter().flat_map(|(_, vals)| vals.iter().cloned()).collect();
    let y_range = config
        .y_range
        .map(|(min, max)| min..max)
        .unwrap_or_else(|| find_range_log(&all_y));

    let y_min = y_range.start;

    let mut chart = ChartBuilder::on(&root)
        .caption(&config.title, ("sans-serif", 24))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(80)
        .build_cartesian_2d(-0.5..(n_categories as f64 - 0.5), y_range.log_scale())?;

    let cats: Vec<String> = categories.iter().map(|s| s.to_string()).collect();

    chart
        .configure_mesh()
        .x_desc(&config.x_label)
        .y_desc(&config.y_label)
        .x_labels(n_categories)
        .x_label_formatter(&|x| {
            let idx = x.round() as i32;
            if idx >= 0 && (idx as usize) < cats.len() {
                cats[idx as usize].clone()
            } else {
                String::new()
            }
        })
        .draw()?;

    for (group_idx, (label, values)) in groups.iter().enumerate() {
        let color = series_color(group_idx);
        let offset = (group_idx as f64 - (n_groups as f64 - 1.0) / 2.0) * bar_width;

        let bars: Vec<_> = values
            .iter()
            .enumerate()
            .filter(|(_, &val)| val > 0.0)
            .map(|(cat_idx, &val)| {
                let x = cat_idx as f64 + offset;
                Rectangle::new(
                    [(x - bar_width / 2.0, y_min), (x + bar_width / 2.0, val)],
                    color.filled(),
                )
            })
            .collect();

        if config.show_legend {
            chart.draw_series(bars)?.label(*label).legend(move |(x, y)| {
                Rectangle::new([(x, y - 5), (x + 20, y + 5)], color.filled())
            });
        } else {
            chart.draw_series(bars)?;
        }
    }

    if config.show_legend {
        chart.configure_series_labels().border_style(BLACK).draw()?;
    }

    root.present()?;
    Ok(())
}

/// Stub for when plotting feature is disabled.
#[cfg(not(feature = "plotting"))]
pub fn line_plot(
    _path: &str,
    _x_data: &[f64],
    _y_data: &[f64],
    _config: &PlotConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    Err("Enable the 'plotting' feature to use this function".into())
}

/// Stub for when plotting feature is disabled.
#[cfg(not(feature = "plotting"))]
pub fn line_plot_with_markers(
    _path: &str,
    _x_data: &[f64],
    _y_data: &[f64],
    _markers: &[(f64, f64)],
    _config: &PlotConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    Err("Enable the 'plotting' feature to use this function".into())
}
