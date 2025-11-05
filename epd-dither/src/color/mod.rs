//! Color types and utilities for palette management and color space operations

pub mod convert;
pub mod distance;
pub mod palette;

pub use palette::Palette;

/// RGB color (8-bit per channel)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgb(pub [u8; 3]);

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self([r, g, b])
    }

    pub fn r(&self) -> u8 {
        self.0[0]
    }

    pub fn g(&self) -> u8 {
        self.0[1]
    }

    pub fn b(&self) -> u8 {
        self.0[2]
    }

    pub fn as_slice(&self) -> &[u8; 3] {
        &self.0
    }
}

/// RGBA color (8-bit per channel including alpha)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgba(pub [u8; 4]);

impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self([r, g, b, a])
    }

    pub fn r(&self) -> u8 {
        self.0[0]
    }

    pub fn g(&self) -> u8 {
        self.0[1]
    }

    pub fn b(&self) -> u8 {
        self.0[2]
    }

    pub fn a(&self) -> u8 {
        self.0[3]
    }

    pub fn to_rgb(&self) -> Rgb {
        Rgb([self.0[0], self.0[1], self.0[2]])
    }
}

impl From<Rgb> for Rgba {
    fn from(rgb: Rgb) -> Self {
        Rgba([rgb.0[0], rgb.0[1], rgb.0[2], 255])
    }
}
