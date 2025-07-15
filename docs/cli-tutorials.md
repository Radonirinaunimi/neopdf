# CLI Tutorials

The `neopdf` CLI tool provides a command-line interface to the `NeoPDF` library, enabling users
to perform PDF interpolation, alpha_s evaluation, and file format conversion directly from the
terminal. Below are detailed usage instructions and examples for each major feature.

### Compute interpolated PDF values $xf(x, Q^2)$

To compute the interpolated function $xf(x, Q^2)$ for a given PDF set, member, and flavor:

```bash
neopdf compute xfx_q2 --pdf-name NNPDF40_nnlo_as_01180 --member 0 --pid 21 1e-3 10.0
```

- `--pdf-name`: Name of the PDF set (can be a `NeoPDF` or `LHAPDF` set).
- `--member`: The member index (0-based).
- `--pid`: The PDG ID of the parton flavor (e.g., 21 for gluon).
- The last two arguments are the $x$ and $Q^2$ values at which to evaluate the PDF.

### Compute strong coupling $\alpha_s$ at a given $Q^2$

To compute the strong coupling constant $\alpha_s(Q^2)$ for a given PDF set and member:

```bash
neopdf compute alphas_q2 --pdf-name NNPDF40_nnlo_as_01180 --member 0 --q2 10
```

- `--q2`: The $Q^2$ value at which to evaluate $\alpha_s$.

### Convert LHAPDF set to NeoPDF format

To convert a standard LHAPDF set into the new, compressed `NeoPDF` format:

```bash
neopdf write convert --pdf-name NNPDF40_nnlo_as_01180 --output output.neopdf.lz4
```

- `--output`: The output file name for the `NeoPDF` grid (should end with `.neopdf.lz4`).

### Combine multiple LHAPDF Nuclear PDFs

To combine several LHAPDF nuclear PDF sets into a single `NeoPDF` grid:

```bash
neopdf write combine --pdf-names nNNPDF30_nlo_as_0118_p nNNPDF30_nlo_as_0118_A12 nNNPDF30_nlo_as_0118_A40 --output combined.neopdf.lz4
```

- `--pdf-names`: List of PDF set names to combine.
- `--output`: Output file for the combined grid.

!!! note Notes

    - To use the `NeoPDF` grid format, simply append `.neopdf.lz4` to the name of the PDF set.
    - The CLI supports both eager and lazy loading of grids, and can operate on both standard and compressed formats.
    - For more advanced usage, see the help output:

    ```bash
    neopdf --help
    ```

    or for subcommands:

    ```bash
    neopdf compute --help
    neopdf write --help
    ```
