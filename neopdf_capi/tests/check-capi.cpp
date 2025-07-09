#include <neopdf_capi.h>
#include <cassert>
#include <cmath>
#include <cstdlib>
#include <iomanip>
#include <iostream>
#include <tuple>
#include <vector>

const double TOLERANCE= 1e-16;

void test_single_pdf() {
    std::cout << "=== Test Loading a Single PDF Member ===\n";

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
    std::cout << std::right
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
            << std::right
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
}

void test_all_pdf_members() {
    std::cout << "=== Test Loading all the PDF Members ===\n";

    NeoPDFArray pdf_array = neopdf_pdf_load_all("NNPDF40_nnlo_as_01180");

    std::cout << "Loaded " << pdf_array.size << " PDF members\n";

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
    for (size_t i = 0; i < pdf_array.size; ++i) {
        NeoPDF* pdf = pdf_array.pdfs[i];
        double result = neopdf_pdf_xfxq2(pdf, pid, x, q2);
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

    // Delete objects from memory.
    neopdf_pdf_array_free(pdf_array);
}


int main() {
    // Test loading single PDF member
    test_single_pdf();

    // Test loading all the PDF members
    test_all_pdf_members();

    return EXIT_SUCCESS;
}
