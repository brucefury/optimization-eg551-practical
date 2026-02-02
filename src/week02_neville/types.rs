//! Core data structures for Neville's interpolation algorithm.

/// Result from the stock Neville's algorithm.
#[derive(Debug, Clone)]
pub struct NevilleResult {
    /// The interpolated value at the target point.
    pub value: f64,
    /// The full triangular pyramid of intermediate values.
    /// pyramid[k][i] represents P_{i,i+k}.
    pub pyramid: Vec<Vec<f64>>,
    /// Deltas between successive column top values: |P[0,k] - P[0,k-1]|.
    pub deltas: Vec<f64>,
}
