import pytest
import numpy as np
from math import sqrt


class TestMetaData:
    @pytest.mark.parametrize("pdfname", ["NNPDF40_nnlo_as_01180", "MSHT20qed_an3lo"])
    def test_metadata_fields(self, neo_pdf, lha_pdf, pdfname):
        neopdf = neo_pdf(pdfname)
        lhapdf = lha_pdf(pdfname)

        neopdf_meta = neopdf.metadata()
        np.testing.assert_equal(neopdf_meta.x_min(), lhapdf.xMin)
        np.testing.assert_equal(neopdf_meta.x_max(), lhapdf.xMax)
        np.testing.assert_equal(neopdf_meta.q_min(), sqrt(lhapdf.q2Min))
        np.testing.assert_equal(neopdf_meta.q_max(), sqrt(lhapdf.q2Max))
        np.testing.assert_equal(neopdf_meta.set_index(), lhapdf.lhapdfID)
