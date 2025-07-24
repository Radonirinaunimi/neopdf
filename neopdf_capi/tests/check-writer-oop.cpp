#include <NeoPDF.hpp>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <vector>

using namespace neopdf;

const double TOLERANCE= 1e-16;

int main() {
    const char* pdfname = "NNPDF40_nnlo_as_01180";
    // Load all PDF members
    NeoPDFs neo_pdfs(pdfname);
    if (neo_pdfs.size() == 0) {
        std::cerr << "Failed to load any PDF members!\n";
        return 1;
    }
    std::cout << "Loaded " << neo_pdfs.size() << " PDF members\n";

    // Get the first PDF as a reference for metadata
    NeoPDF& ref_pdf = neo_pdfs[0];

    // Extract the PID values of the PDF set
    auto pids = ref_pdf.pids();

    // Extract the number of subgrids
    std::size_t num_subgrids = ref_pdf.num_subgrids();

    // Create a grid writer
    GridWriter writer;

    // For each member, build a grid
    for (size_t m = 0; m < neo_pdfs.size(); ++m) {
        NeoPDF& pdf = neo_pdfs[m];

        // Loop over the Subgrids
        for (std::size_t subgrid_idx = 0; subgrid_idx != num_subgrids; subgrid_idx++) {
            // Extract the knot values of the parameters for the subgrid
            auto xs = pdf.subgrid_for_param(NEOPDF_SUBGRID_PARAMS_MOMENTUM, subgrid_idx);
            auto q2s = pdf.subgrid_for_param(NEOPDF_SUBGRID_PARAMS_SCALE, subgrid_idx);
            auto alphas = pdf.subgrid_for_param(NEOPDF_SUBGRID_PARAMS_ALPHAS, subgrid_idx);
            auto nucleons = pdf.subgrid_for_param(NEOPDF_SUBGRID_PARAMS_NUCLEONS, subgrid_idx);
            auto kts = pdf.subgrid_for_param(NEOPDF_SUBGRID_PARAMS_KT, subgrid_idx);

            // Compute grid_data: [q2][x][flavor], instead of [nucleon][alphas][kt][q2][x][flavor]
            // NOTE: This assumes that there is no 'A' and `alphas` dependence.
            std::vector<double> grid_data;
            for (double x : xs) {
                for (double q2 : q2s) {
                    for (int pid : pids) {
                        double val = pdf.xfxQ2(pid, x, q2);
                        grid_data.push_back(val);
                    }
                }
            }

            // Add grid
            writer.add_grid(
                nucleons,
                alphas,
                kts,
                xs,
                q2s,
                grid_data,
                pids
            );
        }
        std::cout << "Added grid for member " << m << "\n";
    }

    // Fill the running of alphas with some random values
    std::vector<double> alphas_qs = {2.0};
    std::vector<double> alphas_vals = {0.118};

    // Extract the ranges for the momentum x and scale Q2
    auto x_range = ref_pdf.param_range(NEOPDF_SUBGRID_PARAMS_MOMENTUM);
    auto q2_range = ref_pdf.param_range(NEOPDF_SUBGRID_PARAMS_SCALE);

    PhysicsParameters phys_params = {
        .flavor_scheme = "variable",
        .order_qcd = 2,
        .alphas_order_qcd = 2,
        .m_w = 80.352,
        .m_z = 91.1876,
        .m_up = 0.0,
        .m_down = 0.0,
        .m_strange = 0.0,
        .m_charm = 1.51,
        .m_bottom = 4.92,
        .m_top = 172.5,
    };

    MetaData meta = {
        .set_desc = "NNPDF40_nnlo_as_01180 collection",
        .set_index = 0,
        .num_members = (uint32_t)neo_pdfs.size(),
        .x_min = x_range[0],
        .x_max = x_range[1],
        .q_min = sqrt(q2_range[0]),
        .q_max = sqrt(q2_range[1]),
        .flavors = pids,
        .format = "neopdf",
        .alphas_q_values = alphas_qs,
        .alphas_vals = alphas_vals,
        .polarised = false,
        .set_type = NEOPDF_SET_TYPE_SPACE_LIKE,
        .interpolator_type = NEOPDF_INTERPOLATOR_TYPE_LOG_BICUBIC,
        .error_type = "replicas",
        .hadron_pid = 2212,
        .phys_params = phys_params,
    };

    // Check if `NEOPDF_DATA_PATH` is defined and store the Grid there.
    const char* filename = "check-writer-oop.neopdf.lz4";
    const char* neopdf_path = std::getenv("NEOPDF_DATA_PATH");
    std::string output_path = neopdf_path
        ? std::string(neopdf_path) + (std::string(neopdf_path).back() == '/' ? "" : "/") + filename
        : filename;

    // Write the PDF Grid into disk
    try {
        writer.compress(meta, output_path);
        std::cout << "Compression succeeded!\n";
    } catch (const std::runtime_error& err) {
        std::cerr << "Compression failed: " << err.what() << "\n";
        return EXIT_FAILURE;
    }

    // If `NEOPDF_DATA_PATH` is defined, reload the grid and check ther results.
    if (neopdf_path) {
        int pid_test = 21;
        double x_test = 1e-3;
        double q2_test1 = 1e2;
        double q2_test2 = 1e4;

        double ref1 = neo_pdfs[0].xfxQ2(pid_test, x_test, q2_test1);
        double ref2 = neo_pdfs[0].xfxQ2(pid_test, x_test, q2_test2);

        NeoPDF wpdf(pdfname);
        double res1 = wpdf.xfxQ2(pid_test, x_test, q2_test1);
        double res2 = wpdf.xfxQ2(pid_test, x_test, q2_test2);

        assert(std::abs(res1 - ref1) < TOLERANCE);
        assert(std::abs(res2 - ref2) < TOLERANCE);
    }

    return EXIT_SUCCESS;
}
