#include <neopdf_capi.h>
#include <cassert>
#include <cmath>
#include <cstdlib>
#include <iomanip>
#include <iostream>
#include <tuple>
#include <vector>

const double TOLERANCE= 1e-16;

int main() {
    NeoPDF* pdf = neopdf_pdf_load("NNPDF40_nnlo_as_01180", 0);

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
    std::cout << std::left
        << std::setw(6) << "pid"
        << std::setw(15) << "x"
        << std::setw(15) << "Q2"
        << std::setw(15) << "LHAPDF"
        << std::setw(15) << "NeoPDF"
        << std::setw(15) << "Rel. Diff." << "\n";
    std::cout << std::string(81, '-') << "\n";

    for (size_t i = 0; i < cases.size(); ++i) {
        int pid;
        double x, q2, expected;

        std::tie(pid, x, q2, expected) = cases[i];
        double result = neopdf_pdf_xfxq2(pdf, pid, x, q2);
        double reldif = std::abs(result - expected) / expected;

        assert(std::abs(result - expected) < TOLERANCE);

        // Print the results as a table
        std::cout << std::scientific << std::setprecision(8)
            << std::left
            << std::setw(6)  << pid
            << std::setw(15) << x
            << std::setw(15) << q2
            << std::right
            << std::setw(15) << expected
            << std::setw(15) << result
            << std::setw(15) << reldif << "\n";

    }

    // Delete PDF object from memory
    neopdf_pdf_free(pdf);

    return EXIT_SUCCESS;
}
