import pytest

from neopdf.pdf import PDF as NeoPDF


@pytest.mark.multipleversions
@pytest.mark.parametrize(
    "pdfname", ["NNPDF40_nnlo_as_01180", "NNPDF40_nnlo_as_01180.neopdf.lz4"]
)
class TestNeoPDF:
    def test_single_member(self, pdfname):
        pdf = NeoPDF.mkPDF(pdfname)
        xfx = pdf.xfxQ2(21, 1e-5, 1e2)
        assert isinstance(xfx, float)
        alphas = pdf.alphasQ2(1e2)
        assert isinstance(alphas, float)

    def test_all_members(self, pdfname):
        pdfs = NeoPDF.mkPDFs(pdfname)
        assert len(pdfs) > 1

        for pdf in pdfs:
            xfx = pdf.xfxQ2(21, 1e-5, 1e2)
            assert isinstance(xfx, float)
            alphas = pdf.alphasQ2(1e2)
            assert isinstance(alphas, float)
