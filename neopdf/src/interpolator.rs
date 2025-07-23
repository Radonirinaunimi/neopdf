//! This module contains the dynamic interpolation traits, InterpolatorFactory, and dynamic dispatch logic for PDF grids.
//!
//! # Contents
//!
//! - [`DynInterpolator`]: Trait for dynamic, multi-dimensional interpolation.
//! - [`InterpolatorFactory`]: Factory for constructing interpolators for SubGrid.
//!
//! # Note
//!
//! Interpolation strategies are defined in `strategy.rs`.
//! The [`SubGrid`] struct is defined in `subgrid.rs`.

use ndarray::{s, OwnedRepr};
use ninterp::error::InterpolateError;
use ninterp::interpolator::{
    Extrapolate, Interp2D, Interp2DOwned, Interp3D, Interp3DOwned, InterpND, InterpNDOwned,
};
use ninterp::prelude::*;
use ninterp::strategy::traits::{Strategy2D, Strategy3D, StrategyND};
use ninterp::strategy::Linear;

use super::metadata::InterpolatorType;
use super::strategy::{
    BilinearInterpolation, LogBicubicInterpolation, LogBilinearInterpolation,
    LogTricubicInterpolation,
};
use super::subgrid::SubGrid;

/// Represents the dimensionality and structure of interpolation needed.
///
/// This enum is used to select the appropriate interpolation strategy based on the
/// dimensions of the PDF grid data.
#[derive(Debug, Clone, Copy)]
pub enum InterpolationConfig {
    /// 2D interpolation, typically in `x` (momentum fraction) and `Q²` (energy scale).
    TwoD,
    /// 3D interpolation, including a dimension for varying nucleon numbers `A`,
    /// in addition to `x` and `Q²`.
    ThreeDNucleons,
    /// 3D interpolation, including a dimension for varying `alpha_s` values,
    /// in addition to `x` and `Q²`.
    ThreeDAlphas,
    /// 3D interpolation, including a dimension for varying `kT` values,
    /// in addition to `x` and `Q²`.
    ThreeDKt,
    /// 4D interpolation, covering nucleon numbers `A`, `alpha_s`, `x`, and `Q²`.
    FourDNucleonsAlphas,
    /// 4D interpolation, covering nucleon numbers `A`, kT, `x`, and `Q²`.
    FourDNucleonsKt,
    /// 4D interpolation, covering `alpha_s`, kT, `x`, and `Q²`.
    FourDAlphasKt,
    /// 5D interpolation, covering nucleon numbers `A`, `alpha_s`, `kT`, `x`, and `Q²`.
    FiveD,
}

impl InterpolationConfig {
    /// Determines the interpolation configuration from the number of nucleons and alpha_s values.
    ///
    /// # Panics
    ///
    /// Panics if the combination of `n_nucleons` and `n_alphas` is not supported.
    pub fn from_dimensions(n_nucleons: usize, n_alphas: usize, n_kts: usize) -> Self {
        match (n_nucleons > 1, n_alphas > 1, n_kts > 1) {
            (false, false, false) => Self::TwoD,
            (true, false, false) => Self::ThreeDNucleons,
            (false, true, false) => Self::ThreeDAlphas,
            (false, false, true) => Self::ThreeDKt,
            (true, true, false) => Self::FourDNucleonsAlphas,
            (true, false, true) => Self::FourDNucleonsKt,
            (false, true, true) => Self::FourDAlphasKt,
            (true, true, true) => Self::FiveD,
        }
    }
}

/// A trait for dynamic interpolation across different dimensions.
pub trait DynInterpolator: Send + Sync {
    fn interpolate_point(&self, point: &[f64]) -> Result<f64, InterpolateError>;
}

// Implement `DynInterpolator` for 2D interpolators.
impl<S> DynInterpolator for Interp2DOwned<f64, S>
where
    S: Strategy2D<OwnedRepr<f64>> + 'static + Clone + Send + Sync,
{
    fn interpolate_point(&self, point: &[f64]) -> Result<f64, InterpolateError> {
        let [x, y] = point
            .try_into()
            .map_err(|_| InterpolateError::Other("Expected 2D point".to_string()))?;
        self.interpolate(&[x, y])
    }
}

