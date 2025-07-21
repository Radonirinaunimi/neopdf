# Installation

This guide provides detailed instructions for installing `NeoPDF `and its APIs for Rust, Python,
Fortran, and C/C++, as well as the command line interface (CLI).

!!! note "Where to store PDF sets?"

    By default, `NeoPDF` stores PDF sets in `${HOME}/.local/share/neopdf`, however this can be
    overwritten via the environment variable `NEOPDF_DATA_PATH` to point to the LHAPDF path for
    example.

    ```bash
    export NEOPDF_DATA_PATH=${LHAPDF_DATA_PATH}
    ```

---

## Rust Crate

To use `neopdf` in your Rust project, simply add the following to your `Cargo.toml`:

```toml
[dependencies]
neopdf = "0.1.0" # select the version
```

Cargo will automatically fetch and compile the dependency the next time you build your project with:

```bash
cargo build
```

---

## Python API

To build the Python API from the source code, make sure that [maturin](https://www.maturin.rs/) is
installed, go into the `neopdf_pyapi` directory, and then simply run:

```bash
maturin develop --release
```

This will build and install the `NeoPDF` Python extension in your current environment. Alternatively,
`neopdf` is  also available on the Python Package Index (PyPI) and therefore can be installed with
your favourite Python's package manager. For example, using [pipx](https://pipx.pypa.io/stable/):

```bash
pipx install neopdf
```

---

## C/C++ API

To build the C-API from source, first install `cargo-c`:

```bash
cargo install cargo-c
```

Then go into the `neopdf_capi` directory and run the following command:

```bash
export CARGO_C_INSTALL_PREFIX=${prefix} # Required for the OOP C++ header
cargo cinstall --release --prefix=${prefix}
```

This will install the library in the `${prefix}` path. This path can then be added to the `PKG_CONFIG_PATH`
and `LD_LIBRARY_PATH` environment variables by running:

```bash
export LD_LIBRARY_PATH=${prefix}/lib:$LD_LIBRARY_PATH
export PKG_CONFIG_PATH=${prefix}/lib/pkgconfig:$PKG_CONFIG_PATH
```

Remember to source your shell configuration or restart your terminal for the changes to take effect.

---

## CLI Tool

To build and install the NeoPDF command-line interface (CLI) from source, simply run:

```bash
cargo install --path neopdf_cli --debug
```

This will compile the CLI in debug mode and make the `neopdf` command available in your
cargo bin directory (usually `~/.cargo/bin`). You can then run `neopdf --help` to see
the available commands.
