//! Color space conversion utilities

use super::Rgb;
use anyhow::{anyhow, Result};

/// Convert a hex color string to RGB
///
/// Supports both 3-digit (#RGB) and 6-digit (#RRGGBB) formats,
/// with or without the leading '#'
///
/// # Examples
/// ```
/// # use epd_dither::color::convert::hex_to_rgb;
/// assert_eq!(hex_to_rgb("#FF0000").unwrap(), [255, 0, 0]);
/// assert_eq!(hex_to_rgb("00FF00").unwrap(), [0, 255, 0]);
/// assert_eq!(hex_to_rgb("#F0F").unwrap(), [255, 0, 255]);
/// ```
pub fn hex_to_rgb(hex: &str) -> Result<[u8; 3]> {
    let hex = hex.trim_start_matches('#');

    // Handle shorthand notation (#RGB -> #RRGGBB)
    let expanded = if hex.len() == 3 {
        hex.chars()
            .flat_map(|c| std::iter::repeat(c).take(2))
            .collect::<String>()
    } else {
        hex.to_string()
    };

    if expanded.len() != 6 {
        return Err(anyhow!("Invalid hex color format: {}", hex));
    }

    let r = u8::from_str_radix(&expanded[0..2], 16)
        .map_err(|_| anyhow!("Invalid red component: {}", &expanded[0..2]))?;
    let g = u8::from_str_radix(&expanded[2..4], 16)
        .map_err(|_| anyhow!("Invalid green component: {}", &expanded[2..4]))?;
    let b = u8::from_str_radix(&expanded[4..6], 16)
        .map_err(|_| anyhow!("Invalid blue component: {}", &expanded[4..6]))?;

    Ok([r, g, b])
}

/// Convert RGB to hex string
pub fn rgb_to_hex(rgb: &Rgb) -> String {
    format!("#{:02X}{:02X}{:02X}", rgb.r(), rgb.g(), rgb.b())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(hex_to_rgb("#000000").unwrap(), [0, 0, 0]);
        assert_eq!(hex_to_rgb("#FFFFFF").unwrap(), [255, 255, 255]);
        assert_eq!(hex_to_rgb("#FF0000").unwrap(), [255, 0, 0]);
        assert_eq!(hex_to_rgb("00FF00").unwrap(), [0, 255, 0]);
        assert_eq!(hex_to_rgb("#0000FF").unwrap(), [0, 0, 255]);

        // Shorthand notation
        assert_eq!(hex_to_rgb("#000").unwrap(), [0, 0, 0]);
        assert_eq!(hex_to_rgb("#FFF").unwrap(), [255, 255, 255]);
        assert_eq!(hex_to_rgb("#F0F").unwrap(), [255, 0, 255]);
    }

    #[test]
    fn test_rgb_to_hex() {
        assert_eq!(rgb_to_hex(&Rgb::new(0, 0, 0)), "#000000");
        assert_eq!(rgb_to_hex(&Rgb::new(255, 255, 255)), "#FFFFFF");
        assert_eq!(rgb_to_hex(&Rgb::new(255, 0, 0)), "#FF0000");
    }
}
