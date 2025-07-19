#pragma once

#include <cstddef>
#include <neopdf_capi.h>
#include <string>
#include <sys/types.h>
#include <vector>
#include <memory>
#include <stdexcept>

/** @brief Object Oriented interface to NeoPDF. */
namespace neopdf {

/** @brief C++ representation of NeoPDFMetaData. */
struct MetaData {
    std::string set_desc;
    uint32_t set_index;
    uint32_t num_members;
    double x_min;
    double x_max;
    double q_min;
    double q_max;
    std::vector<int32_t> flavors;
    std::string format;
    std::vector<double> alphas_q_values;
    std::vector<double> alphas_vals;
    bool polarised;
    SetType set_type;
    InterpolatorType interpolator_type;

    // Conversion to C struct
    NeoPDFMetaData to_c() const {
        NeoPDFMetaData c_meta;
        c_meta.set_desc = set_desc.c_str();
        c_meta.set_index = set_index;
        c_meta.num_members = num_members;
        c_meta.x_min = x_min;
        c_meta.x_max = x_max;
        c_meta.q_min = q_min;
        c_meta.q_max = q_max;
        c_meta.flavors = flavors.data();
        c_meta.num_flavors = flavors.size();
        c_meta.format = format.c_str();
        c_meta.alphas_q_values = alphas_q_values.data();
        c_meta.num_alphas_q = alphas_q_values.size();
        c_meta.alphas_vals = alphas_vals.data();
        c_meta.num_alphas_vals = alphas_vals.size();
        c_meta.polarised = polarised;
        c_meta.set_type = set_type;
        c_meta.interpolator_type = interpolator_type;
        return c_meta;
    }
};

/** @brief Base PDF class that instantiates the PDF object. */
class NeoPDF {
    private:
        /** @brief Underlying raw object. */
        NeoPDFWrapper* raw;

    protected:
        /** @brief Constructor (protected to avoid direct instantiation). */
        NeoPDF(NeoPDFWrapper* pdf) : raw(pdf) {}

        /** @brief Deleted copy/move semantics. */
        NeoPDF() = delete;
        NeoPDF(const NeoPDF&) = delete;
        NeoPDF(NeoPDF&&) = delete;

        NeoPDF& operator=(const NeoPDF&) = delete;
        NeoPDF& operator=(NeoPDF&&) = delete;

    public:
        /** @brief Destructor. */
        virtual ~NeoPDF() { neopdf_pdf_free(this->raw); }

        /**
         * @brief Constructor of the PDF object.
         * @brief `pdf_name` Name of the PDF set.
         * @brief `member` ID number of the PDF member.
         */
        NeoPDF(const std::string& pdf_name, size_t member = 0) {
            this->raw = neopdf_pdf_load(pdf_name.c_str(), member);
        }

        // Needed for `PDFs` to call the protected constructor
        // Static factory method to create PDF objects from NeoPDFWrapper*
        static std::unique_ptr<NeoPDF> from_raw(NeoPDFWrapper* pdf) {
            return std::unique_ptr<NeoPDF>(new NeoPDF(pdf));
        }

        /** @brief Get the minimum value of the x-grid for the PDF. */
        double x_min() const { return neopdf_pdf_x_min(this->raw); }

        /** @brief Get the maximum value of the x-grid for the PDF. */
        double x_max() const { return neopdf_pdf_x_max(this->raw); }

        /** @brief Get the minimum value of the Q2-grid for the PDF. */
        double q2_min() const { return neopdf_pdf_q2_min(this->raw); }

        /** @brief Get the maximum value of the Q2-grid for the PDF. */
        double q2_max() const { return neopdf_pdf_q2_max(this->raw); }

        /** @brief Compute the `xf` value for a given PID, x, and Q2. */
        double xfxQ2(int pid, double x, double q2) const {
            return neopdf_pdf_xfxq2(this->raw, pid, x, q2);
        }

        /** @brief Compute the value of `alphas` at the Q2 value. */
        double alphasQ2(double q2) const {
            return neopdf_pdf_alphas_q2(this->raw, q2);
        }

        /** @brief Get the number of PIDs. */
        size_t num_pids() const {
            return neopdf_pdf_num_pids(this->raw);
        }

        /** @brief Get the PID representation of the PDF Grid. */
        std::vector<int32_t> pids() const {
            size_t num = num_pids();
            std::vector<int32_t> pids(num);
            neopdf_pdf_pids(this->raw, pids.data(), num);
            return pids;
        }

        /** @brief Get the number of subgrids in the PDF Grid. */
        size_t num_subgrids() const {
            return neopdf_pdf_num_subgrids(this->raw);
        }

        /** @brief Get the minimum and maximum value for a given parameter. */
        std::vector<double> param_range(NeopdfSubgridParams param) const {
            std::vector<double> range(2);
            neopdf_pdf_param_range(this->raw, param, range.data());
            return range;
        }

