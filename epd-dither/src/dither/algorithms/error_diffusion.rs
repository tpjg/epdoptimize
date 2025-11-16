//! Error diffusion dithering algorithms

use crate::color::{distance::find_closest_color, Rgb};
use crate::dither::{matrices, ErrorDiffusionKernel};

/// Apply error diffusion dithering to an image
///
/// This modifies the image buffer in place, distributing quantization
/// errors to neighboring pixels according to the chosen kernel.
pub fn apply_error_diffusion(
    buffer: &mut [u8],
    width: usize,
    height: usize,
    palette: &[Rgb],
    kernel: ErrorDiffusionKernel,
    serpentine: bool,
) {
    let diffusion_matrix = matrices::get_diffusion_matrix(kernel);

    for y in 0..height {
        // Serpentine scanning: alternate direction for each row
        let x_range: Box<dyn Iterator<Item = usize>> = if serpentine && y % 2 == 1 {
            Box::new((0..width).rev())
        } else {
            Box::new(0..width)
        };

        for x in x_range {
            let idx = (y * width + x) * 3;

            // Get current pixel color
            let old_pixel = Rgb::new(buffer[idx], buffer[idx + 1], buffer[idx + 2]);

            // Find closest palette color
            let (_, &new_pixel) = find_closest_color(&old_pixel, palette)
                .expect("Palette should not be empty");

            // Set the new color
            buffer[idx] = new_pixel.r();
            buffer[idx + 1] = new_pixel.g();
            buffer[idx + 2] = new_pixel.b();

            // Calculate quantization error
            let error_r = old_pixel.r() as f64 - new_pixel.r() as f64;
            let error_g = old_pixel.g() as f64 - new_pixel.g() as f64;
            let error_b = old_pixel.b() as f64 - new_pixel.b() as f64;

            // Distribute error to neighboring pixels
            for entry in diffusion_matrix {
                let nx = if serpentine && y % 2 == 1 {
                    // For right-to-left scan, flip the x offset
                    x as i32 - entry.offset[0]
                } else {
                    x as i32 + entry.offset[0]
                };
                let ny = y as i32 + entry.offset[1];

                // Check bounds
                if nx < 0 || nx >= width as i32 || ny < 0 || ny >= height as i32 {
                    continue;
                }

                let neighbor_idx = (ny as usize * width + nx as usize) * 3;

                // Add weighted error to neighbor
                buffer[neighbor_idx] = (buffer[neighbor_idx] as f64 + error_r * entry.factor)
                    .clamp(0.0, 255.0) as u8;
                buffer[neighbor_idx + 1] = (buffer[neighbor_idx + 1] as f64 + error_g * entry.factor)
                    .clamp(0.0, 255.0) as u8;
                buffer[neighbor_idx + 2] = (buffer[neighbor_idx + 2] as f64 + error_b * entry.factor)
                    .clamp(0.0, 255.0) as u8;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_diffusion_basic() {
        // Create a simple 2x2 image with gray pixels
        let mut buffer = vec![128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128];

        // Black and white palette
        let palette = vec![Rgb::new(0, 0, 0), Rgb::new(255, 255, 255)];

        apply_error_diffusion(
            &mut buffer,
            2,
            2,
            &palette,
            ErrorDiffusionKernel::FloydSteinberg,
            false,
        );

        // All pixels should now be either 0 or 255
        for &val in &buffer {
            assert!(val == 0 || val == 255, "Pixel value should be 0 or 255, got {}", val);
        }
    }
}
