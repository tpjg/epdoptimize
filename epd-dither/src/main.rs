//! EPD Dither - CLI tool for dithering images for e-ink displays

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use epd_dither::{
    color::{convert, palette::PaletteManager, Rgb},
    dither::{engine, DitheringAlgorithm, DitherOptions, ErrorDiffusionKernel, RandomMode},
};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "epd-dither")]
#[command(author = "EPD Optimize Team")]
#[command(version = "0.1.0")]
#[command(about = "Dither images for e-ink/e-paper displays", long_about = None)]
struct Cli {
    /// Input image file
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    /// Output image file
    #[arg(short, long, value_name = "FILE")]
    output: PathBuf,

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

    /// List available palettes and exit
    #[arg(long)]
    list_palettes: bool,

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

    // Validate input file exists
    if !cli.input.exists() {
        anyhow::bail!("Input file does not exist: {}", cli.input.display());
    }

    if cli.verbose {
        println!("Loading image: {}", cli.input.display());
    }

    // Load image
    let img = image::open(&cli.input)
        .with_context(|| format!("Failed to open image: {}", cli.input.display()))?;
    let mut rgb_img = img.to_rgb8();

    if cli.verbose {
        println!(
            "Image dimensions: {}x{}",
            rgb_img.width(),
            rgb_img.height()
        );
    }

    // Get or create palette
    let palette = if let Some(custom) = &cli.custom_palette {
        let colors = parse_custom_palette(custom)?;
        epd_dither::Palette::new("custom", colors)
    } else {
        palette_manager.get_palette(&cli.palette)?
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
        println!("Saving output: {}", cli.output.display());
    }

    // Save output
    rgb_img
        .save(&cli.output)
        .with_context(|| format!("Failed to save image: {}", cli.output.display()))?;

    if cli.verbose {
        println!("Done!");
    }

    Ok(())
}
