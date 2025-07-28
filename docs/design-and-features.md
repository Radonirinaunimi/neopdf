# Features

`NeoPDF` is designed to be a modern, extensible, and high-performance library for PDF/TMD
interpolation. This page details the physics and technical features, [design rationale](./design.md),
and future plans.

## Summary of the Current Supported Features

<div align="center">
<table style="border-collapse: collapse; width: 100%; border: 1px solid #888;">
  <tr>
    <th style="border: 1px solid #888;"></th>
    <th style="border: 1px solid #888;">Feature</th>
    <th style="border: 1px solid #888;">Status</th>
    <th style="border: 1px solid #888;">Notes</th>
  </tr>
  <tr>
    <td rowspan="4" style="text-align: center; vertical-align: middle; border: 1px solid #888;">APIs & FFIs</td>
    <td style="border: 1px solid #888;">Rust API</td>
    <td style="border: 1px solid #888;">✅</td>
    <td style="border: 1px solid #888;">Fully supported</td>
  </tr>
  <tr>
    <td style="border: 1px solid #888;">Python API</td>
    <td style="border: 1px solid #888;">✅</td>
    <td style="border: 1px solid #888;">Fully supported</td>
  </tr>
  <tr>
    <td style="border: 1px solid #888;">C/C++ API</td>
    <td style="border: 1px solid #888;">✅</td>
    <td style="border: 1px solid #888;">Fully supported</td>
  </tr>
  <tr>
    <td style="border: 1px solid #888;">Fortran API</td>
    <td style="border: 1px solid #888;">❌</td>
    <td style="border: 1px solid #888;">Not yet implemented</td>
  </tr>
  <tr>
    <td rowspan="7" style="text-align: center; vertical-align: middle; border: 1px solid #888;">Features</td>
    <td style="border: 1px solid #888;">Multi-flavor grids</td>
    <td style="border: 1px solid #888;">❌</td>
    <td style="border: 1px solid #888;">Planned</td>
  </tr>
  <tr>
    <td style="border: 1px solid #888;">Nuclear PDFs interpolation</td>
    <td style="border: 1px solid #888;">✅</td>
    <td style="border: 1px solid #888;">Fully supported</td>
  </tr>
  <tr>
    <td style="border: 1px solid #888;">Strong Coupling interpolation</td>
    <td style="border: 1px solid #888;">✅</td>
    <td style="border: 1px solid #888;">Fully supported</td>
  </tr>
  <tr>
    <td style="border: 1px solid #888;">Momentum kT interpolation</td>
    <td style="border: 1px solid #888;">✅</td>
    <td style="border: 1px solid #888;">Fully supported</td>
  </tr>
  <tr>
    <td style="border: 1px solid #888;">Different Hadronic states</td>
    <td style="border: 1px solid #888;">✅</td>
    <td style="border: 1px solid #888;">Fully supported</td>
  </tr>
  <tr>
    <td style="border: 1px solid #888;">Custom interpolation</td>
    <td style="border: 1px solid #888;">✅</td>
    <td style="border: 1px solid #888;">Supported (user-defined strategies)</td>
  </tr>
  <tr>
    <td style="border: 1px solid #888;">Analytical DGLAP interpolation</td>
    <td style="border: 1px solid #888;">❌</td>
    <td style="border: 1px solid #888;">Planned</td>
  </tr>
</table>
</div>

## Physics Features

### Hadron Types and PDF Classification

