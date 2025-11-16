//! EPD Dither - A library for dithering images for e-ink/e-paper displays
//!
//! This library provides high-quality dithering algorithms optimized for
//! e-ink displays with limited color palettes.

pub mod color;
pub mod dither;

pub use color::{Palette, Rgb, Rgba};
pub use dither::{DitherOptions, DitheringAlgorithm, ErrorDiffusionKernel};

/// Process an image with the given dithering options
pub fn process_image(
    img: &mut image::RgbImage,
    options: &DitherOptions,
) -> anyhow::Result<()> {
    dither::engine::dither_image(img, options)
}
