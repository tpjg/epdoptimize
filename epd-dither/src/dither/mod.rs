//! Dithering algorithms and engine

pub mod algorithms;
pub mod engine;
pub mod matrices;

use crate::color::Palette;

/// Dithering algorithm selection
#[derive(Debug, Clone)]
pub enum DitheringAlgorithm {
    /// Error diffusion dithering with various kernels
    ErrorDiffusion(ErrorDiffusionKernel),
    /// Ordered dithering using Bayer matrix
    Ordered { width: u8, height: u8 },
    /// Random dithering
    Random(RandomMode),
    /// Quantization only (no dithering)
    QuantizationOnly,
}

/// Error diffusion kernel types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorDiffusionKernel {
    FloydSteinberg,
    FalseFloydSteinberg,
    Jarvis,
    Stucki,
    Burkes,
    Sierra3,
    Sierra2,
    Sierra2_4A,
}

/// Random dithering mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RandomMode {
    Rgb,
    BlackAndWhite,
}

/// Complete dithering configuration
#[derive(Debug, Clone)]
pub struct DitherOptions {
    pub algorithm: DitheringAlgorithm,
    pub palette: Palette,
    pub serpentine: bool,
}

impl Default for DitherOptions {
    fn default() -> Self {
        Self {
            algorithm: DitheringAlgorithm::ErrorDiffusion(ErrorDiffusionKernel::FloydSteinberg),
            palette: Palette::default(),
            serpentine: false,
        }
    }
}
