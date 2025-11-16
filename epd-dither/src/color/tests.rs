//! Unit tests for Rgb and Rgba color types

use super::{Rgb, Rgba};

#[test]
fn test_rgb_creation() {
    let rgb = Rgb::new(100, 150, 200);
    assert_eq!(rgb.r(), 100);
    assert_eq!(rgb.g(), 150);
    assert_eq!(rgb.b(), 200);
}

#[test]
fn test_rgb_accessors() {
    let rgb = Rgb::new(1, 2, 3);
    assert_eq!(rgb.r(), 1);
    assert_eq!(rgb.g(), 2);
    assert_eq!(rgb.b(), 3);
    assert_eq!(rgb.as_slice(), &[1, 2, 3]);
}

#[test]
fn test_rgb_boundaries() {
    // Test min values
    let black = Rgb::new(0, 0, 0);
    assert_eq!(black.r(), 0);
    assert_eq!(black.g(), 0);
    assert_eq!(black.b(), 0);

    // Test max values
    let white = Rgb::new(255, 255, 255);
    assert_eq!(white.r(), 255);
    assert_eq!(white.g(), 255);
    assert_eq!(white.b(), 255);
}

#[test]
fn test_rgb_equality() {
    let rgb1 = Rgb::new(100, 100, 100);
    let rgb2 = Rgb::new(100, 100, 100);
    let rgb3 = Rgb::new(100, 100, 101);

    assert_eq!(rgb1, rgb2);
    assert_ne!(rgb1, rgb3);
}

#[test]
fn test_rgb_clone() {
    let rgb1 = Rgb::new(50, 75, 100);
    let rgb2 = rgb1;
    assert_eq!(rgb1, rgb2);
}

#[test]
fn test_rgba_creation() {
    let rgba = Rgba::new(100, 150, 200, 255);
    assert_eq!(rgba.r(), 100);
    assert_eq!(rgba.g(), 150);
    assert_eq!(rgba.b(), 200);
    assert_eq!(rgba.a(), 255);
}

#[test]
fn test_rgba_accessors() {
    let rgba = Rgba::new(10, 20, 30, 40);
    assert_eq!(rgba.r(), 10);
    assert_eq!(rgba.g(), 20);
    assert_eq!(rgba.b(), 30);
    assert_eq!(rgba.a(), 40);
}

#[test]
fn test_rgba_to_rgb() {
    let rgba = Rgba::new(100, 150, 200, 128);
    let rgb = rgba.to_rgb();
    assert_eq!(rgb.r(), 100);
    assert_eq!(rgb.g(), 150);
    assert_eq!(rgb.b(), 200);
}

#[test]
fn test_rgba_from_rgb() {
    let rgb = Rgb::new(50, 75, 100);
    let rgba: Rgba = rgb.into();
    assert_eq!(rgba.r(), 50);
    assert_eq!(rgba.g(), 75);
    assert_eq!(rgba.b(), 100);
    assert_eq!(rgba.a(), 255); // Default alpha
}

#[test]
fn test_rgba_boundaries() {
    // Transparent black
    let transparent = Rgba::new(0, 0, 0, 0);
    assert_eq!(transparent.a(), 0);

    // Opaque white
    let opaque_white = Rgba::new(255, 255, 255, 255);
    assert_eq!(opaque_white.a(), 255);

    // Semi-transparent color
    let semi = Rgba::new(128, 128, 128, 128);
    assert_eq!(semi.a(), 128);
}

#[test]
fn test_rgba_equality() {
    let rgba1 = Rgba::new(100, 100, 100, 255);
    let rgba2 = Rgba::new(100, 100, 100, 255);
    let rgba3 = Rgba::new(100, 100, 100, 128);

    assert_eq!(rgba1, rgba2);
    assert_ne!(rgba1, rgba3);
}
