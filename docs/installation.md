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

Cargo will automatically fetch and compile the dependencies the next time you build your project with:

```bash
cargo build
```

---

## Python API

`NeoPDF` is available on the Python Packaged Index (PyPI) as `neopdf-hep` and therefore can be
installed easily with any of the Python's package managers. For example, using
[pipx](https://pipx.pypa.io/stable/):

```bash
pipx install neopdf-hep
```

!!! info "Development Option"

    Alternatively, to build the Python API from source, make sure that [maturin](https://www.maturin.rs/)
    is installed, go into the `neopdf_pyapi` directory, and then simply run:

    ```bash
    maturin develop --release --extras test
    ```

    This will build and install the `NeoPDF` Python extension in your current environment.

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

## Fortran API

In order to use the Fortran API, you have to install first the C/C++ API. Then, simply copy the
`neopdf_fapi/neopdf.f90` in your working directory and generate the Fortran module:

```bash
gfortran -c neopdf.f90
```

If everything went fine, this will generate a `neopdf.mod` file. You can now the module in your
fortran program by including the following:

```fortran
use neopdf
```

---

## CLI Tool

The Command Line Interface (CLI) to the `NeoPDF` APIs is also available on the Python Packaged
Index (PyPI) as `neopdf-cli` and therefore can be installed easily with any of the Python's
package managers. For example, using [pipx](https://pipx.pypa.io/stable/):

```bash
pipx install neopdf-cli
```

!!! info " Development Option"

    To build and install the NeoPDF command-line interface (CLI) from source, simply run:

    ```bash
    cargo install --path neopdf_cli --debug
    ```

    This will compile the CLI in debug mode and make the `neopdf` command available in your
    cargo bin directory (usually `~/.cargo/bin`). You can then run `neopdf --help` to see
    the available commands.
