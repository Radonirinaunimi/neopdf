#include <cstddef>
#include <neopdf_capi.h>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <vector>

template<typename T>
std::vector<T> geomspace(T start, T stop, int num, bool endpoint = false) {
    std::vector<T> result(num);

    if (num == 1) {
        result[0] = start;
        return result;
    }

    T log_start = std::log(start);
    T log_stop = std::log(stop);
    T step = (log_stop - log_start) / (endpoint ? (num - 1) : num);

    for (int i = 0; i < num; ++i) {
        result[i] = std::exp(log_start + i * step);
    }

    return result;
}

int main() {
    const char* pdfname = "NNPDF40_nnlo_as_01180";
    // Load all PDF members
    NeoPDFMembers neo_pdfs = neopdf_pdf_load_all(pdfname);
    if (neo_pdfs.size == 0) {
        std::cerr << "Failed to load any PDF members!\n";
        return 1;
    }
    std::cout << "Loaded " << neo_pdfs.size << " PDF members\n";

    // Extract the PID values of the PDF set
    std::size_t num_pids = neopdf_pdf_num_pids(neo_pdfs.pdfs[0]);
    std::vector<int> pids(num_pids);
    neopdf_pdf_pids(neo_pdfs.pdfs[0], pids.data(), num_pids);

    // Extrac the number of subgrids
    std::size_t num_subgrids = neopdf_pdf_num_subgrids(neo_pdfs.pdfs[0]);

    // // Example grid axes (small for speed)
    // // TODO: Replace `min` and `max` values with the actual ranges
    // std::vector<int> pids = {-5, -4, -3, -2, -1, 21, 1, 2, 3, 4, 5};
    // std::vector<double> xs = geomspace(1e-9, 1.0, 50);
    // std::vector<double> q2s = geomspace(2.73, 1e10, 50);
    // double nucleons[] = {1.0};
    // double alphas[] = {0.118};

    // Create a collection
    NeoPDFGridArrayCollection* collection = neopdf_gridarray_collection_new();
    if (!collection) {
        std::cerr << "Failed to create grid array collection!\n";
        neopdf_pdf_array_free(neo_pdfs);
        return 1;
    }

    // For each member, build a grid
    for (size_t m = 0; m < neo_pdfs.size; ++m) {
        NeoPDFWrapper* pdf = neo_pdfs.pdfs[m];
        NeoPDFGrid* grid = neopdf_grid_new();

        if (!grid) {
            std::cerr << "Failed to create grid for member: " << m << "!\n";
            continue;
        }

        // Loop over the Subgrids
        for (std::size_t subgrid_idx = 0; subgrid_idx != num_subgrids; subgrid_idx++) {
            // Extrac the parameter values of the given subgrid
            // --- nucleons ---
            std::vector<std::size_t> nucleons_subgrid_shape(num_subgrids);
            neopdf_pdf_subgrids_shape_for_param(
                pdf,
                nucleons_subgrid_shape.data(),
                num_subgrids,
                NEOPDF_SUBGRID_PARAMS_NUCLEONS
            );
            std::vector<double> nucleons(nucleons_subgrid_shape[subgrid_idx]);
            neopdf_pdf_subgrids_for_param(
                pdf,
                nucleons.data(),
                NEOPDF_SUBGRID_PARAMS_NUCLEONS,
                num_subgrids,
                nucleons_subgrid_shape.data(),
                subgrid_idx
            );
            // --- alphas ---
            std::vector<std::size_t> alphas_subgrid_shape(num_subgrids);
            neopdf_pdf_subgrids_shape_for_param(
                pdf,
                alphas_subgrid_shape.data(),
                num_subgrids,
                NEOPDF_SUBGRID_PARAMS_ALPHAS
            );
            std::vector<double> alphas(alphas_subgrid_shape[subgrid_idx]);
            neopdf_pdf_subgrids_for_param(
                pdf,
                alphas.data(),
                NEOPDF_SUBGRID_PARAMS_ALPHAS,
                num_subgrids,
                alphas_subgrid_shape.data(),
                subgrid_idx
            );
            // --- Momentum x ---
            std::vector<std::size_t> xs_subgrid_shape(num_subgrids);
            neopdf_pdf_subgrids_shape_for_param(
                pdf,
                xs_subgrid_shape.data(),
                num_subgrids,
                NEOPDF_SUBGRID_PARAMS_MOMENTUM
            );
            std::vector<double> xs(xs_subgrid_shape[subgrid_idx]);
            neopdf_pdf_subgrids_for_param(
                pdf,
                xs.data(),
                NEOPDF_SUBGRID_PARAMS_MOMENTUM,
                num_subgrids,
                xs_subgrid_shape.data(),
                subgrid_idx
            );
            // --- Scale Q2 ---
            std::vector<std::size_t> q2s_subgrid_shape(num_subgrids);
            neopdf_pdf_subgrids_shape_for_param(
                pdf,
                q2s_subgrid_shape.data(),
                num_subgrids,
                NEOPDF_SUBGRID_PARAMS_SCALE
            );
            std::vector<double> q2s(q2s_subgrid_shape[subgrid_idx]);
            neopdf_pdf_subgrids_for_param(
                pdf,
                q2s.data(),
                NEOPDF_SUBGRID_PARAMS_SCALE,
                num_subgrids,
                q2s_subgrid_shape.data(),
                subgrid_idx
            );

            // Compute grid_data: [nucleons][alphas][flavors][xs][q2s]
            std::vector<double> grid_data;
            for (size_t f = 0; f < pids.size(); ++f) {
                int pid = pids[f];
                for (size_t xi = 0; xi < xs.size(); ++xi) {
                    for (size_t qi = 0; qi < q2s.size(); ++qi) {
                        double val = neopdf_pdf_xfxq2(pdf, pid, xs[xi], q2s[qi]);
                        grid_data.push_back(val);
                    }
                }
            }

            // Add subgrid
            int add_subgrid = neopdf_grid_add_subgrid(
                grid,
                nucleons.data(), nucleons.size(),
                alphas.data(), alphas.size(),
                xs.data(), xs.size(),
                q2s.data(), q2s.size(),
                grid_data.data(), grid_data.size()
            );
            if (add_subgrid != NEOPDF_RESULT_SUCCESS) {
                std::cerr << "Failed to add subgrid for member: " << m << "!\n";
                neopdf_grid_free(grid);
                continue;
            }
        }

        // Set flavor PIDs
        int add_flavors = neopdf_grid_set_flavors(grid, pids.data(), pids.size());
        if (add_flavors != 0) {
            std::cerr << "Failed to set flavors for member: " << m << "!\n";
            neopdf_grid_free(grid);
            continue;
        }

        // Add grid to collection
        int add_grid = neopdf_gridarray_collection_add_grid(collection, grid);
        if (add_grid != 0) {
            std::cerr << "Failed to add grid to collection for member: " << m << "!\n";
            neopdf_grid_free(grid);
            continue;
        }
        std::cout << "Added grid for member " << m << "\n";
    }

    // Fill the running of alphas with some random values
    double alphas_qs[] = {2.0};
    double alphas_vals[] = {0.118};

    // Extrac the ranges for the momentum x and scale Q2
    std::vector<double> x_range(2);
    std::vector<double> q2_range(2);
    neopdf_pdf_param_range(neo_pdfs.pdfs[0], NEOPDF_SUBGRID_PARAMS_MOMENTUM, x_range.data());
    neopdf_pdf_param_range(neo_pdfs.pdfs[0], NEOPDF_SUBGRID_PARAMS_SCALE, q2_range.data());

    NeoPDFMetaData meta = {
        .set_desc = "NNPDF40_nnlo_as_01180 collection",
        .set_index = 0,
        .num_members = (uint32_t)neo_pdfs.size,
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
    const char* output_path = "check-writer.neopdf.lz4";
    int result = neopdf_grid_compress(collection, &meta, output_path);
    if (result != 0) {
        std::cerr << "Compression failed with code " << result << "\n";
    } else {
        std::cout << "Compression succeeded! Output: " << output_path << "\n";
    }

    // Cleanup
    neopdf_gridarray_collection_free(collection);
    neopdf_pdf_array_free(neo_pdfs);

    return result == 0 ? 0 : 1;
}
