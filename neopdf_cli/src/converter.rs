//! CLI logic for `NeoPDF` conversion utilities.
//!
//! This module defines the command-line interface for converting LHAPDF sets to `NeoPDF` format
//! and for combining multiple nuclear PDFs into a single `NeoPDF` file with explicit A dependence.

use clap::{Parser, Subcommand};
use neopdf::converter;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

// Loads PDF names from either command line arguments or a file.
///
/// This function handles the mutually exclusive options for providing PDF names:
/// either directly via command line arguments or from a file containing one name per line.
/// Exactly one of the two options must be provided.
///
/// # Arguments
///
/// * `pdf_names` - Optional slice of PDF names provided directly via command line
/// * `names_file` - Optional path to a file containing PDF names (one per line)
///
/// # Returns
///
/// * `Ok(Vec<String>)` - Vector of PDF names loaded successfully
/// * `Err(Box<dyn std::error::Error>)` - If:
///   - Both options are provided or both are None
///   - File cannot be opened or read
///   - I/O error occurs while reading the file
fn load_pdf_names(
    pdf_names: Option<&[String]>,
    names_file: Option<&str>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    match (pdf_names, names_file) {
        (Some(names), None) => Ok(names.to_vec()),
        (None, Some(file_path)) => {
            let file = File::open(file_path)?;
            BufReader::new(file)
                .lines()
                .collect::<Result<Vec<_>, _>>()
                .map_err(Into::into)
        }
        _ => Err("Either --pdf-names or --names-file must be provided.".into()),
    }
}

/// Executes the CLI command based on the parsed arguments.
///
/// This function handles the main application logic for both Convert and Combine commands.
/// It delegates to the appropriate converter functions and propagates any errors that occur.
///
/// # Arguments
///
/// * `cli` - The parsed command line interface structure containing the command and its arguments
///
/// # Returns
///
/// * `Ok(())` - If the command executed successfully
/// * `Err(Box<dyn std::error::Error>)` - If any error occurred during execution
pub fn run_cli(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match &cli.command {
        Commands::Convert { pdf_name, output } => {
            converter::convert_lhapdf(pdf_name, output)?;
        }
        Commands::Combine {
            pdf_names,
            names_file,
            output,
        } => {
            let names = load_pdf_names(pdf_names.as_deref(), names_file.as_deref())?;
            let names_str: Vec<&str> = names.iter().map(String::as_str).collect();
            converter::combine_lhapdf_npdfs(&names_str, output)?;
        }
    }
    Ok(())
}

/// Entry point for the `NeoPDF` CLI.
///
/// Parses command-line arguments and dispatches to the appropriate subcommand handler.
pub fn main(cli: Cli) {
    if let Err(err) = run_cli(cli) {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
