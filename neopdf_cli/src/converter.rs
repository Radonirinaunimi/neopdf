//! CLI logic for `NeoPDF` conversion utilities.
//!
//! This module defines the command-line interface for converting LHAPDF sets to `NeoPDF` format
//! and for combining multiple nuclear PDFs into a single `NeoPDF` file with explicit A dependence.

use clap::{Parser, Subcommand};
use neopdf::converter;

/// Command-line interface for `NeoPDF` conversion utilities.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The subcommand to run.
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands for the `NeoPDF` CLI.
#[derive(Subcommand)]
pub enum Commands {
    /// Convert a single LHAPDF set to `NeoPDF` format.
    Convert {
        /// Name of the LHAPDF set (e.g. `NNPDF40_nnlo_as_01180`)
        #[arg(short, long)]
        pdf_name: String,
        /// Output path for the `NeoPDF` file.
        #[arg(short, long)]
        output: String,
    },
    /// Combine multiple nuclear PDFs into a single `NeoPDF` with A dependence.
    Combine {
        /// List of PDF set names (each with a different A).
        #[arg(short = 'n', long = "pdf-names", required = true)]
        pdf_names: Vec<String>,
        /// Output path for the combined `NeoPDF` file.
        #[arg(short, long)]
        output: String,
    },
}

/// Entry point for the `NeoPDF` CLI.
///
/// Parses command-line arguments and dispatches to the appropriate subcommand handler.
pub fn main(cli: Cli) {
    match &cli.command {
        Commands::Convert { pdf_name, output } => {
            if let Err(err) = converter::convert_lhapdf(pdf_name, output) {
                eprintln!("Error: {err}");
                std::process::exit(1);
            }
        }
        Commands::Combine { pdf_names, output } => {
            let names: Vec<&str> = pdf_names.iter().map(std::string::String::as_str).collect();
            if let Err(err) = converter::combine_lhapdf_npdfs(&names, output) {
                eprintln!("Error: {err}");
                std::process::exit(1);
            }
        }
    }
}
