# C++ OOP API Example

This example demonstrates how to use the NeoPDF C++ OOP API to load PDF sets, evaluate parton distributions, and perform statistical analysis across PDF members.

## Prerequisites

Build and install the C++ API as described in the [installation guide](../installation.md).
- Include the NeoPDF headers and link against the shared library.

## Example: Loading and Evaluating a Single PDF Member

The following function loads a single PDF member and evaluates the PDF for various partons, $x$, and $Q^2$ values. It compares the results to reference values and prints a table of results.

```cpp
#include <neopdf_capi.h>
#include <NeoPDF.hpp>
#include <cassert>
#include <cmath>
#include <iomanip>
#include <iostream>
#include <string>
#include <tuple>
#include <vector>

using namespace neopdf;

const double TOLERANCE= 1e-16;

void test_xfxq2() {
    std::cout << "=== Test xfxQ2 for single PDF member ===\n";

    NeoPDF xpdf("NNPDF40_nnlo_as_01180", 0);

    std::vector<std::tuple<int, double, double, double>> cases = {
        {1, 1e-9, 1.65 * 1.65, 1.4254154},
        {2, 1e-9, 1.65 * 1.65, 1.4257712},
        {21, 1e-9, 1.65 * 1.65, 0.14844111},
        {1, 1.2970848e-9, 1.65 * 1.65, 1.3883271},
        {2, 1.2970848e-9, 1.65 * 1.65, 1.3887002},
        {21, 1.2970848e-9, 1.65 * 1.65, 0.15395356},
        {1, 1.2970848e-9, 1.9429053 * 1.9429053, 1.9235433},
        {2, 1.2970848e-9, 1.9429053 * 1.9429053, 1.9239212},
        {21, 1.2970848e-9, 1.9429053 * 1.9429053, -3.164867}
    };

    // Headers of the table to print the results
    std::cout << std::right
        << std::setw(6) << "pid"
        << std::setw(15) << "x"
        << std::setw(15) << "Q2"
        << std::setw(15) << "LHAPDF"
        << std::setw(15) << "NeoPDF"
        << std::setw(15) << "Rel. Diff." << "\n";
    std::cout << std::string(71, '-') << "\n";

    for (const auto& test_case : cases) {
        int pid;
        double x, q2, expected;

        std::tie(pid, x, q2, expected) = test_case;
        double result = xpdf.xfxQ2(pid, x, q2);
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

## Example: Evaluating $\alpha_s(Q^2)$

This function demonstrates how to evaluate the strong coupling $\alpha_s$ at different $Q^2$ values for a given PDF member.

```cpp
void test_alphas_q2() {
    std::cout << "=== Test alphasQ2 for single PDF member ===\n";

    NeoPDF xpdf("NNPDF40_nnlo_as_01180", 0);

    std::vector<std::tuple<double, double>> cases ={
        {1e5 * 1e5, 0.057798546},
        {1.65 * 1.65, 0.33074891},
        {4.0, 0.30095312523656437},
        {2.75, 0.32992260049326716},
        {100.0, 0.17812270669689784}
    };

    // Headers of the table to print the results
    std::cout << std::right
        << std::setw(15) << "Q2"
        << std::setw(15) << "LHAPDF"
        << std::setw(15) << "NeoPDF"
        << std::setw(15) << "Rel. Diff." << "\n";
    std::cout << std::string(60, '-') << "\n";

    for (const auto& test_case: cases) {
        double q2, expected;

        std::tie(q2, expected) = test_case;
        double result = xpdf.alphasQ2(q2);
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

## Example: Working with All PDF Members

You can load all members of a PDF set and evaluate the same point across all members. This is useful for uncertainty estimation and statistical analysis.

```cpp
void test_all_pdf_members() {
    std::cout << "=== Test PDFs class (loading all members) ===\n";

    NeoPDFs xpdfs("NNPDF40_nnlo_as_01180");

    std::cout << "Loaded " << xpdfs.size() << " PDF members\n";

    // Test case: evaluate a simple point across all members
    int pid = 1;
    double x = 1e-9;
    double q2 = 1.65 * 1.65;

    std::cout << "\nEvaluating xfxQ2 for pid=" << pid
              << ", x=" << std::scientific << x
              << ", Q2=" << q2 << " across all members:\n";

    std::cout << std::right
        << std::setw(8) << "Member"
        << std::setw(15) << "Result" << "\n";
    std::cout << std::string(23, '-') << "\n";

    // Evaluate the same point across all PDF members
    std::vector<double> results;
    for (size_t i = 0; i < xpdfs.size(); ++i) {
        double result = xpdfs[i].xfxQ2(pid, x, q2);
        results.push_back(result);

        std::cout << std::right
            << std::setw(8) << i
            << std::scientific << std::setprecision(8)
            << std::setw(15) << result << "\n";
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

## Example: Raw C API Interoperability

You can also use the raw C API to load all PDF members and manage memory manually.

```cpp
void test_raw_load_all() {
    std::cout << "=== Test raw neopdf_pdf_load_all ===\n";
    NeoPDFMembers raw_pdfs = neopdf_pdf_load_all("NNPDF40_nnlo_as_01180");
    std::cout << "Loaded " << raw_pdfs.size << " PDF members (raw call)\n";
    neopdf_pdf_array_free(raw_pdfs);
}

## Main Function

The main function runs all the above tests:

```cpp
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

## Summary

- Load and evaluate single or multiple PDF members
- Compute $x f(x, Q^2)$ and $\alpha_s(Q^2)$
- Perform statistical analysis across PDF members
- Interoperate with the raw C API if needed

API reference documentation is coming soon.

```
