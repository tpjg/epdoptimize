#!/usr/bin/env bash

set -e

# Ensure we use bash 4+ features or fallback to simpler approach
BASH_VERSION_MAJOR="${BASH_VERSION%%.*}"

RUST_BIN="./epd-dither/target/release/epd-dither"
JS_SCRIPT="./compare_with_js.js"
TEST_IMAGE="examples/example-dither.jpg"
OUTPUT_DIR="test_outputs"
PALETTE="spectra6"

# Create organized output directories
mkdir -p "$OUTPUT_DIR/rust"
mkdir -p "$OUTPUT_DIR/js"
mkdir -p "$OUTPUT_DIR/diff"

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

echo "=========================================="
echo "Rust vs JavaScript Dithering Comparison"
echo "=========================================="
echo "Rust binary: $RUST_BIN"
echo "JS script: $JS_SCRIPT (using bun)"
echo "Test image: $TEST_IMAGE"
echo "Palette: $PALETTE"
echo "Output directory: $OUTPUT_DIR"
echo ""

# Verify prerequisites
if [ ! -f "$RUST_BIN" ]; then
    echo "ERROR: Rust binary not found at $RUST_BIN"
    echo "Please run: cd epd-dither && cargo build --release && cd .."
    exit 1
fi

if [ ! -f "$TEST_IMAGE" ]; then
    echo "ERROR: Test image not found at $TEST_IMAGE"
    exit 1
fi

# Detect JS runtime (prefer bun, fallback to node)
JS_RUNTIME=""
if command -v bun &> /dev/null; then
    JS_RUNTIME="bun run"
    echo "Using bun for JavaScript execution"
elif command -v node &> /dev/null; then
    JS_RUNTIME="node"
    echo "Using node for JavaScript execution"
else
    echo "ERROR: Neither bun nor node found. Please install one of them."
    exit 1
fi

if [ ! -f "$JS_SCRIPT" ]; then
    echo "ERROR: JS comparison script not found at $JS_SCRIPT"
    exit 1
fi

# Check if canvas is installed
if ! $JS_RUNTIME -e "require('canvas')" 2>/dev/null; then
    echo ""
    echo "WARNING: canvas package not found!"
    echo "The JS comparison requires the canvas package."
    echo ""
    echo "To install it:"
    echo "  npm install canvas"
    echo "or:"
    echo "  bun install canvas"
    echo ""
    echo "For more details, see: INSTALL_CANVAS.md"
    echo ""
    read -p "Continue with Rust-only testing? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
    SKIP_JS=true
fi

# Check if ImageMagick is available for comparison
HAS_IMAGEMAGICK=false
if command -v compare &> /dev/null; then
    HAS_IMAGEMAGICK=true
    echo "ImageMagick detected - will compute visual differences"
    echo ""
fi

failed_tests=()

# Use simple arrays instead of associative arrays for compatibility
rust_times=()
js_times=()
differences=()
algo_names=()

idx=0
for algo in "${algorithms[@]}"; do
    echo "========================================"
    echo "Testing: $algo"
    echo "========================================"

    rust_output="$OUTPUT_DIR/rust/${algo}.png"
    js_output="$OUTPUT_DIR/js/${algo}.png"
    diff_output="$OUTPUT_DIR/diff/${algo}.png"

    # Run Rust version
    echo "→ Running Rust version..."
    rust_start=$(date +%s%N 2>/dev/null || python3 -c "import time; print(int(time.time() * 1000000000))")
    if $RUST_BIN -i "$TEST_IMAGE" -o "$rust_output" -a "$algo" -p "$PALETTE" 2>/dev/null; then
        rust_end=$(date +%s%N 2>/dev/null || python3 -c "import time; print(int(time.time() * 1000000000))")
        rust_time=$((($rust_end - $rust_start) / 1000000)) # Convert to milliseconds

        # Verify Rust output
        if [ ! -f "$rust_output" ]; then
            echo "  ✗ ERROR: Rust output not created"
            failed_tests+=("$algo: Rust output not created")
            continue
        fi

        rust_size=$(stat -c%s "$rust_output" 2>/dev/null || stat -f%z "$rust_output")
        echo "  ✓ Rust completed in ${rust_time}ms (${rust_size} bytes)"
    else
        echo "  ✗ ERROR: Rust command failed"
        failed_tests+=("$algo: Rust failed")
        continue
    fi

    # Run JavaScript version (if not skipped)
    if [ "$SKIP_JS" = true ]; then
        echo "→ Skipping JavaScript version (canvas not installed)"
        idx=$((idx + 1))
        echo ""
        continue
    fi

    echo "→ Running JavaScript version..."
    js_start=$(date +%s%N 2>/dev/null || python3 -c "import time; print(int(time.time() * 1000000000))")
    if $JS_RUNTIME "$JS_SCRIPT" "$TEST_IMAGE" "$js_output" "$algo" "$PALETTE" 2>&1 | grep -v "Debugger" | tail -5; then
        js_end=$(date +%s%N 2>/dev/null || python3 -c "import time; print(int(time.time() * 1000000000))")
        js_time=$((($js_end - $js_start) / 1000000)) # Convert to milliseconds

        # Verify JS output
        if [ ! -f "$js_output" ]; then
            echo "  ✗ ERROR: JS output not created"
            failed_tests+=("$algo: JS output not created")
            continue
        fi

        js_size=$(stat -c%s "$js_output" 2>/dev/null || stat -f%z "$js_output")
        echo "  ✓ JavaScript completed in ${js_time}ms (${js_size} bytes)"
    else
        echo "  ✗ ERROR: JavaScript command failed"
        failed_tests+=("$algo: JS failed")
        continue
    fi

    # Store results using indexed arrays
    algo_names[$idx]="$algo"
    rust_times[$idx]=$rust_time
    js_times[$idx]=$js_time

    # Compare outputs with ImageMagick if available
    if [ "$HAS_IMAGEMAGICK" = true ] && [ "$algo" != "random-rgb" ] && [ "$algo" != "random-bw" ]; then
        echo "→ Comparing outputs..."
        if rmse_output=$(compare -metric RMSE "$rust_output" "$js_output" "$diff_output" 2>&1 | head -1); then
            # Extract RMSE value
            rmse=$(echo "$rmse_output" | awk '{print $1}' | cut -d'(' -f1)
            differences[$idx]=$rmse
            echo "  RMSE difference: $rmse"
        fi
    else
        if [ "$algo" = "random-rgb" ] || [ "$algo" = "random-bw" ]; then
            echo "  ⊘ Skipping comparison (random algorithm)"
            differences[$idx]="N/A"
        fi
    fi

    # Calculate speedup
    if [ $rust_time -gt 0 ]; then
        speedup=$(echo "scale=1; $js_time / $rust_time" | bc)
        echo "  ⚡ Speedup: ${speedup}x faster"
    fi

    idx=$((idx + 1))
    echo ""
