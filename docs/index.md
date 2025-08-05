# NeoPDF

![Codecov](https://img.shields.io/codecov/c/github/Radonirinaunimi/neopdf?style=flat-square&logo=codecov&logoColor=red&color=blue)
![MSRV](https://img.shields.io/crates/msrv/neopdf?style=flat-square&logo=rust&color=red)
![Crates.io](https://img.shields.io/crates/v/neopdf?style=flat-square&logo=rust&color=blue)
![PyPI - Python Version](https://img.shields.io/pypi/pyversions/neopdf-hep?style=flat-square&logo=python)
![PyPI - Version](https://img.shields.io/pypi/v/neopdf-hep?style=flat-square&logo=python&logoColor=yellow&color=%1d881d)
![GitHub License](https://img.shields.io/github/license/Radonirinaunimi/neopdf?style=flat-square&logo=gplv3&logoColor=red)
![GitHub repo size](https://img.shields.io/github/repo-size/Radonirinaunimi/neopdf?style=flat-square&logo=github)

`NeoPDF` is a fast, reliable, and scalable interpolation library for both **Collinear** Parton
Distribution Functions (PDFs) and **Transverse Momentum Dependent** Distributions (TMDs) with
**modern features**, designed for both present and future hadron collider experiments. It aims
to be a modern, high-performance alternative to both [LHAPDF](https://www.lhapdf.org/) and
[TMDlib](https://tmdlib.hepforge.org/), focusing on:

<div class="feature-grid">
  <div class="feature-card">
      <strong>🚀 Performance</strong>
      <p>Written in Rust 🦀 for speed and safety, with zero-cost abstractions and efficient memory management.</p>
  </div>
  <div class="feature-card">
      <strong>🧩 Flexibility</strong>
      <p>Supports multiple interpolation strategies: allows for an efficient implementation of custom interpolations.</p>
  </div>
  <div class="feature-card">
      <strong>🌐 Multi-Language Support</strong>
      <p>Native Rust 🦀 API, with bindings for various programming languages such as Python 🐍, Fortran, C, and C++.</p>
  </div>
  <div class="feature-card">
      <strong>📊 (Physics) Features & Extensibility</strong>
      <p>Very extensible: makes it easy to introduce new (Physics) features without introducing <b>technical debts</b>.</p>
  </div>
</div>

## Motivation

The need for a fast and reliable PDF interpolation is critical in high-energy physics, especially
for precision calculations at hadron colliders. Existing solutions like LHAPDF or TMDlib, while
widely used, have limitations in terms of extensibility and features. `NeoPDF` addresses these by:

- Providing a modern, modular codebase.
- Enabling easy integration into new and existing workflows.
- Supporting advanced features such as multi-dimensional interpolations.

## High-Level Architecture

- **Core Library (Rust)**: Implements all the interpolation logics, grid management, and PDF
    metadata handling.
- **FFI Bindings**: Exposes the core functionalities to Python, Fortran, C, and C++, enabling
    easier interoperability with other codes that can link to these programming languages.
- **CLI Tools**: Command-line utilities that allow users to inspect the contents of a gird,
    convert LHAPDF/TMDlib format into `NeoPDF`, and perform interpolations.

## Getting Started

- **[Installation Guide](./installation.md)**: Complete installation instructions for all platforms and APIs
- **[Development with Pixi](./development-with-pixi.md)**: Comprehensive guide for developers using Pixi for environment management
- **[CLI Tutorials](./cli-tutorials.md)**: Learn how to use the command-line interface
- **[Examples](./examples/)**: Code examples for Python, C++, C, and Fortran APIs

## Source Code & Bug Report

- [GitHub Repository](https://github.com/radonirinaunimi/neopdf)
- Issues and feature requests welcome!
