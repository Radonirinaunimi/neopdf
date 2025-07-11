import lhapdf
import pytest
import numpy as np

from neopdf.pdf import PDF

NB_POINTS = 100
PDFSET_NAME = "NNPDF40_nnlo_as_01180"


@pytest.fixture(scope="session")
def neo_pdf() -> PDF:
    return PDF(PDFSET_NAME)


@pytest.fixture(scope="session")
def xq2_points() -> tuple[float, float]:
    xs = np.logspace(-9, 0, NB_POINTS)
    q2s = np.logspace(1, 8, NB_POINTS)
    return xs, q2s


@pytest.fixture(scope="session")
def lha_pdf() -> lhapdf:
    return lhapdf.mkPDF(PDFSET_NAME)
