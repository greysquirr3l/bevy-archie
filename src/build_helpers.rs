//! Build script helpers for controller asset management.
//!
//! This module provides utilities for use in build.rs scripts to help
//! manage controller icons and other assets at compile time.
//!
//! # Example build.rs
//!
//! ```rust,no_run
//! use bevy_archie::build_helpers::{generate_icon_manifest, ControllerIconConfig};
//!
//! let config = ControllerIconConfig::default();
//! let _manifest = generate_icon_manifest("assets/icons", &config);
//! ```

use std::collections::HashMap;
use std::fmt::Write;
use std::fs;
use std::path::Path;

/// Configuration for controller icon generation.
#[derive(Debug, Clone)]
pub struct ControllerIconConfig {
    /// Supported controller types
    pub controller_types: Vec<ControllerType>,
    /// Icon sizes to generate/validate
    pub icon_sizes: Vec<u32>,
    /// Output format for generated manifests
    pub manifest_format: ManifestFormat,
    /// Whether to validate icons exist
    pub validate_icons: bool,
}

impl Default for ControllerIconConfig {
    fn default() -> Self {
        Self {
            controller_types: vec![
                ControllerType::Xbox,
                ControllerType::PlayStation,
                ControllerType::Nintendo,
                ControllerType::Generic,
            ],
            icon_sizes: vec![32, 64, 128],
            manifest_format: ManifestFormat::Json,
            validate_icons: true,
        }
    }
}

/// Supported controller types for icons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControllerType {
    /// Xbox controller family
    Xbox,
    /// `PlayStation` controller family
    PlayStation,
    /// Nintendo controller family
    Nintendo,
    /// Steam Deck
    SteamDeck,
    /// Generic/fallback controller
    Generic,
}

impl ControllerType {
    /// Get the directory name for this controller type.
    #[must_use]
    pub fn dir_name(self) -> &'static str {
        match self {
            Self::Xbox => "xbox",
            Self::PlayStation => "playstation",
            Self::Nintendo => "nintendo",
            Self::SteamDeck => "steamdeck",
            Self::Generic => "generic",
        }
    }

    /// Get all button names for this controller type.
    #[must_use]
    #[expect(clippy::too_many_lines, reason = "exhaustive controller button mapping requires comprehensive listing")]
    pub fn button_names(self) -> &'static [&'static str] {
        match self {
            Self::Xbox => &[
                "a",
                "b",
                "x",
                "y",
                "lb",
                "rb",
                "lt",
                "rt",
                "start",
                "select",
                "guide",
                "ls",
                "rs",
                "dpad_up",
                "dpad_down",
                "dpad_left",
                "dpad_right",
            ],
            Self::PlayStation => &[
                "cross",
                "circle",
                "square",
                "triangle",
                "l1",
                "r1",
                "l2",
                "r2",
                "options",
                "share",
                "ps",
                "l3",
                "r3",
                "dpad_up",
                "dpad_down",
                "dpad_left",
                "dpad_right",
                "touchpad",
            ],
            Self::Nintendo => &[
                "a",
                "b",
                "x",
                "y",
                "l",
                "r",
                "zl",
                "zr",
                "plus",
                "minus",
                "home",
                "ls",
                "rs",
                "dpad_up",
                "dpad_down",
                "dpad_left",
                "dpad_right",
            ],
            Self::SteamDeck => &[
                "a",
                "b",
                "x",
                "y",
                "l1",
                "r1",
                "l2",
                "r2",
                "l4",
                "r4",
                "l5",
                "r5",
                "start",
                "select",
                "steam",
                "qam",
                "ls",
                "rs",
                "dpad_up",
                "dpad_down",
                "dpad_left",
                "dpad_right",
                "trackpad_left",
                "trackpad_right",
            ],
            Self::Generic => &[
                "a",
                "b",
                "x",
                "y",
                "lb",
                "rb",
                "lt",
                "rt",
                "start",
                "select",
                "ls",
                "rs",
                "dpad_up",
                "dpad_down",
                "dpad_left",
                "dpad_right",
            ],
        }
    }
}

/// Format for generated manifests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestFormat {
    /// JSON format
    Json,
    /// RON (Rusty Object Notation) format
    Ron,
    /// TOML format
    Toml,
}

/// A manifest entry for a controller icon.
#[derive(Debug, Clone)]
pub struct IconManifestEntry {
    /// Button/input name
    pub name: String,
    /// Controller type
    pub controller_type: ControllerType,
    /// Available sizes
    pub sizes: Vec<u32>,
    /// Relative path to icon
    pub path: String,
}

/// Result of icon manifest generation.
#[derive(Debug)]
pub struct IconManifest {
    /// All icon entries
    pub entries: Vec<IconManifestEntry>,
    /// Missing icons (if validation enabled)
    pub missing: Vec<String>,
    /// Extra/unexpected files found
    pub extras: Vec<String>,
}

