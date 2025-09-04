#ifndef TMDLIB_HPP
#define TMDLIB_HPP

#include "neopdf_tmdlib/src/lib.rs.h"
#include "rust/cxx.h"

#include <memory>

namespace TMDlib {
    class TMD;
}

std::unique_ptr<TMDlib::TMD> make_tmd();

void tmd_init(TMDlib::TMD& tmd, rust::Str setname, int member);
void tmd_init_set(TMDlib::TMD& tmd, rust::Str setname);
size_t tmd_get_num_members(TMDlib::TMD& tmd);
double tmd_get_xmin(TMDlib::TMD& tmd);
double tmd_get_xmax(TMDlib::TMD& tmd);
double tmd_get_q2min(TMDlib::TMD& tmd);
double tmd_get_q2max(TMDlib::TMD& tmd);
double tmd_get_ktmin(TMDlib::TMD& tmd);
double tmd_get_ktmax(TMDlib::TMD& tmd);
rust::Vec<double> tmd_pdf(TMDlib::TMD& tmd, double x, double kt, double q);
void tmd_set_verbosity(TMDlib::TMD& tmd, int verbosity);

#endif