done

echo "=========================================="
echo "Test Summary"
echo "=========================================="
echo "Total algorithms tested: ${#algorithms[@]}"
echo "Failed tests: ${#failed_tests[@]}"
echo ""

# Performance comparison table
echo "Performance Comparison:"
echo "----------------------------------------"
printf "%-20s %10s %10s %8s\n" "Algorithm" "Rust (ms)" "JS (ms)" "Speedup"
echo "----------------------------------------"
for i in "${!algo_names[@]}"; do
    if [ -n "${rust_times[$i]}" ] && [ -n "${js_times[$i]}" ]; then
        speedup=$(echo "scale=1; ${js_times[$i]} / ${rust_times[$i]}" | bc)
        printf "%-20s %10s %10s %7sx\n" "${algo_names[$i]}" "${rust_times[$i]}" "${js_times[$i]}" "$speedup"
    fi
done
echo "----------------------------------------"

# Calculate average speedup
total_speedup=0
count=0
for i in "${!rust_times[@]}"; do
    if [ -n "${rust_times[$i]}" ] && [ -n "${js_times[$i]}" ]; then
        speedup=$(echo "scale=2; ${js_times[$i]} / ${rust_times[$i]}" | bc)
        total_speedup=$(echo "$total_speedup + $speedup" | bc)
        count=$((count + 1))
    fi
done
if [ $count -gt 0 ]; then
    avg_speedup=$(echo "scale=1; $total_speedup / $count" | bc)
    echo "Average speedup: ${avg_speedup}x faster"
fi
echo ""

# Visual difference comparison (if available)
if [ "$HAS_IMAGEMAGICK" = true ]; then
    echo "Visual Differences (RMSE):"
    echo "----------------------------------------"
    printf "%-20s %15s\n" "Algorithm" "RMSE"
    echo "----------------------------------------"
    for i in "${!algo_names[@]}"; do
        if [ -n "${differences[$i]}" ]; then
            printf "%-20s %15s\n" "${algo_names[$i]}" "${differences[$i]}"
        fi
    done
    echo "----------------------------------------"
    echo "Note: Lower RMSE = more similar to JS version"
    echo "Diff images saved to: $OUTPUT_DIR/diff/"
    echo ""
fi

# Show where outputs are
echo "Outputs saved to:"
echo "  Rust:   $OUTPUT_DIR/rust/"
echo "  JS:     $OUTPUT_DIR/js/"
if [ "$HAS_IMAGEMAGICK" = true ]; then
    echo "  Diffs:  $OUTPUT_DIR/diff/"
fi
echo ""

# Final result
if [ ${#failed_tests[@]} -eq 0 ]; then
    echo "✓ All tests passed!"
    echo ""
    echo "You can now visually compare the outputs:"
    echo "  open $OUTPUT_DIR/rust/${algorithms[0]}.png"
    echo "  open $OUTPUT_DIR/js/${algorithms[0]}.png"
    exit 0
else
    echo "✗ Some tests failed:"
    for failure in "${failed_tests[@]}"; do
        echo "  - $failure"
    done
    exit 1
fi
