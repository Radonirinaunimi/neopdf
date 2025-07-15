# CLI interface for NeoPDF

This crate provides a command-line interface (CLI) for the `neopdf`
Rust library, allowing users to interact with its functionalities
directly from the terminal.

To compute the interpolated function $xf (x, Q^2)$:
```bash
neopdf compute xfx_q2 --pdf-name NNPDF40_nnlo_as_01180 --member 0 --pid 21 1e-3 10.0
```

Similarly, to compute $\alpha_s(Q^2)$:
```bash
neopdf compute alphas_q2 --pdf-name NNPDF40_nnlo_as_01180 --member 0 --q2 10
```

To convert a given LHAPDF set into the new format:
```bash
neopdf write convert --pdf-name NNPDF40_nnlo_as_01180 --output output.neopdf.lz4
```

To combine nuclear PDFs into a combined `NeOPDF` grid:
```bash
neopdf write combine --pdf-names nNNPDF30_nlo_as_0118_p nNNPDF30_nlo_as_0118_A12 nNNPDF30_nlo_as_0118_A40 --output combined.neopdf.lz4
```

**NOTE:** To use the NeoPDF grid format, simply append to the name of the PDF set(s)
`neopdf.lz4`.
