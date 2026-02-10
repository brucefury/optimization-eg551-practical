//! Core data structures for the Golden Section Search algorithm.

/// Stopping criterion for the GSS algorithm.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StoppingCriterion {
    /// Stop when the bracket width (b - a) falls below epsilon.
    IntervalWidth,
    /// Stop when |f(x1) - f(x2)| falls below epsilon.
    FunctionValueDiff,
}

/// Result from the Golden Section Search algorithm.
#[derive(Debug, Clone)]
pub struct GssResult {
    pub a: f64,
    pub b: f64,
    /// Left interior point.
    pub x1: f64,
    /// Right interior point.
    pub x2: f64,
    /// Function value at x1.
    pub fx1: f64,
    /// Function value at x2.
    pub fx2: f64,
    /// Number of iterations performed.
    pub iterations: usize,
    /// Final bracket width (b - a).
    pub interval_width: f64,
    /// Final |f(x1) - f(x2)|.
    pub function_value_diff: f64,
}
