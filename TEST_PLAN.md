# Comprehensive Test Plan for Rust CLI Tool

## Overview

This document outlines the complete testing strategy to validate the Rust implementation against the JavaScript original.

## Prerequisites

### 1. Build Both Versions

```bash
# Build Rust version
cd epd-dither
cargo build --release
cargo test
cd ..

# Build JavaScript version
npm install
npm run build
```

### 2. Prepare Test Images

```bash
# Create test images directory
mkdir -p test_outputs
mkdir -p test_inputs

# Copy example image
cp examples/example-dither.jpg test_inputs/

# Download additional test images (various sizes and complexities)
# - Small image (100x100)
# - Medium image (800x600)
# - Large image (1920x1080)
# - Gradient test
# - Color test pattern
```

## Test Categories

### 1. Algorithm Correctness Tests

Test each dithering algorithm produces correct output.

#### Test Script: `test_algorithms.sh`

```bash
#!/bin/bash

set -e

RUST_BIN="./epd-dither/target/release/epd-dither"
TEST_IMAGE="test_inputs/example-dither.jpg"
OUTPUT_DIR="test_outputs"

algorithms=(
    "floyd-steinberg"
    "false-floyd-steinberg"
    "jarvis"
    "stucki"
    "burkes"
    "sierra3"
    "sierra2"
    "sierra24a"
    "ordered"
    "random-rgb"
    "random-bw"
    "none"
)

echo "Testing all algorithms..."
for algo in "${algorithms[@]}"; do
    echo "Testing algorithm: $algo"
    $RUST_BIN -i "$TEST_IMAGE" -o "$OUTPUT_DIR/rust_${algo}.png" -a "$algo" -v

    # Verify output exists and is valid PNG
    if [ ! -f "$OUTPUT_DIR/rust_${algo}.png" ]; then
        echo "ERROR: Output file not created for $algo"
        exit 1
    fi

    # Check file size is reasonable
    size=$(stat -f%z "$OUTPUT_DIR/rust_${algo}.png" 2>/dev/null || stat -c%s "$OUTPUT_DIR/rust_${algo}.png")
    if [ "$size" -lt 1000 ]; then
        echo "ERROR: Output file too small for $algo (${size} bytes)"
        exit 1
    fi

    echo "✓ $algo passed"
done

echo "All algorithm tests passed!"
```

### 2. Palette Tests

Test all built-in palettes and custom palettes.

#### Test Script: `test_palettes.sh`

```bash
#!/bin/bash

set -e

RUST_BIN="./epd-dither/target/release/epd-dither"
TEST_IMAGE="test_inputs/example-dither.jpg"
OUTPUT_DIR="test_outputs"

# Test built-in palettes
palettes=("default" "spectra6" "acep" "gameboy")

echo "Testing built-in palettes..."
for palette in "${palettes[@]}"; do
    echo "Testing palette: $palette"
    $RUST_BIN -i "$TEST_IMAGE" -o "$OUTPUT_DIR/rust_palette_${palette}.png" -p "$palette" -v
    echo "✓ $palette passed"
done

# Test custom palettes
echo "Testing custom palettes..."
$RUST_BIN -i "$TEST_IMAGE" -o "$OUTPUT_DIR/rust_custom_bw.png" -c "#000000,#FFFFFF" -v
$RUST_BIN -i "$TEST_IMAGE" -o "$OUTPUT_DIR/rust_custom_rgb.png" -c "#000,#F00,#0F0,#00F,#FFF" -v
$RUST_BIN -i "$TEST_IMAGE" -o "$OUTPUT_DIR/rust_custom_3color.png" -c "#000000,#808080,#FFFFFF" -v

echo "All palette tests passed!"
```

### 3. JS vs Rust Comparison Tests

Compare output between JS and Rust implementations.

#### Test Script: `compare_js_rust.sh`

