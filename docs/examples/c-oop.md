# C++ OOP API Example

This example briefly demonstrates how to use the `NeoPDF` C++ OOP API to load and evaluate parton
distributions.

## Prerequisites

Build and install the C++ API as described in the [installation guide](../installation.md). The
C++ OOP header is needed for the following examples.

## Example 1: Loading and Evaluating PDFs

This example demonstrates the use of the `NeoPDF` C++ OOP API to load both single and multiple PDF
members, evaluate parton distributions for a range of $x$ and $Q^2$ values, and compare results
to LHAPDF.

**Technical details:**

- The `NeoPDF` and `NeoPDFs` objects manage their own memory and automatically release resources
  when they go out of scope (RAII).
- The evaluation of $x f(x, Q^2)$ and $\alpha_s(Q^2)$ is vectorized over the input axes for
  efficiency.
- The code asserts that the results from `NeoPDF` and LHAPDF agree within a tight tolerance,
  providing a robust validation.

```cpp linenums="1"
#include <LHAPDF/PDF.h>
#include <LHAPDF/GridPDF.h>
#include <neopdf_capi.h>
#include <NeoPDF.hpp>
#include <cassert>
#include <cmath>
#include <iomanip>
#include <iostream>
#include <string>
#include <vector>

using namespace neopdf;

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

template<typename T>
std::vector<T> linspace(T start, T stop, int num, bool endpoint = true) {
    std::vector<T> result(num);

    if (num == 1) {
        result[0] = start;
        return result;
    }

    T step = (stop - start) / (endpoint ? (num - 1) : num);

    for (int i = 0; i < num; ++i) {
        result[i] = start + i * step;
    }

    return result;
}

void test_xfxq2() {
    std::cout << "=== Test xfxQ2 for single PDF member ===\n";

    // disable LHAPDF banners to guarantee deterministic output
    LHAPDF::setVerbosity(0);

    std::string pdfname = "NNPDF40_nnlo_as_01180";
    NeoPDF neo_pdf(pdfname.c_str(), 0);
    const LHAPDF::PDF* basepdf = LHAPDF::mkPDF(pdfname);
    const LHAPDF::GridPDF& lha_pdf = * dynamic_cast<const LHAPDF::GridPDF*>(basepdf);

    std::vector<int> pids = {-5, -4, -3, -2, -1, 21, 1, 2, 3, 4, 5};
    std::vector<double> xs = geomspace(neo_pdf.x_min(), neo_pdf.x_max(), 200);
    std::vector<double> q2s = geomspace(neo_pdf.q2_min(), neo_pdf.q2_max(), 200);

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
                double expected = lha_pdf.xfxQ2(pid, x, q2);
                double result = neo_pdf.xfxQ2(pid, x, q2);
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
}

void test_alphas_q2() {
    std::cout << "=== Test alphasQ2 for single PDF member ===\n";

    // disable LHAPDF banners to guarantee deterministic output
    LHAPDF::setVerbosity(0);

    std::string pdfname = "NNPDF40_nnlo_as_01180";
    NeoPDF neo_pdf(pdfname.c_str(), 0);
    const LHAPDF::PDF* basepdf = LHAPDF::mkPDF(pdfname);
    const LHAPDF::GridPDF& lha_pdf = * dynamic_cast<const LHAPDF::GridPDF*>(basepdf);

    std::vector<double> q2_points = linspace(4.0, 1e10, 500);

    // Headers of the table to print the results
    std::cout << std::right
        << std::setw(15) << "Q2"
        << std::setw(15) << "LHAPDF"
        << std::setw(15) << "NeoPDF"
        << std::setw(15) << "Rel. Diff." << "\n";
    std::cout << std::string(60, '-') << "\n";

    for (const auto& q2: q2_points) {
        double expected = lha_pdf.alphasQ2(q2);
        double result = neo_pdf.alphasQ2(q2);
        double reldif = std::abs(result - expected) / expected;

        assert(std::abs(result - expected) < TOLERANCE);

        // Print the results as a table
        std::cout << std::scientific << std::setprecision(8)
            << std::right
            << std::setw(15) << q2
            << std::right
            << std::setw(15) << expected
            << std::setw(15) << result
            << std::setw(15) << reldif << "\n";
    }
}

void test_all_pdf_members() {
    std::cout << "=== Test PDFs class (loading all members) ===\n";

    // disable LHAPDF banners to guarantee deterministic output
    LHAPDF::setVerbosity(0);

    std::string pdfname = "NNPDF40_nnlo_as_01180";
    NeoPDFs neo_pdfs(pdfname.c_str());

    std::cout << "Loaded " << neo_pdfs.size() << " PDF members\n";

    // Test case: evaluate a simple point across all members
    int pid = 1;
    double x = 1e-9;
    double q2 = 1.65 * 1.65;

    std::cout << std::right
        << std::setw(8) << "Member"
        << std::setw(15) << "LHAPDF"
        << std::setw(15) << "NeoPDF"
        << std::setw(15) << "Rel. Diff." << "\n";
    std::cout << std::string(53, '-') << "\n";

    // Evaluate the same point across all PDF members
    std::vector<double> results;
    for (size_t i = 0; i < neo_pdfs.size(); ++i) {
        const LHAPDF::PDF* basepdf = LHAPDF::mkPDF(pdfname, i);
        const LHAPDF::GridPDF& lha_pdf = * dynamic_cast<const LHAPDF::GridPDF*>(basepdf);

        double expected = lha_pdf.xfxQ2(pid, x, q2);
        double result = neo_pdfs[i].xfxQ2(pid, x, q2);

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
}

void test_raw_load_all() {
    std::cout << "=== Test raw neopdf_pdf_load_all ===\n";
    NeoPDFMembers raw_pdfs = neopdf_pdf_load_all("NNPDF40_nnlo_as_01180");
    std::cout << "Loaded " << raw_pdfs.size << " PDF members (raw call)\n";
    neopdf_pdf_array_free(raw_pdfs);
}


int main() {
    // Test the computation of the PDF interpolations
    test_xfxq2();

    // Test the computation of the `alphas` interpolations
    test_alphas_q2();

    // Test the PDF interpolations by loading all the members
    test_all_pdf_members();

    return EXIT_SUCCESS;
}
```

