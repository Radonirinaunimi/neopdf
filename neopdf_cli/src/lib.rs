//! Command Line Interface (CLI) for `neopdf`
//!
//! This crate provides a command-line interface for converting LHAPDF sets to `NeoPDF` format,
//! combining nuclear PDFs, and evaluating PDF values and `alpha_s` at given kinematics.

mod converter;
mod pdf;

use clap::{Parser, Subcommand};

/// Top-level CLI for `NeoPDF`, supporting conversion and evaluation subcommands.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// TODO
    #[command(subcommand)]
    pub command: TopLevelCommand,
}

/// Top-level subcommands for the `NeoPDF` CLI.
#[derive(Subcommand)]
pub enum TopLevelCommand {
    /// Conversion and combination of PDF sets
    Write(converter::Cli),
    /// Evaluate PDF values and `alpha_s` at given kinematics
    Compute(pdf::PdfCli),
}

/// Entry point for the `NeoPDF` CLI.
///
/// Dispatches to the appropriate subcommand handler.
pub fn main() {
    let cli = Cli::parse();
    match cli.command {
        TopLevelCommand::Write(args) => converter::main(args),
        TopLevelCommand::Compute(args) => pdf::main(args),
    }
}
