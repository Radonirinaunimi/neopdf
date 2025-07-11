import pytest
import numpy as np

from itertools import product


class TestPDFInterpolations:
    @pytest.mark.parametrize("pid", [nf for nf in range(-5, 6) if nf != 0])
    def test_xfxq2(self, neo_pdf, lha_pdf, xq2_points, pid):
        xs, q2s = xq2_points
        for x, q2 in product(xs, q2s):
            ref = lha_pdf.xfxQ2(pid, x, q2)
            res = neo_pdf.xfxQ2(pid, x, q2)
            np.testing.assert_equal(res, ref)

    @pytest.mark.parametrize("pid", [21])
    def test_xfxq2s(self, neo_pdf, lha_pdf, xq2_points, pid):
        xs, q2s = xq2_points
        res = neo_pdf.xfxQ2s([pid], xs, q2s)
        ref = [lha_pdf.xfxQ2(pid, x, q2) for x, q2 in product(xs, q2s)]
        np.testing.assert_equal(res, [ref])

    @pytest.mark.parametrize("q2_point", np.logspace(2, 10, 100))
    def test_alphasQ2(self, neo_pdf, lha_pdf, q2_point):
        ref = lha_pdf.alphasQ2(q2_point)
        res = neo_pdf.alphasQ2(q2_point)
        np.testing.assert_equal(res, ref)
