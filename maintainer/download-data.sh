#!/bin/bash

set -eou pipefail

LHAPDF_SETS=(
  NNPDF40_nnlo_as_01180
  nNNPDF30_nlo_as_0118_p
  nNNPDF30_nlo_as_0118_A2_Z1
  nNNPDF30_nlo_as_0118_A4_Z2
  nNNPDF30_nlo_as_0118_A6_Z3
  nNNPDF30_nlo_as_0118_A9_Z4
  NNPDF40_nnlo_as_01160
  NNPDF40_nnlo_as_01170
  NNPDF40_nnlo_as_01175
  NNPDF40_nnlo_as_01185
  NNPDF40_nnlo_as_01190
)

NEOPDF_SETS=(
  NNPDF40_nnlo_as_01180
  nNNPDF30_nlo_as_0118
)

# Store the data in the root of the repository
cd ..
test -d neopdf-data || mkdir neopdf-data

# Download LHAPDF sets
for lha in "${LHAPDF_SETS[@]}"; do
  # see the following link why `--no-same-owner` may be necessary:
  # https://github.com/habitat-sh/builder/issues/365#issuecomment-382862233
  curl "https://lhapdfsets.web.cern.ch/current/${lha}.tar.gz" | tar xzf - --no-same-owner -C neopdf-data
done

# Download NeoPDF sets
for neo in "${NEOPDF_SETS[@]}"; do
  wget --no-verbose --no-clobber -P neopdf-data "https://data.nnpdf.science/neopdf/data/${neo}.neopdf.lz4"
done
