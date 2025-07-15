# CLI Tutorials

The `neopdf` CLI tool provides a command-line interface to the `NeoPDF` library, enabling users
to perform PDF interpolation, alpha_s evaluation, file format conversion, and metadata inspection directly from the terminal. Below are detailed usage instructions and examples for each major feature.

!!! note Notes

    - To use the `NeoPDF` grid format, if available, simply append `.neopdf.lz4` to the name of the PDF set
    - The CLI supports both eager and lazy loading of grids, and can operate on both standard and compressed formats
    - For more advanced usage, see the help output:

    ```bash
    neopdf --help
    ```

---

## Reading PDF Metadata and Grid Information

### Read Metadata for a PDF Set

To read the metadata for a given PDF set:

```bash
neopdf read metadata --pdf-name NNPDF40_nnlo_as_01180
```

- `--pdf-name`: Name of the PDF set (can be a `NeoPDF` or `LHAPDF` set).

The above command will print out the following output:

```yaml
Set Description: NNPDF4.0 NNLO global fit, alphas(MZ)=0.1180. mem=0 => average on replicas; mem=1-100 => PDF replicas
Set Index: 331100
Number of Members: 101
XMin: 0.000000001
XMax: 1
QMin: 1.65
QMax: 100000
Flavors: [-5, -4, -3, -2, -1, 21, 1, 2, 3, 4, 5]
Format: lhagrid1
...
Polarized: false
Set Type: Pdf
Interpolator Type: LogBicubic
```

### Get the Number of Subgrids

To get the number of subgrids a PDF contains:

```bash
neopdf read num_subgrids --pdf-name NNPDF40_nnlo_as_01180
```

- `--pdf-name`: Name of the PDF set.

### Get Knot Values for a Subgrid

To get the knot values for a given subgrid:

```bash
neopdf read subgrid_info --pdf-name NNPDF40_nnlo_as_01180 --member 0 --subgrid-index 1
```

- `--pdf-name`: Name of the PDF set.
- `--member`: The member index (0-based).
- `--subgrid-index`: The index of the subgrid to inspect.

The above command will print out the following output:

```yaml
Nucleon Numbers A: [1.0], shape=[1], strides=[1], layout=CFcf (0xf), const ndim=1
Alphas values: [0.118], shape=[1], strides=[1], layout=CFcf (0xf), const ndim=1
x values: [1e-9, 1.2970848e-9, ...], shape=[196], strides=[1], layout=CFcf (0xf), const ndim=1
Q2 values: [2.7224999999999997, ...], shape=[12], strides=[1], layout=CFcf (0xf), const ndim=1
```

---

## Computing Interpolated Values

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

---

## Converting LHAPDF into NeoPDF Format

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
