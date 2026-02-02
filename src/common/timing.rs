//! Timing utilities for measuring and reporting execution durations.

use std::time::{Duration, Instant};

/// Result of a timed operation.
pub struct TimedResult<T> {
    pub value: T,
    pub duration: Duration,
}

/// Execute a function and measure its duration.
pub fn timed<T, F: FnOnce() -> T>(f: F) -> TimedResult<T> {
    let start = Instant::now();
    let value = f();
    let duration = start.elapsed();
    TimedResult { value, duration }
}

/// Format a duration in a human-readable way.
pub fn format_duration(d: Duration) -> String {
    let nanos = d.as_nanos();
    if nanos < 1_000 {
        format!("{}ns", nanos)
    } else if nanos < 1_000_000 {
        format!("{:.2}us", nanos as f64 / 1_000.0)
    } else if nanos < 1_000_000_000 {
        format!("{:.2}ms", nanos as f64 / 1_000_000.0)
    } else {
        format!("{:.2}s", d.as_secs_f64())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timed_returns_value() {
        let result = timed(|| 42);
        assert_eq!(result.value, 42);
    }

    #[test]
    fn test_timed_measures_duration() {
        let result = timed(|| std::thread::sleep(Duration::from_millis(10)));
        assert!(result.duration >= Duration::from_millis(10));
    }

    #[test]
    fn test_format_duration_nanoseconds() {
        assert_eq!(format_duration(Duration::from_nanos(500)), "500ns");
    }

    #[test]
    fn test_format_duration_microseconds() {
        let formatted = format_duration(Duration::from_micros(500));
        assert!(formatted.contains("us"));
    }

    #[test]
    fn test_format_duration_milliseconds() {
        let formatted = format_duration(Duration::from_millis(50));
        assert!(formatted.contains("ms"));
    }

    #[test]
    fn test_format_duration_seconds() {
        let formatted = format_duration(Duration::from_secs(2));
        assert!(formatted.contains("s"));
    }
}