        /** @brief Get the shape of the subgrids in the order of their index for a given parameter. */
        std::vector<size_t> subgrids_shape_for_param(NeopdfSubgridParams param) const {
            size_t num = num_subgrids();
            std::vector<size_t> shape(num);
            neopdf_pdf_subgrids_shape_for_param(this->raw, shape.data(), num, param);
            return shape;
        }

        /** @brief Get the grid values of a parameter for a given subgrid. */
        std::vector<double> subgrid_for_param(NeopdfSubgridParams param, size_t subgrid_index) const {
            std::vector<size_t> shape = subgrids_shape_for_param(param);
            std::vector<double> values(shape[subgrid_index]);
            neopdf_pdf_subgrids_for_param(
                this->raw,
                values.data(),
                param,
                shape.size(),
                shape.data(),
                subgrid_index
            );
            return values;
        }
};

/** @brief Class to load and manage multiple PDF members. */
class NeoPDFs {
    private:
        std::vector<std::unique_ptr<NeoPDF>> pdf_members;

    public:
        /**
         * @brief Constructor that loads all PDF members for a given PDF set.
         * @param pdf_name Name of the PDF set.
         */
        NeoPDFs(const std::string& pdf_name) {
            NeoPDFMembers raw_pdfs = neopdf_pdf_load_all(pdf_name.c_str());

            for (size_t i = 0; i < raw_pdfs.size; ++i) {
                pdf_members.push_back(NeoPDF::from_raw(raw_pdfs.pdfs[i]));
            }
        }

        /** @brief Get the number of loaded PDF members. */
        size_t size() const { return pdf_members.size(); }

        /** @brief Access a specific PDF member by index. */
        NeoPDF& operator[](size_t index) { return *pdf_members[index]; }

        /** @brief Access a specific PDF member by index (const version). */
        const NeoPDF& operator[](size_t index) const { return *pdf_members[index]; }

        /** @brief Access a specific PDF member by index with bounds checking. */
        NeoPDF& at(size_t index) { return *pdf_members.at(index); }

        /** @brief Access a specific PDF member by index with bounds checking (const version). */
        const NeoPDF& at(size_t index) const { return *pdf_members.at(index); }
};

/** @brief Class for writing NeoPDF grid data to a file. */
class GridWriter {
    private:
        NeoPDFGridArrayCollection* collection_raw;

    public:
        /** @brief Constructor. */
        GridWriter() {
            collection_raw = neopdf_gridarray_collection_new();
            if (!collection_raw) {
                throw std::runtime_error("Failed to create `NeoPDFGridArrayCollection`");
            }
        }

        /** @brief Destructor. */
        ~GridWriter() {
            if (collection_raw) {
                neopdf_gridarray_collection_free(collection_raw);
            }
        }

        /**
         * @brief Adds a subgrid to the writer.
         *
         * @param nucleons Vector of nucleon numbers.
         * @param alphas Vector of alpha_s values.
         * @param xs Vector of x values.
         * @param q2s Vector of Q2 values.
         * @param grid_data Vector of grid data.
         * @param flavors Vector of flavor IDs.
         */
        void add_grid(
            const std::vector<double>& nucleons,
            const std::vector<double>& alphas,
            const std::vector<double>& xs,
            const std::vector<double>& q2s,
            const std::vector<double>& grid_data,
            const std::vector<int32_t>& flavors
        ) {
            NeoPDFGrid* grid = neopdf_grid_new();
            if (!grid) {
                throw std::runtime_error("Failed to create `NeoPDFGrid`");
            }

            NeopdfResult result = neopdf_grid_add_subgrid(
                grid,
                nucleons.data(), nucleons.size(),
                alphas.data(), alphas.size(),
                xs.data(), xs.size(),
                q2s.data(), q2s.size(),
                grid_data.data(), grid_data.size()
            );
            if (result != NeopdfResult::NEOPDF_RESULT_SUCCESS) {
                neopdf_grid_free(grid);
                throw std::runtime_error("Failed to add subgrid");
            }

            result = neopdf_grid_set_flavors(grid, flavors.data(), flavors.size());
            if (result != NeopdfResult::NEOPDF_RESULT_SUCCESS) {
                neopdf_grid_free(grid);
                throw std::runtime_error("Failed to set flavors");
            }

            result = neopdf_gridarray_collection_add_grid(collection_raw, grid);
            if (result != NeopdfResult::NEOPDF_RESULT_SUCCESS) {
                neopdf_grid_free(grid);
                throw std::runtime_error("Failed to add grid to collection");
            }
        }

        /**
         * @brief Compresses the added grids and writes them to a file.
         *
         * @param metadata The metadata for the PDF set.
         * @param output_path The path to the output file.
         */
        void compress(const MetaData& metadata, const std::string& output_path) {
            NeoPDFMetaData c_meta = metadata.to_c();
            NeopdfResult result = neopdf_grid_compress(collection_raw, &c_meta, output_path.c_str());
            if (result != NeopdfResult::NEOPDF_RESULT_SUCCESS) {
                throw std::runtime_error("Failed to compress grid data");
            }
        }
};

} // namespace neopdf
