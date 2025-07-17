# C API Example

This example demonstrates how to use the NeoPDF C API to load PDF sets, evaluate parton
distributions, and perform statistical analysis across PDF members.

## Prerequisites

Build and install the C API as described in the [installation guide](../installation.md).
- Include the NeoPDF C headers and link against the shared library.

## Example 1: Loading and Evaluating PDFs

This example demonstrates the use of the NeoPDF C API to load both single and multiple PDF
members, evaluate parton distributions for a range of $x$ and $Q^2$ values, and compare
results to LHAPDF.

**Technical details:**
- The C API requires explicit memory management: objects created with `neopdf_pdf_load` or
  `neopdf_pdf_load_all` must be freed with `neopdf_pdf_free` or `neopdf_pdf_array_free` to
  avoid memory leaks.
- The evaluation of $x f(x, Q^2)$ and $\alpha_s(Q^2)$ is performed in nested loops for each parton,
  $x$, and $Q^2$ value, which can be parallelized for performance in user code.
- The code asserts that the results from NeoPDF and LHAPDF agree within a tight tolerance,
  providing robust validation.
- Error handling is performed via assertions and return codes; failures will terminate the program
  or print error messages.
- The example demonstrates direct interoperability with LHAPDF by using both APIs side by side for
  validation.

```cpp linenums="1"
#include <LHAPDF/PDF.h>
#include <neopdf_capi.h>
#include <cassert>
#include <cmath>
#include <cstdlib>
#include <iomanip>
#include <iostream>
#include <vector>

const double TOLERANCE= 1e-16;

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

void test_single_pdf() {
    std::cout << "=== Test Loading a Single PDF Member ===\n";

    // disable LHAPDF banners to guarantee deterministic output
    LHAPDF::setVerbosity(0);

    std::string pdfname = "NNPDF40_nnlo_as_01180";
    NeoPDFWrapper* neo_pdf = neopdf_pdf_load(pdfname.c_str(), 0);
    auto lha_pdf = std::unique_ptr<LHAPDF::PDF>(LHAPDF::mkPDF(pdfname, 0));

    std::vector<int> pids = {-5, -4, -3, -2, -1, 21, 1, 2, 3, 4, 5};
    std::vector<double> xs = geomspace(lha_pdf->xMin(), lha_pdf->xMax(), 200);
    std::vector<double> q2s = geomspace(lha_pdf->q2Min(), lha_pdf->q2Max(), 200);

    // Headers of the table to print the results
    std::cout << std::right
        << std::setw(6) << "pid"
        << std::setw(15) << "x"
        << std::setw(15) << "Q2"
        << std::setw(15) << "LHAPDF"
        << std::setw(15) << "NeoPDF"
        << std::setw(15) << "Rel. Diff." << "\n";
    std::cout << std::string(81, '-') << "\n";

    for (const auto &pid: pids) {
        for (const auto &x: xs) {
            for (const auto &q2: q2s) {
                double expected = lha_pdf->xfxQ2(pid, x, q2);
                double result = neopdf_pdf_xfxq2(neo_pdf, pid, x, q2);
                double reldif = std::abs(result - expected) / expected;

                assert(std::abs(result - expected) < TOLERANCE);

                // Print the results as a table
                std::cout << std::scientific << std::setprecision(8)
                    << std::right
                    << std::setw(6)  << pid
                    << std::setw(15) << x
                    << std::setw(15) << q2
                    << std::right
                    << std::setw(15) << expected
                    << std::setw(15) << result
                    << std::setw(15) << reldif << "\n";
                }
            }

    }

    // Delete PDF object from memory
    neopdf_pdf_free(neo_pdf);
}

void test_all_pdf_members() {
    std::cout << "=== Test Loading all the PDF Members ===\n";

    // disable LHAPDF banners to guarantee deterministic output
    LHAPDF::setVerbosity(0);

    std::string pdfname = "NNPDF40_nnlo_as_01180";
    NeoPDFMembers neo_pdfs = neopdf_pdf_load_all(pdfname.c_str());

    std::cout << "Loaded " << neo_pdfs.size << " PDF members\n";

    // Test case: evaluate a simple point across all members
    int pid = 1;
    double x = 1e-9;
    double q2 = 1.65 * 1.65;

    std::cout << "\nEvaluating xfxQ2 for pid=" << pid
              << ", x=" << std::scientific << x
              << ", Q2=" << q2 << " across all members:\n";

    std::cout << std::right
        << std::setw(8) << "Member"
        << std::setw(15) << "LHAPDF"
        << std::setw(15) << "NeoPDF"
        << std::setw(15) << "Rel. Diff." << "\n";
    std::cout << std::string(53, '-') << "\n";

    // Evaluate the same point across all PDF members
    std::vector<double> results;
    for (size_t i = 0; i < neo_pdfs.size; ++i) {
        NeoPDFWrapper* pdf = neo_pdfs.pdfs[i];
        auto lha_pdf = std::unique_ptr<LHAPDF::PDF>(LHAPDF::mkPDF(pdfname, i));

        double expected = lha_pdf->xfxQ2(pid, x, q2);
        double result = neopdf_pdf_xfxq2(pdf, pid, x, q2);

        double reldif = std::abs(result - expected) / expected;
        assert(std::abs(result - expected) < TOLERANCE);
        results.push_back(result);

        std::cout << std::right
            << std::setw(8) << i
            << std::scientific << std::setprecision(8)
            << std::setw(15) << expected
            << std::setw(15) << result
            << std::setw(15) << reldif << "\n";
    }

    // Calculate some statistics
    double sum = 0.0;
    for (double result : results) {
        sum += result;
    }
    double mean = sum / results.size();

    double variance = 0.0;
    for (double result : results) {
        variance += (result - mean) * (result - mean);
    }
    variance /= results.size();
    double std_dev = std::sqrt(variance);

    std::cout << "\nStatistics across all members:\n";
    std::cout << "Mean: " << std::scientific << std::setprecision(8) << mean << "\n";
    std::cout << "Std Dev: " << std_dev << "\n";
    std::cout << "Relative Std Dev: " << std_dev / mean << "\n";

    // Delete objects from memory.
    neopdf_pdf_array_free(neo_pdfs);
}


int main() {
    // Test loading single PDF member
    test_single_pdf();

    // Test loading all the PDF members
    test_all_pdf_members();

    return EXIT_SUCCESS;
}
```