```bash
#!/bin/bash

set -e

RUST_BIN="./epd-dither/target/release/epd-dither"
TEST_IMAGE="test_inputs/example-dither.jpg"
OUTPUT_DIR="test_outputs/comparison"
mkdir -p "$OUTPUT_DIR"

# Test configurations to compare
configs=(
    "floyd-steinberg:spectra6"
    "jarvis:spectra6"
    "stucki:acep"
    "ordered:default"
)

echo "Comparing Rust vs JavaScript outputs..."

for config in "${configs[@]}"; do
    IFS=':' read -r algo palette <<< "$config"
    echo "Testing: algorithm=$algo, palette=$palette"

    # Run Rust version
    echo "  Running Rust..."
    time $RUST_BIN -i "$TEST_IMAGE" \
        -o "$OUTPUT_DIR/rust_${algo}_${palette}.png" \
        -a "$algo" -p "$palette" -v

    # Run JavaScript version
    echo "  Running JavaScript..."
    time node compare_with_js.js "$TEST_IMAGE" \
        "$OUTPUT_DIR/js_${algo}_${palette}.png" \
        "$algo" "$palette"

    # Compare file sizes
    rust_size=$(stat -c%s "$OUTPUT_DIR/rust_${algo}_${palette}.png" 2>/dev/null || stat -f%z "$OUTPUT_DIR/rust_${algo}_${palette}.png")
    js_size=$(stat -c%s "$OUTPUT_DIR/js_${algo}_${palette}.png" 2>/dev/null || stat -f%z "$OUTPUT_DIR/js_${algo}_${palette}.png")

    echo "  Rust output: ${rust_size} bytes"
    echo "  JS output: ${js_size} bytes"

    # Visual comparison (requires ImageMagick)
    if command -v compare &> /dev/null; then
        echo "  Running pixel comparison..."
        compare -metric RMSE \
            "$OUTPUT_DIR/rust_${algo}_${palette}.png" \
            "$OUTPUT_DIR/js_${algo}_${palette}.png" \
            "$OUTPUT_DIR/diff_${algo}_${palette}.png" 2>&1 | \
            tee "$OUTPUT_DIR/diff_${algo}_${palette}.txt"
    fi

    echo "✓ $config compared"
done

echo "Comparison complete! Check $OUTPUT_DIR for results"
```

### 4. Performance Benchmarks

Measure and compare performance.

#### Test Script: `benchmark.sh`

```bash
#!/bin/bash

set -e

RUST_BIN="./epd-dither/target/release/epd-dither"
TEST_IMAGE="test_inputs/example-dither.jpg"
ITERATIONS=10

echo "Performance Benchmark"
echo "====================="
echo "Test image: $TEST_IMAGE"
echo "Iterations: $ITERATIONS"
echo ""

# Benchmark Rust
echo "Benchmarking Rust (Floyd-Steinberg, Spectra 6)..."
rust_times=()
for i in $(seq 1 $ITERATIONS); do
    start=$(date +%s%N)
    $RUST_BIN -i "$TEST_IMAGE" -o "/tmp/rust_bench.png" -a floyd-steinberg -p spectra6 2>/dev/null
    end=$(date +%s%N)
    duration=$((($end - $start) / 1000000)) # Convert to milliseconds
    rust_times+=($duration)
    echo "  Run $i: ${duration}ms"
done

# Calculate average
rust_avg=0
for time in "${rust_times[@]}"; do
    rust_avg=$((rust_avg + time))
done
rust_avg=$((rust_avg / ITERATIONS))

echo ""
echo "Results:"
echo "  Rust average: ${rust_avg}ms"
echo ""

# Benchmark JavaScript (if node is available)
if command -v node &> /dev/null; then
    echo "Benchmarking JavaScript..."
    node benchmark_js.js "$TEST_IMAGE" $ITERATIONS
fi
```

### 5. Edge Case Tests

Test boundary conditions and error handling.

#### Test Cases:

1. **Very small image (1x1 pixel)**
   ```bash
   convert -size 1x1 xc:gray test_inputs/tiny.png
   $RUST_BIN -i test_inputs/tiny.png -o test_outputs/tiny_out.png
   ```

2. **Very large image (4K)**
   ```bash
   convert -size 3840x2160 gradient: test_inputs/large_gradient.png
   $RUST_BIN -i test_inputs/large_gradient.png -o test_outputs/large_out.png -v
   ```

3. **Invalid input file**
   ```bash
   $RUST_BIN -i nonexistent.jpg -o out.png 2>&1 | grep "does not exist"
   ```

4. **Invalid palette**
   ```bash
   $RUST_BIN -i test.jpg -o out.png -p invalid_palette 2>&1 | grep "not found"
   ```