// Implement `DynInterpolator` for 3D interpolators.
impl<S> DynInterpolator for Interp3DOwned<f64, S>
where
    S: Strategy3D<OwnedRepr<f64>> + 'static + Clone + Send + Sync,
{
    fn interpolate_point(&self, point: &[f64]) -> Result<f64, InterpolateError> {
        let [x, y, z] = point
            .try_into()
            .map_err(|_| InterpolateError::Other("Expected 3D point".to_string()))?;
        self.interpolate(&[x, y, z])
    }
}

// Implement `DynInterpolator` for N-dimensional interpolators.
impl<S> DynInterpolator for InterpNDOwned<f64, S>
where
    S: StrategyND<OwnedRepr<f64>> + 'static + Clone + Send + Sync,
{
    fn interpolate_point(&self, point: &[f64]) -> Result<f64, InterpolateError> {
        self.interpolate(point)
    }
}

/// Factory for creating dynamic interpolators based on interpolation type and grid dimensions.
pub struct InterpolatorFactory;

impl InterpolatorFactory {
    pub fn create(
        interp_type: InterpolatorType,
        subgrid: &SubGrid,
        pid_index: usize,
    ) -> Box<dyn DynInterpolator> {
        match subgrid.interpolation_config() {
            InterpolationConfig::TwoD => Self::interpolator_xfxq2(interp_type, subgrid, pid_index),
            InterpolationConfig::ThreeDNucleons => {
                Self::interpolator_xfxq2_nucleons(interp_type, subgrid, pid_index)
            }
            InterpolationConfig::ThreeDAlphas => {
                Self::interpolator_xfxq2_alphas(interp_type, subgrid, pid_index)
            }
            InterpolationConfig::ThreeDKt => {
                Self::interpolator_xfxq2_kts(interp_type, subgrid, pid_index)
            }
            InterpolationConfig::FourDNucleonsAlphas => {
                Self::interpolator_xfxq2_nucleons_alphas(interp_type, subgrid, pid_index)
            }
            InterpolationConfig::FourDNucleonsKt => {
                Self::interpolator_xfxq2_nucleons_kts(interp_type, subgrid, pid_index)
            }
            InterpolationConfig::FourDAlphasKt => {
                Self::interpolator_xfxq2_alphas_kts(interp_type, subgrid, pid_index)
            }
            InterpolationConfig::FiveD => {
                Self::interpolator_xfxq2_5dim(interp_type, subgrid, pid_index)
            }
        }
    }

    fn interpolator_xfxq2(
        interp_type: InterpolatorType,
        subgrid: &SubGrid,
        pid_index: usize,
    ) -> Box<dyn DynInterpolator> {
        let grid_slice = subgrid.grid_slice(pid_index).to_owned();
        match interp_type {
            InterpolatorType::Bilinear => Box::new(
                Interp2D::new(
                    subgrid.xs.to_owned(),
                    subgrid.q2s.to_owned(),
                    grid_slice,
                    BilinearInterpolation,
                    Extrapolate::Error,
                )
                .expect("Failed to create 2D interpolator"),
            ),
            InterpolatorType::LogBilinear => Box::new(
                Interp2D::new(
                    subgrid.xs.to_owned(),
                    subgrid.q2s.to_owned(),
                    grid_slice,
                    LogBilinearInterpolation,
                    Extrapolate::Error,
                )
                .expect("Failed to create 2D interpolator"),
            ),
            InterpolatorType::LogBicubic => Box::new(
                Interp2D::new(
                    subgrid.xs.to_owned(),
                    subgrid.q2s.to_owned(),
                    grid_slice,
                    LogBicubicInterpolation::default(),
                    Extrapolate::Error,
                )
                .expect("Failed to create 2D interpolator"),
            ),
            _ => panic!("Unsupported 2D interpolator: {:?}", interp_type),
        }
    }

