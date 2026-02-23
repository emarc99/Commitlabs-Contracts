#![no_std]

//! Shared utility library for Soroban smart contracts
//!
//! This library provides common functions, helpers, and patterns used across
//! all CommitLabs contracts including:
//! - Math utilities (safe math, percentages)
//! - Time utilities (timestamps, durations)
//! - Validation utilities
//! - Storage helpers
//! - Error helpers
//! - Access control patterns
//! - Event emission patterns
//! - Rate limiting helpers

pub mod access_control;
pub mod batch;
pub mod emergency;
pub mod error_codes;
pub mod errors;
pub mod events;
pub mod math;
pub mod pausable;
pub mod rate_limiting;
pub mod storage;
pub mod time;
pub mod validation;

#[cfg(test)]
mod tests;

// Re-export commonly used items
// These imports are primarily for external consumers of the crate.  We
// allow unused imports here to avoid warnings in the library itself.
#[allow(unused_imports)]
pub use access_control::*;
#[allow(unused_imports)]
pub use batch::*;
#[allow(unused_imports)]
pub use emergency::EmergencyControl;
#[allow(unused_imports)]
pub use error_codes::*;
#[allow(unused_imports)]
pub use errors::*;
#[allow(unused_imports)]
pub use events::*;
#[allow(unused_imports)]
pub use math::*;
#[allow(unused_imports)]
pub use pausable::*;
#[allow(unused_imports)]
pub use rate_limiting::*;
#[allow(unused_imports)]
pub use storage::*;
#[allow(unused_imports)]
pub use time::*;
#[allow(unused_imports)]
pub use validation::*;