!!! danger "Multiple types of Convolutions"

    PDF interpolations are mainly used to convolve with partonic cross-sections in order to
    get theoretical predictions. For various technical reasons, these theory predictions are
    stored in some fast interpolating grids. Mondern interpolating libraries such as
    [PineAPPL](https://github.com/NNPDF/pineappl) supports grids with arbitrarily many
    convolutions. For instance, it support processes such as:

    ```bash
    Proton + Proton -> π + π + X
    ```

    An interpolation grid of this kind needs two different convolution functions: a (polarised) PDF for the
    protons and a fragmentation function for the pions. When users convolve this grid with the two functions,
    they must either pass the functions in the right order to avoid calculating wrong predictions
    (see this [issue](https://gitlab.com/hepcedar/lhapdf/-/issues/79) for more details). `NeoPDF` circumvents
    this issue by adding the following keys to the metadata:

    ``` yaml
    Particle: 2212/212/... # Hadron PID
    Polarized: true/false
    SetType: SpaceLike/TimeLike
    ```

`NeoPDF` provides comprehensive support for different types of hadronic structure functions and most
importantly distinguish between them, which is essential for precision QCD calculations.

- **Collinear vs. Transverse Momentum Dependent Distributions**:

    * **Collinear Parton Distribution Functions (PDFs)** describe the probability of finding a parton
      (quark or gluon) carrying a fraction $x$ of the longitudinal momentum of the parent hadron at a
      given scale $Q^2$. These are functions of $(x, Q^2)$ and are the standard objects used in collinear
      factorization for high-energy processes.
    * **Transverse Momentum Dependent Distributions (TMDs)**, or TMD PDFs, generalize the concept of PDFs by
      including the dependence on the parton's intrinsic transverse momentum $k_T$. TMDs are functions of
      $(x, k_T, Q^2)$ and are essential for describing processes sensitive to the transverse structure of
      hadrons, such as low-$p_T$ Drell-Yan, semi-inclusive deep inelastic scattering (SIDIS), and certain
      jet observables.
    * `NeoPDF` natively supports both collinear PDFs and TMDs, allowing users to interpolate and evaluate
      distributions with or without $k_T$ dependence. The library automatically distinguishes between these
      types based on the grid metadata, ensuring correct usage in phenomenological applications.

- **Parton vs Nuclear PDFs (Or TMDs, respectively)**:

    * **Parton PDFs** describe the momentum distribution of quarks and gluons within protons and
      neutrons, fundamental for understanding the internal structure of hadrons. These are crucial
      for Standard Model predictions at hadron colliders like the LHC.
    * **Nuclear PDFs** extend this framework to describe parton distributions within nuclei, accounting
      for nuclear binding effects, shadowing, and anti-shadowing. These are essential for heavy-ion
      collisions and understanding nuclear structure effects in high-energy physics experiments.

- **Polarized vs Unpolarized PDFs (Or TMDs, respectively)**:

    * **Unpolarized PDFs** represent the standard momentum distributions and are used in most collider
      physics calculations.
    * **Polarized PDFs** describe the spin-dependent parton distributions, crucial for understanding the
      proton's spin structure and for experiments with polarized beams. These are essential for the RHIC
      spin program and future electron-ion colliders (EIC).

    The difference between polarized and unpolarized PDFs provides direct insight into the proton's spin
    decomposition and tests of QCD in the spin sector.

- **Timelike vs Spacelike PDFs (Or TMDs, respectively)**:

    - **Spacelike PDFs** (the standard case) describe parton distributions in deep-inelastic scattering
      and hadron-hadron collisions.
    - **Timelike PDFs** (Fragmentation Functions) describe the hadronization of partons into hadrons,
      essential for understanding jet structure and hadron production in $e^+e^-$ collisions and hadron-hadron
      collisions.

    This distinction is crucial for precision phenomenology, as the evolution equations and factorization
    theorems differ between the two cases.

### Multi-Parameter Interpolations

`NeoPDF` supports interpolation across multiple physical parameters:

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

- **Transverse Momentum Dependence $k_T$**:
  `NeoPDF` provides full support for interpolation in the transverse momentum $k_T$ variable, enabling
  access to TMD PDFs and related distributions. Users can:

    * Interpolate TMD grids as functions of $(x, k_T, Q^2)$, supporting both regular and logarithmic
      $k_T$ binning.
    * Seamlessly switch between collinear and TMD modes depending on the grid type, with automatic
      handling of $k_T$ integration or projection as needed.
    * Study $k_T$-dependent observables and perform phenomenological analyses that require access to
      the full transverse momentum structure of the parton distributions.

  This feature is crucial for modern QCD analyses, including TMD factorization, resummation, and the
  study of nonperturbative effects in hadron structure.

### Multi-Flavor Grids (Planned)

`NeoPDF` will support grids with varying numbers of active flavors $n_f$, providing a consistent
treatment of heavy quark effects across all scales. Advanced schemes like FONLL and ACOT require
careful handling of flavor thresholds and mass effects. `NeoPDF`'s multi-flavor support will enable:

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

`NeoPDF` provides native APIs across multiple programming languages, enabling seamless integration
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

`NeoPDF` maintains API compatibility with LHAPDF, enabling seamless migration:

- **Drop-in Replacement**:
  Existing LHAPDF code can often be migrated by simply changing import statements, with no
  modifications to the core physics logic required.

- **Preserved Function Signatures**:
  Key functions like `xfxQ2()`, `alphasQ2()`, and `mkPDF()` maintain the same signatures as
  LHAPDF, ensuring compatibility with existing analysis codes.

This compatibility is crucial for the physics community, as it allows for immediate adoption
without requiring extensive code rewrites or validation efforts.

### Thread and Memory Safety

`NeoPDF` leverages Rust's safety guarantees for robust multi-threaded applications:

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

`NeoPDF`'s modular architecture enables easy extension and customization:

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

`NeoPDF` is designed for high-performance computing environments:

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

- **API Layer**: Exposes a consistent interface in different programming languages.
- **Core Engine**: Handles all logics, grid management, and interpolation.
- **Grid Data**: Efficiently loads and manages PDF grid data and metadata.
- **Interpolation Strategies**: Pluggable, with default (log)-bicubic, bilinear, and (log)-tricubic.
  Relies on [ninterp](https://github.com/NREL/ninterp) for the N-dimensional interpolation.

## Benchmark Against LHAPDF

`NeoPDF` implements (log)-bicubic interpolation by default, with optional $N$-dimensional strategies.
Lower-dimensional (bilinear, (log)-tricubic) are also available for performance tuning.

!!! success "Benchmark against LHAPDF"

    The difference between NeoPDF and LHAPDF, using the default interpolation, is **below machine
    precision** for floating-point numbers.

    ![diff_NNPDF40_nnlo_as_01180_flav21](https://github.com/user-attachments/assets/d47bfa13-9930-4247-89fb-f2c2eab68bd7)
