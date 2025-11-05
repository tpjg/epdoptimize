//! Color distance calculations in RGB color space

use super::Rgb;

/// Calculate Euclidean distance between two colors in RGB space
///
/// This is the same method used in the original JavaScript implementation.
/// For better perceptual accuracy, consider using LAB color space in the future.
///
/// # Examples
/// ```
/// # use epd_dither::color::{Rgb, distance::euclidean_distance};
/// let black = Rgb::new(0, 0, 0);
/// let white = Rgb::new(255, 255, 255);
/// let distance = euclidean_distance(&black, &white);
/// assert!(distance > 440.0 && distance < 442.0); // sqrt(3 * 255^2) ≈ 441.67
/// ```
pub fn euclidean_distance(color1: &Rgb, color2: &Rgb) -> f64 {
    let r_diff = color1.r() as f64 - color2.r() as f64;
    let g_diff = color1.g() as f64 - color2.g() as f64;
    let b_diff = color1.b() as f64 - color2.b() as f64;

    (r_diff * r_diff + g_diff * g_diff + b_diff * b_diff).sqrt()
}

/// Find the closest color in a palette to the given color
///
/// Returns the index and reference to the closest color
pub fn find_closest_color<'a>(
    color: &Rgb,
    palette: &'a [Rgb],
) -> Option<(usize, &'a Rgb)> {
    if palette.is_empty() {
        return None;
    }

    palette
        .iter()
        .enumerate()
        .map(|(idx, palette_color)| {
            let distance = euclidean_distance(color, palette_color);
            (idx, palette_color, distance)
        })
        .min_by(|(_, _, dist1), (_, _, dist2)| {
            dist1.partial_cmp(dist2).unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(idx, color, _)| (idx, color))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_distance() {
        let black = Rgb::new(0, 0, 0);
        let white = Rgb::new(255, 255, 255);
        let red = Rgb::new(255, 0, 0);

        // Distance between same colors should be 0
        assert_eq!(euclidean_distance(&black, &black), 0.0);

        // Distance between black and white
        let bw_distance = euclidean_distance(&black, &white);
        assert!(bw_distance > 441.0 && bw_distance < 442.0);

        // Distance is symmetric
        assert_eq!(
            euclidean_distance(&black, &white),
            euclidean_distance(&white, &black)
        );
    }

    #[test]
    fn test_find_closest_color() {
        let palette = vec![
            Rgb::new(0, 0, 0),       // black
            Rgb::new(255, 255, 255), // white
            Rgb::new(255, 0, 0),     // red
        ];

        // Dark gray should be closest to black
        let dark_gray = Rgb::new(50, 50, 50);
        let (idx, _) = find_closest_color(&dark_gray, &palette).unwrap();
        assert_eq!(idx, 0);

        // Light gray should be closest to white
        let light_gray = Rgb::new(200, 200, 200);
        let (idx, _) = find_closest_color(&light_gray, &palette).unwrap();
        assert_eq!(idx, 1);

        // Orange should be closest to red
        let orange = Rgb::new(255, 100, 0);
        let (idx, _) = find_closest_color(&orange, &palette).unwrap();
        assert_eq!(idx, 2);
    }
}
