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

} // namespace neopdf
