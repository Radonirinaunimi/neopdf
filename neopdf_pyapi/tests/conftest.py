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
def xq2_points():
    def _xq2_points(
        xmin: float, xmax: float, q2min: float, q2max: float
    ) -> tuple[float, float]:
        xs = np.geomspace(xmin, xmax, num=150)
        q2s = np.geomspace(q2min, q2max, num=150)
        return xs, q2s

    return _xq2_points
