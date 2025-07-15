//! Command Line Interface (CLI) for `neopdf`
//!
//! This crate provides a command-line interface for converting LHAPDF sets to NeoPDF format
//! and for combining multiple nuclear PDFs into a single NeoPDF file with A dependence.

mod converter;

pub use converter::main;
