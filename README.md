# NeoPDF

NeoPDF is a fast, reliable, and scalable interpolation library for
Parton Distribution Functions (PDFs) with modern features targeting
both present and future hadron colliders.

**NOTE:** Although the C/C++/Python APIs should be stable, the main
`neopdf` crate is still WIP, especially when it regards the design
structure of the data management.

Physics' features
-----------------

- Differentiates between hadron types: parton vs nuclear, polarised vs
  unpolarised, timelike vs spacelike
- Supports interpolations of PDFs extracted with different values of
  $\alpha_s(M_Z)$
- Supports interpolations of the nuclear dependence $(A, Z)$
- Allows for multi-flavor grids with different $n_f$ (relevant for a
  General Mass Variable Flavour Number Scheme such as FONLL)
- Could easily support analytical interpolation using DGLAP equations 

Technical features
------------------
- No-code change when switching to `neopdf` using the Fortran/C/C++/Python
  APIs
  ```python
  from neopdf.pdf import PDF as lhapdf
  # everything else the same
  pdf = lhapdf.mkPDF("NNPDF40_nnlo_as_01180")
  pdf.xfxQ2(21, 1e-9, 1e2)
  ```
- Thread and memory safety
- Safer Foreign Function Interface (FFI) and interoperability with
  other languages
- Faster & easier scalability to accommodate new features

Benchmark against LHAPDF
------------------------

NeoPDF also implements as default a (log)-bicubic interpolation but one
can also opt for the $N$-dimensional interpolation strategy. However, for
better runtime performance and accuracy, various lower-dimensional
interpolation strategies are also implemented. These include: bilinear
and (log)-tricubic interpolations.

The difference between NeoPDF and LHAPDF, when using the default interpolation,
is **below** the machine precision for floating numbers.

![diff_NNPDF40_nnlo_as_01180_flav21](https://github.com/user-attachments/assets/d47bfa13-9930-4247-89fb-f2c2eab68bd7)
