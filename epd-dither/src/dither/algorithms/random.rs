//! Random dithering algorithms

use rand::Rng;

use crate::dither::RandomMode;

/// Apply random dithering to a pixel
pub fn apply_random_dither(pixel: [u8; 3], mode: RandomMode) -> [u8; 3] {
    match mode {
        RandomMode::Rgb => random_dither_rgb(pixel),
        RandomMode::BlackAndWhite => random_dither_bw(pixel),
    }
}

/// RGB random dithering - each channel independently
fn random_dither_rgb(pixel: [u8; 3]) -> [u8; 3] {
    let mut rng = rand::thread_rng();

    [
        if pixel[0] < rng.gen_range(0..=255) {
            0
        } else {
            255
        },
        if pixel[1] < rng.gen_range(0..=255) {
            0
        } else {
            255
        },
        if pixel[2] < rng.gen_range(0..=255) {
            0
        } else {
            255
        },
    ]
}

/// Black and white random dithering - uses luminosity
fn random_dither_bw(pixel: [u8; 3]) -> [u8; 3] {
    let mut rng = rand::thread_rng();

    // Calculate average RGB value (simple luminosity)
    let average = (pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3;

    if average < rng.gen_range(0..=255) as u32 {
        [0, 0, 0]
    } else {
        [255, 255, 255]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_dither_rgb() {
        let pixel = [128, 128, 128];
        let result = random_dither_rgb(pixel);

        // Result should be either 0 or 255 for each channel
        for &val in &result {
            assert!(val == 0 || val == 255);
        }
    }

    #[test]
    fn test_random_dither_bw() {
        let pixel = [128, 128, 128];
        let result = random_dither_bw(pixel);

        // Result should be either all black or all white
        assert!(
            result == [0, 0, 0] || result == [255, 255, 255],
            "Got {:?}",
            result
        );
    }
}
