//! Image scaling and resizing for e-ink displays

use anyhow::Result;
use image::{imageops::FilterType, RgbImage};

/// Fit mode for resizing images to target resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FitMode {
    /// Add letterbox/pillarbox bars to preserve aspect ratio (default)
    Letterbox,
    /// Crop image to fill display while preserving aspect ratio
    Crop,
    /// Stretch image to fill display (may distort)
    Fill,
    /// Contain image within bounds (like letterbox but without bars)
    Contain,
}

impl FitMode {
    /// Parse fit mode from string
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "letterbox" => Ok(FitMode::Letterbox),
            "crop" => Ok(FitMode::Crop),
            "fill" | "stretch" => Ok(FitMode::Fill),
            "contain" => Ok(FitMode::Contain),
            _ => anyhow::bail!("Invalid fit mode: {}. Valid options: letterbox, crop, fill, contain", s),
        }
    }
}

/// Scaling filter algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalingFilter {
    /// Nearest neighbor (fastest, lowest quality)
    Nearest,
    /// Triangle/bilinear (fast, medium quality)
    Triangle,
    /// Catmull-Rom cubic (good balance)
    CatmullRom,
    /// Gaussian (smooth)
    Gaussian,
    /// Lanczos3 (best quality, recommended for photos)
    Lanczos3,
}

impl ScalingFilter {
    /// Parse scaling filter from string
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "nearest" => Ok(ScalingFilter::Nearest),
            "triangle" | "bilinear" => Ok(ScalingFilter::Triangle),
            "catmull-rom" | "catmullrom" => Ok(ScalingFilter::CatmullRom),
            "gaussian" => Ok(ScalingFilter::Gaussian),
            "lanczos3" | "lanczos" => Ok(ScalingFilter::Lanczos3),
            _ => anyhow::bail!("Invalid scaling filter: {}. Valid options: nearest, triangle, catmull-rom, gaussian, lanczos3", s),
        }
    }

    /// Convert to image crate's FilterType
    pub fn to_filter_type(&self) -> FilterType {
        match self {
            ScalingFilter::Nearest => FilterType::Nearest,
            ScalingFilter::Triangle => FilterType::Triangle,
            ScalingFilter::CatmullRom => FilterType::CatmullRom,
            ScalingFilter::Gaussian => FilterType::Gaussian,
            ScalingFilter::Lanczos3 => FilterType::Lanczos3,
        }
    }
}

/// Calculate dimensions for resizing with given fit mode
fn calculate_dimensions(
    src_width: u32,
    src_height: u32,
    target_width: u32,
    target_height: u32,
    fit_mode: FitMode,
) -> (u32, u32, i32, i32) {
    match fit_mode {
        FitMode::Fill => {
            // Stretch to fill
            (target_width, target_height, 0, 0)
        }
        FitMode::Contain | FitMode::Letterbox => {
            // Scale to fit inside target, preserving aspect ratio
            let src_ratio = src_width as f64 / src_height as f64;
            let target_ratio = target_width as f64 / target_height as f64;

            let (scaled_width, scaled_height) = if src_ratio > target_ratio {
                // Source is wider - fit to width
                let width = target_width;
                let height = (target_width as f64 / src_ratio).round() as u32;
                (width, height)
            } else {
                // Source is taller - fit to height
                let width = (target_height as f64 * src_ratio).round() as u32;
                let height = target_height;
                (width, height)
            };

            // Calculate centering offset for letterbox
            let offset_x = ((target_width as i32 - scaled_width as i32) / 2).max(0);
            let offset_y = ((target_height as i32 - scaled_height as i32) / 2).max(0);

            (scaled_width, scaled_height, offset_x, offset_y)
        }
        FitMode::Crop => {
            // Scale to fill, then crop
            let src_ratio = src_width as f64 / src_height as f64;
            let target_ratio = target_width as f64 / target_height as f64;

            let (scaled_width, scaled_height) = if src_ratio > target_ratio {
                // Source is wider - fit to height, crop width
                let width = (target_height as f64 * src_ratio).round() as u32;
                let height = target_height;
                (width, height)
            } else {
                // Source is taller - fit to width, crop height
                let width = target_width;
                let height = (target_width as f64 / src_ratio).round() as u32;
                (width, height)
            };

            // Calculate crop offset (negative means we'll crop)
            let offset_x = -((scaled_width as i32 - target_width as i32) / 2).max(0);
            let offset_y = -((scaled_height as i32 - target_height as i32) / 2).max(0);

            (scaled_width, scaled_height, offset_x, offset_y)
        }
    }
}

