# Python API Example

This example demonstrates how to use the NeoPDF Python API to load a PDF set, evaluate parton distributions, and handle common tasks.

## Prerequisites

Ensure you have built and installed the Python API as described in the [installation guide](../installation.md).

## Basic Usage

```python linenums="1"
from neopdf.pdf import PDF

# Load a PDF set (e.g., NNPDF40_nnlo_as_01180, member 0)
pdf = PDF("NNPDF40_nnlo_as_01180", member=0)

# Evaluate the PDF for a given parton (pid), x, and Q^2
pid = 21  # gluon
x = 1e-3
Q2 = 100.0
value = pdf.xfxQ2(pid, x, Q2)
print(f"PDF value for pid={pid}, x={x}, Q2={Q2}: {value}")

# Evaluate alpha_s at a given Q^2
alphas = pdf.alphasQ2(Q2)
print(f"alpha_s(Q2={Q2}) = {alphas}")
```

## Looping Over All Members

```python linenums="1"
from neopdf.pdf import PDFs

# Load all members of a PDF set
pdfs = PDFs("NNPDF40_nnlo_as_01180")
print(f"Loaded {len(pdfs)} PDF members")

# Evaluate the same point for all members
results = [pdf.xfxQ2(pid, x, Q2) for pdf in pdfs]
print("Results across all members:", results)
```
