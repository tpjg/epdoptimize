#!/bin/bash

set -e

RUST_BIN="./epd-dither/target/release/epd-dither"
TEST_IMAGE="examples/example-dither.jpg"
OUTPUT_DIR="test_outputs"

mkdir -p "$OUTPUT_DIR"

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

echo "================================"
echo "Testing All Dithering Algorithms"
echo "================================"
echo "Rust binary: $RUST_BIN"
echo "Test image: $TEST_IMAGE"
echo "Output directory: $OUTPUT_DIR"
echo ""

if [ ! -f "$RUST_BIN" ]; then
    echo "ERROR: Rust binary not found at $RUST_BIN"
    echo "Please run: cd epd-dither && cargo build --release && cd .."
    exit 1
fi

if [ ! -f "$TEST_IMAGE" ]; then
    echo "ERROR: Test image not found at $TEST_IMAGE"
    exit 1
fi

failed_tests=()

for algo in "${algorithms[@]}"; do
    echo "Testing algorithm: $algo"
    output_file="$OUTPUT_DIR/rust_${algo}.png"

    # Run dithering
    if $RUST_BIN -i "$TEST_IMAGE" -o "$output_file" -a "$algo" -v; then
        # Verify output exists
        if [ ! -f "$output_file" ]; then
            echo "  ✗ ERROR: Output file not created"
            failed_tests+=("$algo: output not created")
            continue
        fi

        # Check file size is reasonable
        if command -v stat &> /dev/null; then
            size=$(stat -c%s "$output_file" 2>/dev/null || stat -f%z "$output_file")
            if [ "$size" -lt 1000 ]; then
                echo "  ✗ ERROR: Output file too small (${size} bytes)"
                failed_tests+=("$algo: output too small")
                continue
            fi
            echo "  Output size: ${size} bytes"
        fi

        # Try to verify it's a valid PNG (if file command exists)
        if command -v file &> /dev/null; then
            filetype=$(file -b "$output_file")
            if [[ ! "$filetype" =~ PNG ]]; then
                echo "  ✗ ERROR: Output is not a valid PNG: $filetype"
                failed_tests+=("$algo: invalid PNG")
                continue
            fi
        fi

        echo "  ✓ $algo passed"
    else
        echo "  ✗ ERROR: Command failed"
        failed_tests+=("$algo: command failed")
    fi
    echo ""
done

echo "================================"
echo "Test Summary"
echo "================================"
echo "Total algorithms tested: ${#algorithms[@]}"
echo "Failed tests: ${#failed_tests[@]}"

if [ ${#failed_tests[@]} -eq 0 ]; then
    echo ""
    echo "✓ All algorithm tests passed!"
    exit 0
else
    echo ""
    echo "✗ Some tests failed:"
    for failure in "${failed_tests[@]}"; do
        echo "  - $failure"
    done
    exit 1
fi
