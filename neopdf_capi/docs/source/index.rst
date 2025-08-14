.. _neopdf:

Welcome to the NeoPDF's C++ APIs documentation!
===============================================

**NeoPDF** is a fast, reliable, and scalable interpolation library for both **collinear**
and **transverse momentum dependent** Parton Distribution Functions with **modern features**
designed for both present and future hadron collider experiments.

- Beyond interpolations over the kinematic variables :math:`(x, k_T, Q^2)`, `NeoPDF` also
  supports interpolations along the nucleon numbers :math:`A` (relevant for **nuclear** PDFs
  and TMDs) and the strong coupling :math:`\alpha_s`.

- `NeoPDF` implements its own file format using binary serialization and `LZ4 <https://lz4.org/>`_
  compression, prioritizing speed and efficiency over human-readable formats. A command Line
  Interface (CLI) is provided to easily inspect and perform various operations on NeoPDF grids.

.. toctree::
   :maxdepth: 3
   :caption: Contents:

   api
   examples
