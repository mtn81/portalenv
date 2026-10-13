//! **PortalEnv** is a Rust-friendly, ergonomic and type-safe environment configuration library.
//!

pub mod docs;

pub use portalenv_core::*;
pub use portalenv_macro::*;

#[cfg(doctest)]
#[doc = include_str!("../../../README.md")]
pub struct ReadmeDoctests;
