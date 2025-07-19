#include <NeoPDF.hpp>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <vector>

int main() {
    const char* pdfname = "NNPDF40_nnlo_as_01180";
    // Load all PDF members
    neopdf::NeoPDFs neo_pdfs(pdfname);
    if (neo_pdfs.size() == 0) {
        std::cerr << "Failed to load any PDF members!\n";
        return 1;
    }
    std::cout << "Loaded " << neo_pdfs.size() << " PDF members\n";

    // Get the first PDF as a reference for metadata
    neopdf::NeoPDF& ref_pdf = neo_pdfs[0];

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
        neopdf::NeoPDF& pdf = neo_pdfs[m];
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
            if (neopdf_grid_add_subgrid(
                    grid,
                    nucleons.data(), nucleons.size(),
                    alphas.data(), alphas.size(),
                    xs.data(), xs.size(),
                    q2s.data(), q2s.size(),
                    grid_data.data(), grid_data.size()
                ) != NEOPDF_RESULT_SUCCESS) {
                std::cerr << "Failed to add subgrid for member: " << m << "!\n";
                neopdf_grid_free(grid);
                member_ok = false;
                break;
            }
        }

        if (!member_ok) continue;

        // Set flavor PIDs
        if (neopdf_grid_set_flavors(grid, pids.data(), pids.size()) != NEOPDF_RESULT_SUCCESS) {
            std::cerr << "Failed to set flavors for member: " << m << "!\n";
            neopdf_grid_free(grid);
            continue;
        }

        // Add grid to collection
        if (neopdf_gridarray_collection_add_grid(collection, grid) != NEOPDF_RESULT_SUCCESS) {
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
        .polarised = 0,
        .set_type = 0, // NEOPDF_SET_TYPE_PDF
        .interpolator_type = 2, // NEOPDF_INTERP_LOGBICUBIC
    };

    // Write to disk
    const char* output_path = "check-writer-oop.neopdf.lz4";
    int result = neopdf_grid_compress(collection, &meta, output_path);
    if (result != 0) {
        std::cerr << "Compression failed with code " << result << "\n";
    } else {
        std::cout << "Compression succeeded! Output: " << output_path << "\n";
    }

    // Cleanup
    neopdf_gridarray_collection_free(collection);
    // The `neo_pdfs` object will be automatically destroyed, freeing the PDF members.

    return result == 0 ? 0 : 1;
}
