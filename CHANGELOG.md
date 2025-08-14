# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added a new module `alphas.rs` to store the logics of computing the strong
  coupling `alpha_s`. It contains a new struct `AlphaSAnalytic` to compute the
  `alpha_s` values analytically instead of interpolating.
- Added Chebyshev interpolation strategy for 1D, 2D, and 3D data.
- Added `pdf:mkpdfs_lazy` that loads the PDF members lazily and propagated the
  methods into the Python, C/C++, and Fortran APIs.
- Added `gridpdf::ForcePositive` enum to set the clipping method to negative
  interpolated values.
- Python API: Added `pdf:LoaderMehod` to select the method to load all the PDF
  members.

### Changed

- Modified `GridArray::pid_index` to accept both `0` and `21` for the Gluon.
- Modified the NeoPDF format with the inclusion of `alphas_type` and
  `number_flavors` in the `MetaData` struct. This breaks the lazy loader using
  the `LazyGridArrayIterator` struct.

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
