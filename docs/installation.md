# Installation

This guide provides detailed instructions for installing `NeoPDF `and its APIs for Rust, Python,
Fortran, and C/C++.

---

## Rust Crate

To use `neopdf` in your Rust project, simply add the following to your `Cargo.toml`:

```toml
[dependencies]
neopdf = "0.1.0"
```

Cargo will automatically fetch and compile the dependency the next time you build your project with:

```bash
cargo build
```

You can find the latest version of `neopdf` on [crates.io](https://crates.io/crates/neopdf).

---

## Python API

To build the Python API from the source code, make sure that [maturin](https://www.maturin.rs/) is
installed, go into the `neopdf_pyapi` directory, and then simply run:

```bash
maturin develop --release
```

This will build and install the `NeoPDF` Python extension in your current environment. `neopdf` is
also available on ...

```bash
pipx install neopdf
```

---

## C/C++ API

To build the C-API from source, first install `cargo-c`:

```bash
cargo install cargo-c
```

Then run the following command:

```bash
export CARGO_C_INSTALL_PREFIX=${prefix} # Needed if you want the OOP C++ header
cargo cinstall --release --prefix=${prefix}
```

This will install the library in the `${prefix}` path. This path can then be added to the `PKG_CONFIG_PATH` and `LD_LIBRARY_PATH` environment variables by running:

```bash
export LD_LIBRARY_PATH=${prefix}/lib:$LD_LIBRARY_PATH
export PKG_CONFIG_PATH=${prefix}/lib/pkgconfig:$PKG_CONFIG_PATH
```

Remember to source your shell configuration or restart your terminal for the changes to take effect.
