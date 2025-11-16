//! Main dithering engine that coordinates the various algorithms

use super::{
    algorithms::{error_diffusion, ordered, random},
    DitheringAlgorithm, DitherOptions,
};
use crate::color::{distance::find_closest_color, Rgb};
use anyhow::Result;
use image::RgbImage;

/// Apply dithering to an image according to the given options
pub fn dither_image(img: &mut RgbImage, options: &DitherOptions) -> Result<()> {
    let (width, height) = img.dimensions();
    let width = width as usize;
    let height = height as usize;

    match &options.algorithm {
        DitheringAlgorithm::ErrorDiffusion(kernel) => {
            error_diffusion::apply_error_diffusion(
                img.as_mut(),
                width,
                height,
                &options.palette.colors,
                *kernel,
                options.serpentine,
            );
        }

        DitheringAlgorithm::Ordered {
            width: matrix_width,
            height: matrix_height,
        } => {
            let threshold_map = ordered::create_bayer_matrix(*matrix_width, *matrix_height);
            let threshold = 256.0 / 4.0; // Match JS implementation

            for y in 0..height {
                for x in 0..width {
                    let pixel = img.get_pixel(x as u32, y as u32);
                    let old_color = [pixel[0], pixel[1], pixel[2]];

                    // Apply ordered dither
                    let dithered = ordered::apply_ordered_dither(
                        old_color,
                        x,
                        y,
                        &threshold_map,
                        threshold,
                    );

                    // Quantize to palette
                    let quantized_rgb = Rgb::new(dithered[0], dithered[1], dithered[2]);
                    let (_, &new_color) = find_closest_color(&quantized_rgb, &options.palette.colors)
                        .expect("Palette should not be empty");

                    img.put_pixel(x as u32, y as u32, image::Rgb([
                        new_color.r(),
                        new_color.g(),
                        new_color.b(),
                    ]));
                }
            }
        }

        DitheringAlgorithm::Random(mode) => {
            for y in 0..height {
                for x in 0..width {
                    let pixel = img.get_pixel(x as u32, y as u32);
                    let old_color = [pixel[0], pixel[1], pixel[2]];

                    let dithered = random::apply_random_dither(old_color, *mode);

                    img.put_pixel(x as u32, y as u32, image::Rgb(dithered));
                }
            }
        }

        DitheringAlgorithm::QuantizationOnly => {
            // Just quantize to nearest palette color, no dithering
            for y in 0..height {
                for x in 0..width {
                    let pixel = img.get_pixel(x as u32, y as u32);
                    let old_color = Rgb::new(pixel[0], pixel[1], pixel[2]);

                    let (_, &new_color) = find_closest_color(&old_color, &options.palette.colors)
                        .expect("Palette should not be empty");

                    img.put_pixel(x as u32, y as u32, image::Rgb([
                        new_color.r(),
                        new_color.g(),
                        new_color.b(),
                    ]));
                }
            }
        }
    }

    Ok(())
}

/// Replace colors in an image with device-specific colors
///
/// This is used after dithering to convert the calibrated colors
/// back to the actual device color values.
pub fn replace_colors(
    img: &mut RgbImage,
    original_colors: &[Rgb],
    replacement_colors: &[Rgb],
) -> Result<()> {
    if original_colors.len() != replacement_colors.len() {
        anyhow::bail!(
            "Original and replacement color arrays must have the same length ({} vs {})",
            original_colors.len(),
            replacement_colors.len()
        );
    }

    let (width, height) = img.dimensions();
    let mut error_count = 0;

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let current_color = Rgb::new(pixel[0], pixel[1], pixel[2]);

            // Find matching color in original palette
            if let Some(idx) = original_colors.iter().position(|&c| c == current_color) {
                let new_color = replacement_colors[idx];
                img.put_pixel(x, y, image::Rgb([
                    new_color.r(),
                    new_color.g(),
                    new_color.b(),
                ]));
            } else {
                error_count += 1;
            }
        }
    }

    if error_count > 0 {
        eprintln!(
            "Warning: {} pixels were not replaced (colors didn't match exactly)",
            error_count
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Palette;

    #[test]
    fn test_quantization_only() {
        let mut img = RgbImage::new(2, 2);
        img.put_pixel(0, 0, image::Rgb([100, 100, 100]));
        img.put_pixel(1, 0, image::Rgb([200, 200, 200]));
        img.put_pixel(0, 1, image::Rgb([50, 50, 50]));
        img.put_pixel(1, 1, image::Rgb([150, 150, 150]));

        let palette = Palette::new(
            "test",
            vec![Rgb::new(0, 0, 0), Rgb::new(255, 255, 255)],
        );

        let options = DitherOptions {
            algorithm: DitheringAlgorithm::QuantizationOnly,
            palette,
            serpentine: false,
        };

        dither_image(&mut img, &options).unwrap();

        // All pixels should be either black or white
        for pixel in img.pixels() {
            assert!(
                (pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0)
                    || (pixel[0] == 255 && pixel[1] == 255 && pixel[2] == 255)
            );
        }
    }
}