    fn interpolator_xfxq2_nucleons(
        interp_type: InterpolatorType,
        subgrid: &SubGrid,
        pid_index: usize,
    ) -> Box<dyn DynInterpolator> {
        let grid_data = subgrid
            .grid
            .slice(s![.., 0, pid_index, 0, .., ..])
            .to_owned();
        let reshaped_data = grid_data
            .into_shape_with_order((subgrid.nucleons.len(), subgrid.xs.len(), subgrid.q2s.len()))
            .expect("Failed to reshape 3D data");
        match interp_type {
            InterpolatorType::LogTricubic => Box::new(
                Interp3D::new(
                    subgrid.nucleons.to_owned(),
                    subgrid.xs.to_owned(),
                    subgrid.q2s.to_owned(),
                    reshaped_data,
                    LogTricubicInterpolation,
                    Extrapolate::Error,
                )
                .expect("Failed to create 3D interpolator"),
            ),
            _ => panic!("Unsupported 3D interpolator: {:?}", interp_type),
        }
    }

    fn interpolator_xfxq2_alphas(
        interp_type: InterpolatorType,
        subgrid: &SubGrid,
        pid_index: usize,
    ) -> Box<dyn DynInterpolator> {
        let grid_data = subgrid
            .grid
            .slice(s![0, .., pid_index, 0, .., ..])
            .to_owned();
        let reshaped_data = grid_data
            .into_shape_with_order((subgrid.alphas.len(), subgrid.xs.len(), subgrid.q2s.len()))
            .expect("Failed to reshape 3D data");
        match interp_type {
            InterpolatorType::LogTricubic => Box::new(
                Interp3D::new(
                    subgrid.alphas.to_owned(),
                    subgrid.xs.to_owned(),
                    subgrid.q2s.to_owned(),
                    reshaped_data,
                    LogTricubicInterpolation,
                    Extrapolate::Error,
                )
                .expect("Failed to create 3D interpolator"),
            ),
            _ => panic!("Unsupported 3D interpolator: {:?}", interp_type),
        }
    }

    fn interpolator_xfxq2_kts(
        interp_type: InterpolatorType,
        subgrid: &SubGrid,
        pid_index: usize,
    ) -> Box<dyn DynInterpolator> {
        let grid_data = subgrid
            .grid
            .slice(s![0, 0, pid_index, .., .., ..])
            .to_owned();
        let reshaped_data = grid_data
            .into_shape_with_order((subgrid.kts.len(), subgrid.xs.len(), subgrid.q2s.len()))
            .expect("Failed to reshape 3D data");
        match interp_type {
            InterpolatorType::LogTricubic => Box::new(
                Interp3D::new(
                    subgrid.kts.to_owned(),
                    subgrid.xs.to_owned(),
                    subgrid.q2s.to_owned(),
                    reshaped_data,
                    LogTricubicInterpolation,
                    Extrapolate::Error,
                )
                .expect("Failed to create 3D interpolator"),
            ),
            _ => panic!("Unsupported 3D interpolator: {:?}", interp_type),
        }
    }

    fn interpolator_xfxq2_nucleons_alphas(
        interp_type: InterpolatorType,
        subgrid: &SubGrid,
        pid_index: usize,
    ) -> Box<dyn DynInterpolator> {
        let grid_data = subgrid
            .grid
            .slice(s![.., .., pid_index, 0, .., ..])
            .to_owned();
        let coords = vec![
            subgrid.nucleons.to_owned(),
            subgrid.alphas.to_owned(),
            subgrid.xs.to_owned(),
            subgrid.q2s.to_owned(),
        ];
        let reshaped_data = grid_data
            .into_shape_with_order((
                subgrid.nucleons.len(),
                subgrid.alphas.len(),
                subgrid.xs.len(),
                subgrid.q2s.len(),
            ))
            .expect("Failed to reshape 4D data");
        match interp_type {
            InterpolatorType::InterpNDLinear => Box::new(
                InterpND::new(coords, reshaped_data.into_dyn(), Linear, Extrapolate::Error)
                    .expect("Failed to create 4D interpolator"),
            ),
            _ => panic!("Unsupported 4D interpolator: {:?}", interp_type),
        }
    }

