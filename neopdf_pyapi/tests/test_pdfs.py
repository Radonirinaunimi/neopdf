import pytest
import numpy as np

from itertools import product


class TestPDFInterpolations:
    @pytest.mark.parametrize("pdfname", ["NNPDF40_nnlo_as_01180"])
    @pytest.mark.parametrize("pid", [nf for nf in range(-5, 6) if nf != 0])
    def test_xfxq2(self, neo_pdf, lha_pdf, xq2_points, pdfname, pid):
        xs, q2s = xq2_points
        neopdf = neo_pdf(pdfname)
        lhapdf = lha_pdf(pdfname)

        for x, q2 in product(xs, q2s):
            ref = lhapdf.xfxQ2(pid, x, q2)
            res = neopdf.xfxQ2(pid, x, q2)
            np.testing.assert_equal(res, ref)

    @pytest.mark.parametrize("pdfname", ["NNPDF40_nnlo_as_01180"])
    @pytest.mark.parametrize("pid", [21])
    def test_xfxq2s(self, neo_pdf, lha_pdf, xq2_points, pdfname, pid):
        xs, q2s = xq2_points
        neopdf = neo_pdf(pdfname)
        lhapdf = lha_pdf(pdfname)

        res = neopdf.xfxQ2s([pid], xs, q2s)
        ref = [lhapdf.xfxQ2(pid, x, q2) for x, q2 in product(xs, q2s)]
        np.testing.assert_equal(res, [ref])

    @pytest.mark.parametrize("pdfname", ["NNPDF40_nnlo_as_01180"])
    @pytest.mark.parametrize("q2_point", np.logspace(2, 10, 200))
    def test_alphasQ2(self, neo_pdf, lha_pdf, pdfname, q2_point):
        neopdf = neo_pdf(pdfname)
        lhapdf = lha_pdf(pdfname)

        ref = lhapdf.alphasQ2(q2_point)
        res = neopdf.alphasQ2(q2_point)
        np.testing.assert_equal(res, ref)
