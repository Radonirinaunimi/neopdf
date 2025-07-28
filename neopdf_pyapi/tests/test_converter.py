import neopdf
import pytest
import os


@pytest.fixture
def create_dummy_files(tmp_path):
    dummy_content = """
    ---
    Some contents
    ---
    """
    lhapdf_dir = tmp_path / "LHAPDF"
    lhapdf_dir.mkdir()
    base_name = "NNPDF40_nnlo_as_01180"
    files = [f"{base_name}.info", f"{base_name}_0001.dat"]

    for filename in files:
        set_path = lhapdf_dir / filename
        set_path.write_text(dummy_content)

    return str(tmp_path)


def test_convert_lhapdf(create_dummy_files):
    lhapdf_path = create_dummy_files
    os.environ["LHAPDF_DATA_PATH"] = lhapdf_path
    output_path = os.path.join(lhapdf_path, "test.neopdf.lz4")
    neopdf.converter.convert_lhapdf("NNPDF40_nnlo_as_01180", output_path)
    assert os.path.exists(output_path)