    fn interpolator_xfxq2_nucleons_kts(
        interp_type: InterpolatorType,
        subgrid: &SubGrid,
        pid_index: usize,
    ) -> Box<dyn DynInterpolator> {
        let grid_data = subgrid
            .grid
            .slice(s![.., 0, pid_index, .., .., ..])
            .to_owned();
        let coords = vec![
            subgrid.nucleons.to_owned(),
            subgrid.kts.to_owned(),
            subgrid.xs.to_owned(),
            subgrid.q2s.to_owned(),
        ];
        let reshaped_data = grid_data
            .into_shape_with_order((
                subgrid.nucleons.len(),
                subgrid.kts.len(),
                subgrid.xs.len(),
                subgrid.q2s.len(),
            ))
            .expect("Failed to reshape 4D data");
        match interp_type {
            InterpolatorType::InterpNDLinear => Box::new(
                InterpND::new(coords, reshaped_data.into_dyn(), Linear, Extrapolate::Error)
                    .expect("Failed to create 4D interpolator"),
            ),
            _ => panic!("Unsupported 4D interpolator: {:?}", interp_type),
        }
    }

    fn interpolator_xfxq2_alphas_kts(
        interp_type: InterpolatorType,
        subgrid: &SubGrid,
        pid_index: usize,
    ) -> Box<dyn DynInterpolator> {
        let grid_data = subgrid
            .grid
            .slice(s![0, .., pid_index, .., .., ..])
            .to_owned();
        let coords = vec![
            subgrid.alphas.to_owned(),
            subgrid.kts.to_owned(),
            subgrid.xs.to_owned(),
            subgrid.q2s.to_owned(),
        ];
        let reshaped_data = grid_data
            .into_shape_with_order((
                subgrid.alphas.len(),
                subgrid.kts.len(),
                subgrid.xs.len(),
                subgrid.q2s.len(),
            ))
            .expect("Failed to reshape 4D data");
        match interp_type {
            InterpolatorType::InterpNDLinear => Box::new(
                InterpND::new(coords, reshaped_data.into_dyn(), Linear, Extrapolate::Error)
                    .expect("Failed to create 4D interpolator"),
            ),
            _ => panic!("Unsupported 4D interpolator: {:?}", interp_type),
        }
    }

    fn interpolator_xfxq2_5dim(
        interp_type: InterpolatorType,
        subgrid: &SubGrid,
        pid_index: usize,
    ) -> Box<dyn DynInterpolator> {
        let grid_data = subgrid
            .grid
            .slice(s![.., .., pid_index, .., .., ..])
            .to_owned();
        let coords = vec![
            subgrid.nucleons.to_owned(),
            subgrid.alphas.to_owned(),
            subgrid.kts.to_owned(),
            subgrid.xs.to_owned(),
            subgrid.q2s.to_owned(),
        ];
        let reshaped_data = grid_data
            .into_shape_with_order((
                subgrid.nucleons.len(),
                subgrid.alphas.len(),
                subgrid.kts.len(),
                subgrid.xs.len(),
                subgrid.q2s.len(),
            ))
            .expect("Failed to reshape 5D data");
        match interp_type {
            InterpolatorType::InterpNDLinear => Box::new(
                InterpND::new(coords, reshaped_data.into_dyn(), Linear, Extrapolate::Error)
                    .expect("Failed to create 5D interpolator"),
            ),
            _ => panic!("Unsupported 5D interpolator: {:?}", interp_type),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolation_config() {
        assert!(matches!(
            InterpolationConfig::from_dimensions(1, 1, 1),
            InterpolationConfig::TwoD
        ));
        assert!(matches!(
            InterpolationConfig::from_dimensions(2, 1, 1),
            InterpolationConfig::ThreeDNucleons
        ));
        assert!(matches!(
            InterpolationConfig::from_dimensions(1, 2, 1),
            InterpolationConfig::ThreeDAlphas
        ));
        assert!(matches!(
            InterpolationConfig::from_dimensions(1, 1, 2),
            InterpolationConfig::ThreeDKt
        ));
        assert!(matches!(
            InterpolationConfig::from_dimensions(2, 2, 1),
            InterpolationConfig::FourDNucleonsAlphas
        ));
        assert!(matches!(
            InterpolationConfig::from_dimensions(2, 1, 2),
            InterpolationConfig::FourDNucleonsKt
        ));
        assert!(matches!(
            InterpolationConfig::from_dimensions(1, 2, 2),
            InterpolationConfig::FourDAlphasKt
        ));
        assert!(matches!(
            InterpolationConfig::from_dimensions(2, 2, 2),
            InterpolationConfig::FiveD
        ));
    }
}
