# Installation

This guide provides detailed instructions for installing NeoPDF and its APIs for Rust, Python, and C/C++.

## Rust Crate

To integrate NeoPDF into your Rust project, you need to add it as a dependency in your `Cargo.toml` file.

1.  **Open `Cargo.toml`**: Navigate to the root of your Rust project and open the `Cargo.toml` file.
2.  **Add Dependency**: Add the following line under the `[dependencies]` section:

    ```toml
    neopdf = "0.1.0"
    ```
    *(Note: You can replace `"0.1.0"` with the latest version available on [crates.io](https://crates.io/crates/neopdf)).*

3.  **Build Project**: Cargo will automatically fetch and compile the dependency the next time you build your project with `cargo build`.

## Python API

The Python API is distributed as a Python package. You can install it using `pip` after building it from the source with `maturin`.

1.  **Prerequisites**: Ensure you have Python, pip, and Rust's package manager, `cargo`, installed. You will also need `maturin`. If you don't have maturin, install it with pip:
    ```bash
    pip install maturin
    ```
2.  **Clone the repository**:
    ```bash
    git clone https://github.com/radonirinaunimi/neopdf.git
    cd neopdf/neopdf_pyapi
    ```
3.  **Build and Install**: Run the following command to build the Python wheel and install it in your current Python environment:
    ```bash
    maturin develop --release
    ```
    The `--release` flag ensures that the Rust code is compiled with optimizations for better performance.

## C/C++ API

The C/C++ API provides a shared library (`.so`, `.dll`, or `.dylib`) that can be linked against C or C++ projects.

1.  **Prerequisites**: You need the Rust toolchain and `cargo-c`, a tool for building and installing C-compatible Rust libraries.
    ```bash
    cargo install cargo-c
    ```
2.  **Build and Install**: Run the following commands to build and install the C-API.
    ```bash
    # Set an installation prefix (e.g., /usr/local or a custom path)
    export CARGO_C_INSTALL_PREFIX=${HOME}/.local

    # Build and install the library
    cargo cinstall --release --prefix=${CARGO_C_INSTALL_PREFIX}
    ```
3.  **Configure Environment**: To allow your system to find the newly installed library, you need to update your environment variables. Add these lines to your shell's startup file (e.g., `.bashrc`, `.zshrc`):
    ```bash
    export LD_LIBRARY_PATH=${CARGO_C_INSTALL_PREFIX}/lib:$LD_LIBRARY_PATH
    export PKG_CONFIG_PATH=${CARGO_C_INSTALL_PREFIX}/lib/pkgconfig:$PKG_CONFIG_PATH
    ```
    Remember to source your startup file (e.g., `source ~/.bashrc`) or restart your shell for the changes to take effect.
