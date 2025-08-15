//! Launches the `NeoPDF` command-line interface (CLI).

use clap::{Parser, Subcommand};

use neopdf_cli::converter;
use neopdf_cli::install;
use neopdf_cli::pdf;
use neopdf_cli::read;

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
    /// Commands for reading PDF set information.
    Read(read::ReadCli),
    /// Install a PDF set from one of the supported repositories.
    Install(install::Cli),
}

/// Entry point for the `NeoPDF` CLI.
///
/// Dispatches to the appropriate subcommand handler.
pub fn main() {
    let cli = Cli::parse();
    match cli.command {
        TopLevelCommand::Write(args) => converter::main(args),
        TopLevelCommand::Compute(args) => pdf::main(args),
        TopLevelCommand::Read(args) => read::main(args),
        TopLevelCommand::Install(args) => install::main(args),
    }
}
