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
PDF XPDF("NNPDF40_nnlo_as_01180", 0);

void test_xfxq2() {
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
        double result = XPDF.xfxQ2(pid, x, q2);
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

void test_alphas_q2() {
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
        double result = XPDF.alphasQ2(q2);
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

int main() {
    // Test the computation of the PDF interpolations
    test_xfxq2();

    // Test the computation of the `alphas` interpolations
    test_alphas_q2();

    return EXIT_SUCCESS;
}
