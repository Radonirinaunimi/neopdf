import neopdf.writer as writer
import neopdf.parser as parser
import pytest


@pytest.mark.parametrize("pdf_name", ["NNPDF40_nnlo_as_01180"])
def test_writer(pdf_name, tmp_path):
    lhapdf_set = parser.LhapdfSet(pdf_name)
    members = lhapdf_set.members()
    metadata = lhapdf_set.info()
    grids = [m[1] for m in members]

    output_path = tmp_path / f"{pdf_name}.neopdf.lz4"
    writer.compress(grids, metadata, str(output_path))

    decompressed_members = writer.decompress(str(output_path))
    assert len(decompressed_members) == len(members)

    extracted_metadata = writer.extract_metadata(str(output_path))
    assert extracted_metadata.set_index() == metadata.set_index()
