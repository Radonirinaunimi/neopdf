//! CLI logic for evaluating PDF values (xfxQ2) and alpha_s(Q2) for a given set, member, and kinematics.
//!
//! This module provides subcommands for evaluating PDF values and alpha_s at specified kinematic points.

use clap::{Args, Parser, Subcommand};
use std::process;

/// Command-line interface for PDF and alpha_s evaluation.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct PdfCli {
    #[command(subcommand)]
    pub command: PdfCommands,
}

/// Subcommands for PDF and alpha_s evaluation.
#[derive(Subcommand)]
pub enum PdfCommands {
    /// Evaluate xf(x, Q2, pid, ...) for a given set, member, and input values.
    XfxQ2(XfxQ2Args),
    /// Evaluate alpha_s(Q2) for a given set, member, and Q2 value.
    AlphasQ2(AlphasQ2Args),
}

/// Arguments for the xfxQ2 subcommand.
#[derive(Args)]
pub struct XfxQ2Args {
    /// Name of the PDF set (LHAPDF or NeoPDF file)
    #[arg(short, long)]
    pub pdf_name: String,
    /// Member index (0-based)
    #[arg(short, long)]
    pub member: usize,
    /// PDG flavor ID
    #[arg(short = 'i', long)]
    pub pid: i32,
    /// Input values (e.g. x Q2 or A x Q2, ...)
    #[arg(required = true)]
    pub inputs: Vec<f64>,
}

/// Arguments for the alphas_q2 subcommand.
#[derive(Args)]
pub struct AlphasQ2Args {
    /// Name of the PDF set (LHAPDF or NeoPDF file)
    #[arg(short, long)]
    pub pdf_name: String,
    /// Member index (0-based)
    #[arg(short, long)]
    pub member: usize,
    /// Q^2 value
    #[arg(short, long)]
    pub q2: f64,
}

/// Entry point for the pdf CLI.
pub fn main() {
    let cli = PdfCli::parse();
    match &cli.command {
        PdfCommands::XfxQ2(args) => {
            let pdf = neopdf::pdf::PDF::load(&args.pdf_name, args.member);
            if args.inputs.len() < 2 {
                eprintln!("Error: At least [x, Q2] must be provided as input.");
                process::exit(1);
            }
            let val = pdf.xfxq2(args.pid, &args.inputs);
            println!("{val}");
        }
        PdfCommands::AlphasQ2(args) => {
            let pdf = neopdf::pdf::PDF::load(&args.pdf_name, args.member);
            let val = pdf.alphas_q2(args.q2);
            println!("{val}");
        }
    }
}