5. **Invalid custom colors**
   ```bash
   $RUST_BIN -i test.jpg -o out.png -c "invalid,colors" 2>&1 | grep "Invalid"
   ```

### 6. Color Accuracy Tests

Verify color quantization is accurate.

#### Test Script: `test_color_accuracy.py`

```python
#!/usr/bin/env python3

from PIL import Image
import sys

def check_palette_colors(image_path, expected_colors):
    """Verify image only contains expected palette colors"""
    img = Image.open(image_path)
    img_rgb = img.convert('RGB')

    pixels = set(img_rgb.getdata())
    expected_set = set(expected_colors)

    unexpected = pixels - expected_set

    if unexpected:
        print(f"ERROR: Found unexpected colors: {unexpected}")
        return False

    print(f"✓ All pixels match palette ({len(pixels)} unique colors found)")
    return True

# Test cases
tests = [
    ("test_outputs/rust_floyd-steinberg.png", [
        (0, 0, 0), (255, 255, 255), (0, 0, 255),
        (0, 255, 0), (255, 0, 0), (255, 255, 0)
    ]),
]

for img_path, palette in tests:
    print(f"Checking {img_path}...")
    if not check_palette_colors(img_path, palette):
        sys.exit(1)

print("All color accuracy tests passed!")
```

### 7. Device Color Replacement Tests

Test color mapping functionality.

```bash
# Test with explicit device colors
$RUST_BIN -i test.jpg -o out_device.png -p spectra6 -d spectra6 -v

# Test auto-detection
$RUST_BIN -i test.jpg -o out_auto.png -p spectra6 -v

# Test skip color replacement
$RUST_BIN -i test.jpg -o out_no_replace.png -p spectra6 --no-color-replace -v

# Compare outputs
compare out_device.png out_auto.png diff_device_vs_auto.png
```

## Test Execution Order

1. **Unit Tests** (Rust): `cargo test`
2. **Algorithm Tests**: `./test_algorithms.sh`
3. **Palette Tests**: `./test_palettes.sh`
4. **Comparison Tests**: `./compare_js_rust.sh`
5. **Performance Tests**: `./benchmark.sh`
6. **Edge Case Tests**: Manual execution
7. **Color Accuracy**: `./test_color_accuracy.py`

## Success Criteria

### Must Pass:
- ✅ All `cargo test` unit tests pass
- ✅ All algorithms produce valid output images
- ✅ All palettes work correctly
- ✅ No crashes or panics on valid input
- ✅ Proper error messages for invalid input
- ✅ Output contains only palette colors

### Should Match:
- ✅ Rust output should be visually similar to JS output
- ✅ RMSE between Rust and JS should be < 5% (allowing for floating-point differences)
- ✅ Rust should be 10-50x faster than JS

### Quality Checks:
- ✅ Binary size < 10MB
- ✅ Memory usage reasonable
- ✅ No memory leaks (valgrind on Linux)

## Troubleshooting

### If comparison fails:
1. Check algorithm implementation matches JS exactly
2. Verify error diffusion kernel coefficients
3. Check color distance calculation
4. Verify serpentine scanning direction

### If performance is slow:
1. Ensure release build: `cargo build --release`
2. Check for debug assertions
3. Profile with `cargo flamegraph`

## Reporting Results

Create a test report with:
- Test execution date/time
- Rust version and commit hash
- Pass/fail status for each test
- Performance comparison table
- Visual diff images for comparison tests
- Any issues or discrepancies found

## Automated Testing

To run all tests automatically:

```bash
#!/bin/bash
# run_all_tests.sh

set -e

echo "Starting comprehensive test suite..."
echo "===================================="

echo "1. Building Rust project..."
cd epd-dither && cargo build --release && cargo test && cd ..

echo "2. Running algorithm tests..."
./test_algorithms.sh

echo "3. Running palette tests..."
./test_palettes.sh

echo "4. Running comparison tests..."
./compare_js_rust.sh

echo "5. Running benchmarks..."
./benchmark.sh

echo "6. Running color accuracy tests..."
python3 test_color_accuracy.py

echo ""
echo "===================================="
echo "All tests completed successfully!"
echo "Check test_outputs/ for results"
```

## Next Steps After Testing

1. Document any discrepancies
2. Fix bugs found during testing
3. Optimize performance if needed
4. Update documentation with test results
5. Create release binaries
