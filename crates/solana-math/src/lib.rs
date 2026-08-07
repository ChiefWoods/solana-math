//! `solana-math` provides checked arithmetic and conversion traits that
//! return explicit errors instead of panicking.
//!
//! Functionality is organized into trait implementations in [`impls`] and uses
//! [`SafeMathError`] for overflow, underflow, and conversion failures.

#![cfg_attr(not(any(test, feature = "std")), no_std)]

mod debug_log;
pub mod error;
pub mod impls;

pub use error::*;
pub use impls::*;
