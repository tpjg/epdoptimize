//! Integration tests for the EPD dither library

use epd_dither::{
    color::{palette::PaletteManager, Palette, Rgb},
    dither::{DitheringAlgorithm, DitherOptions, ErrorDiffusionKernel},
    process_image,
};
use image::RgbImage;

#[test]
fn test_basic_dithering() {
    // Create a simple gradient image
    let mut img = RgbImage::new(100, 100);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let value = ((x + y) as f32 / 200.0 * 255.0) as u8;
        *pixel = image::Rgb([value, value, value]);
    }

    let palette = Palette::new("test", vec![Rgb::new(0, 0, 0), Rgb::new(255, 255, 255)]);

    let options = DitherOptions {
        algorithm: DitheringAlgorithm::ErrorDiffusion(ErrorDiffusionKernel::FloydSteinberg),
        palette,
        serpentine: false,
    };

    // Should not panic
    process_image(&mut img, &options).expect("Dithering should succeed");

    // All pixels should be either black or white
    for pixel in img.pixels() {
        assert!(
            (pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0)
                || (pixel[0] == 255 && pixel[1] == 255 && pixel[2] == 255),
            "Pixel should be black or white, got {:?}",
            pixel
        );
    }
}

#[test]
fn test_all_error_diffusion_algorithms() {
    let mut img = RgbImage::new(10, 10);
    for pixel in img.pixels_mut() {
        *pixel = image::Rgb([128, 128, 128]);
    }

    let palette = Palette::new("test", vec![Rgb::new(0, 0, 0), Rgb::new(255, 255, 255)]);

    let algorithms = vec![
        ErrorDiffusionKernel::FloydSteinberg,
        ErrorDiffusionKernel::FalseFloydSteinberg,
        ErrorDiffusionKernel::Jarvis,
        ErrorDiffusionKernel::Stucki,
        ErrorDiffusionKernel::Burkes,
        ErrorDiffusionKernel::Sierra3,
        ErrorDiffusionKernel::Sierra2,
        ErrorDiffusionKernel::Sierra2_4A,
    ];

    for algo in algorithms {
        let mut test_img = img.clone();
        let options = DitherOptions {
            algorithm: DitheringAlgorithm::ErrorDiffusion(algo),
            palette: palette.clone(),
            serpentine: false,
        };

        process_image(&mut test_img, &options).expect("Dithering should succeed");
    }
}

#[test]
fn test_ordered_dithering() {
    let mut img = RgbImage::new(10, 10);
    for pixel in img.pixels_mut() {
        *pixel = image::Rgb([128, 128, 128]);
    }

    let palette = Palette::new("test", vec![Rgb::new(0, 0, 0), Rgb::new(255, 255, 255)]);

    let options = DitherOptions {
        algorithm: DitheringAlgorithm::Ordered {
            width: 4,
            height: 4,
        },
        palette,
        serpentine: false,
    };

    process_image(&mut img, &options).expect("Dithering should succeed");
}

#[test]
fn test_palette_manager() {
    let manager = PaletteManager::new().expect("Should load palettes");

    // Test loading default palette
    let default_palette = manager
        .get_palette("default")
        .expect("Default palette should exist");
    assert_eq!(default_palette.name, "default");
    assert!(!default_palette.is_empty());

    // Test loading spectra6 palette
    let spectra6 = manager
        .get_palette("spectra6")
        .expect("Spectra6 palette should exist");
    assert_eq!(spectra6.len(), 6);

    // Test device colors
    let device_colors = manager
        .get_device_colors("spectra6")
        .expect("Spectra6 device colors should exist");
    assert_eq!(device_colors.len(), 6);
}

#[test]
fn test_serpentine_mode() {
    let mut img = RgbImage::new(10, 10);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let value = ((x + y) * 255 / 20) as u8;
        *pixel = image::Rgb([value, value, value]);
    }

    let palette = Palette::new("test", vec![Rgb::new(0, 0, 0), Rgb::new(255, 255, 255)]);

    let options = DitherOptions {
        algorithm: DitheringAlgorithm::ErrorDiffusion(ErrorDiffusionKernel::FloydSteinberg),
        palette,
        serpentine: true,
    };

    process_image(&mut img, &options).expect("Serpentine dithering should succeed");
}

#[test]
fn test_multi_color_palette() {
    let mut img = RgbImage::new(20, 20);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = (x * 255 / 20) as u8;
        let g = (y * 255 / 20) as u8;
        let b = 128;
        *pixel = image::Rgb([r, g, b]);
    }

    let palette = Palette::new(
        "test",
        vec![
            Rgb::new(0, 0, 0),
            Rgb::new(255, 0, 0),
            Rgb::new(0, 255, 0),
            Rgb::new(0, 0, 255),
            Rgb::new(255, 255, 255),
        ],
    );

    let options = DitherOptions {
        algorithm: DitheringAlgorithm::ErrorDiffusion(ErrorDiffusionKernel::FloydSteinberg),
        palette: palette.clone(),
        serpentine: false,
    };

    process_image(&mut img, &options).expect("Multi-color dithering should succeed");

    // Verify all pixels are in the palette
    for pixel in img.pixels() {
        let color = Rgb::new(pixel[0], pixel[1], pixel[2]);
        assert!(
            palette.colors.contains(&color),
            "Pixel {:?} not in palette",
            color
        );
    }
}
