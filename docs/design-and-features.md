# Design & Features

NeoPDF is designed to be a modern, extensible, and high-performance library for PDF interpolation.
This page details the physics and technical features, design rationale, and future plans.

## Summary of the Current Supported Features

| Feature                        | Status      | Notes                                  |
|------------------------------- |-------------|----------------------------------------|
| Rust API                       | ✅          | Fully supported                        |
| Python API                     | ✅          | Fully supported                        |
| C/C++ API                      | ✅          | Fully supported                        |
| Fortran API                    | ❌          | Not yet implemented                    |
| Multi-flavor grids             | ❌          | Planned                                |
| Nuclear PDFs interpolation     | ✅          | Fully supported                        |
| Strong Coupling interpolation  | ✅          | Fully supported                        |
| Different Hadronic states      | ✅          | Fully supported                        |
| Custom interpolation           | ✅          | Supported (user-defined strategies)    |
| Analytical DGLAP interpolation | ❌          | Planned                                |

## Physics Features

### Hadron Types and PDF Classification

NeoPDF provides comprehensive support for different types of hadronic structure functions and most
importantly distinguish between them, which is essential for precision QCD calculations.

- **Parton vs Nuclear PDFs**:

    * **Parton PDFs** describe the momentum distribution of quarks and gluons within protons and
      neutrons, fundamental for understanding the internal structure of hadrons. These are crucial
      for Standard Model predictions at hadron colliders like the LHC.
    * **Nuclear PDFs** extend this framework to describe parton distributions within nuclei, accounting
      for nuclear binding effects, shadowing, and anti-shadowing. These are essential for heavy-ion
      collisions and understanding nuclear structure effects in high-energy physics experiments.

- **Polarized vs Unpolarized PDFs**:

    * **Unpolarized PDFs** represent the standard momentum distributions and are used in most collider
      physics calculations.
    * **Polarized PDFs** describe the spin-dependent parton distributions, crucial for understanding the
      proton's spin structure and for experiments with polarized beams. These are essential for the RHIC
      spin program and future electron-ion colliders (EIC).

    The difference between polarized and unpolarized PDFs provides direct insight into the proton's spin
    decomposition and tests of QCD in the spin sector.

- **Timelike vs Spacelike PDFs**:

    - **Spacelike PDFs** (the standard case) describe parton distributions in deep-inelastic scattering
      and hadron-hadron collisions.
    - **Timelike PDFs** (Fragmentation Functions) describe the hadronization of partons into hadrons,
      essential for understanding jet structure and hadron production in $e^+e^-$ collisions and hadron-hadron
      collisions.

    This distinction is crucial for precision phenomenology, as the evolution equations and factorization
    theorems differ between the two cases.

