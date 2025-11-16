# EPD Optimize - Rust Rewrite Implementation Plan

## Executive Summary

This document outlines the plan to rewrite the `epdoptimize` JavaScript library as a standalone Rust CLI binary tool. The goal is to provide a fast, portable command-line tool for optimizing images for e-ink/e-paper displays through color reduction and dithering.

## Current Package Analysis

### Core Functionality
1. **Image dithering** with multiple algorithms (error diffusion, ordered, random)
2. **Color palette reduction** for limited e-ink displays (2-7 colors)
3. **Color calibration** - maps actual e-ink colors (duller) to better internal representations
4. **Color replacement** - swaps calibrated colors with device-native colors
5. **Predefined palettes** for Spectra 6, AcEP displays, plus custom palette support

### Key Algorithms Implemented
- **8 error diffusion kernels**: Floyd-Steinberg, Jarvis, Stucki, Burkes, Sierra variants
- **Bayer matrix** generation for ordered dithering (up to 8x8)
- **Color distance calculation** using Euclidean RGB space
- **Random dithering** (RGB and B&W modes)

### Current Architecture
```
src/
├── index.ts                              # Main exports
├── dither/
│   ├── dither.ts                         # Core dithering engine (~200 LOC)
│   ├── data/
│   │   ├── diffusion-maps.ts             # Error diffusion matrices
│   │   ├── default-palettes.json         # Color palettes for dithering
│   │   └── default-device-colors.json    # Actual device output colors
│   └── functions/
│       ├── bayer-matrix.ts               # Ordered dithering matrix generation
│       ├── color-helpers.ts              # Hex to RGB conversion
│       ├── find-closest-palette-color.ts # Color matching algorithm
│       └── utilities.ts                  # Random number helpers
└── replaceColors/
    └── replaceColors.ts                  # Color swapping for final output
```

## Why Rust Over Go

1. ⚡ **Performance critical** - Image processing is CPU-intensive; Rust's zero-cost abstractions shine here
2. 🎯 **Memory efficiency** - Direct pixel buffer manipulation without GC pauses
3. 📦 **Smaller binaries** - Can produce <5MB standalone executables
4. 🔧 **Excellent ecosystem** - `image`, `imageproc`, `clap` crates are battle-tested
5. 🚀 **SIMD potential** - Easy to optimize hot loops with vectorization

## Rust Project Structure

```
epd-dither/              # Rust CLI tool name
├── Cargo.toml           # Dependencies: image, clap, serde, serde_json
├── src/
│   ├── main.rs          # CLI interface & argument parsing
│   ├── lib.rs           # Library exports (for reuse)
│   ├── dither/
│   │   ├── mod.rs       # Dithering module exports
│   │   ├── engine.rs    # Core dithering logic
│   │   ├── algorithms/
│   │   │   ├── mod.rs
│   │   │   ├── error_diffusion.rs  # Error diffusion kernels
│   │   │   ├── ordered.rs          # Bayer matrix dithering
│   │   │   └── random.rs           # Random dithering
│   │   └── matrices.rs  # Diffusion kernel definitions
│   ├── color/
│   │   ├── mod.rs
│   │   ├── palette.rs   # Palette management
│   │   ├── distance.rs  # Color distance calculations
│   │   └── convert.rs   # Color space conversions
│   └── data/
│       ├── palettes.json
│       └── device_colors.json
├── tests/               # Integration tests
└── benches/             # Performance benchmarks
```

## Implementation Phases

### Phase 1: Project Setup & Core Structure (1 hour)

**Tasks:**
- Initialize Cargo project with `cargo new epd-dither`
- Set up directory structure
- Configure Cargo.toml with dependencies
- Copy palette JSON files from JS project
- Set up basic module structure

**Dependencies (Cargo.toml):**
```toml
[dependencies]
image = "0.24"           # Image loading/saving
clap = { version = "4.4", features = ["derive"] }  # CLI parsing
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"       # JSON palette files
anyhow = "1.0"           # Error handling
thiserror = "1.0"        # Error types
rand = "0.8"             # Random number generation

[dev-dependencies]
criterion = "0.5"        # Benchmarking
```

### Phase 2: Core Types & Traits (2 hours)

**Define core data structures:**
```rust
// Color types
struct Rgb([u8; 3]);
struct Rgba([u8; 4]);

// Palette
struct Palette {
    colors: Vec<Rgb>,
    name: String,
}

// Dithering configuration
enum DitheringAlgorithm {
    ErrorDiffusion(ErrorDiffusionKernel),
    Ordered { width: u8, height: u8 },
    Random(RandomMode),
    QuantizationOnly,
}

enum ErrorDiffusionKernel {
    FloydSteinberg,
    FalseFloydSteinberg,
    Jarvis,
    Stucki,
    Burkes,
    Sierra3,
    Sierra2,
    Sierra2_4A,
}

struct DitherOptions {
    algorithm: DitheringAlgorithm,
    palette: Palette,
    serpentine: bool,
}
```

### Phase 3: Algorithm Implementation (8-12 hours)

**Priority order:**
1. Color utilities (hex parsing, RGB conversions, distance calculation)
2. Palette matching (find closest color using Euclidean distance)
3. Error diffusion (Floyd-Steinberg first, then others)
4. Bayer matrix generation (ordered dithering)
5. Random dithering
6. Color replacement (final device color mapping)

