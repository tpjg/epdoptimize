//! EPD Dither - CLI tool for dithering images for e-ink displays

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use epd_dither::{
    color::{convert, palette::PaletteManager, Rgb},
    device::DeviceManager,
    dither::{engine, DitheringAlgorithm, DitherOptions, ErrorDiffusionKernel, RandomMode},
    scaling::{self, FitMode, ScalingFilter},
};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "epd-dither")]
#[command(author = "EPD Optimize Team")]
#[command(version = "0.1.0")]
#[command(about = "Dither images for e-ink/e-paper displays", long_about = None)]
struct Cli {
    /// Input image file
    #[arg(short, long, value_name = "FILE", required_unless_present_any = ["list_devices", "list_palettes"])]
    input: Option<PathBuf>,

    /// Output image file
    #[arg(short, long, value_name = "FILE", required_unless_present_any = ["list_devices", "list_palettes"])]
    output: Option<PathBuf>,

    /// Dithering algorithm
    #[arg(short, long, value_enum, default_value = "floyd-steinberg")]
    algorithm: Algorithm,

    /// Color palette name
    #[arg(short, long, default_value = "spectra6")]
    palette: String,

    /// Custom palette (comma-separated hex colors, e.g., "#000,#fff,#f00")
    #[arg(short, long, value_name = "COLORS")]
    custom_palette: Option<String>,

    /// Device color set name for final color replacement
    #[arg(short, long)]
    device_colors: Option<String>,

    /// Use serpentine scanning for error diffusion
    #[arg(short, long)]
    serpentine: bool,

    /// Bayer matrix size for ordered dithering (format: WxH)
    #[arg(long, default_value = "4x4")]
    bayer_size: String,

    /// Skip device color replacement
    #[arg(long)]
    no_color_replace: bool,

    /// Use preset device configuration (overrides palette, resolution, and other settings)
    #[arg(short = 'd', long)]
    device: Option<String>,

    /// Target width for output image (auto-scales before dithering)
    #[arg(long)]
    target_width: Option<u32>,

    /// Target height for output image (auto-scales before dithering)
    #[arg(long)]
    target_height: Option<u32>,

    /// Fit mode when resizing (letterbox, crop, fill, contain)
    #[arg(long, default_value = "letterbox")]
    fit_mode: String,

    /// Scaling algorithm (nearest, triangle, catmull-rom, gaussian, lanczos3)
    #[arg(long, default_value = "lanczos3")]
    scaling_algorithm: String,

    /// Background color for letterbox mode (hex color, e.g., #ffffff)
    #[arg(long, default_value = "#ffffff")]
    letterbox_color: String,

    /// List available palettes and exit
    #[arg(long)]
    list_palettes: bool,

    /// List available devices and exit
    #[arg(long)]
    list_devices: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Algorithm {
    /// Floyd-Steinberg error diffusion (default)
    FloydSteinberg,
    /// False Floyd-Steinberg (simplified)
    FalseFloydSteinberg,
    /// Jarvis, Judice, and Ninke
    Jarvis,
    /// Stucki error diffusion
    Stucki,
    /// Burkes error diffusion
    Burkes,
    /// Sierra-3 error diffusion
    Sierra3,
    /// Sierra-2 error diffusion
    Sierra2,
    /// Sierra-2-4A (lightweight)
    Sierra24a,
    /// Ordered dithering (Bayer matrix)
    Ordered,
    /// Random RGB dithering
    RandomRgb,
    /// Random black and white dithering
    RandomBw,
    /// Quantization only (no dithering)
    None,
}

impl Algorithm {
    fn to_dithering_algorithm(&self, bayer_size: (u8, u8)) -> DitheringAlgorithm {
        match self {
            Algorithm::FloydSteinberg => {
                DitheringAlgorithm::ErrorDiffusion(ErrorDiffusionKernel::FloydSteinberg)
            }
            Algorithm::FalseFloydSteinberg => {
                DitheringAlgorithm::ErrorDiffusion(ErrorDiffusionKernel::FalseFloydSteinberg)
            }
            Algorithm::Jarvis => {
                DitheringAlgorithm::ErrorDiffusion(ErrorDiffusionKernel::Jarvis)
            }
            Algorithm::Stucki => {
                DitheringAlgorithm::ErrorDiffusion(ErrorDiffusionKernel::Stucki)
            }
            Algorithm::Burkes => {
                DitheringAlgorithm::ErrorDiffusion(ErrorDiffusionKernel::Burkes)
            }
            Algorithm::Sierra3 => {
                DitheringAlgorithm::ErrorDiffusion(ErrorDiffusionKernel::Sierra3)
            }
            Algorithm::Sierra2 => {
                DitheringAlgorithm::ErrorDiffusion(ErrorDiffusionKernel::Sierra2)
            }
            Algorithm::Sierra24a => {
                DitheringAlgorithm::ErrorDiffusion(ErrorDiffusionKernel::Sierra2_4A)
            }
            Algorithm::Ordered => DitheringAlgorithm::Ordered {
                width: bayer_size.0,
                height: bayer_size.1,
            },
            Algorithm::RandomRgb => DitheringAlgorithm::Random(RandomMode::Rgb),
            Algorithm::RandomBw => DitheringAlgorithm::Random(RandomMode::BlackAndWhite),
            Algorithm::None => DitheringAlgorithm::QuantizationOnly,
        }
    }
}

fn parse_bayer_size(size_str: &str) -> Result<(u8, u8)> {
    let parts: Vec<&str> = size_str.split('x').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid bayer size format. Expected WxH (e.g., 4x4)");
    }

