//! `solana-safe-math` provides checked arithmetic and conversion traits that
//! return explicit errors instead of panicking.
//!
//! Functionality is organized into trait implementations in [`impls`] and uses
//! [`SafeMathError`] for overflow, underflow, and conversion failures.

pub mod error;
pub mod impls;

pub use error::*;
pub use impls::*;
