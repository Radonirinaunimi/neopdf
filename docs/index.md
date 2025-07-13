# NeoPDF

`NeoPDF` is a fast, reliable, and scalable interpolation library for Parton Distribution Functions
(PDFs), designed for both present and future hadron collider experiments. It aims to be a modern,
high-performance alternative to LHAPDF, focusing on:

- **Performance**: Written in Rust for speed and safety, with zero-cost abstractions and efficient
    memory management.
- **Flexibility**: Supports multiple interpolation strategies and is easily extensible. The
    abstraction of the interpolation crate makes it easier and efficient to implement custom
    interpolation methods.
- **Multi-language Support**: Native Rust API, with bindings for Python, Fortran, C, and C++.
- **Safety**: Thread-safe and memory-safe by design, leveraging Rust's guarantees.

## Motivation

The need for a fast and reliable PDF interpolation is critical in high-energy physics, especially
for precision calculations at hadron colliders. Existing solutions like LHAPDF, while widely used,
have limitations in terms of extensibility and features. `NeoPDF` addresses these by:

- Providing a modern, modular codebase.
- Enabling easy integration into new and existing workflows.
- Supporting advanced features such as multi-dimensional interpolations.

## High-Level Architecture

- **Core Library (Rust)**: Implements all interpolation logic, grid management, and PDF metadata handling.
- **FFI Bindings**: Exposes the core functionality to Python, Fortran, C, and C++.
- **CLI Tools**: (Planned) Command-line utilities for PDF inspection and conversion.

```mermaid
graph TD
    A[User Code (Rust/Python/C/C++)] --> B[NeoPDF API];
    B --> C[Core Interpolation Engine (Rust)];
    C --> D[PDF Grid Data & Metadata];
```

## Source Code & Bug Report

- [GitHub Repository](https://github.com/radonirinaunimi/neopdf)
- Issues and feature requests welcome!