**Key optimizations:**
- Use `&[u8]` slices for pixel data (zero-copy)
- Pre-compute error diffusion matrices as constants
- Cache color distance calculations where possible
- Consider SIMD for color distance (if profiling shows benefit)

**Error Diffusion Matrices to Implement:**
- Floyd-Steinberg: 4 neighbors, factors [7/16, 3/16, 5/16, 1/16]
- False Floyd-Steinberg: 3 neighbors
- Jarvis: 12 neighbors across 3 rows
- Stucki: 12 neighbors
- Burkes: 7 neighbors across 2 rows
- Sierra3: 10 neighbors
- Sierra2: 7 neighbors
- Sierra2-4A: 3 neighbors

### Phase 4: CLI Interface (2 hours)

**Command structure using `clap`:**
```bash
epd-dither [OPTIONS] <INPUT> -o <OUTPUT>

Options:
  -i, --input <FILE>          Input image file
  -o, --output <FILE>         Output image file
  -a, --algorithm <TYPE>      Dithering algorithm [default: floyd-steinberg]
                              [possible: floyd-steinberg, jarvis, stucki,
                               burkes, sierra3, ordered, random, none]
  -p, --palette <PALETTE>     Color palette [default: spectra6]
                              [possible: spectra6, acep, gameboy, custom]
  -c, --custom-palette <COLORS>  Custom palette (comma-separated hex)
  -d, --device-colors <TYPE>  Device color mapping [default: spectra6]
  -s, --serpentine            Use serpentine scanning
  --bayer-size <WxH>          Bayer matrix size for ordered [default: 4x4]
  --no-color-replace          Skip device color replacement
  --show-palettes             List available palettes
  -v, --verbose               Verbose output

Examples:
  # Basic usage with defaults (Spectra 6, Floyd-Steinberg)
  epd-dither input.jpg -o output.png

  # Use different algorithm and palette
  epd-dither input.jpg -o output.png -a jarvis -p acep

  # Custom palette
  epd-dither input.jpg -o output.png -c "#000,#fff,#f00,#0f0,#00f"

  # Ordered dithering with 8x8 Bayer matrix
  epd-dither input.jpg -o output.png -a ordered --bayer-size 8x8
```

### Phase 5: Image I/O (3 hours)

**Use `image` crate for:**
- Loading: JPEG, PNG, BMP, WebP, etc.
- Saving: PNG (lossless, suitable for dithered output)
- Format conversion: RGB8, RGBA8 buffers

**Process flow:**
```rust
1. Load image → DynamicImage
2. Convert to RGB8/RGBA8 buffer
3. Apply dithering algorithm (mutate buffer)
4. Optionally replace colors with device colors
5. Create new image from buffer
6. Save to output format
```

### Phase 6: Testing & Validation (4 hours)

**Test strategy:**
1. Unit tests for each algorithm
2. Visual regression tests (compare with JS output)
3. Performance benchmarks vs JavaScript version
4. Edge cases: 1x1 images, large images, extreme palettes

**Validation approach:**
- Copy the example image from the demo
- Run both JS and Rust versions with identical settings
- Compare outputs pixel-by-pixel (allow minor floating-point differences)

**Tests to write:**
- Color conversion accuracy (hex → RGB)
- Color distance calculation
- Palette matching correctness
- Each dithering algorithm produces expected output
- Bayer matrix generation matches expected values
- Error diffusion kernel coefficients sum correctly

### Phase 7: Distribution & Packaging (2 hours)

**Build targets:**
```bash
# Native binary for Linux/macOS/Windows
cargo build --release

# Optimized for size
cargo build --release --features slim

# Static binary (Linux)
RUSTFLAGS='-C target-feature=+crt-static' cargo build --release
```

**Distribution options:**
- GitHub Releases with pre-built binaries
- `cargo install epd-dither`
- Docker container for reproducible builds
- Homebrew/APT packages (later)

## Timeline Estimate

| Phase | Complexity | Time Estimate |
|-------|-----------|---------------|
| 1. Project Setup | Low | 1 hour |
| 2. Core Types | Low | 2 hours |
| 3. Algorithms | Medium-High | 8-12 hours |
| 4. CLI Interface | Low | 2 hours |
| 5. Image I/O | Medium | 3 hours |
| 6. Testing | Medium | 4 hours |
| 7. Distribution | Low | 2 hours |
| **Total** | | **22-28 hours** |

## Expected Benefits

1. **Performance**: 10-50x faster than JavaScript version
2. **Portability**: Single binary, no runtime dependencies
3. **Size**: ~5MB binary vs hundreds of MB with Node.js
4. **Reliability**: Strong type system catches errors at compile time
5. **Usability**: Easy CLI for batch processing and automation

## Future Enhancements

1. **WebAssembly target** - Run in browser (replace the JS version!)
2. **Batch processing** - Multiple images in one command
3. **Real-time preview** - Interactive palette/algorithm selection
4. **GPU acceleration** - For very large images
5. **Advanced color spaces** - LAB, LUV for better perceptual matching
6. **Automatic palette extraction** - K-means clustering from input image

## Success Criteria

- ✅ All 8 dithering algorithms produce visually identical output to JS version
- ✅ CLI is intuitive and well-documented
- ✅ Binary size < 10MB
- ✅ Processing speed > 10x faster than JS
- ✅ Works on Linux, macOS, Windows
- ✅ Comprehensive test coverage (>80%)

---

*Plan created: 2025-11-05*
