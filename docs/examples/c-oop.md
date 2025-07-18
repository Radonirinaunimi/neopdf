# C++ OOP API Example

This example demonstrates how to use the NeoPDF C++ OOP API to load PDF sets, evaluate parton
distributions, and perform statistical analysis across PDF members.

## Prerequisites

Build and install the C++ API as described in the [installation guide](../installation.md).
- Include the NeoPDF headers and link against the shared library.

## Example 1: Loading and Evaluating PDFs

This example demonstrates the use of the NeoPDF C++ OOP API to load both single and multiple PDF
members, evaluate parton distributions for a range of $x$ and $Q^2$ values, and compare results
to LHAPDF.

**Technical details:**
- The `NeoPDF` and `NeoPDFs` objects manage their own memory and automatically release resources
  when they go out of scope (RAII).
- The evaluation of $x f(x, Q^2)$ and $\alpha_s(Q^2)$ is vectorized over the input axes for
  efficiency.
- The code asserts that the results from NeoPDF and LHAPDF agree within a tight tolerance,
  providing a robust validation.
- The use of standard containers (`std::vector`) ensures safe and efficient memory management.
- Error handling is performed via assertions; any mismatch or failure will terminate the program,
  making debugging straightforward.
- The example demonstrates direct interoperability with LHAPDF by using both APIs side by side.

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

## Example 2: Writing a NeoPDF Grid

This example illustrates how to fill and write a NeoPDF grid using the C++ OOP API. It
demonstrates the process of constructing a grid for each PDF member and serializing the
collection to disk.

**Technical details:**
- The grid axes are defined as vectors for $x$, $Q^2$, parton IDs, nucleons, and $\alpha_s$
  values, allowing for flexible and high-dimensional grids.
- The grid data is stored in a flattened vector, with the layout `[nucleons][alphas][flavors][xs][q2s]`,
  which is required by the NeoPDF format for efficient storage and access.
- The `GridWriter` class manages the collection of grids and handles compression and serialization to disk.
- Metadata is filled in a `MetaData` object, which includes information about the set, axis ranges, flavors,
  and interpolation type. This metadata is essential for correct interpretation of the grid file.
- All memory management is automatic; no manual deallocation is required.
- The output file is compressed and written in the `.neopdf.lz4` format, suitable for use with NeoPDF tools
  and APIs.
- Error handling is performed via standard output and return codes; any failure in grid creation or writing
  is reported to the user.

```cpp linenums="1"
#include <NeoPDF.hpp>
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
    neopdf::NeoPDFs neo_pdfs(pdfname);
    if (neo_pdfs.size() == 0) {
        std::cerr << "Failed to load any PDF members!\n";
        return 1;
    }
    std::cout << "Loaded " << neo_pdfs.size() << " PDF members\n";

    // Example grid axes (small for speed)
    // TODO: Replace `min` and `max` values with the actual ranges
    std::vector<int32_t> pids = {-5, -4, -3, -2, -1, 21, 1, 2, 3, 4, 5};
    std::vector<double> xs = geomspace(1e-9, 1.0, 50);
    std::vector<double> q2s = geomspace(2.73, 1e10, 50);
    std::vector<double> nucleons = {1.0};
    std::vector<double> alphas = {0.118};

    // Create a GridWriter
    neopdf::GridWriter writer;

    // For each member, build a grid and add to writer
    for (size_t m = 0; m < neo_pdfs.size(); ++m) {
        neopdf::NeoPDF& pdf = neo_pdfs[m];

        // Compute grid_data: [nucleons][alphas][flavors][xs][q2s]
        std::vector<double> grid_data;
        for (size_t f = 0; f < pids.size(); ++f) {
            int pid = pids[f];
            for (size_t xi = 0; xi < xs.size(); ++xi) {
                for (size_t qi = 0; qi < q2s.size(); ++qi) {
                    double val = pdf.xfxQ2(pid, xs[xi], q2s[qi]);
                    grid_data.push_back(val);
                }
            }
        }

        // Add grid to writer
        writer.add_grid(
            nucleons,
            alphas,
            xs,
            q2s,
            grid_data,
            pids
        );
        std::cout << "Added grid for member " << m << "\n";
    }

    // Fill metadata
    neopdf::MetaData meta;
    meta.set_desc = "NNPDF40_nnlo_as_01180 collection";
    meta.set_index = 0;
    meta.num_members = (uint32_t)neo_pdfs.size();
    meta.x_min = xs.front();
    meta.x_max = xs.back();
    meta.q_min = q2s.front();
    meta.q_max = q2s.back();
    meta.flavors = pids;
    meta.format = "neopdf";
    meta.alphas_q_values = {2.0};
    meta.alphas_vals = {0.118};
    meta.polarised = false;
    meta.set_type = neopdf::SetType::Pdf;
    meta.interpolator_type = neopdf::InterpolatorType::LogBicubic;

    // Write to disk
    const std::string output_path = "check-writer-oop.neopdf.lz4";
    writer.compress(meta, output_path);

    std::cout << "Compression succeeded! Output: " << output_path << "\n";

    return 0;
}
```
