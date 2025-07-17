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