## Example 2: Filling and Writing a NeoPDF Grid

This example illustrates how to fill and write a `NeoPDF` grid using the C++ OOP API. It
demonstrates the process of constructing a grid for each PDF member and serializing the
collection to disk.

The filling of the PDF grid in the following example assumes no dependence in the nucleon
numbers $A$ and strong coupling $\alpha_s$ (standard LHAPDF-like PDF). Refer to the Section
below in the case the grid should explicitly depend on more parameters.

**Technical details:**

- The grid axes are defined as vectors for $x$, $Q^2$, parton IDs, nucleons, and $\alpha_s$
  values.
- The grid data is stored in a 5D array, with the layout `[nucleons][alphas][flavors][xs][q2s]`.
- The `GridWriter` class manages the collection of grids and handles compression and serialization to disk.
- Metadata is filled in a `MetaData` object, which includes information about the set, axis ranges, flavors,
  and interpolation type. This metadata is essential for correct interpretation of the grid file.
- All memory management is automatic; no manual deallocation is required.
- The output file is compressed and written in the `.neopdf.lz4` format, suitable for use with `NeoPDF` (CLI)
  tools and APIs.

!!! tip "NOTE"

    The following example fills the `NeoPDF` grid by re-computing the values of the subgrids
    from another set. This makes it possible to explicitly check that the filling of the grid
    is correct. However, this makes the codes very verbose. To easily spot the parts that
    actually fills the grid, some lines are highlighted.

```cpp linenums="1" hl_lines="62-70 73-80 92 100 117-135 145"
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
        .set_type = SET_TYPE_PDF,
        .interpolator_type = INTERPOLATOR_TYPE_LOG_BICUBIC,
    };

    // Check if `NEOPDF_DATA_PATH` is defined and store the Grid there.
    const char* filename = "check-writer.neopdf.lz4";
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

    return result == 0 ? 0 : 1;
}
```

## Example 3: Filling and Writing a NeoPDF Grid with generic Parameters

In the case where the PDF grid depends on more parameters, the filling of `grid_data`
in the above example simply now becomes:

```cpp linenums="1"
std::vector<double> grid_data;
for (double nucleon : nucleons) {
    for (double alpha_s : alphas) {
        for (double x : xs) {
            for (double q2 : q2s) {
                for (int pid : pids) {
                    std::vector<double> params = {nucleon, alpha_s, x, q2};
                    double val = pdf.xfxQ2_ND(pid, params.data());
                    grid_data.push_back(val);
                }
            }
        }
    }
}
```