/// Resize image to target dimensions with specified fit mode and filter
pub fn resize_image(
    img: &RgbImage,
    target_width: u32,
    target_height: u32,
    fit_mode: FitMode,
    filter: ScalingFilter,
    background_color: [u8; 3],
) -> Result<RgbImage> {
    let (scaled_width, scaled_height, offset_x, offset_y) =
        calculate_dimensions(img.width(), img.height(), target_width, target_height, fit_mode);

    // Resize the image
    let resized = image::imageops::resize(img, scaled_width, scaled_height, filter.to_filter_type());

    if fit_mode == FitMode::Letterbox {
        // Create canvas with background color
        let mut canvas = RgbImage::from_pixel(target_width, target_height, image::Rgb(background_color));

        // Copy resized image onto canvas
        image::imageops::overlay(&mut canvas, &resized, offset_x as i64, offset_y as i64);

        Ok(canvas)
    } else if fit_mode == FitMode::Crop {
        // Crop from center
        let crop_x = (-offset_x) as u32;
        let crop_y = (-offset_y) as u32;

        Ok(image::imageops::crop_imm(&resized, crop_x, crop_y, target_width, target_height).to_image())
    } else {
        // Fill or Contain - already at target size
        Ok(resized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fit_mode_parsing() {
        assert_eq!(FitMode::from_str("letterbox").unwrap(), FitMode::Letterbox);
        assert_eq!(FitMode::from_str("crop").unwrap(), FitMode::Crop);
        assert_eq!(FitMode::from_str("fill").unwrap(), FitMode::Fill);
        assert_eq!(FitMode::from_str("stretch").unwrap(), FitMode::Fill);
        assert_eq!(FitMode::from_str("contain").unwrap(), FitMode::Contain);
        assert!(FitMode::from_str("invalid").is_err());
    }

    #[test]
    fn test_scaling_filter_parsing() {
        assert_eq!(
            ScalingFilter::from_str("lanczos3").unwrap(),
            ScalingFilter::Lanczos3
        );
        assert_eq!(
            ScalingFilter::from_str("catmull-rom").unwrap(),
            ScalingFilter::CatmullRom
        );
        assert!(ScalingFilter::from_str("invalid").is_err());
    }

    #[test]
    fn test_calculate_dimensions_letterbox() {
        // Wide source into square target - should letterbox top/bottom
        let (w, h, ox, oy) = calculate_dimensions(1600, 900, 800, 800, FitMode::Letterbox);
        assert_eq!(w, 800);
        assert_eq!(h, 450);
        assert_eq!(ox, 0);
        assert_eq!(oy, 175); // (800 - 450) / 2

        // Tall source into wide target - should letterbox left/right
        let (w, h, ox, oy) = calculate_dimensions(900, 1600, 800, 600, FitMode::Letterbox);
        assert_eq!(w, 337); // 600 * (900/1600)
        assert_eq!(h, 600);
        assert_eq!(ox, 231); // (800 - 337) / 2
        assert_eq!(oy, 0);
    }

    #[test]
    fn test_calculate_dimensions_fill() {
        let (w, h, ox, oy) = calculate_dimensions(1920, 1080, 800, 480, FitMode::Fill);
        assert_eq!(w, 800);
        assert_eq!(h, 480);
        assert_eq!(ox, 0);
        assert_eq!(oy, 0);
    }

    #[test]
    fn test_resize_image_basic() {
        let img = RgbImage::from_pixel(1600, 1200, image::Rgb([255, 0, 0]));
        let resized = resize_image(
            &img,
            800,
            600,
            FitMode::Fill,
            ScalingFilter::Nearest,
            [255, 255, 255],
        )
        .unwrap();

        assert_eq!(resized.width(), 800);
        assert_eq!(resized.height(), 600);
    }
}
