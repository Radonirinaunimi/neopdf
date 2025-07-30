# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 30/07/2025

### Added

- Initial implementation of the `neopdf` crate for collinear and transverse
  momentum dependent Parton Distribution Functions (PDFs) interpolation. This
  includes various features such as: interpolation logic for both collinear
  and TMD PDFs with support for interpolation of the nucleon numbers `A` and
  the strong coupling; reading and writing PDF grid files in the NeoPDF format.
- Python bindings via the `neopdf_pyapi` crate.
- C API interface via the `neopdf_capi` crate for C/C++ interoperability.
- Fortran interface via the `neopdf_fapi` crate for Fortran integration.
- Command line interface via the `neopdf_cli` crate for PDF manipulation
  and inspection from the terminal.
- Comprehensive documentation and usage examples for all interfaces.
