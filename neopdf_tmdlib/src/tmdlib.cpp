#include "neopdf_tmdlib/src/tmdlib.hpp"
#include "tmdlib/TMDlib.h"
#include <string>
#include <vector>
#include <iterator>
#include <cmath>

template <typename T>
rust::Vec<T> std_vector_to_rust_vec(std::vector<T> vector)
{
    rust::Vec<T> result;
    result.reserve(vector.size());
    std::move(vector.begin(), vector.end(), std::back_inserter(result));
    return result;
}

std::unique_ptr<TMDlib::TMD> make_tmd() {
    return std::unique_ptr<TMDlib::TMD>(new TMDlib::TMD());
}

void tmd_init(TMDlib::TMD& tmd, rust::Str setname, int member) {
    tmd.TMDinit(std::string(setname), member);
}

void tmd_init_set(TMDlib::TMD& tmd, rust::Str setname) {
    tmd.TMDinit(std::string(setname));
}

size_t tmd_get_num_members(TMDlib::TMD& tmd) {
    return tmd.TMDgetNumMembers();
}

double tmd_get_xmin(TMDlib::TMD& tmd) {
    return tmd.TMDgetXmin();
}

double tmd_get_xmax(TMDlib::TMD& tmd) {
    return tmd.TMDgetXmax();
}

double tmd_get_q2min(TMDlib::TMD& tmd) {
    return tmd.TMDgetQ2min();
}

double tmd_get_q2max(TMDlib::TMD& tmd) {
    return tmd.TMDgetQ2max();
}

rust::Vec<double> tmd_pdf(TMDlib::TMD& tmd, double x, double kt, double q) {
    std::vector<double> pdfs = tmd.TMDpdf(x, 0.0, kt, q);
    return std_vector_to_rust_vec(pdfs);
}

void tmd_set_verbosity(TMDlib::TMD& tmd, int verbosity) {
    tmd.setVerbosity(verbosity);
}
