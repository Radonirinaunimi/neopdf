<h1 align="center">NeoPDF</h1>
<div align="center">
    <a href="https://app.codecov.io/gh/Radonirinaunimi/neopdf/tree/master"><img
        alt="Codecov"
        src="https://img.shields.io/codecov/c/github/Radonirinaunimi/neopdf?style=for-the-badge&logo=codecov&logoColor=red&color=blue"
        height="22"
    /></a>
    <a href="https://gribnau.dev/cargo-msrv/"><img
        alt="MSRV"
        src="https://img.shields.io/crates/msrv/neopdf?style=for-the-badge&logo=rust&color=red"
        height="22"
    /></a>
    <a href="https://crates.io/crates/neopdf"><img
        alt="Crates.io"
        src="https://img.shields.io/crates/v/neopdf?style=for-the-badge&logo=rust&color=blue"
        height="22"
    /></a>
    <a href="https://pypi.org/project/neopdf-hep/"><img
        alt="PyPI - Version"
        src="https://img.shields.io/pypi/v/neopdf-hep?style=for-the-badge&logo=python&logoColor=yellow&color=%1d881d"
        height="22"
    /></a>
    <a href="https://github.com/Radonirinaunimi/neopdf?tab=GPL-3.0-1-ov-file"><img
        alt="GitHub License"
        src="https://img.shields.io/github/license/Radonirinaunimi/neopdf?style=for-the-badge&logo=gplv3&logoColor=red"
        height="22"
    /></a>
</div>

<p align="justify">
  <b>NeoPDF</b> is a fast, reliable, and scalable interpolation library for both <b>collinear</b>
  and <b>transverse momentum dependent</b> Parton Distribution Functions with <b>modern features</b>
  designed for both present and future hadron collider experiments:

  <ul>
    <li>
    <p align="justify">
      Beyond interpolations over the kinematic variables (<b>x</b>, <b>kT</b>, <b>Q2</b>), NeoPDF
      also supports interpolations along the nucleon numbers <b>A</b> (relevant for <b>nuclear</b> PDFs
      and TMDs) and the strong coupling <b>αs</b>.
    </p>
    </li>
    <li>
    <p align="justify">
      NeoPDF implements its own file format using binary serialization and <a href="https://lz4.org/">LZ4</a>
      compression, prioritizing speed and efficiency over human-readable formats. A command Line
      Interface (CLI) is provided to easily inspect and perform various operations on NeoPDF grids.
    </p>
    </li>
  </ul>
</p>

## Quick Links

- [Documentation](https://radonirinaunimi.github.io/neopdf/) | [Rust Crate Documentation](https://docs.rs/neopdf/0.1.1/neopdf/)
- [Installation](https://radonirinaunimi.github.io/neopdf/installation/)
- [Physics and technical features](https://radonirinaunimi.github.io/neopdf/design-and-features/)
- [NeoPDF Design](https://radonirinaunimi.github.io/neopdf/design/)
- [CLI tutorials](https://radonirinaunimi.github.io/neopdf/cli-tutorials/)
- [Tutorials and examples](https://radonirinaunimi.github.io/neopdf/examples/python/)
