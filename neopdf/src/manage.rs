use flate2::read::GzDecoder;
use reqwest::blocking::get;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use tar::Archive;

#[derive(Debug, Deserialize, Serialize)]
pub struct ManageData {
    neopdf_path: PathBuf,
    set_name: String,
    pdfset_path: PathBuf,
}

impl ManageData {
    pub fn new(set_name: &str) -> Self {
        let data_path = Self::get_data_path();
        let xpdf_path = data_path.join(set_name);

        let manager = Self {
            neopdf_path: data_path,
            set_name: set_name.to_string(),
            pdfset_path: xpdf_path,
        };
        manager.ensure_pdf_installed().unwrap();

        manager
    }

    pub fn get_data_path() -> PathBuf {
        // Check for NEOPDF_DATA_PATH environment variable first
        if let Ok(neopdf_data_path) = std::env::var("NEOPDF_DATA_PATH") {
            let neopdf_dir = PathBuf::from(neopdf_data_path);

            if !neopdf_dir.exists() {
                std::fs::create_dir_all(&neopdf_dir).unwrap();
            }

            return neopdf_dir;
        }

        // Falls back to the XDG data directory if the env. variable is not set.
        // TODO: Make this more robust and not platform-dependent
        let home = std::env::var("HOME")
            .map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "HOME environment variable not found",
                )
            })
            .unwrap();
        let data_dir = PathBuf::from(home).join(".local").join("share");
        let neopdf_dir = data_dir.join("neopdf");

        if !neopdf_dir.exists() {
            std::fs::create_dir_all(&neopdf_dir).unwrap();
        }

        neopdf_dir
    }

    /// Download the PDF set and extract it into the designated path.
    /// The download happens in memory so no `*.tar.*` is written.
    pub fn download_pdf(&self) -> Result<(), Box<dyn Error>> {
        let url = format!(
            "https://lhapdfsets.web.cern.ch/current/{}.tar.gz",
            self.set_name
        );
        println!("Downloading PDF set from: {}", url);

        let response = get(&url)?;
        if !response.status().is_success() {
            return Err(format!(
                "Failed to download PDF set '{}': HTTP {}",
                self.set_name,
                response.status()
            )
            .into());
        }

        let bytes = response.bytes()?;
        let tar = GzDecoder::new(Cursor::new(bytes));
        let mut archive = Archive::new(tar);

        archive.unpack(&self.neopdf_path)?;

        Ok(())
    }

    /// Check that the PDF set is installed in the correct path.
    pub fn is_pdf_installed(&self) -> bool {
        self.pdfset_path.exists() && self.pdfset_path.is_dir()
    }

    /// Ensure that the PDF set is installed, otherwise download it.
    pub fn ensure_pdf_installed(&self) -> Result<(), Box<dyn Error>> {
        if self.is_pdf_installed() {
            return Ok(());
        }

        println!("PDF set '{}' not found, downloading...", self.set_name);
        self.download_pdf()
    }

    /// Get the name of the PDF set.
    pub fn set_name(&self) -> &str {
        &self.set_name
    }

    /// Get the path where PDF sets are stored.
    pub fn data_path(&self) -> &Path {
        &self.neopdf_path
    }

    /// Get the full path to this specific PDF set.
    pub fn set_path(&self) -> &Path {
        &self.pdfset_path
    }

    /// Get the path to the .info file for this PDF set.
    pub fn info_path(&self) -> PathBuf {
        self.pdfset_path.join(format!(
            "{}.info",
            self.pdfset_path.file_name().unwrap().to_str().unwrap()
        ))
    }

    /// Get the path to a specific .dat member file for this PDF set.
    pub fn dat_path(&self, member: usize) -> PathBuf {
        self.pdfset_path.join(format!(
            "{}_{:04}.dat",
            self.pdfset_path.file_name().unwrap().to_str().unwrap(),
            member
        ))
    }
}