## Example 2: Writing a NeoPDF Grid

This example illustrates how to fill and write a NeoPDF grid using the C API. It demonstrates
the process of constructing a grid for each PDF member and serializing the collection to disk.

**Technical details:**
- The grid axes are defined as arrays for $x$, $Q^2$, parton IDs, nucleons, and $\alpha_s$ values,
  allowing for flexible and high-dimensional grids.
- The grid data is stored in a flattened array, with the layout `[nucleons][alphas][flavors][xs][q2s]`,
  which is required by the NeoPDF format for efficient storage and access.
- The `NeoPDFGridArrayCollection` manages the collection of grids and handles compression and
  serialization to disk.
- Metadata is filled in a `NeoPDFMetaData` struct, which includes information about the set, axis
  ranges, flavors, and interpolation type. This metadata is essential for correct interpretation
  of the grid file.
- All memory management must be handled manually; every object created must be freed with the
  appropriate function to avoid leaks.
- The output file is compressed and written in the `.neopdf.lz4` format, suitable for use with
  NeoPDF tools and APIs.
- Error handling is performed via return codes; any failure in grid creation or writing is reported
  to the user, and resources are cleaned up accordingly.

```cpp linenums="1"
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

    // Example grid axes (small for speed)
    // TODO: Replace `min` and `max` values with the actual ranges
    std::vector<int> pids = {-5, -4, -3, -2, -1, 21, 1, 2, 3, 4, 5};
    std::vector<double> xs = geomspace(1e-9, 1.0, 50);
    std::vector<double> q2s = geomspace(2.73, 1e10, 50);
    double nucleons[] = {1.0};
    double alphas[] = {0.118};

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
            nucleons, 1,
            alphas, 1,
            xs.data(), xs.size(),
            q2s.data(), q2s.size(),
            grid_data.data(), grid_data.size()
        );
        if (add_subgrid != NEO_PDF_RESULT_SUCCESS) {
            std::cerr << "Failed to add subgrid for member: " << m << "!\n";
            neopdf_grid_free(grid);
            continue;
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

    // Fill metadata (example values)
    double alphas_qs[] = {2.0};
    double alphas_vals[] = {0.118};
    NeoPDFMetaData meta = {
        .set_desc = "NNPDF40_nnlo_as_01180 collection",
        .set_index = 0,
        .num_members = (uint32_t)neo_pdfs.size,
        .x_min = xs.front(),
        .x_max = xs.back(),
        .q_min = q2s.front(),
        .q_max = q2s.back(),
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
```
