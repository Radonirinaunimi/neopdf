#![allow(missing_docs)]

use assert_cmd::Command;
use predicates::str;
use std::io::Write;

const HELP_STR: &str = "Conversion and combination of PDF sets

Usage: neopdf write <COMMAND>

Commands:
  convert  Convert a single LHAPDF set to `NeoPDF` format
  combine  Combine multiple nuclear PDFs into a single `NeoPDF` with A dependence
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
";

#[test]
fn help() {
    Command::cargo_bin("neopdf")
        .unwrap()
        .args(["write", "--help"])
        .assert()
        .success()
        .stdout(HELP_STR);
}

#[test]
fn convert_lhapdf() {
    let output = assert_fs::NamedTempFile::new("nnpdf40.neopdf.lz4").unwrap();

    Command::cargo_bin("neopdf")
        .unwrap()
        .args([
            "write",
            "convert",
            "--pdf-name",
            "NNPDF40_nnlo_as_01180",
            "--output",
            output.path().to_str().unwrap(),
        ])
        .assert()
        .success();

    Command::cargo_bin("neopdf")
        .unwrap()
        .args([
            "compute",
            "xfx_q2",
            "--pdf-name",
            output.path().to_str().unwrap(),
            "--member",
            "0",
            "--pid",
            "21",
            "1e-3",
            "10.0",
        ])
        .assert()
        .success()
        .stdout("7.1276606679158565\n");
}

#[test]
fn combine_nuclear_pdfs() {
    let output = assert_fs::NamedTempFile::new("nnnpdf30.neopdf.lz4").unwrap();
    let npdfs_list = [
        "nNNPDF30_nlo_as_0118_p",
        "nNNPDF30_nlo_as_0118_A2_Z1",
        "nNNPDF30_nlo_as_0118_A4_Z2",
        "nNNPDF30_nlo_as_0118_A6_Z3",
        "nNNPDF30_nlo_as_0118_A9_Z4",
    ];
    let npdfs_str = npdfs_list.join("\n");
    let mut temp_file = tempfile::NamedTempFile::new().unwrap();
    write!(temp_file, "{npdfs_str}").unwrap();

    Command::cargo_bin("neopdf")
        .unwrap()
        .args([
            "write",
            "combine",
            "--names-file",
            temp_file.path().to_str().unwrap(),
            "--output",
            output.path().to_str().unwrap(),
        ])
        .assert()
        .success();

    Command::cargo_bin("neopdf")
        .unwrap()
        .args([
            "compute",
            "xfx_q2",
            "--pdf-name",
            output.path().to_str().unwrap(),
            "--member",
            "10",
            "--pid",
            "21",
            "4",
            "1e-5",
            "50",
        ])
        .assert()
        .success()
        .stdout("63.389564472386645\n");
}
