import numpy as np
from math import sqrt


class TestMetaData:
    def test_metadata_fields(self, neo_pdf, lha_pdf):
        neopdf_meta = neo_pdf.metadata()
        np.testing.assert_equal(neopdf_meta.x_min(), lha_pdf.xMin)
        np.testing.assert_equal(neopdf_meta.x_max(), lha_pdf.xMax)
        np.testing.assert_equal(neopdf_meta.q_min(), sqrt(lha_pdf.q2Min))
        np.testing.assert_equal(neopdf_meta.q_max(), sqrt(lha_pdf.q2Max))
        np.testing.assert_equal(neopdf_meta.set_index(), lha_pdf.lhapdfID)
