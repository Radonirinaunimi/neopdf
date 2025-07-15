//! CLI logic for `NeoPDF` conversion utilities.
//!
//! This module defines the command-line interface for converting LHAPDF sets to `NeoPDF` format
//! and for combining multiple nuclear PDFs into a single `NeoPDF` file with explicit A dependence.

use clap::{Parser, Subcommand};
use neopdf::converter;
use std::fs::File;
use std::io::{self, BufRead};

/// Command-line interface for `NeoPDF` conversion utilities.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// TODO
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
        #[arg(
            short = 'n',
            long = "pdf-names",
            required_unless_present = "names_file"
        )]
        pdf_names: Option<Vec<String>>,
        /// Path to a file containing PDF set names, one per line.
        #[arg(short = 'f', long = "names-file", conflicts_with = "pdf_names")]
        names_file: Option<String>,
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
        Commands::Combine {
            pdf_names,
            names_file,
            output,
        } => {
            let names: Vec<String> = if let Some(file_path) = names_file {
                let file = File::open(file_path).expect("Could not open names file");
                io::BufReader::new(file)
                    .lines()
                    .filter_map(|line| line.ok())
                    .collect()
            } else if let Some(names_vec) = pdf_names {
                names_vec.to_vec()
            } else {
                eprintln!("Error: Either --pdf-names or --names-file must be provided.");
                std::process::exit(1);
            };
            let names_str: Vec<&str> = names.iter().map(std::string::String::as_str).collect();
            if let Err(err) = converter::combine_lhapdf_npdfs(&names_str, output) {
                eprintln!("Error: {err}");
                std::process::exit(1);
            }
        }
    }
}
