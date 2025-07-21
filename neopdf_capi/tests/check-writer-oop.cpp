#include "neopdf_capi.h"
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

    // Create a collection
    NeoPDFGridArrayCollection* collection = neopdf_gridarray_collection_new();
    if (!collection) {
        std::cerr << "Failed to create grid array collection!\n";
        return 1;
    }

    // For each member, build a grid
    for (size_t m = 0; m < neo_pdfs.size(); ++m) {
        NeoPDF& pdf = neo_pdfs[m];
        NeoPDFGrid* grid = neopdf_grid_new();

        if (!grid) {
            std::cerr << "Failed to create grid for member: " << m << "!\n";
            continue;
        }

        bool member_ok = true;
        // Loop over the Subgrids
        for (std::size_t subgrid_idx = 0; subgrid_idx != num_subgrids; subgrid_idx++) {
            // Extract the parameter values of the given subgrid
            auto xs = pdf.subgrid_for_param(NEOPDF_SUBGRID_PARAMS_MOMENTUM, subgrid_idx);
            auto q2s = pdf.subgrid_for_param(NEOPDF_SUBGRID_PARAMS_SCALE, subgrid_idx);
            auto alphas = pdf.subgrid_for_param(NEOPDF_SUBGRID_PARAMS_ALPHAS, subgrid_idx);
            auto nucleons = pdf.subgrid_for_param(NEOPDF_SUBGRID_PARAMS_NUCLEONS, subgrid_idx);
            auto kts = pdf.subgrid_for_param(NEOPDF_SUBGRID_PARAMS_KT, subgrid_idx);

            // Compute grid_data: [q2s][xs][flavors], instead of [nucleons][alphas][q2s][xs][flavors]
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

            // Add subgrid
            int add_subgrid =neopdf_grid_add_subgrid(
                grid,
                nucleons.data(), nucleons.size(),
                alphas.data(), alphas.size(),
                kts.data(), kts.size(),
                xs.data(), xs.size(),
                q2s.data(), q2s.size(),
                grid_data.data(), grid_data.size()
            );
            if (add_subgrid != NEOPDF_RESULT_SUCCESS) {
                std::cerr << "Failed to add subgrid for member: " << m << "!\n";
                neopdf_grid_free(grid);
                member_ok = false;
                break;
            }
        }

        if (!member_ok) continue;

        // Set flavor PIDs
        int add_flavors = neopdf_grid_set_flavors(grid, pids.data(), pids.size());
        if (add_flavors != NEOPDF_RESULT_SUCCESS) {
            std::cerr << "Failed to set flavors for member: " << m << "!\n";
            neopdf_grid_free(grid);
            continue;
        }

        // Add grid to collection
        int add_grid = neopdf_gridarray_collection_add_grid(collection, grid);
        if (add_grid != NEOPDF_RESULT_SUCCESS) {
            std::cerr << "Failed to add grid to collection for member: " << m << "!\n";
            neopdf_grid_free(grid);
            continue;
        }
        std::cout << "Added grid for member " << m << "\n";
    }

    // Fill the running of alphas with some random values
    double alphas_qs[] = {2.0};
    double alphas_vals[] = {0.118};

    // Extract the ranges for the momentum x and scale Q2
    auto x_range = ref_pdf.param_range(NEOPDF_SUBGRID_PARAMS_MOMENTUM);
    auto q2_range = ref_pdf.param_range(NEOPDF_SUBGRID_PARAMS_SCALE);

    NeoPDFMetaData meta = {
        .set_desc = "NNPDF40_nnlo_as_01180 collection",
        .set_index = 0,
        .num_members = (uint32_t)neo_pdfs.size(),
        .x_min = x_range[0],
        .x_max = x_range[1],
        .q_min = sqrt(q2_range[0]),
        .q_max = sqrt(q2_range[1]),
        .flavors = pids.data(),
        .num_flavors = (size_t)pids.size(),
        .format = "neopdf",
        .alphas_q_values = alphas_qs,
        .num_alphas_q = 1,
        .alphas_vals = alphas_vals,
        .num_alphas_vals = 1,
        .polarised = false,
        .set_type = SET_TYPE_SPACE_LIKE,
        .interpolator_type = INTERPOLATOR_TYPE_LOG_BICUBIC,
        .error_type = "replicas",
        .hadron_pid = 2212,
    };

    // Check if `NEOPDF_DATA_PATH` is defined and store the Grid there.
    const char* filename = "check-writer-oop.neopdf.lz4";
    const char* neopdf_path = std::getenv("NEOPDF_DATA_PATH");
    std::string output_path = neopdf_path
        ? std::string(neopdf_path) + (std::string(neopdf_path).back() == '/' ? "" : "/") + filename
        : filename;

    // Write the PDF Grid into disk
    int result = neopdf_grid_compress(collection, &meta, output_path.c_str());
    if (result != 0) {
        std::cerr << "Compression failed with code " << result << "\n";
    } else {
        std::cout << "Compression succeeded!" << "\n";
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

    // Cleanup
    neopdf_gridarray_collection_free(collection);
    // The `neo_pdfs` object will be automatically destroyed, freeing the PDF members.

    return result == 0 ? 0 : 1;
}
