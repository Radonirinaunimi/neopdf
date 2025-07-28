//! CLI logic for reading PDF set information.

use clap::{Args, Parser, Subcommand};

/// Command-line interface for reading PDF set information.
#[derive(Parser, Clone)]
pub struct ReadCli {
    /// The subcommand to execute.
    #[command(subcommand)]
    pub command: ReadCommands,
}

/// Subcommands for reading PDF set information.
#[derive(Subcommand, Clone)]
pub enum ReadCommands {
    /// Print the metadata of a PDF set.
    #[command(name = "metadata")]
    Metadata(MetadataArgs),
    /// Print the number of subgrids in a PDF set.
    #[command(name = "num_subgrids")]
    NumSubgrids(PdfNameArgs),
    /// Print the subgrid info (nucleons, alphas, x, Q2) for a given subgrid index.
    #[command(name = "subgrid_info")]
    SubgridInfo(SubgridInfoArgs),
    /// Print the git version of the code that generated the PDF.
    #[command(name = "git-version")]
    GitVersion(PdfNameArgs),
}

/// Arguments for the metadata subcommand.
#[derive(Args, Clone)]
pub struct MetadataArgs {
    /// Name of the PDF set (LHAPDF or `NeoPDF` file)
    #[arg(short, long)]
    pub pdf_name: String,
}

/// Arguments for commands that only require a PDF name.
#[derive(Args, Clone)]
pub struct PdfNameArgs {
    /// Name of the PDF set (LHAPDF or `NeoPDF` file)
    #[arg(short, long)]
    pub pdf_name: String,
}

/// Arguments for the `subgrid_info` subcommand.
#[derive(Args, Clone)]
pub struct SubgridInfoArgs {
    /// Name of the PDF set (LHAPDF or `NeoPDF` file)
    #[arg(short, long)]
    pub pdf_name: String,
    /// Member index (0-based)
    #[arg(short, long)]
    pub member: usize,
    /// Subgrid index
    #[arg(short, long)]
    pub subgrid_index: usize,
}

/// Entry point for the read CLI.
#[allow(clippy::needless_pass_by_value)]
pub fn main(cli: ReadCli) {
    match &cli.command {
        ReadCommands::Metadata(args) => {
            let pdf = neopdf::pdf::PDF::load(&args.pdf_name, 0);
            println!("{}", pdf.metadata());
        }
        ReadCommands::NumSubgrids(args) => {
            let pdf = neopdf::pdf::PDF::load(&args.pdf_name, 0);
            println!("{}", pdf.num_subgrids());
        }
        ReadCommands::SubgridInfo(args) => {
            let pdf = neopdf::pdf::PDF::load(&args.pdf_name, args.member);
            let subgrid = pdf.subgrid(args.subgrid_index);
            println!("Nucleon Numbers A: {:?}", subgrid.nucleons);
            println!("Alphas values: {:?}", subgrid.alphas);
            println!("kT values: {:?}", subgrid.kts);
            println!("x values: {:?}", subgrid.xs);
            println!("Q2 values: {:?}", subgrid.q2s);
        }
        ReadCommands::GitVersion(args) => {
            let pdf = neopdf::pdf::PDF::load(&args.pdf_name, 0);
            println!("{}", pdf.metadata().git_version);
        }
    }
}
