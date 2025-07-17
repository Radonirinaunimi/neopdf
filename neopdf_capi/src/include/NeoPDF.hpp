#pragma once

#include <cstddef>
#include <neopdf_capi.h>
#include <string>
#include <sys/types.h>
#include <vector>
#include <memory>

/** @brief Object Oriented interface to NeoPDF. */
namespace neopdf {

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


/**
 * @brief Class for constructing and compressing NeoPDF grids using the C API.
 *
 * This class provides a C++ RAII wrapper for the NeoPDF_Grid handle, allowing users to
 * add subgrids, set flavor IDs, and compress the grid to disk with associated metadata.
 * It is designed for high-level, ergonomic use from C++ code.
 */
class NeoPDFGrid {
private:
    NeoPDFGrid* grid; ///< Opaque grid handle managed by this class.
    std::vector<int> flavors; ///< Stores flavor IDs for the grid.
public:
    /**
     * @brief Construct a new NeoPDFGrid object (empty grid).
     */
    NeoPDFGrid() : grid(neopdf_grid_new()) {}
    /**
     * @brief Destructor. Frees the underlying grid handle.
     */
    ~NeoPDFGrid() { if (grid) neopdf_grid_free(grid); }
    NeoPDFGrid(const NeoPDFGrid&) = delete;
    NeoPDFGrid& operator=(const NeoPDFGrid&) = delete;
    /**
     * @brief Move constructor.
     */
    NeoPDFGrid(NeoPDFGrid&& other) noexcept : grid(other.grid), flavors(std::move(other.flavors)) { other.grid = nullptr; }
    /**
     * @brief Move assignment operator.
     */
    NeoPDFGrid& operator=(NeoPDFGrid&& other) noexcept {
        if (this != &other) {
            if (grid) neopdf_grid_free(grid);
            grid = other.grid;
            flavors = std::move(other.flavors);
            other.grid = nullptr;
        }
        return *this;
    }

    /**
     * @brief Add a subgrid to the grid.
     *
     * @param nucleons Vector of nucleon numbers.
     * @param alphas Vector of alpha_s values.
     * @param xs Vector of x values.
     * @param q2s Vector of Q^2 values.
     * @param grid_data Flat vector of grid data (see documentation for layout).
     * @throws std::runtime_error on failure.
     */
    void add_subgrid(const std::vector<double>& nucleons,
                     const std::vector<double>& alphas,
                     const std::vector<double>& xs,
                     const std::vector<double>& q2s,
                     const std::vector<double>& grid_data) {
        if (neopdf_grid_add_subgrid(grid,
                nucleons.data(), nucleons.size(),
                alphas.data(), alphas.size(),
                xs.data(), xs.size(),
                q2s.data(), q2s.size(),
                grid_data.data(), grid_data.size()) != 0) {
            throw std::runtime_error("Failed to add subgrid");
        }
    }

    /**
     * @brief Set the flavor IDs for the grid.
     *
     * @param fl Vector of PDG flavor IDs.
     * @throws std::runtime_error on failure.
     */
    void set_flavors(const std::vector<int>& fl) {
        flavors = fl;
        if (neopdf_grid_set_flavors(grid, flavors.data(), flavors.size()) != 0) {
            throw std::runtime_error("Failed to set flavors");
        }
    }

    /**
     * @brief Metadata for the PDF set, used for compression.
     *
     * This struct mirrors the fields of the Rust MetaData struct.
     */
    struct MetaData {
        std::string set_desc; ///< Description of the PDF set.
        uint32_t set_index = 0; ///< Index of the PDF set.
        uint32_t num_members = 1; ///< Number of members in the PDF set.
        double x_min = 0, x_max = 0, q_min = 0, q_max = 0; ///< Valid ranges for x and Q.
        std::vector<int> flavors; ///< List of PDG flavor IDs.
        std::string format; ///< Format string (e.g., "neopdf").
        std::vector<double> alphas_q_values; ///< AlphaS Q values.
        std::vector<double> alphas_vals; ///< AlphaS values.
        bool polarised = false; ///< Polarisation flag.
        NeoPDF_SetType set_type = NEOPDF_SET_TYPE_PDF; ///< Set type enum.
        NeoPDF_InterpolatorType interpolator_type = NEOPDF_INTERP_LOGBICUBIC; ///< Interpolator type enum.
    };

    /**
     * @brief Compress and write the grid to disk.
     *
     * @param meta Metadata describing the PDF set.
     * @param out_path Output file path (e.g., "output.neopdf.lz4").
     * @throws std::runtime_error on failure.
     */
    void compress(const MetaData& meta, const std::string& out_path) {
        NeoPDFMetaData cmeta = {0};
        cmeta.set_desc = meta.set_desc.c_str();
        cmeta.set_index = meta.set_index;
        cmeta.num_members = meta.num_members;
        cmeta.x_min = meta.x_min;
        cmeta.x_max = meta.x_max;
        cmeta.q_min = meta.q_min;
        cmeta.q_max = meta.q_max;
        cmeta.flavors = meta.flavors.data();
        cmeta.num_flavors = meta.flavors.size();
        cmeta.format = meta.format.c_str();
        cmeta.alphas_q_values = meta.alphas_q_values.data();
        cmeta.num_alphas_q = meta.alphas_q_values.size();
        cmeta.alphas_vals = meta.alphas_vals.data();
        cmeta.num_alphas_vals = meta.alphas_vals.size();
        cmeta.polarised = meta.polarised;
        cmeta.set_type = meta.set_type;
        cmeta.interpolator_type = meta.interpolator_type;
        int result = neopdf_grid_compress(grid, &cmeta, out_path.c_str());
        if (result != 0) {
            throw std::runtime_error("Compression failed");
        }
    }
};

} // namespace neopdf