    let width = parts[0]
        .parse::<u8>()
        .context("Invalid width in bayer size")?;
    let height = parts[1]
        .parse::<u8>()
        .context("Invalid height in bayer size")?;

    if width == 0 || height == 0 || width > 8 || height > 8 {
        anyhow::bail!("Bayer matrix size must be between 1x1 and 8x8");
    }

    Ok((width, height))
}

fn parse_custom_palette(palette_str: &str) -> Result<Vec<Rgb>> {
    palette_str
        .split(',')
        .map(|hex| {
            let hex = hex.trim();
            convert::hex_to_rgb(hex)
                .map(Rgb)
                .with_context(|| format!("Invalid hex color: {}", hex))
        })
        .collect()
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let palette_manager = PaletteManager::new()?;
    let device_manager = DeviceManager::new()?;

    // Handle --list-devices
    if cli.list_devices {
        println!("Available E-Ink Devices:\n");

        let devices_by_tech = device_manager.devices_by_technology();
        let mut techs: Vec<_> = devices_by_tech.keys().collect();
        techs.sort();

        for tech in techs {
            println!("{}:", tech);
            for (id, spec) in &devices_by_tech[tech] {
                println!(
                    "  {:20} - {} ({}×{}, {} PPI, palette: {})",
                    id,
                    spec.name,
                    spec.resolution.width,
                    spec.resolution.height,
                    spec.ppi,
                    spec.palette
                );
            }
            println!();
        }

        println!("Usage: epd-dither -i input.jpg -o output.png --device <device-id>");
        println!("Example: epd-dither -i photo.jpg -o photo.png --device spectra6-7.3");
        return Ok(());
    }

    // Handle --list-palettes
    if cli.list_palettes {
        println!("Available palettes:");
        for name in palette_manager.list_palettes() {
            let palette = palette_manager.get_palette(&name)?;
            println!("  {} ({} colors)", name, palette.len());
        }
        println!("\nAvailable device color sets:");
        for name in palette_manager.list_device_colors() {
            println!("  {}", name);
        }
        return Ok(());
    }

    // Unwrap input/output (guaranteed to exist after list commands)
    let input = cli.input.as_ref().expect("Input file required");
    let output = cli.output.as_ref().expect("Output file required");

    // Validate input file exists
    if !input.exists() {
        anyhow::bail!("Input file does not exist: {}", input.display());
    }

    if cli.verbose {
        println!("Loading image: {}", input.display());
    }

    // Load device settings if specified
    let device_spec = if let Some(device_id) = &cli.device {
        if cli.verbose {
            println!("Loading device configuration: {}", device_id);
        }
        Some(device_manager.get_device(device_id)?)
    } else {
        None
    };

    // Load image
    let img = image::open(input)
        .with_context(|| format!("Failed to open image: {}", input.display()))?;
    let mut rgb_img = img.to_rgb8();

    if cli.verbose {
        println!(
            "Input image dimensions: {}x{}",
            rgb_img.width(),
            rgb_img.height()
        );
    }

    // Determine target resolution (device > CLI args > original)
    let (target_width, target_height) = if let Some(ref device) = device_spec {
        if cli.verbose {
            println!(
                "Using device resolution: {}x{}",
                device.resolution.width, device.resolution.height
            );
        }
        (device.resolution.width, device.resolution.height)
    } else if let (Some(w), Some(h)) = (cli.target_width, cli.target_height) {
        if cli.verbose {
            println!("Using custom target resolution: {}x{}", w, h);
        }
        (w, h)
    } else {
        // No resizing
        (rgb_img.width(), rgb_img.height())
    };

    // Resize image if needed
    if target_width != rgb_img.width() || target_height != rgb_img.height() {
        if cli.verbose {
            println!(
                "Resizing image from {}x{} to {}x{} (fit mode: {}, filter: {})",
                rgb_img.width(),
                rgb_img.height(),
                target_width,
                target_height,
                cli.fit_mode,
                cli.scaling_algorithm
            );
        }

        let fit_mode = FitMode::from_str(&cli.fit_mode)?;
        let scaling_filter = ScalingFilter::from_str(&cli.scaling_algorithm)?;
        let letterbox_color = convert::hex_to_rgb(&cli.letterbox_color)
            .with_context(|| format!("Invalid letterbox color: {}", cli.letterbox_color))?;

        rgb_img = scaling::resize_image(
            &rgb_img,
            target_width,
            target_height,
            fit_mode,
            scaling_filter,
            letterbox_color,
        )?;

        if cli.verbose {
            println!("Resized to: {}x{}", rgb_img.width(), rgb_img.height());
        }
    }

    // Determine palette (device > CLI arg > custom)
    let palette_name = if let Some(ref device) = device_spec {
        &device.palette
    } else {
        &cli.palette
    };

    // Get or create palette
    let palette = if let Some(custom) = &cli.custom_palette {
        let colors = parse_custom_palette(custom)?;
        epd_dither::Palette::new("custom", colors)
    } else {
        palette_manager.get_palette(palette_name)?
    };

    if cli.verbose {
        println!("Using palette: {} ({} colors)", palette.name, palette.len());
    }

    // Parse bayer size
    let bayer_size = parse_bayer_size(&cli.bayer_size)?;

    // Create dither options
    let options = DitherOptions {
        algorithm: cli.algorithm.to_dithering_algorithm(bayer_size),
        palette: palette.clone(),
        serpentine: cli.serpentine,
    };

    if cli.verbose {
        println!("Algorithm: {:?}", cli.algorithm);
        println!("Dithering image...");
    }

    // Apply dithering
    epd_dither::process_image(&mut rgb_img, &options)?;

    // Optionally replace colors with device colors
    if !cli.no_color_replace {
        if let Some(device_colors_name) = &cli.device_colors {
            if cli.verbose {
                println!("Replacing colors with device colors: {}", device_colors_name);
            }

            let device_colors = palette_manager.get_device_colors(device_colors_name)?;

            if palette.colors.len() != device_colors.len() {
                eprintln!(
                    "Warning: Palette has {} colors but device color set has {} colors",
                    palette.colors.len(),
                    device_colors.len()
                );
            }

            engine::replace_colors(&mut rgb_img, &palette.colors, &device_colors)?;
        } else if cli.palette != "custom" {
            // Auto-detect matching device colors
            if cli.verbose {
                println!("Auto-detecting device colors for palette: {}", cli.palette);
            }

            if let Ok(device_colors) = palette_manager.get_device_colors(&cli.palette) {
                engine::replace_colors(&mut rgb_img, &palette.colors, &device_colors)?;
            }
        }
    }

    if cli.verbose {
        println!("Saving output: {}", output.display());
    }

    // Save output
    rgb_img
        .save(output)
        .with_context(|| format!("Failed to save image: {}", output.display()))?;

    if cli.verbose {
        println!("Done!");
    }

    Ok(())
}
