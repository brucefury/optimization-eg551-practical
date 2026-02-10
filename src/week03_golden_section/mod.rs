//! Week 3: Golden Section Search
//!
//! One-dimensional optimization using the golden section method.

pub mod types;
pub mod gss;

pub use types::{GssResult, StoppingCriterion};
pub use gss::gss;
