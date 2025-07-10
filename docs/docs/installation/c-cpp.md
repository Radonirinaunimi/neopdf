# C/C++ API

To build the C-API, first install `cargo-c`:

```bash
cargo install cargo-c
```

then run the following command:

```bash
export CARGO_C_INSTALL_PREFIX=${prefix} # Needed if you want the OOP C++ header
cargo cinstall --release --prefix=${prefix}
```

This will install the library in in the `${prefix}` path. This path
can then be added to the `PKG_CONFIG_PATH` and `LD_LIBRARY_PATH`
environment variables by running:

```bash
export LD_LIBRARY_PATH=${prefix}/lib:$LD_LIBRARY_PATH
export PKG_CONFIG_PATH=${prefix}/lib/pkgconfig:$PKG_CONFIG_PATH
```
