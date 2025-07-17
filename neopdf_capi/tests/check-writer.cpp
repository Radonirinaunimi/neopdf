#include <neopdf_capi.h>
#include <stdio.h>
#include <stdlib.h>

int main() {
    // Example data for a single subgrid
    double nucleons[] = {1.0};
    double alphas[] = {0.118};
    double xs[] = {0.01, 0.1, 0.5};
    double q2s[] = {2.0, 10.0};
    int flavors[] = {21, 1, 2};
    double grid_data[] = {
        // [nucleons][alphas][flavors][xs][q2s] = [1][1][3][3][2]
        // For each flavor, for each x, for each q2
        // Flavor 21
        0.1, 0.2, 0.3, 0.4, 0.5, 0.6,
        // Flavor 1
        0.7, 0.8, 0.9, 1.0, 1.1, 1.2,
        // Flavor 2
        1.3, 1.4, 1.5, 1.6, 1.7, 1.8
    };

    // create grid handle
    NeoPDFGrid* grid = neopdf_grid_new();
    if (!grid) {
        fprintf(stderr, "Failed to create grid handle\n");
        return 1;
    }

    // add subgrid
    int _add_subgrid = neopdf_grid_add_subgrid(
        grid,
        nucleons, 1,
        alphas, 1,
        xs, 3,
        q2s, 2,
        grid_data, 3*3*2
    );
    if (_add_subgrid != NEO_PDF_RESULT_SUCCESS) {
        fprintf(stderr, "Failed to add subgrid!\n");
        neopdf_grid_free(grid);
        return 1;
    }

    // set flavor PIDs
    int _add_pids = neopdf_grid_set_flavors(grid, flavors, 3);
    if (_add_pids != NEO_PDF_RESULT_SUCCESS) {
        fprintf(stderr, "Failed to set flavors\n");
        neopdf_grid_free(grid);
        return 1;
    }

    // fill the metadata
    double alphas_qs[] = {2.0};
    double alphas_vals[] = {0.118};
    NeoPDFMetaData meta = {
        .set_desc = "Test PDF set",
        .set_index = 0,
        .num_members = 1,
        .x_min = 0.01,
        .x_max = 0.5,
        .q_min = 2.0,
        .q_max = 10.0,
        .flavors = flavors,
        .num_flavors = 3,
        .format = "neopdf",
        .alphas_q_values = alphas_qs,
        .num_alphas_q = 1,
        .alphas_vals = alphas_vals,
        .num_alphas_vals = 1,
        .polarised = 0,
        .set_type = 0, // Corresponds to NEOPDF_SET_TYPE_PDF
        .interpolator_type = 2, // Corresponds to NEOPDF_INTERP_LOGBICUBIC
    };

    // Compress and write to file
    int result = neopdf_grid_compress(grid, &meta, "test_output.neopdf.lz4");
    if (result != 0) {
        fprintf(stderr, "Compression failed with code %d\n", result);
        neopdf_grid_free(grid);
        return 1;
    }

    printf("Compression succeeded!\n");
    neopdf_grid_free(grid);
    return 0;
}
