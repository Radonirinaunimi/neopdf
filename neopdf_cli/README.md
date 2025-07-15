# CLI interface for NeoPDF

This crate provides a command-line interface (CLI) for the `neopdf`
Rust library, allowing users to interact with its functionalities
directly from the terminal.

To convert a given LHAPDF set into the new format:
```bash
neopdf convert --pdf-name NNPDF40_nnlo_as_01180 --output output.neopdf.lz4
```

To combine nuclear PDFs into a combined `NeOPDF` grid:
```bash
neopdf combine --pdf-names nNNPDF30_nlo_as_0118_p nNNPDF30_nlo_as_0118_A12 nNNPDF30_nlo_as_0118_A40 --output combined.neopdf.lz4
```
