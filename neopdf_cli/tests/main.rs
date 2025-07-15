#![allow(missing_docs)]

use assert_cmd::Command;
use predicates::str;

const HELP_STR: &str = "CLI interface to NeoPDF

Usage: neopdf <COMMAND>

Commands:
  write    Conversion and combination of PDF sets
  compute  Evaluate PDF values and `alpha_s` at given kinematics
  read     Commands for reading PDF set information
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
";

#[test]
fn help() {
    Command::cargo_bin("neopdf")
        .unwrap()
        .args(["--help"])
        .assert()
        .success()
        .stdout(HELP_STR);
}
