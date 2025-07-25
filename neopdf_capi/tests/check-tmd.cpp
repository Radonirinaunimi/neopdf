#include "neopdf_capi.h"
#include "tmdlib/TMDlib.h"
#include <NeoPDF.hpp>
#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstddef>
#include <cstdlib>
#include <fstream>
#include <initializer_list>
#include <string>
#include <sstream>
#include <tmdlib/factories.h>
#include <vector>

using namespace TMDlib;
using namespace neopdf;

const double TOLERANCE = 1e-16;

struct Kinematics {
    std::vector<double> xs;
    std::vector<double> kts;
    std::vector<double> qs;
};

std::vector<double> compute_tmds(TMD& tmd, double x, double kt, double q2) {
    double xbar = 0.0;
    double mu = sqrt(q2);
    std::vector<double> pdfs = tmd.TMDpdf(x, xbar, kt, mu);

    return pdfs;
}

std::vector<double> parse_array(const std::string& line) {
    std::vector<double> result;
    size_t start = line.find('[');
    size_t end = line.find(']');

    if (start == std::string::npos || end == std::string::npos) {
        return result;
    }

    double value;
    std::string content = line.substr(start + 1, end - start - 1);
    std::replace(content.begin(), content.end(), ',', ' ');
    std::stringstream ss(content);
    while (ss >> value) { result.push_back(value); }

    return result;
}

Kinematics read_kinematics() {
    // Parse the Kinematics separately as it is difficult to retrieve
    std::ifstream input_kins("MAP22_N3LL.kinematics");

    if (!input_kins.is_open()) {
        input_kins.open("raw.data");
        if (!input_kins.is_open()) {
            return {{}, {}, {}};
        }
    }

    std::string line;
    std::vector<double> xs, kts, qs;
    while (std::getline(input_kins, line)) {
        if (line.find("qToQg:") != std::string::npos) { kts = parse_array(line); }
        else if (line.find("Qg:") != std::string::npos) { qs = parse_array(line); }
        else if (line.find("xg:") != std::string::npos) { xs = parse_array(line); }
    }
    input_kins.close();

    return { xs, kts, qs };
}

int main() {
    std::string setname = "MAP22_grids_FF_Km_N3LL";

    TMD tmd;
    tmd.setVerbosity(0);

    // Extract various informations from the TMD
    tmd.TMDinit(setname);
    std::size_t n_members = tmd.TMDgetNumMembers();
    double xmin = tmd.TMDgetXmin();
    double xmax = tmd.TMDgetXmax();
    double q2min = tmd.TMDgetQ2min();
    double q2max = tmd.TMDgetQ2max();

    // Define the kinematics
    Kinematics kins = read_kinematics();
    std::vector<double> xs = kins.xs;
    std::vector<double> qs = kins.qs;
    std::vector<double> kts = kins.kts;

    // Square the energy scale Q
    std::vector<double> q2s(qs.size());
    for (size_t i = 0; i < qs.size(); ++i) { q2s[i] = qs[i] * qs[i]; }

    // Define some physical parameters
    std::vector<int> pids = {-6, -5, -4, -3, -2, -1, 21, 1, 2, 3, 4, 5, 6};
    std::vector<double> nucleons = { 1.0 }; // assume to be a Proton
    std::vector<double> alphas = { 0.118 }; // assume to be determined with as=0.118

    // Instantiate NeoPDF grid writer
    GridWriter neopdf_writer;

    for (std::size_t m = 0; m != n_members; m++) {
        tmd.TMDinit(setname, m);
        std::cout << "Member " << m << " loaded!" << "\n";

        // Start a new grid for the current member
        neopdf_writer.new_grid();

        std::vector<double> grid_data;
        for (double kt : kts) {
            for (double x : xs) {
                for (double q2 : q2s) {
                    std::vector<double> tmd_pds = tmd.TMDpdf(x, 0.0, kt, sqrt(q2));
                    for (std::size_t pid_idx = 0; pid_idx != pids.size(); pid_idx++) {
                        grid_data.push_back(tmd_pds[pid_idx]);
                    }
                }
            }
        }

        // Add subgrid member to the Grid
        neopdf_writer.add_subgrid(
            nucleons,
            alphas,
            kts,
            xs,
            q2s,
            grid_data
        );

        // Finalize the Grid (inc. its subgrids) for this member.
        neopdf_writer.push_grid(pids);
    }

    // Fill the running of alphas with some random values
    std::vector<double> alphas_qs = { 91.1876 };
    std::vector<double> alphas_vals = { 0.118 };

    // Construct the Metadata
    PhysicsParameters phys_params = {
        .flavor_scheme = "fixed",
        .order_qcd = 2,
        .alphas_order_qcd = 2,
        .m_w = 80.352,
        .m_z = 91.1876,
        .m_up = 0.0,
        .m_down = 0.0,
        .m_strange = 0.0,
        .m_charm = 1.51,
        .m_bottom = 4.92,
        .m_top = 172.5,
    };

    MetaData meta = {
        .set_desc = "NNPDF40_nnlo_as_01180 collection",
        .set_index = 0,
        .num_members = (uint32_t)n_members,
        .x_min = xmin,
        .x_max = xmax,
        .q_min = sqrt(q2min),
        .q_max = sqrt(q2max),
        .flavors = pids,
        .format = "neopdf",
        .alphas_q_values = alphas_qs,
        .alphas_vals = alphas_vals,
        .polarised = false,
        .set_type = NEOPDF_SET_TYPE_SPACE_LIKE,
        .interpolator_type = NEOPDF_INTERPOLATOR_TYPE_LOG_TRICUBIC,
        .error_type = "replicas",
        .hadron_pid = 2212,
        .phys_params = phys_params,
    };

    // Check if `NEOPDF_DATA_PATH` is defined and store the Grid there.
    const char* filename = "check-tmds.neopdf.lz4";
    const char* neopdf_path = std::getenv("NEOPDF_DATA_PATH");
    std::string output_path = neopdf_path
        ? std::string(neopdf_path) + (std::string(neopdf_path).back() == '/' ? "" : "/") + filename
        : filename;

    // Write the PDF Grid into disk
    try {
        neopdf_writer.compress(meta, output_path);
        std::cout << "Compression succeeded!\n";
    } catch (const std::runtime_error& err) {
        std::cerr << "Compression failed: " << err.what() << "\n";
        return EXIT_FAILURE;
    }


    // If `NEOPDF_DATA_PATH` is defined, reload the grid and check ther results.
    if (neopdf_path) {
        int irep = 12;
        int pid_test_idx = 2;
        double x_test = xs[20];
        double q_test = qs[25];

        // Re-load the NeoPDF and TMDLib TMD sets
        NeoPDF wpdf(filename, irep);
        tmd.TMDinit(setname, irep);

        for (double kt : kts) {
            std::vector<double> refs = tmd.TMDpdf(x_test, 0.0, kt, q_test);
            double ref = refs[pid_test_idx]; // Up Quark

            std::vector<double> params = { kt,x_test, q_test * q_test };
            double res = wpdf.xfxQ2_ND(pids[pid_test_idx], params);

            assert(std::abs(ref - res) < TOLERANCE);
        }
    }

    return EXIT_SUCCESS;
}
