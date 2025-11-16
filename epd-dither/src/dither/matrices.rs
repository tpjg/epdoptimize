//! Error diffusion matrices and kernel definitions

use crate::dither::ErrorDiffusionKernel;

/// An error diffusion matrix entry
#[derive(Debug, Clone, Copy)]
pub struct DiffusionEntry {
    /// Offset from current pixel [x, y]
    pub offset: [i32; 2],
    /// Factor to multiply the error by
    pub factor: f64,
}

/// Get the error diffusion matrix for a given kernel
pub fn get_diffusion_matrix(kernel: ErrorDiffusionKernel) -> &'static [DiffusionEntry] {
    match kernel {
        ErrorDiffusionKernel::FloydSteinberg => &FLOYD_STEINBERG,
        ErrorDiffusionKernel::FalseFloydSteinberg => &FALSE_FLOYD_STEINBERG,
        ErrorDiffusionKernel::Jarvis => &JARVIS,
        ErrorDiffusionKernel::Stucki => &STUCKI,
        ErrorDiffusionKernel::Burkes => &BURKES,
        ErrorDiffusionKernel::Sierra3 => &SIERRA3,
        ErrorDiffusionKernel::Sierra2 => &SIERRA2,
        ErrorDiffusionKernel::Sierra2_4A => &SIERRA2_4A,
    }
}

/// Floyd-Steinberg diffusion matrix
/// Distributes error to 4 neighbors with weights [7, 3, 5, 1] / 16
///
/// ```text
///       X   7/16
///  3/16 5/16 1/16
/// ```
const FLOYD_STEINBERG: [DiffusionEntry; 4] = [
    DiffusionEntry {
        offset: [1, 0],
        factor: 7.0 / 16.0,
    },
    DiffusionEntry {
        offset: [-1, 1],
        factor: 3.0 / 16.0,
    },
    DiffusionEntry {
        offset: [0, 1],
        factor: 5.0 / 16.0,
    },
    DiffusionEntry {
        offset: [1, 1],
        factor: 1.0 / 16.0,
    },
];

/// False Floyd-Steinberg (simplified)
/// Distributes error to 3 neighbors with weights [3, 3, 2] / 8
///
/// ```text
///     X   3/8
///  3/8 2/8
/// ```
const FALSE_FLOYD_STEINBERG: [DiffusionEntry; 3] = [
    DiffusionEntry {
        offset: [1, 0],
        factor: 3.0 / 8.0,
    },
    DiffusionEntry {
        offset: [0, 1],
        factor: 3.0 / 8.0,
    },
    DiffusionEntry {
        offset: [1, 1],
        factor: 2.0 / 8.0,
    },
];

/// Jarvis, Judice, and Ninke diffusion
/// Spreads error over 3 rows for smooth gradients
const JARVIS: [DiffusionEntry; 12] = [
    DiffusionEntry {
        offset: [1, 0],
        factor: 7.0 / 48.0,
    },
    DiffusionEntry {
        offset: [2, 0],
        factor: 5.0 / 48.0,
    },
    DiffusionEntry {
        offset: [-2, 1],
        factor: 3.0 / 48.0,
    },
    DiffusionEntry {
        offset: [-1, 1],
        factor: 5.0 / 48.0,
    },
    DiffusionEntry {
        offset: [0, 1],
        factor: 7.0 / 48.0,
    },
    DiffusionEntry {
        offset: [1, 1],
        factor: 5.0 / 48.0,
    },
    DiffusionEntry {
        offset: [2, 1],
        factor: 3.0 / 48.0,
    },
    DiffusionEntry {
        offset: [-2, 2],
        factor: 1.0 / 48.0,
    },
    DiffusionEntry {
        offset: [-1, 2],
        factor: 3.0 / 48.0,
    },
    DiffusionEntry {
        offset: [0, 2],
        factor: 5.0 / 48.0,
    },
    DiffusionEntry {
        offset: [1, 2],
        factor: 3.0 / 48.0,
    },
    DiffusionEntry {
        offset: [2, 2],
        factor: 1.0 / 48.0,
    },
];

