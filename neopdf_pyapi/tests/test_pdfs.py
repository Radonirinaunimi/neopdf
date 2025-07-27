import pytest
import numpy as np

from itertools import product


class TestPDFInterpolations:
    @pytest.mark.parametrize(
        "pdfname",
        [
            "NNPDF40_nnlo_as_01180",
            "MSHT20qed_an3lo",
            "CT18NNLO_as_0118",
            "NNPDFpol20_nnlo_as_01180",
            "nNNPDF30_nlo_as_0118_A56_Z26",
        ],
    )
    @pytest.mark.parametrize("pid", [nf for nf in range(-5, 6) if nf != 0])
    def test_xfxq2(self, neo_pdf, lha_pdf, xq2_points, pdfname, pid):
        neopdf = neo_pdf(pdfname)
        lhapdf = lha_pdf(pdfname)

        params_range = {
            "xmin": lhapdf.xMin,
            "xmax": lhapdf.xMax,
            "q2min": lhapdf.q2Min,
            "q2max": lhapdf.q2Max,
        }
        xs, q2s = xq2_points(**params_range)

        for x, q2 in product(xs, q2s):
            ref = lhapdf.xfxQ2(pid, x, q2)
            res = neopdf.xfxQ2(pid, x, q2)
            np.testing.assert_equal(res, ref)

    @pytest.mark.parametrize("pdfname", ["NNPDF40_nnlo_as_01180", "MSHT20qed_an3lo"])
    @pytest.mark.parametrize("pid", [21])
    def test_xfxq2s(self, neo_pdf, lha_pdf, xq2_points, pdfname, pid):
        neopdf = neo_pdf(pdfname)
        lhapdf = lha_pdf(pdfname)

        params_range = {
            "xmin": lhapdf.xMin,
            "xmax": lhapdf.xMax,
            "q2min": lhapdf.q2Min,
            "q2max": lhapdf.q2Max,
        }
        xs, q2s = xq2_points(**params_range)

        res = neopdf.xfxQ2s([pid], xs, q2s)
        ref = [lhapdf.xfxQ2(pid, x, q2) for x, q2 in product(xs, q2s)]
        np.testing.assert_equal(res, [ref])

    @pytest.mark.parametrize("pdfname", ["NNPDF40_nnlo_as_01180", "MSHT20qed_an3lo"])
    def test_alphasQ2(self, neo_pdf, lha_pdf, pdfname):
        neopdf = neo_pdf(pdfname)
        lhapdf = lha_pdf(pdfname)
        qs = neopdf.metadata().alphas_q()
        q2_points = [q * q for q in np.linspace(qs[0], qs[-1], num=300)]

        for q2_point in q2_points:
            ref = lhapdf.alphasQ2(q2_point)
            res = neopdf.alphasQ2(q2_point)
            np.testing.assert_equal(res, ref)
