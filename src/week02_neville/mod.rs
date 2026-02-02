//! Week 2: Neville's Interpolation Algorithm
//!
//! This module implements the classical (stock) Neville's polynomial interpolation,
//! building a full triangular pyramid of intermediate values.
//!
//! # Example
//!
//! ```
//! use optimization_eg551_practical::week02_neville::neville_stock;
//!
//! // Sample points from y = x^2
//! let points = vec![(-1.0, 1.0), (0.0, 0.0), (1.0, 1.0), (2.0, 4.0)];
//!
//! let result = neville_stock(&points, 0.5);
//! assert!((result.value - 0.25).abs() < 1e-10);
//! ```

pub mod types;
pub mod stock;
pub mod problems;

// Re-export main types and functions for convenience
pub use types::NevilleResult;
pub use stock::neville_stock;
