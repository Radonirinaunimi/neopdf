//! CLI logic for NeoPDF conversion utilities.
//!
//! This module defines the command-line interface for converting LHAPDF sets to NeoPDF format
//! and for combining multiple nuclear PDFs into a single NeoPDF file with explicit A dependence.

use clap::{Parser, Subcommand};
use std::process;

/// Command-line interface for NeoPDF conversion utilities.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The subcommand to run.
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands for the NeoPDF CLI.
#[derive(Subcommand)]
pub enum Commands {
    /// Convert a single LHAPDF set to NeoPDF format.
    Convert {
        /// Name of the LHAPDF set (e.g. NNPDF40_nnlo_as_01180)
        #[arg(short, long)]
        pdf_name: String,
        /// Output path for the NeoPDF file.
        #[arg(short, long)]
        output: String,
    },
    /// Combine multiple nuclear PDFs into a single NeoPDF with A dependence.
    Combine {
        /// List of PDF set names (each with a different A).
        #[arg(short = 'n', long = "names", required = true)]
        pdf_names: Vec<String>,
        /// Output path for the combined NeoPDF file.
        #[arg(short, long)]
        output: String,
    },
}

/// Entry point for the NeoPDF CLI.
///
/// Parses command-line arguments and dispatches to the appropriate subcommand handler.
pub fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Convert { pdf_name, output } => {
            if let Err(e) = neopdf::converter::convert_lhapdf_to_neopdf(pdf_name, &output) {
                eprintln!("Error: {e}");
                process::exit(1);
            }
        }
        Commands::Combine { pdf_names, output } => {
            let names: Vec<&str> = pdf_names.iter().map(|s| s.as_str()).collect();
            if let Err(e) =
                neopdf::converter::combine_nuclear_pdfs_with_a_dependence(&names, &output)
            {
                eprintln!("Error: {e}");
                process::exit(1);
            }
        }
    }
}
