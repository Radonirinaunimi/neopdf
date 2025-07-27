import neopdf.pdf as pdf
import pytest


@pytest.mark.parametrize("pdf_name", ["NNPDF40_nnlo_as_01180", "MSHT20qed_an3lo"])
class TestPDF:
    def test_pdf_creation(self, pdf_name):
        new_pdf = pdf.PDF(pdf_name)
        assert new_pdf is not None

    def test_mkpdf(self, pdf_name):
        new_pdf = pdf.PDF.mkPDF(pdf_name)
        assert new_pdf is not None

    def test_mkpdfs(self, pdf_name):
        pdfs = pdf.PDF.mkPDFs(pdf_name)
        assert len(pdfs) > 0

    def test_pdf_methods(self, neo_pdf, pdf_name):
        neopdf = neo_pdf(pdf_name)
        assert len(neopdf.pids()) > 0
        assert len(neopdf.subgrids()) > 0
        assert neopdf.x_min() > 0
        assert neopdf.x_max() > 0
        assert neopdf.q2_min() > 0
        assert neopdf.q2_max() > 0
