# Build and Test Instructions

## Current Status

The Rust implementation is **code-complete** but has **not been compiled yet** due to network restrictions preventing dependency downloads from crates.io.

### What's Done ✅
- All source code written (~1,500 lines)
- All algorithms implemented (8 error diffusion + ordered + random)
- Full CLI interface with clap
- Comprehensive test infrastructure created
- Code issues found and fixed:
  - ✅ Fixed import path in `matrices.rs`
  - ✅ Added Serialize/Deserialize derives for Rgb/Rgba types

### What's Not Done ❌
- ❌ Cargo dependencies not downloaded (network restriction)
- ❌ Project not compiled
- ❌ Tests not executed
- ❌ No comparison with JS implementation yet

## Build Instructions

### Step 1: Build the Rust Project

```bash
cd epd-dither

# This should download dependencies and build
cargo build --release

# If successful, you'll see:
# Finished release [optimized] target(s) in X.XXs

# Binary will be at:
# ./target/release/epd-dither
```

### Step 2: Run Unit Tests

```bash
# Inside epd-dither directory
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_basic_dithering
```

### Step 3: Test the CLI

```bash
# Go back to project root
cd ..

# List available palettes
./epd-dither/target/release/epd-dither --list-palettes

# Try basic dithering
./epd-dither/target/release/epd-dither \
    -i examples/example-dither.jpg \
    -o test_output.png \
    -v
```

## Testing Instructions

### Quick Test

```bash
# Run algorithm test suite
./test_algorithms.sh

# This tests all 12 algorithms
```

### Full Test Suite

```bash
# 1. Build JS version first
npm install
npm run build

# 2. Run comprehensive tests
./test_algorithms.sh          # Test all algorithms
node compare_with_js.js \     # Compare specific case
    examples/example-dither.jpg \
    test_js_output.png \
    floyd-steinberg \
    spectra6
```

### Compare JS vs Rust

```bash
# Test same configuration on both
INPUT="examples/example-dither.jpg"

# Rust version
time ./epd-dither/target/release/epd-dither \
    -i "$INPUT" -o rust_out.png \
    -a floyd-steinberg -p spectra6 -v

# JS version
time node compare_with_js.js \
    "$INPUT" js_out.png \
    floyd-steinberg spectra6

# Visual comparison (requires ImageMagick)
compare -metric RMSE rust_out.png js_out.png diff.png
```

## Troubleshooting

### Build Errors

#### "failed to get dependency from crates.io"

**Cause**: Network restrictions or firewall blocking crates.io

**Solution**:
- Check internet connection
- Check firewall settings
- Try: `cargo clean && cargo build --release`
- If behind proxy, configure cargo:
  ```bash
  # In ~/.cargo/config.toml
  [http]
  proxy = "http://your-proxy:port"
  ```

#### "cannot find crate"

**Cause**: Dependencies not downloaded

**Solution**: Run `cargo fetch` first, then `cargo build`

#### Compilation warnings

```bash
# Check for warnings
cargo clippy

# Auto-fix some issues
cargo fix --allow-dirty
```

### Runtime Errors

#### "Image file not found"

**Solution**: Use absolute paths or check current directory

#### "Palette not found"

**Solution**: Use `--list-palettes` to see available options

#### Segmentation fault / panic

**Possible causes**:
- Very large image (check memory usage)
- Corrupted input file
- Bug in algorithm implementation

**Debug**:
```bash
# Run with backtrace
RUST_BACKTRACE=1 ./epd-dither/target/release/epd-dither -i test.jpg -o out.png
```

## Expected Test Results

### Performance Expectations

| Image Size | Rust (expected) | JS (baseline) | Speedup |
|-----------|----------------|---------------|---------|
| 800x600   | 100-200ms      | 2-5s          | 10-50x  |
| 1920x1080 | 400-800ms      | 8-20s         | 10-50x  |
| 4K        | 1.5-3s         | 30-60s        | 10-50x  |

### Quality Expectations

- **Visual similarity**: Rust output should look nearly identical to JS
- **Pixel differences**: RMSE < 5% (some floating-point variance expected)
- **Color accuracy**: All pixels should be exactly from the palette
- **File size**: Similar to JS output (±10%)

### Known Differences

1. **Random dithering**: Will produce different output each run (expected)
2. **Floating-point precision**: Minor differences in error diffusion (< 1 pixel difference)
3. **Rounding**: May differ slightly at color boundaries

## Validation Checklist

Before declaring success, verify:

- [ ] `cargo build --release` completes without errors
- [ ] `cargo test` all tests pass
- [ ] `cargo clippy` no warnings
- [ ] `./test_algorithms.sh` all algorithms work
- [ ] Comparison with JS shows < 5% RMSE
- [ ] Performance is 10x+ faster than JS
- [ ] Binary size < 10MB
- [ ] `--help` shows correct usage
- [ ] `--list-palettes` works
- [ ] Custom palettes work
- [ ] Error messages are helpful
- [ ] No panics on valid input
- [ ] Proper error handling for invalid input

## Files to Check After Build

```bash
# Check binary size
ls -lh epd-dither/target/release/epd-dither

# Should be around 3-8 MB

# Check it runs
./epd-dither/target/release/epd-dither --version
./epd-dither/target/release/epd-dither --help

# Test basic functionality
./epd-dither/target/release/epd-dither \
    -i examples/example-dither.jpg \
    -o /tmp/test.png \
    && echo "Success!" \
    || echo "Failed!"
```

## Next Steps After Successful Build

1. **Run all tests**: `./test_algorithms.sh`
2. **Compare with JS**: Visually inspect outputs
3. **Benchmark**: Compare performance
4. **Fix any bugs**: Debug issues found during testing
5. **Document results**: Update completion report
6. **Create release**: Strip binary, create distribution package

## If Build Fails

Please report:
1. Exact error message
2. Rust version (`rustc --version`)
3. Cargo version (`cargo --version`)
4. Operating system
5. Steps to reproduce

Common fixes:
```bash
# Update Rust
rustup update

# Clean and rebuild
cd epd-dither
cargo clean
cargo build --release

# Check for missing system dependencies (Linux)
sudo apt-get install build-essential pkg-config libssl-dev
```

## Network Issue Resolution

If you continue to see crates.io access errors:

```bash
# Check connectivity
curl -I https://crates.io

# Try sparse index (Rust 1.70+)
export CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
cargo build --release

# Or use a mirror
# Add to ~/.cargo/config.toml:
[source.crates-io]
replace-with = "mirror"

[source.mirror]
registry = "https://github.com/rust-lang/crates.io-index"
```

## Contact

If you encounter issues not covered here, please check:
- TEST_PLAN.md - Comprehensive testing documentation
- RUST_IMPLEMENTATION_COMPLETE.md - Implementation details
- epd-dither/README.md - User documentation
