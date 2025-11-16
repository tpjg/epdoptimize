//! Device database and management for e-ink displays

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Resolution of a display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

/// Recommended settings for a device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendedSettings {
    pub algorithm: String,
    pub serpentine: bool,
    pub fit_mode: String,
    pub scaling_algorithm: String,
}

/// E-Ink device specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSpec {
    pub name: String,
    pub display_technology: String,
    pub size_inches: f32,
    pub resolution: Resolution,
    pub ppi: u32,
    pub palette: String,
    pub recommended_settings: RecommendedSettings,
}

/// Database of all devices
#[derive(Debug, Deserialize)]
struct DeviceDatabase {
    devices: HashMap<String, DeviceSpec>,
}

/// Manager for e-ink device specifications
pub struct DeviceManager {
    devices: HashMap<String, DeviceSpec>,
}

impl DeviceManager {
    /// Create a new device manager with built-in device database
    pub fn new() -> Result<Self> {
        let json_data = include_str!("../data/devices.json");
        let database: DeviceDatabase =
            serde_json::from_str(json_data).context("Failed to parse devices.json")?;

        Ok(Self {
            devices: database.devices,
        })
    }

    /// Get a device specification by ID
    pub fn get_device(&self, device_id: &str) -> Result<DeviceSpec> {
        self.devices
            .get(device_id)
            .cloned()
            .with_context(|| format!("Device '{}' not found", device_id))
    }

    /// List all available device IDs
    pub fn list_devices(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.devices.keys().cloned().collect();
        ids.sort();
        ids
    }

    /// Get all devices grouped by technology
    pub fn devices_by_technology(&self) -> HashMap<String, Vec<(String, DeviceSpec)>> {
        let mut grouped: HashMap<String, Vec<(String, DeviceSpec)>> = HashMap::new();

        for (id, spec) in &self.devices {
            grouped
                .entry(spec.display_technology.clone())
                .or_insert_with(Vec::new)
                .push((id.clone(), spec.clone()));
        }

        // Sort devices within each technology group by size
        for devices in grouped.values_mut() {
            devices.sort_by(|a, b| {
                a.1.size_inches
                    .partial_cmp(&b.1.size_inches)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }

        grouped
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_manager_creation() {
        let manager = DeviceManager::new().unwrap();
        assert!(!manager.devices.is_empty());
    }

    #[test]
    fn test_get_device() {
        let manager = DeviceManager::new().unwrap();
        let device = manager.get_device("spectra6-7.3").unwrap();
        assert_eq!(device.resolution.width, 800);
        assert_eq!(device.resolution.height, 480);
        assert_eq!(device.palette, "spectra6");
    }

    #[test]
    fn test_list_devices() {
        let manager = DeviceManager::new().unwrap();
        let devices = manager.list_devices();
        assert!(devices.contains(&"spectra6-7.3".to_string()));
        assert!(devices.contains(&"acep-7.3".to_string()));
    }
}
