//! Color palette management and loading

use super::{convert, Rgb};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A color palette for dithering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Palette {
    pub name: String,
    pub colors: Vec<Rgb>,
}

impl Palette {
    /// Create a new palette from a list of colors
    pub fn new(name: impl Into<String>, colors: Vec<Rgb>) -> Self {
        Self {
            name: name.into(),
            colors,
        }
    }

    /// Create a palette from hex color strings
    pub fn from_hex_strings(name: impl Into<String>, hex_colors: &[&str]) -> Result<Self> {
        let colors: Result<Vec<_>> = hex_colors
            .iter()
            .map(|hex| convert::hex_to_rgb(hex).map(Rgb))
            .collect();

        Ok(Self {
            name: name.into(),
            colors: colors?,
        })
    }

    /// Get the number of colors in the palette
    pub fn len(&self) -> usize {
        self.colors.len()
    }

    /// Check if the palette is empty
    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::from_hex_strings("default", &["#000000", "#FFFFFF"])
            .expect("Default palette should always be valid")
    }
}

/// Palette manager for loading and managing predefined palettes
pub struct PaletteManager {
    palettes: HashMap<String, Vec<String>>,
    device_colors: HashMap<String, Vec<String>>,
}

impl PaletteManager {
    /// Load palettes from embedded JSON data
    pub fn new() -> Result<Self> {
        let palettes_json = include_str!("../data/palettes.json");
        let device_colors_json = include_str!("../data/device_colors.json");

        let palettes: HashMap<String, Vec<String>> = serde_json::from_str(palettes_json)
            .map_err(|e| anyhow!("Failed to parse palettes.json: {}", e))?;

        let device_colors: HashMap<String, Vec<String>> = serde_json::from_str(device_colors_json)
            .map_err(|e| anyhow!("Failed to parse device_colors.json: {}", e))?;

        Ok(Self {
            palettes,
            device_colors,
        })
    }

    /// Get a palette by name
    pub fn get_palette(&self, name: &str) -> Result<Palette> {
        let hex_colors = self
            .palettes
            .get(name)
            .ok_or_else(|| anyhow!("Palette '{}' not found", name))?;

        let colors: Result<Vec<_>> = hex_colors
            .iter()
            .map(|hex| convert::hex_to_rgb(hex).map(Rgb))
            .collect();

        Ok(Palette {
            name: name.to_string(),
            colors: colors?,
        })
    }

    /// Get device colors by name
    pub fn get_device_colors(&self, name: &str) -> Result<Vec<Rgb>> {
        let hex_colors = self
            .device_colors
            .get(name)
            .ok_or_else(|| anyhow!("Device colors '{}' not found", name))?;

        hex_colors
            .iter()
            .map(|hex| convert::hex_to_rgb(hex).map(Rgb))
            .collect()
    }

    /// List all available palette names
    pub fn list_palettes(&self) -> Vec<String> {
        let mut names: Vec<_> = self.palettes.keys().cloned().collect();
        names.sort();
        names
    }

    /// List all available device color sets
    pub fn list_device_colors(&self) -> Vec<String> {
        let mut names: Vec<_> = self.device_colors.keys().cloned().collect();
        names.sort();
        names
    }
}

impl Default for PaletteManager {
    fn default() -> Self {
        Self::new().expect("Failed to load embedded palettes")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette_creation() {
        let palette = Palette::from_hex_strings("test", &["#000000", "#FFFFFF"]).unwrap();
        assert_eq!(palette.name, "test");
        assert_eq!(palette.len(), 2);
        assert_eq!(palette.colors[0], Rgb::new(0, 0, 0));
        assert_eq!(palette.colors[1], Rgb::new(255, 255, 255));
    }

    #[test]
    fn test_palette_manager() {
        let manager = PaletteManager::new().unwrap();

        // Check that we can load default palette
        let palette = manager.get_palette("default").unwrap();
        assert_eq!(palette.name, "default");
        assert!(!palette.is_empty());

        // List palettes
        let palettes = manager.list_palettes();
        assert!(palettes.contains(&"default".to_string()));
        assert!(palettes.contains(&"spectra6".to_string()));
    }
}
