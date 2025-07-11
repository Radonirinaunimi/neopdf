# Design & Features

NeoPDF is designed to be a modern, extensible, and high-performance library for PDF interpolation. This page details the physics and technical features, design rationale, and future plans.

## Physics Features

- **Hadron Types**: Differentiates between parton and nuclear PDFs, supports polarized and unpolarized, timelike and spacelike distributions.
- **Multi-Parameter Support**: (Planned) Interpolations for PDFs with different $\alpha_s(M_Z)$ and nuclear dependence $(A, Z)$.
- **Multi-Flavor Grids**: Supports grids with varying $n_f$ (important for schemes like FONLL).
- **Analytical Interpolation**: (Planned) Support for DGLAP-based analytical interpolation.

## Technical Features

- **Language Interoperability**: Native Rust API, with safe and idiomatic bindings for Python, C, and C++.
- **No-Code Migration**: Minimal code changes required to switch from LHAPDF in Python, C, or C++.
- **Thread & Memory Safety**: Rust guarantees, plus careful FFI design for safe multi-threaded use.
- **Extensible Interpolation**: Easily add new interpolation strategies or grid types.
- **Performance**: Optimized for both speed and accuracy, with benchmarks against LHAPDF.

## Architecture Overview

```mermaid
graph TD;
    A[User (Rust/Python/C/C++)] --> B[API Layer];
    B --> C[Core Engine (Rust)];
    C --> D[Grid Data & Metadata];
    C --> E[Interpolation Strategies];
    D --> F[PDF Set Files];
```

- **API Layer**: Exposes a consistent interface in each language.
- **Core Engine**: Handles all logic, grid management, and interpolation.
- **Grid Data**: Efficiently loads and manages PDF grid data and metadata.
- **Interpolation Strategies**: Pluggable, with default (log)-bicubic, bilinear, and (log)-tricubic.

## Feature Support Table

| Feature                        | Status      | Notes                                  |
|------------------------------- |------------|----------------------------------------|
| Rust API                       | ✅          | Fully supported                        |
| Python API                     | ✅          | Fully supported                        |
| C/C++ API                      | ✅          | Fully supported                        |
| Multi-flavor grids             | ✅          |                                        |
| Nuclear PDFs                   | 🚧         | Planned                                |
| Polarized PDFs                 | 🚧         | Planned                                |
| Custom interpolation           | 🚧         | Planned (user-defined strategies)      |
| Analytical DGLAP interpolation | 🚧         | Planned                                |

## Benchmark Against LHAPDF

NeoPDF implements (log)-bicubic interpolation by default, with optional $N$-dimensional strategies. Lower-dimensional (bilinear, (log)-tricubic) are also available for performance tuning.

> **Precision**: The difference between NeoPDF and LHAPDF, using the default interpolation, is **below machine precision** for floating-point numbers.

![diff_NNPDF40_nnlo_as_01180_flav21](https://github.com/user-attachments/assets/d47bfa13-9930-4247-89fb-f2c2eab68bd7)

## Extensibility & Future Plans

- Add support for nuclear and polarized PDFs
- Enable user-defined/custom interpolation strategies
- Provide CLI tools for PDF inspection and conversion
- Expand documentation and add more usage examples