The [PineAPPL](https://github.com/NNPDF/pineappl) fast interpolation supports grids with arbitrarily many
convolutions. For instance, it support processes such as:

```bash
proton + proton -> Pion + Pion + X
```

An interpolation grid of this kind needs two different convolution functions: a (polarised) PDF for the
protons and a fragmentation function for the pions. When users convolve this grid with the two functions,
they must either pass the functions in the right order to avoid calculating wrong predictions. NeoPDF
circumvents this issue by adding the following keys to the metadata:

``` yaml
HadronPID: 2212/212/...
Polarized: true/false
SetType: PDf/Fragfn
```

### Multi-Parameter Interpolations

NeoPDF supports interpolation across multiple physical parameters:

- **$\alpha_s(M_Z)$ Dependence**:
  The strong coupling constant $\alpha_s(M_Z)$ is a fundamental parameter of QCD that affects PDF
  evolution and cross-section predictions. Different PDF sets use different values (typically
  ranging from 0.116 to 0.120), and interpolating between them allows for:

    * Uncertainty quantification in $\alpha_s$ determination
    * Consistent treatment of $\alpha_s$ variations in global fits
    * Testing the sensitivity of observables to the strong coupling constant

- **Nuclear Dependence $(A, Z)$**:
  Nuclear PDFs depend on the atomic mass number $A$ and atomic number $Z$ of the target nucleus.
  Interpolating in $(A, Z)$ space enables:

    * Predictions for nuclei not included in existing sets
    * Systematic studies of nuclear effects across the periodic table
    * Applications to heavy-ion physics and neutrino-nucleus scattering

### Multi-Flavor Grids (Planned)

NeoPDF will support grids with varying numbers of active flavors $n_f$, providing a consistent
treatment of heavy quark effects across all scales. Advanced schemes like FONLL and ACOT require
careful handling of flavor thresholds and mass effects. NeoPDF's multi-flavor support will enable:

  - Precision predictions for heavy flavor production
  - Proper matching across flavor thresholds
  - Consistent treatment of charm and bottom quark effects

### Analytical Interpolation (Planned)

Future versions will support **DGLAP**-based analytical interpolation. Such an analytical-based
interpolation will provide:

  - Consistent treatment of scale evolution
  - Reduced interpolation artifacts
  - More accurate extrapolation beyond the grid boundaries

## Technical Features

### Language Interoperability

NeoPDF provides native APIs across multiple programming languages, enabling seamless integration
into diverse computational workflows:

- **Native Rust API**:
  The core library is written in Rust, providing zero-cost abstractions, memory safety, and high
  performance. Rust's ownership system ensures thread safety and prevents common programming errors
  that could lead to incorrect physics results.

- **Python Bindings**:
  Comprehensive Python interface using PyO3, enabling integration with the rich ecosystem of
  scientific Python libraries (NumPy, SciPy, Matplotlib, etc.). This is crucial for data analysis,
  visualization, and integration with existing physics analysis frameworks.

- **C/C++ Bindings**:
  Direct C and C++ interfaces for integration with legacy codes and high-performance computing
  applications. The C API provides a stable ABI for long-term compatibility, while the C++ API
  offers object-oriented convenience.

### No-Code Migration

NeoPDF maintains API compatibility with LHAPDF, enabling seamless migration:

- **Drop-in Replacement**:
  Existing LHAPDF code can often be migrated by simply changing import statements, with no
  modifications to the core physics logic required.

- **Preserved Function Signatures**:
  Key functions like `xfxQ2()`, `alphasQ2()`, and `mkPDF()` maintain the same signatures as
  LHAPDF, ensuring compatibility with existing analysis codes.

This compatibility is crucial for the physics community, as it allows for immediate adoption
without requiring extensive code rewrites or validation efforts.

### Thread and Memory Safety

NeoPDF leverages Rust's safety guarantees for robust multi-threaded applications:

- **Memory Safety**:
  Rust's ownership system prevents common memory errors (use-after-free, double-free, data races)
  that could lead to incorrect physics results or program crashes.

- **Thread Safety**:
  Built-in support for safe concurrent access to PDF objects, essential for parallel event
  generation and Monte Carlo simulations.

- **FFI Safety**:
  Careful design of the foreign function interface ensures that safety guarantees extend to Python,
  C, and C++ code, preventing crashes and undefined behavior.

### Extensible Interpolation

NeoPDF's modular architecture enables easy extension and customization:

- **Pluggable Interpolation Strategies**:
  The library supports multiple interpolation algorithms (bilinear, bicubic, tricubic, N-dimensional)
  and can be extended with custom interpolation schemes.

- **Custom Grid Types**:
  The framework can accommodate new grid formats and data structures, enabling support for emerging
  PDF sets and specialized applications.

- **Performance Tuning**:
  Different interpolation strategies can be selected based on the specific requirements of accuracy
  vs. speed, allowing optimization for different use cases.

### Performance Optimization

NeoPDF is designed for high-performance computing environments:

- **Zero-Cost Abstractions**:
  Rust's compilation model ensures that high-level abstractions don't incur runtime overhead, providing
  both safety and performance.

- **Cache-Friendly Design**:
  Data structures are optimized for modern CPU cache hierarchies, reducing memory access latency and
  improving performance for large-scale calculations.

- **SIMD Optimization**:
  The library can leverage CPU vector instructions for parallel evaluation of multiple points, crucial
  for Monte Carlo event generation and large-scale simulations.

- **Benchmarking**:
  Comprehensive benchmarking against LHAPDF ensures that performance improvements don't come at the
  cost of accuracy, maintaining the precision required for physics calculations.

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

## Benchmark Against LHAPDF

NeoPDF implements (log)-bicubic interpolation by default, with optional $N$-dimensional strategies. Lower-dimensional (bilinear, (log)-tricubic) are also available for performance tuning.

> **Precision**: The difference between NeoPDF and LHAPDF, using the default interpolation, is **below machine precision** for floating-point numbers.

![diff_NNPDF40_nnlo_as_01180_flav21](https://github.com/user-attachments/assets/d47bfa13-9930-4247-89fb-f2c2eab68bd7)
