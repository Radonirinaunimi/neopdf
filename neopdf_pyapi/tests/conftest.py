import lhapdf
import pytest
import numpy as np

from neopdf.pdf import PDF


@pytest.fixture(scope="session")
def neo_pdf():
    cached_pdf = {}

    def _init_pdf(pdfname: str) -> PDF:
        if pdfname not in cached_pdf:
            cached_pdf[pdfname] = PDF(pdfname)
        return cached_pdf[pdfname]

    return _init_pdf


@pytest.fixture(scope="session")
def lha_pdf():
    cached_pdf = {}

    def _init_pdf(pdfname: str) -> PDF:
        if pdfname not in cached_pdf:
            cached_pdf[pdfname] = lhapdf.mkPDF(pdfname)
        return cached_pdf[pdfname]

    return _init_pdf


@pytest.fixture(scope="session")
def xq2_points() -> tuple[float, float]:
    xs = np.logspace(-9, 0, 200)
    q2s = np.logspace(1, 8, 200)
    return xs, q2s
