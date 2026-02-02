//! Week 2: Neville's Interpolation Algorithm

pub mod types;
pub mod neville_functional;
pub mod neville;
pub mod problems;

// Re-export
pub use types::NevilleResult;
pub use neville::neville;
pub use neville_functional::neville_functional;