/// Stucki diffusion matrix
const STUCKI: [DiffusionEntry; 12] = [
    DiffusionEntry {
        offset: [1, 0],
        factor: 8.0 / 42.0,
    },
    DiffusionEntry {
        offset: [2, 0],
        factor: 4.0 / 42.0,
    },
    DiffusionEntry {
        offset: [-2, 1],
        factor: 2.0 / 42.0,
    },
    DiffusionEntry {
        offset: [-1, 1],
        factor: 4.0 / 42.0,
    },
    DiffusionEntry {
        offset: [0, 1],
        factor: 8.0 / 42.0,
    },
    DiffusionEntry {
        offset: [1, 1],
        factor: 4.0 / 42.0,
    },
    DiffusionEntry {
        offset: [2, 1],
        factor: 2.0 / 42.0,
    },
    DiffusionEntry {
        offset: [-2, 2],
        factor: 1.0 / 42.0,
    },
    DiffusionEntry {
        offset: [-1, 2],
        factor: 2.0 / 42.0,
    },
    DiffusionEntry {
        offset: [0, 2],
        factor: 4.0 / 42.0,
    },
    DiffusionEntry {
        offset: [1, 2],
        factor: 2.0 / 42.0,
    },
    DiffusionEntry {
        offset: [2, 2],
        factor: 1.0 / 42.0,
    },
];

/// Burkes diffusion matrix
const BURKES: [DiffusionEntry; 7] = [
    DiffusionEntry {
        offset: [1, 0],
        factor: 8.0 / 32.0,
    },
    DiffusionEntry {
        offset: [2, 0],
        factor: 4.0 / 32.0,
    },
    DiffusionEntry {
        offset: [-2, 1],
        factor: 2.0 / 32.0,
    },
    DiffusionEntry {
        offset: [-1, 1],
        factor: 4.0 / 32.0,
    },
    DiffusionEntry {
        offset: [0, 1],
        factor: 8.0 / 32.0,
    },
    DiffusionEntry {
        offset: [1, 1],
        factor: 4.0 / 32.0,
    },
    DiffusionEntry {
        offset: [2, 1],
        factor: 2.0 / 32.0,
    },
];

/// Sierra-3 diffusion matrix
const SIERRA3: [DiffusionEntry; 10] = [
    DiffusionEntry {
        offset: [1, 0],
        factor: 5.0 / 32.0,
    },
    DiffusionEntry {
        offset: [2, 0],
        factor: 3.0 / 32.0,
    },
    DiffusionEntry {
        offset: [-2, 1],
        factor: 2.0 / 32.0,
    },
    DiffusionEntry {
        offset: [-1, 1],
        factor: 4.0 / 32.0,
    },
    DiffusionEntry {
        offset: [0, 1],
        factor: 5.0 / 32.0,
    },
    DiffusionEntry {
        offset: [1, 1],
        factor: 4.0 / 32.0,
    },
    DiffusionEntry {
        offset: [2, 1],
        factor: 2.0 / 32.0,
    },
    DiffusionEntry {
        offset: [-1, 2],
        factor: 2.0 / 32.0,
    },
    DiffusionEntry {
        offset: [0, 2],
        factor: 3.0 / 32.0,
    },
    DiffusionEntry {
        offset: [1, 2],
        factor: 2.0 / 32.0,
    },
];

/// Sierra-2 diffusion matrix
const SIERRA2: [DiffusionEntry; 7] = [
    DiffusionEntry {
        offset: [1, 0],
        factor: 4.0 / 16.0,
    },
    DiffusionEntry {
        offset: [2, 0],
        factor: 3.0 / 16.0,
    },
    DiffusionEntry {
        offset: [-2, 1],
        factor: 1.0 / 16.0,
    },
    DiffusionEntry {
        offset: [-1, 1],
        factor: 2.0 / 16.0,
    },
    DiffusionEntry {
        offset: [0, 1],
        factor: 3.0 / 16.0,
    },
    DiffusionEntry {
        offset: [1, 1],
        factor: 2.0 / 16.0,
    },
    DiffusionEntry {
        offset: [2, 1],
        factor: 1.0 / 16.0,
    },
];

/// Sierra-2-4A diffusion matrix (lightweight, fast)
const SIERRA2_4A: [DiffusionEntry; 3] = [
    DiffusionEntry {
        offset: [1, 0],
        factor: 2.0 / 4.0,
    },
    DiffusionEntry {
        offset: [-1, 1],
        factor: 1.0 / 4.0,
    },
    DiffusionEntry {
        offset: [0, 1],
        factor: 1.0 / 4.0,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_factors_sum() {
        // All factors should sum to 1.0 (or very close due to floating point)
        let matrices = [
            FLOYD_STEINBERG.as_slice(),
            FALSE_FLOYD_STEINBERG.as_slice(),
            JARVIS.as_slice(),
            STUCKI.as_slice(),
            BURKES.as_slice(),
            SIERRA3.as_slice(),
            SIERRA2.as_slice(),
            SIERRA2_4A.as_slice(),
        ];

        for matrix in matrices {
            let sum: f64 = matrix.iter().map(|e| e.factor).sum();
            assert!(
                (sum - 1.0).abs() < 0.0001,
                "Matrix factors should sum to 1.0, got {}",
                sum
            );
        }
    }
}
