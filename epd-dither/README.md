# EPD Dither - Rust CLI Tool

A fast, standalone command-line tool for dithering images for e-ink/e-paper displays. This is a Rust rewrite of the [epdoptimize](https://github.com/Utzel-Butzel/epdoptimize) JavaScript library.

## Features

- **8 Error Diffusion Algorithms**: Floyd-Steinberg, Jarvis, Stucki, Burkes, Sierra variants
- **Ordered Dithering**: Bayer matrix up to 8x8
- **Random Dithering**: RGB and black & white modes
- **Color Palette Support**: Built-in palettes for Spectra 6, AcEP, and custom palettes
- **Device Color Mapping**: Automatic color replacement for e-ink displays
- **High Performance**: 10-50x faster than JavaScript version
- **Single Binary**: No runtime dependencies

## Installation

### From Source

```bash
cargo install --path .
```

### Pre-built Binaries

Download from [GitHub Releases](https://github.com/Utzel-Butzel/epdoptimize/releases)

## Usage

### Basic Usage

```bash
# Dither an image with default settings (Spectra 6, Floyd-Steinberg)
epd-dither -i input.jpg -o output.png

# Use verbose mode to see progress
epd-dither -i input.jpg -o output.png -v
```

### Different Algorithms

```bash
# Use Jarvis error diffusion
epd-dither -i input.jpg -o output.png -a jarvis

# Use ordered dithering with 8x8 Bayer matrix
epd-dither -i input.jpg -o output.png -a ordered --bayer-size 8x8

# Use random black and white dithering
epd-dither -i input.jpg -o output.png -a random-bw
```

### Palettes

```bash
# List all available palettes
epd-dither --list-palettes

# Use AcEP palette
epd-dither -i input.jpg -o output.png -p acep

# Use custom palette
epd-dither -i input.jpg -o output.png -c "#000,#fff,#f00,#0f0,#00f"
```

### Advanced Options

```bash
# Serpentine scanning for smoother gradients
epd-dither -i input.jpg -o output.png -s

# Skip device color replacement
epd-dither -i input.jpg -o output.png --no-color-replace

# Specify device colors explicitly
epd-dither -i input.jpg -o output.png -p spectra6 -d spectra6
```

## Command-Line Options

```
Options:
  -i, --input <FILE>          Input image file
  -o, --output <FILE>         Output image file
  -a, --algorithm <TYPE>      Dithering algorithm [default: floyd-steinberg]
  -p, --palette <PALETTE>     Color palette name [default: spectra6]
  -c, --custom-palette <COLORS>  Custom palette (comma-separated hex)
  -d, --device-colors <TYPE>  Device color mapping
  -s, --serpentine            Use serpentine scanning
  --bayer-size <WxH>          Bayer matrix size [default: 4x4]
  --no-color-replace          Skip device color replacement
  --list-palettes             List available palettes
  -v, --verbose               Verbose output
  -h, --help                  Print help
  -V, --version               Print version
```

## Algorithms

### Error Diffusion
- `floyd-steinberg` - Classic, high quality (default)
- `false-floyd-steinberg` - Simplified, faster
- `jarvis` - Smooth gradients, more blur
- `stucki` - Similar to Jarvis
- `burkes` - Good balance of speed and quality
- `sierra3` - High quality, less blur than Jarvis
- `sierra2` - Reduced computation
- `sierra24a` - Lightweight, very fast

### Other Algorithms
- `ordered` - Bayer matrix ordered dithering
- `random-rgb` - Random RGB dithering
- `random-bw` - Random black and white
- `none` - Quantization only, no dithering

## Supported Palettes

- `default` - Black and white (2 colors)
- `spectra6` - E Ink Spectra 6 (6 colors)
- `acep` - E Ink AcEP (7 colors)
- `gameboy` - Game Boy palette (4 colors)

## Examples

### Convert Photo for Spectra 6 Display

```bash
epd-dither -i vacation.jpg -o vacation-dithered.png -v
```

### High-Quality Art with Jarvis Algorithm

```bash
epd-dither -i artwork.jpg -o artwork-dithered.png -a jarvis -s
```

### Fast Processing with Sierra-2-4A

```bash
epd-dither -i document.png -o document-dithered.png -a sierra24a
```

### Custom 3-Color Palette

```bash
epd-dither -i logo.png -o logo-dithered.png -c "#000000,#FFFFFF,#FF0000"
```

## Performance

Typical performance on a modern CPU:
- 1920x1080 image: ~0.5-2 seconds (depending on algorithm)
- 800x600 image: ~0.1-0.5 seconds

Floyd-Steinberg is ~10-50x faster than the JavaScript version.

## Building

### Debug Build

```bash
cargo build
./target/debug/epd-dither --help
```

### Release Build (Optimized)

```bash
cargo build --release
./target/release/epd-dither --help
```

### Run Tests

```bash
cargo test
```

### Run Benchmarks

```bash
cargo bench
```

## Project Structure

```
epd-dither/
├── src/
│   ├── main.rs              # CLI interface
│   ├── lib.rs               # Library exports
│   ├── color/
│   │   ├── mod.rs           # Color types
│   │   ├── convert.rs       # Color conversion (hex, RGB)
│   │   ├── distance.rs      # Color distance calculation
│   │   └── palette.rs       # Palette management
│   ├── dither/
│   │   ├── mod.rs           # Dithering module
│   │   ├── engine.rs        # Main dithering engine
│   │   ├── matrices.rs      # Error diffusion kernels
│   │   └── algorithms/
│   │       ├── error_diffusion.rs
│   │       ├── ordered.rs
│   │       └── random.rs
│   └── data/
│       ├── palettes.json
│       └── device_colors.json
├── tests/                   # Integration tests
├── benches/                 # Performance benchmarks
└── Cargo.toml
```

## Development

### Running Examples

```bash
# Test with a sample image
curl -O https://via.placeholder.com/800x600.jpg -o test.jpg
cargo run -- -i test.jpg -o test-out.png -v
```

### Adding a New Algorithm

1. Add the algorithm variant to `DitheringAlgorithm` enum
2. Implement the algorithm in `src/dither/algorithms/`
3. Update the engine to handle the new algorithm
4. Add tests

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Apache-2.0

## Credits

Based on [epdoptimize](https://github.com/Utzel-Butzel/epdoptimize) by paperlesspaper.