/// Generate an icon manifest from an assets directory.
#[must_use]
pub fn generate_icon_manifest(
    assets_dir: impl AsRef<Path>,
    config: &ControllerIconConfig,
) -> IconManifest {
    let assets_dir = assets_dir.as_ref();
    let mut manifest = IconManifest {
        entries: Vec::new(),
        missing: Vec::new(),
        extras: Vec::new(),
    };

    for controller_type in &config.controller_types {
        let controller_dir = assets_dir.join(controller_type.dir_name());

        if !controller_dir.exists() {
            if config.validate_icons {
                manifest
                    .missing
                    .push(format!("Directory missing: {}", controller_dir.display()));
            }
            continue;
        }

        // Check for each expected button
        for button_name in controller_type.button_names() {
            let mut found_sizes = Vec::new();

            for size in &config.icon_sizes {
                let icon_path = controller_dir.join(format!("{button_name}_{size}.png"));
                if icon_path.exists() {
                    found_sizes.push(*size);
                } else if config.validate_icons {
                    manifest.missing.push(format!(
                        "Icon missing: {button_name} for {controller_type:?} at {size}x{size}"
                    ));
                }
            }

            if !found_sizes.is_empty() {
                manifest.entries.push(IconManifestEntry {
                    name: (*button_name).to_string(),
                    controller_type: *controller_type,
                    sizes: found_sizes,
                    path: format!("{}/{}", controller_type.dir_name(), button_name),
                });
            }
        }
    }

    manifest
}

/// Write the manifest to a file.
///
/// # Errors
///
/// Returns an error if the file can't be written.
pub fn write_manifest(
    manifest: &IconManifest,
    output_path: impl AsRef<Path>,
    format: ManifestFormat,
) -> Result<(), std::io::Error> {
    let content = match format {
        ManifestFormat::Json => {
            let mut map: HashMap<String, Vec<HashMap<String, serde_json::Value>>> = HashMap::new();

            for entry in &manifest.entries {
                let controller_key = format!("{:?}", entry.controller_type).to_lowercase();
                let entry_map = map.entry(controller_key).or_default();

                let mut icon_data = HashMap::new();
                icon_data.insert("name".into(), serde_json::Value::String(entry.name.clone()));
                icon_data.insert("path".into(), serde_json::Value::String(entry.path.clone()));
                icon_data.insert(
                    "sizes".into(),
                    serde_json::Value::Array(
                        entry
                            .sizes
                            .iter()
                            .map(|s| serde_json::Value::Number((*s).into()))
                            .collect(),
                    ),
                );

                entry_map.push(icon_data);
            }

            serde_json::to_string_pretty(&map)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?
        }
        ManifestFormat::Ron => {
            // Simplified RON output
            let mut output = String::from("(\n");
            for entry in &manifest.entries {
                let _ = writeln!(
                    output,
                    "  ({:?}, \"{}\", {:?}, \"{}\"),",
                    entry.controller_type, entry.name, entry.sizes, entry.path
                );
            }
            output.push(')');
            output
        }
        ManifestFormat::Toml => {
            // Simplified TOML output
            let mut output = String::new();
            for entry in &manifest.entries {
                let _ = write!(
                    output,
                    "[[{:?}]]\nname = \"{}\"\npath = \"{}\"\nsizes = {:?}\n\n",
                    entry.controller_type, entry.name, entry.path, entry.sizes
                );
            }
            output
        }
    };

    fs::write(output_path, content)
}

/// Validate that all required icons exist.
///
/// Returns a list of missing icons.
#[must_use]
pub fn validate_icons(assets_dir: impl AsRef<Path>, config: &ControllerIconConfig) -> Vec<String> {
    let mut config = config.clone();
    config.validate_icons = true;

    generate_icon_manifest(assets_dir, &config).missing
}

/// Generate Rust code for icon constants.
///
/// This can be used in build.rs to generate a constants file.
#[must_use]
pub fn generate_icon_constants(manifest: &IconManifest) -> String {
    let mut output = String::from("// Auto-generated icon constants\n\n");

    // Group by controller type
    let mut by_type: HashMap<&str, Vec<&IconManifestEntry>> = HashMap::new();
    for entry in &manifest.entries {
        by_type
            .entry(entry.controller_type.dir_name())
            .or_default()
            .push(entry);
    }

    for (controller_type, entries) in &by_type {
        let _ = write!(output, "\npub mod {controller_type} {{\n");

        for entry in entries {
            let const_name = entry.name.to_uppercase().replace('-', "_");
            let _ = writeln!(
                output,
                "    pub const {const_name}: &str = \"{}\";",
                entry.path
            );
        }

        output.push_str("}\n");
    }

    output
}

/// Print cargo rerun-if-changed directives for icon assets.
pub fn print_rerun_if_changed(assets_dir: impl AsRef<Path>) {
    let assets_dir = assets_dir.as_ref();

    println!("cargo:rerun-if-changed={}", assets_dir.display());

    if let Ok(entries) = fs::read_dir(assets_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            println!("cargo:rerun-if-changed={}", path.display());

            if path.is_dir()
                && let Ok(sub_entries) = fs::read_dir(&path)
            {
                for sub_entry in sub_entries.flatten() {
                    println!("cargo:rerun-if-changed={}", sub_entry.path().display());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_controller_type_buttons() {
        let xbox = ControllerType::Xbox;
        assert!(xbox.button_names().contains(&"a"));
        assert!(xbox.button_names().contains(&"lb"));

        let ps = ControllerType::PlayStation;
        assert!(ps.button_names().contains(&"cross"));
        assert!(ps.button_names().contains(&"touchpad"));
    }

    #[test]
    fn test_config_default() {
        let config = ControllerIconConfig::default();
        assert!(!config.controller_types.is_empty());
        assert!(!config.icon_sizes.is_empty());
        assert!(config.validate_icons);
    }

    #[test]
    fn test_generate_constants() {
        let manifest = IconManifest {
            entries: vec![IconManifestEntry {
                name: "a".into(),
                controller_type: ControllerType::Xbox,
                sizes: vec![32, 64],
                path: "xbox/a".into(),
            }],
            missing: vec![],
            extras: vec![],
        };

        let output = generate_icon_constants(&manifest);
        assert!(output.contains("pub mod xbox"));
        assert!(output.contains("pub const A: &str"));
    }
}
