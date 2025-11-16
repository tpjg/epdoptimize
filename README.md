# EDP Optimize

[Interactive demo](https://utzel-butzel.github.io/epdoptimize/) 📦📦 📦

A library and CLI tool for reducing image colors and dithering them to fit (color) eInk displays for better visual quality.

Available in two implementations:
- **JavaScript/TypeScript**: NPM library for use in web applications and Node.js
- **Rust**: High-performance CLI tool for batch processing and server-side use

### Why?

E-paper displays have a more limited color range than LCD screens. By their nature, they cannot reproduce the full sRGB spectrum and can only reflect light rather than emit it. To help minimize these "limitations", this library applies color calibration that translates internal colors to better match the display’s capabilities.

We are using it for our eInk picture frames at [paperlesspaper](https://paperlesspaper.de/en).

The library works with both front end js (using the Browser Canvas API) and node.js (using [node-canvas](https://www.npmjs.com/package/canvas))

[Blogpost](https://paperlesspaper.de/en/blog/dither-eink-tool-open-source)

Btw. you can order our new Spectra 6 eInk picture frame [here](https://paperlesspaper.de/buy-7-inch-epaper-picture-frame). 🎉

[![Node.js Package](https://github.com/Utzel-Butzel/epdoptimize/actions/workflows/npm-publish.yml/badge.svg)](https://github.com/Utzel-Butzel/epdoptimize/actions/workflows/npm-publish.yml)

## Supported Displays

- [AcEP](https://www.eink.com/brand/detail/Gallery)
- [Spectra 6](https://www.eink.com/brand?bookmark=Spectra)

You can easily add your own displays and use custom color tables.

![Intro image](https://raw.githubusercontent.com/Utzel-Butzel/epdoptimize/refs/heads/main/intro-image.jpg)

## Features

- **Device Presets:** Built-in configurations for popular e-ink displays with automatic resolution and palette selection
- **Smart Resizing:** Automatic image scaling to match device resolution before dithering for optimal quality
- **Fit Modes:** Letterbox, crop, fill, and contain options for handling aspect ratios
- **Dithering Algorithms:** Multiple high-quality dithering options to improve color blending and gradients
- **Color Calibration:** Match device-specific color characteristics for more accurate results

## Installation

### JavaScript/TypeScript (NPM)

```bash
npm install epdoptimize
```

### Rust CLI

The Rust implementation provides a standalone CLI tool for high-performance image processing.

#### Building from Source

```bash
cd epd-dither
cargo build --release
```

The optimized binary will be located at `./target/release/epd-dither`

#### Release Build Optimizations

The Rust implementation is configured for maximum performance with:
- Full optimization (`opt-level = 3`)
- Link-time optimization (LTO)
- Single codegen unit for better inlining
- Stripped debug symbols

Expected binary size: 3-5 MB (standalone, no runtime required)

## Usage Examples

### JavaScript/TypeScript

```html
<canvas id="inputCanvas" />
<canvas id="ditheredCanvas" />
<canvas id="ditheredCanvasWithDeviceColors" />
```

```js
import { ditherImage, getDefaultPalettes, getDeviceColors } from 'epdoptimize';

// Access the canvas elements
const inputCanvas = document.getElementById("inputCanvas");
const ditheredCanvas =  document.getElementById("ditheredCanvas");
const ditheredCanvasWithDeviceColors =  document.getElementById("ditheredCanvasWithDeviceColors");

const palette = getDefaultPalettes('spectra6');
const spectra6colors = getDeviceColors('spectra6'); // Spectra 6 color set (can be default, spectra6 or acep)

const options = {
  algorithm: 'floydSteinberg',
  palette,
};

// Dither the image
const dithered = ditherImage(inputCanvas, ditheredCanvas, options);

// Convert the colors to the displays native colors
const prepared = replaceColors(ditheredCanvas,ditheredCanvasWithDeviceColors {
    originalColors: palette,
    replaceColors: spectra6colors
});

```

### Rust CLI

#### Device Presets (Recommended)

The easiest way to prepare images is using device presets, which automatically configure resolution, palette, and optimal settings:

```bash
# List all available devices
epd-dither --list-devices

# Process image for a specific device (auto-resizes to device resolution)
epd-dither -i photo.jpg -o output.png --device spectra6-7.3

# Process for 13.3" Spectra 6 display (1600×1200)
epd-dither -i photo.jpg -o output.png --device spectra6-13.3 --verbose

# Process for ACeP Gallery display
epd-dither -i photo.jpg -o output.png --device acep-7.3
```

**Supported Devices:**
- **Spectra 6**: 4.0", 5.65", 7.3", 8.14", 10.3", 13.3", 25.3", 31.5" displays (6-color)
- **ACeP/Gallery**: 5.65", 7.3", 13.3" displays (7-color)
- **Carta**: 6.0", 10.3", 13.3" displays (e-readers, monochrome high-res)
- **Waveshare**: 7.5" 3-color displays

#### Image Scaling Options

When processing images for specific resolutions, you can control how they're resized:

```bash
# Letterbox mode (default) - adds borders to preserve aspect ratio
epd-dither -i photo.jpg -o output.png \
  --target-width 800 --target-height 480 \
  --fit-mode letterbox \
  --letterbox-color "#ffffff"

# Crop mode - fills display by cropping edges
epd-dither -i photo.jpg -o output.png \
  --target-width 1600 --target-height 1200 \
  --fit-mode crop

# Fill mode - stretches to fill (may distort aspect ratio)
epd-dither -i photo.jpg -o output.png \
  --target-width 800 --target-height 480 \
  --fit-mode fill

# Contain mode - fits within bounds without borders
epd-dither -i photo.jpg -o output.png \
  --target-width 800 --target-height 480 \
  --fit-mode contain
```

**Scaling Algorithms:**
- `lanczos3` (default) - Best quality for photos, slightly slower
- `catmull-rom` - Good balance of quality and speed
- `gaussian` - Smooth results
- `triangle` - Faster, medium quality
- `nearest` - Fastest, lowest quality

```bash
# Use different scaling algorithm
epd-dither -i photo.jpg -o output.png \
  --device spectra6-7.3 \
  --scaling-algorithm catmull-rom
```

#### Basic Usage

```bash
# Dither an image with default settings (Floyd-Steinberg, black & white)
epd-dither -i photo.jpg -o photo-dithered.png

# Use Spectra 6 palette
epd-dither -i photo.jpg -o photo-dithered.png -p spectra6

# Use Jarvis algorithm with serpentine scanning
epd-dither -i photo.jpg -o output.png -a jarvis --serpentine

# Ordered dithering with 8x8 Bayer matrix
epd-dither -i photo.jpg -o output.png -a ordered --bayer-size 8x8

# Custom palette
epd-dither -i logo.png -o logo-out.png -c "#000000,#ffffff,#ff0000"

# List available palettes
epd-dither --list-palettes
```

#### Available Algorithms

- `floyd-steinberg` (default) - Classic error diffusion
- `false-floyd-steinberg` - Simplified Floyd-Steinberg
- `jarvis` - Jarvis, Judice, and Ninke
- `stucki` - Stucki error diffusion
- `burkes` - Burkes error diffusion
- `sierra3` - Sierra-3 (original)
- `sierra2` - Sierra-2 (reduced)
- `sierra2-4a` - Sierra-2-4A (lightweight)
- `ordered` - Bayer matrix ordered dithering
- `random-rgb` - Random dithering (per-channel)
- `random-bw` - Random dithering (black & white)
- `quantize` - Color quantization only (no dithering)

#### Performance

The Rust implementation is **10-50x faster** than the JavaScript version:
- No garbage collection pauses
- Native code execution
- Optimized memory layout
- Single standalone binary (3-5 MB)

## Dithering Options (JavaScript)

| Option                   | Type             | Default          | Description                                                                                                                                 |
| ------------------------ | ---------------- | ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| `ditheringType`          | string           | "errorDiffusion" | The main dithering algorithm. Options: `errorDiffusion`, `ordered`, `random`, `quantizationOnly`.                                           |
| `errorDiffusionMatrix`   | string           | "floydSteinberg" | Error diffusion kernel. Options: `floydSteinberg`, `falseFloydSteinberg`, `jarvis`, `stucki`, `burkes`, `sierra3`, `sierra2`, `sierra2-4a`. |
| `serpentine`             | boolean          | false            | If true, alternates scan direction for each row (serpentine scanning) in error diffusion.                                                   |
| `orderedDitheringType`   | string           | "bayer"          | Type of ordered dithering. Currently only `bayer` is supported.                                                                             |
| `orderedDitheringMatrix` | [number, number] | [4, 4]           | Size of the Bayer matrix for ordered dithering.                                                                                             |
| `randomDitheringType`    | string           | "blackAndWhite"  | Type of random dithering. Options: `blackAndWhite`, `rgb`.                                                                                  |
| `palette`                | string/array     | "default"        | Palette to use for quantization. Can be a string (predefined) or a custom array of colors.                                                  |
| `sampleColorsFromImage`  | boolean          | false            | If true, generates palette by sampling colors from the image.                                                                               |
| `numberOfSampleColors`   | number           | 10               | Number of colors to sample from the image if `sampleColorsFromImage` is true.                                                               |

Add these options to your `ditherImage` call to customize dithering behavior for your use case.

![Convertion Example](https://paperlesspaper.de/_next/image?url=https%3A%2F%2Fres.cloudinary.com%2Fwirewire%2Fimage%2Fupload%2Feink-color-convertion-1.jpg.jpg&w=3840&q=75)

## How It Works

### Color Calibration

eInk displays often render colors less vibrantly than their digital values suggest (e.g., a device red like `#ff0000` may appear duller in reality). By calibrating with real-world color measurements, the library ensures that dithering and color reduction use the actual appearance of colors on your target display. After processing, you can map the calibrated colors back to the device's required values.

### Dithering Algorithms

Dithering helps create the illusion of intermediate colors by distributing quantization errors across neighboring pixels. This is especially important for eInk displays with limited color palettes.

#### Available Diffusion Algorithms

| Algorithm               | Description                                                                                        |
| ----------------------- | -------------------------------------------------------------------------------------------------- |
| **floydSteinberg**      | Classic Floyd-Steinberg error diffusion. Distributes error to four neighbors. Visually pleasing.   |
| **falseFloydSteinberg** | Simplified Floyd-Steinberg. Distributes error to three neighbors. Faster, slightly different look. |
| **jarvis**              | Jarvis, Judice, and Ninke. Spreads error over three rows for smooth gradients, more blurring.      |
| **stucki**              | Similar to Jarvis, different weights. Balances smoothness and sharpness.                           |
| **burkes**              | Simplified Stucki. Fewer neighbors, less computation, good results.                                |
| **sierra3**             | Sierra-3 (original). High-quality, less blurring than Jarvis.                                      |
| **sierra2**             | Reduced Sierra-3. Fewer neighbors, faster, less diffusion.                                         |
| **sierra2-4a**          | Lightweight, fast. Distributes error to three neighbors. Good for speed-critical use.              |

## Using Your Own Colors

You can use your own custom color palette by passing an array of colors to the `palette` option. Colors should be provided as hex strings (e.g., `#FF0000`).

**Example:**

```js
const myPalette = [
  "#000000", // black
  "#FFFFFF", // white
  "#FF0000", // red
  "#00FF00", // green
  "#0000FF", // blue
];

const options = {
  ditheringType: "errorDiffusion",
  palette: myPalette,
};

const dithered = ditherImage(image, options);
```

## Resources

- [paperlesspaper](https://paperlesspaper.de)

## Credits

- [Dither me this](https://github.com/DitheringIdiot/dither-me-this)
- [Inkify](https://github.com/cmdwtf/Inkify)

---

_Contributions and feedback are welcome!_
