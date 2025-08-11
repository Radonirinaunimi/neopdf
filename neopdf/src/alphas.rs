//! This module provides implementations for calculating the strong coupling constant `alpha_s`.
//!
//! It includes support for different calculation methods, such as analytic formulas and
//! interpolation from tabulated values, mirroring the functionality available in `LHAPDF`.

use crate::metadata::MetaData;
use crate::strategy::AlphaSCubicInterpolation;
use ninterp::interpolator::Extrapolate;
use ninterp::prelude::*;

/// Enum representing the different methods for alpha_s calculation.
pub enum AlphaS {
    Analytic(AlphaSAnalytic),
    Ipol(AlphaSIpol),
}

impl AlphaS {
    /// Creates a new `AlphaS` calculator from PDF metadata.
    pub fn from_metadata(meta: &MetaData) -> Result<Self, String> {
        // Placeholder for  `meta.alphas_type` logics
        if meta.alphas_vals.is_empty() {
            Ok(AlphaS::Analytic(AlphaSAnalytic::from_metadata(meta)?))
        } else {
            Ok(AlphaS::Ipol(AlphaSIpol::from_metadata(meta)?))
        }
    }

    /// Calculates alpha_s at a given Q^2.
    pub fn alphas_q2(&self, q2: f64) -> f64 {
        match self {
            AlphaS::Analytic(a) => a.alphas_q2(q2),
            AlphaS::Ipol(i) => i.alphas_q2(q2),
        }
    }
}

/// Strong coupling calculator using analytic formulas.
pub struct AlphaSAnalytic {
    qcd_order: u32,
    lambda3: f64,
    lambda4: f64,
    lambda5: f64,
    m_charm_sq: f64,
    m_bottom_sq: f64,
    m_top_sq: f64,
}

impl AlphaSAnalytic {
    pub fn from_metadata(meta: &MetaData) -> Result<Self, String> {
        // Using hardcoded default lambda values.
        Ok(Self {
            qcd_order: meta.alphas_order_qcd,
            lambda3: 0.339,
            lambda4: 0.296,
            lambda5: 0.213,
            m_charm_sq: meta.m_charm * meta.m_charm,
            m_bottom_sq: meta.m_bottom * meta.m_bottom,
            m_top_sq: meta.m_top * meta.m_top,
        })
    }

    fn num_flavors_q2(&self, q2: f64) -> u32 {
        if q2 > self.m_top_sq && self.m_top_sq > 0.0 {
            6
        } else if q2 > self.m_bottom_sq && self.m_bottom_sq > 0.0 {
            5
        } else if q2 > self.m_charm_sq && self.m_charm_sq > 0.0 {
            4
        } else {
            3
        }
    }

    fn lambda_qcd(&self, nf: u32) -> f64 {
        match nf {
            3 => self.lambda3,
            4 => self.lambda4,
            5 | 6 => self.lambda5,
            _ => 0.0,
        }
    }

    /// Calculates alpha_s(Q2) using the analytic running formula.
    pub fn alphas_q2(&self, q2: f64) -> f64 {
        let nf = self.num_flavors_q2(q2);
        let lambda_qcd = self.lambda_qcd(nf);

        if q2 <= lambda_qcd * lambda_qcd {
            return f64::INFINITY;
        }

        let beta0 = (33.0 - 2.0 * nf as f64) / (12.0 * std::f64::consts::PI);
        let t = (q2 / (lambda_qcd * lambda_qcd)).ln();

        if self.qcd_order == 0 {
            return 0.130;
        }

        1.0 / (beta0 * t)
    }
}

/// Strong coupling calculator using interpolation.
pub struct AlphaSIpol {
    interpolator: Interp1DOwned<f64, AlphaSCubicInterpolation>,
}

impl AlphaSIpol {
    pub fn from_metadata(meta: &MetaData) -> Result<Self, String> {
        let q2_values: Vec<f64> = meta.alphas_q_values.iter().map(|&q| q * q).collect();
        let interpolator = Interp1D::new(
            q2_values.into(),
            meta.alphas_vals.to_owned().into(),
            AlphaSCubicInterpolation,
            Extrapolate::Error,
        )
        .map_err(|e| e.to_string())?;
        Ok(Self { interpolator })
    }

    pub fn alphas_q2(&self, q2: f64) -> f64 {
        self.interpolator.interpolate(&[q2]).unwrap_or(0.0)
    }
}
