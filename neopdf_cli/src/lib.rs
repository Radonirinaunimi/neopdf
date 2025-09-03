//! Command Line Interface (CLI) for `neopdf`
//!
//! This crate provides a command-line interface for converting LHAPDF sets to `NeoPDF` format,
//! combining nuclear PDFs, and evaluating PDF values and `alpha_s` at given kinematics.

pub mod converter;
pub mod install;
pub mod pdf;
pub mod read;
#[cfg(feature = "tmdlib")]
pub mod tmd_converter;
