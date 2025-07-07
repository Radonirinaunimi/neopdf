# C-API for NeoPDF

This crate provides a C-compatible API for the `neopdf` Rust library,
enabling interoperability with C/C++ and other languages that can
link to C libraries.

To build the C-API, first install `cargo-c`:

```
cargo install cargo-c
```

then run the following command:

```
cargo cinstall --release --prefix=${prefix}
```

This will install the library in in the `${prefix}` path. This path
can then be added to the `PKG_CONFIG_PATH` and `LD_LIBRARY_PATH`
environment variables by running:

```
export LD_LIBRARY_PATH=${prefix}/lib:$LD_LIBRARY_PATH
export PKG_CONFIG_PATH=${prefix}/lib/pkgconfig:$PKG_CONFIG_PATH
```
