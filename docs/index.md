# NeoPDF

NeoPDF is a fast, reliable, and scalable interpolation library for Parton Distribution Functions (PDFs), designed for both present and future hadron collider experiments. It aims to be a modern, high-performance replacement for LHAPDF, focusing on:

- **Performance**: Written in Rust for speed and safety, with zero-cost abstractions and efficient memory management.
- **Flexibility**: Supports multiple interpolation strategies, multi-flavor grids, and is easily extensible.
- **Multi-language Support**: Native Rust API, with bindings for Python, C, and C++.
- **Safety**: Thread-safe and memory-safe by design, leveraging Rust's guarantees.

## Motivation

The need for fast and reliable PDF interpolation is critical in high-energy physics, especially for precision calculations at hadron colliders. Existing solutions like LHAPDF, while widely used, have limitations in terms of performance, extensibility, and safety. NeoPDF addresses these by:

- Providing a modern, modular codebase.
- Enabling easy integration into new and existing workflows.
- Supporting advanced features such as multi-dimensional interpolation and future extensions (e.g., nuclear PDFs, polarized PDFs).

## Project Goals

- **Replace LHAPDF** in both performance-critical and research environments.
- **Enable new physics** by supporting advanced PDF features and custom interpolation schemes.
- **Foster community contributions** with clear documentation and a robust codebase.

## High-Level Architecture

- **Core Library (Rust)**: Implements all interpolation logic, grid management, and PDF metadata handling.
- **FFI Bindings**: Exposes the core functionality to Python, C, and C++ via safe interfaces.
- **CLI Tools**: (Planned) Command-line utilities for PDF inspection and conversion.

```mermaid
graph TD;
    A[User Code (Rust/Python/C/C++)] --> B[NeoPDF API];
    B --> C[Core Interpolation Engine (Rust)];
    C --> D[PDF Grid Data & Metadata];
```

## Use Cases

- High-precision QCD calculations at the LHC and future colliders
- Fast PDF evaluation in Monte Carlo event generators
- Research and development of new PDF sets and interpolation strategies
- Educational tools for particle physics

## Source Code & Community

- [GitHub Repository](https://github.com/radonirinaunimi/neopdf)
- Issues and feature requests welcome!
