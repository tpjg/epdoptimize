//! Ordered dithering using Bayer matrices

/// Generate a Bayer threshold matrix of the given size
///
/// The JS implementation uses a pre-computed 8x8 matrix and extracts
/// smaller matrices from it. We'll do the same for compatibility.
pub fn create_bayer_matrix(width: u8, height: u8) -> Vec<Vec<usize>> {
    let width = width.min(8) as usize;
    let height = height.min(8) as usize;

    // Pre-computed 8x8 Bayer matrix (from original JS implementation)
    #[rustfmt::skip]
    const BIG_MATRIX: [[usize; 8]; 8] = [
        [0,  48, 12, 60, 3,  51, 15, 63],
        [32, 16, 44, 28, 35, 19, 47, 31],
        [8,  56, 4,  52, 11, 59, 7,  55],
        [40, 24, 36, 20, 43, 27, 39, 32],
        [2,  50, 14, 62, 1,  49, 13, 61],
        [34, 18, 46, 30, 33, 17, 45, 29],
        [10, 58, 6,  54, 9,  57, 5,  53],
        [42, 26, 38, 22, 41, 25, 37, 21],
    ];

    // If we want the full 8x8, return it directly
    if width == 8 && height == 8 {
        return BIG_MATRIX.iter().map(|row| row.to_vec()).collect();
    }

    // Extract smaller matrix and re-index
    let mut matrix: Vec<Vec<usize>> = (0..height)
        .map(|y| (0..width).map(|x| BIG_MATRIX[y][x]).collect())
        .collect();

    // Create index mapping (sort values and assign new indices)
    let mut flat: Vec<usize> = matrix.iter().flat_map(|row| row.iter().copied()).collect();
    flat.sort_unstable();

    let mut index_map = std::collections::HashMap::new();
    for (i, &val) in flat.iter().enumerate() {
        index_map.entry(val).or_insert(i);
    }

    // Re-map matrix values
    for row in &mut matrix {
        for cell in row {
            *cell = index_map[cell];
        }
    }

    matrix
}

/// Apply ordered dithering to a pixel value
///
/// # Arguments
/// * `pixel` - The RGB pixel values [r, g, b]
/// * `x` - X coordinate of the pixel
/// * `y` - Y coordinate of the pixel
/// * `threshold_map` - The Bayer matrix
/// * `threshold` - Threshold adjustment value
pub fn apply_ordered_dither(
    pixel: [u8; 3],
    x: usize,
    y: usize,
    threshold_map: &[Vec<usize>],
    threshold: f64,
) -> [u8; 3] {
    let matrix_height = threshold_map.len();
    let matrix_width = threshold_map[0].len();

    let factor = threshold_map[y % matrix_height][x % matrix_width] as f64
        / (matrix_height * matrix_width) as f64;

    let adjustment = factor * threshold;

    [
        (pixel[0] as f64 + adjustment).clamp(0.0, 255.0) as u8,
        (pixel[1] as f64 + adjustment).clamp(0.0, 255.0) as u8,
        (pixel[2] as f64 + adjustment).clamp(0.0, 255.0) as u8,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bayer_matrix_generation() {
        // Test 4x4 matrix
        let matrix = create_bayer_matrix(4, 4);
        assert_eq!(matrix.len(), 4);
        assert_eq!(matrix[0].len(), 4);

        // Test that all values are unique and in range
        let mut flat: Vec<_> = matrix.iter().flat_map(|row| row.iter().copied()).collect();
        flat.sort_unstable();
        assert_eq!(flat, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

        // Test 8x8 matrix
        let matrix_8x8 = create_bayer_matrix(8, 8);
        assert_eq!(matrix_8x8.len(), 8);
        assert_eq!(matrix_8x8[0].len(), 8);
    }

    #[test]
    fn test_apply_ordered_dither() {
        let matrix = create_bayer_matrix(4, 4);
        let pixel = [128, 128, 128];

        // Just verify it doesn't panic and produces valid RGB values
        let result = apply_ordered_dither(pixel, 0, 0, &matrix, 64.0);
        assert!(result[0] <= 255);
        assert!(result[1] <= 255);
        assert!(result[2] <= 255);
    }
}
