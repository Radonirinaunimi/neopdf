import lhapdf
import pytest
import numpy as np

from neopdf.pdf import PDF as NeoPDF
from neopdf.pdf import LazyPDFs
from typing import List, Dict, Iterator


@pytest.fixture(scope="session")
def neo_pdf():
    cached_pdf = {}

    def _init_pdf(pdfname: str) -> Dict[str, NeoPDF]:
        if pdfname not in cached_pdf:
            cached_pdf[pdfname] = NeoPDF(pdfname)
        return cached_pdf[pdfname]

    return _init_pdf


@pytest.fixture(scope="session")
def neo_pdfs():
    cached_pdf = {}

    def _init_pdf(pdfname: str) -> Dict[str, List[NeoPDF]]:
        if pdfname not in cached_pdf:
            cached_pdf[pdfname] = NeoPDF.mkPDFs(pdfname)
        return cached_pdf[pdfname]

    return _init_pdf


@pytest.fixture(scope="session")
def neo_pdfs_lazy():
    cached_pdf = {}

    def _init_pdf(pdfname: str) -> Dict[str, Iterator[LazyPDFs]]:
        if pdfname not in cached_pdf:
            cached_pdf[pdfname] = NeoPDF.mkPDFs_lazy(pdfname)
        return cached_pdf[pdfname]

    return _init_pdf


@pytest.fixture(scope="session")
def lha_pdf():
    cached_pdf = {}

    def _init_pdf(pdfname: str) -> Dict[str, lhapdf.PDF]:
        if pdfname not in cached_pdf:
            cached_pdf[pdfname] = lhapdf.mkPDF(pdfname)
        return cached_pdf[pdfname]

    return _init_pdf


@pytest.fixture(scope="session")
def lha_pdfs():
    cached_pdf = {}

    def _init_pdf(pdfname: str) -> Dict[str, List[lhapdf.PDF]]:
        if pdfname not in cached_pdf:
            cached_pdf[pdfname] = lhapdf.mkPDFs(pdfname)
        return cached_pdf[pdfname]

    return _init_pdf


@pytest.fixture(scope="session")
def xq2_points():
    def _xq2_points(
        xmin: float, xmax: float, q2min: float, q2max: float
    ) -> tuple[np.ndarray, np.ndarray]:
        xs = np.geomspace(xmin, xmax, num=200)
        q2s = np.geomspace(q2min, q2max, num=200)
        return xs, q2s

    return _xq2_points
