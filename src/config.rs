//! Controller configuration and settings.
//!
//! This module contains configuration types for controller behavior,
//! including deadzone settings, sensitivity, and layout preferences.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Controller layout type for icon display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, Reflect)]
pub enum ControllerLayout {
    /// Xbox-style layout (A/B/X/Y with colored buttons)
    #[default]
    Xbox,
    /// PlayStation-style layout (Cross/Circle/Square/Triangle)
    PlayStation,
    /// Nintendo-style layout (A/B/X/Y with swapped positions)
    Nintendo,
    /// Steam Controller / Steam Deck layout
    Steam,
    /// Google Stadia Controller (Xbox-style layout in Bluetooth mode)
    Stadia,
    /// Generic layout for unrecognized controllers
    Generic,
}

impl ControllerLayout {
    /// Detect controller layout from controller name.
    #[must_use]
    pub fn from_name(name: &str) -> Self {
        let name_lower = name.to_lowercase();

        // Xbox controllers
        if name_lower.contains("xbox") || name_lower.contains("microsoft") {
            return Self::Xbox;
        }

        // PlayStation controllers
        if name_lower.contains("ps3")
            || name_lower.contains("ps4")
            || name_lower.contains("ps5")
            || name_lower.contains("playstation")
            || name_lower.contains("dualshock")
            || name_lower.contains("dualsense")
            || name_lower.contains("sony")
        {
            return Self::PlayStation;
        }

        // Nintendo controllers
        if name_lower.contains("nintendo")
            || name_lower.contains("switch")
            || name_lower.contains("joycon")
            || name_lower.contains("joy-con")
            || name_lower.contains("pro controller")
            || name_lower.contains("gamecube")
            || name_lower.contains("wii")
        {
            return Self::Nintendo;
        }

        // Steam controllers
        if name_lower.contains("steam") || name_lower.contains("valve") {
            return Self::Steam;
        }

        // Stadia controllers (post-shutdown Bluetooth mode)
        if name_lower.contains("stadia") || name_lower.contains("google") {
            return Self::Stadia;
        }

        Self::Generic
    }

    /// Get the display name for a button on this layout.
    #[must_use]
    pub const fn button_name(self, button: GamepadButton) -> &'static str {
        match (self, button) {
            // Face buttons
            (Self::PlayStation, GamepadButton::South) => "Cross",
            (Self::PlayStation, GamepadButton::East) => "Circle",
            (Self::PlayStation, GamepadButton::West) => "Square",
            (Self::PlayStation, GamepadButton::North) => "Triangle",

            (Self::Nintendo, GamepadButton::South) => "B",
            (Self::Nintendo, GamepadButton::East) => "A",
            (Self::Nintendo, GamepadButton::West) => "Y",
            (Self::Nintendo, GamepadButton::North) => "X",

            // Stadia uses Xbox-style naming
            (Self::Stadia, GamepadButton::South) => "A",
            (Self::Stadia, GamepadButton::East) => "B",
            (Self::Stadia, GamepadButton::West) => "X",
            (Self::Stadia, GamepadButton::North) => "Y",

            (_, GamepadButton::South) => "A",
            (_, GamepadButton::East) => "B",
            (_, GamepadButton::West) => "X",
            (_, GamepadButton::North) => "Y",

            // Shoulder buttons
            (Self::Xbox, GamepadButton::LeftTrigger) => "LB",
            (Self::Xbox, GamepadButton::RightTrigger) => "RB",
            (Self::Stadia, GamepadButton::LeftTrigger) => "L1",
            (Self::Stadia, GamepadButton::RightTrigger) => "R1",
            (Self::Nintendo, GamepadButton::LeftTrigger) => "L",
            (Self::Nintendo, GamepadButton::RightTrigger) => "R",
            (_, GamepadButton::LeftTrigger) => "L1",
            (_, GamepadButton::RightTrigger) => "R1",

            // Triggers
            (Self::Xbox, GamepadButton::LeftTrigger2) => "LT",
            (Self::Xbox, GamepadButton::RightTrigger2) => "RT",
            (Self::Stadia, GamepadButton::LeftTrigger2) => "L2",
            (Self::Stadia, GamepadButton::RightTrigger2) => "R2",
            (Self::Nintendo, GamepadButton::LeftTrigger2) => "ZL",
            (Self::Nintendo, GamepadButton::RightTrigger2) => "ZR",
            (_, GamepadButton::LeftTrigger2) => "L2",
            (_, GamepadButton::RightTrigger2) => "R2",

            // Stick clicks
            (_, GamepadButton::LeftThumb) => "L3",
            (_, GamepadButton::RightThumb) => "R3",

            // System buttons
            (Self::PlayStation, GamepadButton::Select) => "Share",
            (Self::PlayStation, GamepadButton::Start) => "Options",
            (Self::Nintendo, GamepadButton::Select) => "Minus",
            (Self::Nintendo, GamepadButton::Start) => "Plus",
            (Self::Xbox, GamepadButton::Select) => "View",
            (Self::Xbox, GamepadButton::Start) => "Menu",
            (Self::Stadia, GamepadButton::Select) => "Options",
            (Self::Stadia, GamepadButton::Start) => "Menu",
            (_, GamepadButton::Select) => "Select",
            (_, GamepadButton::Start) => "Start",

            // D-pad
            (_, GamepadButton::DPadUp) => "D-Pad Up",
            (_, GamepadButton::DPadDown) => "D-Pad Down",
            (_, GamepadButton::DPadLeft) => "D-Pad Left",
            (_, GamepadButton::DPadRight) => "D-Pad Right",

            // Other
            (_, GamepadButton::Mode) => "Home",
            (_, GamepadButton::C) => "C",
            (_, GamepadButton::Z) => "Z",
            _ => "Button",
        }
    }
}

/// Global controller configuration resource.
#[derive(Debug, Clone, Resource, Serialize, Deserialize, Reflect)]
#[reflect(Resource)]
#[allow(clippy::struct_excessive_bools)]
pub struct ControllerConfig {
    /// Analog stick deadzone (0.0 - 1.0).
    /// Values below this threshold are ignored.
    pub deadzone: f32,

    /// Minimum configurable deadzone.
    pub min_deadzone: f32,

    /// Maximum configurable deadzone.
    pub max_deadzone: f32,

    /// Left stick sensitivity multiplier (0.2 - 3.0).
    pub left_stick_sensitivity: f32,

    /// Right stick sensitivity multiplier (0.2 - 3.0).
    pub right_stick_sensitivity: f32,

    /// Minimum configurable sensitivity.
    pub min_sensitivity: f32,

    /// Maximum configurable sensitivity.
    pub max_sensitivity: f32,

    /// Whether to automatically detect controller layout from name.
    pub auto_detect_layout: bool,

    /// Force a specific controller layout (overrides auto-detection).
    pub forced_layout: Option<ControllerLayout>,

    /// Current detected/forced controller layout.
    pub current_layout: ControllerLayout,

    /// Enable vibration/haptic feedback.
    pub vibration_enabled: bool,

    /// Vibration intensity multiplier (0.0 - 1.0).
    pub vibration_intensity: f32,

    /// Invert X axis on left stick.
    pub invert_left_x: bool,

    /// Invert Y axis on left stick.
    pub invert_left_y: bool,

    /// Invert X axis on right stick.
    pub invert_right_x: bool,

    /// Invert Y axis on right stick.
    pub invert_right_y: bool,

    /// Swap left and right sticks.
    pub swap_sticks: bool,

    /// Time in seconds before a button press is considered "held".
    pub hold_threshold: f32,

    /// Time in seconds between repeated inputs when holding.
    pub repeat_delay: f32,

    /// Time in seconds between subsequent repeats.
    pub repeat_rate: f32,
}

impl Default for ControllerConfig {
    fn default() -> Self {
        Self {
            deadzone: 0.15,
            min_deadzone: 0.05,
            max_deadzone: 0.5,
            left_stick_sensitivity: 1.0,
            right_stick_sensitivity: 1.0,
            min_sensitivity: 0.2,
            max_sensitivity: 3.0,
            auto_detect_layout: true,
            forced_layout: None,
            current_layout: ControllerLayout::default(),
            vibration_enabled: true,
            vibration_intensity: 1.0,
            invert_left_x: false,
            invert_left_y: false,
            invert_right_x: false,
            invert_right_y: false,
            swap_sticks: false,
            hold_threshold: 0.5,
            repeat_delay: 0.5,
            repeat_rate: 0.1,
        }
    }
}

impl ControllerConfig {
    /// Get the effective deadzone value clamped to valid range.
    #[must_use]
    pub fn effective_deadzone(&self) -> f32 {
        self.deadzone.clamp(self.min_deadzone, self.max_deadzone)
    }

    /// Get the effective left stick sensitivity value clamped to valid range.
    #[must_use]
    pub fn effective_left_sensitivity(&self) -> f32 {
        self.left_stick_sensitivity
            .clamp(self.min_sensitivity, self.max_sensitivity)
    }

    /// Get the effective right stick sensitivity value clamped to valid range.
    #[must_use]
    pub fn effective_right_sensitivity(&self) -> f32 {
        self.right_stick_sensitivity
            .clamp(self.min_sensitivity, self.max_sensitivity)
    }

    /// Get the current layout (forced or detected).
    #[must_use]
    pub fn layout(&self) -> ControllerLayout {
        self.forced_layout.unwrap_or(self.current_layout)
    }

    /// Apply deadzone and sensitivity to an axis value for the left stick.
    #[must_use]
    pub fn apply_deadzone_left(&self, value: f32) -> f32 {
        let deadzone = self.effective_deadzone();
        if value.abs() < deadzone {
            0.0
        } else {
            // Remap the value to 0.0-1.0 range after deadzone
            let sign = value.signum();
            let normalized = (value.abs() - deadzone) / (1.0 - deadzone);
            sign * normalized * self.effective_left_sensitivity()
        }
    }

    /// Apply deadzone and sensitivity to an axis value for the right stick.
    #[must_use]
    pub fn apply_deadzone_right(&self, value: f32) -> f32 {
        let deadzone = self.effective_deadzone();
        if value.abs() < deadzone {
            0.0
        } else {
            let sign = value.signum();
            let normalized = (value.abs() - deadzone) / (1.0 - deadzone);
            sign * normalized * self.effective_right_sensitivity()
        }
    }

    /// Apply deadzone to a 2D axis (stick) with per-stick sensitivity.
    #[must_use]
    pub fn apply_deadzone_2d(&self, x: f32, y: f32, is_left_stick: bool) -> Vec2 {
        let deadzone = self.effective_deadzone();
        let magnitude = (x * x + y * y).sqrt();

        if magnitude < deadzone {
            Vec2::ZERO
        } else {
            // Remap with circular deadzone
            let sensitivity = if is_left_stick {
                self.effective_left_sensitivity()
            } else {
                self.effective_right_sensitivity()
            };
            let normalized_magnitude =
                ((magnitude - deadzone) / (1.0 - deadzone)).min(1.0) * sensitivity;
            let direction = Vec2::new(x, y) / magnitude;
            direction * normalized_magnitude
        }
    }

    /// Apply inversion to stick input based on configuration.
    #[must_use]
    pub fn apply_inversion(&self, mut value: Vec2, is_left_stick: bool) -> Vec2 {
        if is_left_stick {
            if self.invert_left_x {
                value.x = -value.x;
            }
            if self.invert_left_y {
                value.y = -value.y;
            }
        } else {
            if self.invert_right_x {
                value.x = -value.x;
            }
            if self.invert_right_y {
                value.y = -value.y;
            }
        }
        value
    }

    /// Save configuration to a JSON file.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails or the file cannot be written.
    pub fn save_to_file(&self, path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self).map_err(std::io::Error::other)?;
        std::fs::write(path, json)
    }

    /// Load configuration from a JSON file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or contains invalid JSON.
    pub fn load_from_file(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        serde_json::from_str(&json)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /// Get the default config file path for the current platform.
    #[must_use]
    pub fn default_config_path() -> std::path::PathBuf {
        if let Some(config_dir) = dirs::config_dir() {
            config_dir.join("bevy_archie").join("controller.json")
        } else {
            std::path::PathBuf::from("controller_config.json")
        }
    }

    /// Load configuration from the default path, or return default if not found.
    #[must_use]
    pub fn load_or_default() -> Self {
        let path = Self::default_config_path();
        Self::load_from_file(&path).unwrap_or_default()
    }

    /// Save configuration to the default path, creating directories if needed.
    ///
    /// # Errors
    ///
    /// Returns an error if directories cannot be created or the file cannot be written.
    pub fn save_default(&self) -> std::io::Result<()> {
        let path = Self::default_config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        self.save_to_file(&path)
    }
}

/// Event fired when controller configuration changes.
#[derive(Debug, Clone, Message)]
pub struct ControllerConfigChanged {
    /// The field that changed.
    pub field: ConfigField,
}

/// Which configuration field changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigField {
    Deadzone,
    Sensitivity,
    Layout,
    Vibration,
    InvertAxis,
    SwapSticks,
    Timing,
}

/// Plugin for registering configuration types.
pub(crate) fn register_config_types(app: &mut App) {
    app.register_type::<ControllerConfig>()
        .register_type::<ControllerLayout>()
        .init_resource::<ControllerConfig>()
        .add_message::<ControllerConfigChanged>();
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    // ========== ControllerLayout Tests ==========

    #[test]
    fn test_controller_layout_default() {
        let layout = ControllerLayout::default();
        assert_eq!(layout, ControllerLayout::Xbox);
    }

    #[test]
    fn test_controller_layout_from_name_xbox() {
        assert_eq!(
            ControllerLayout::from_name("Xbox Controller"),
            ControllerLayout::Xbox
        );
        assert_eq!(
            ControllerLayout::from_name("Microsoft Wireless"),
            ControllerLayout::Xbox
        );
    }

    #[test]
    fn test_controller_layout_from_name_playstation() {
        assert_eq!(
            ControllerLayout::from_name("PS4 Controller"),
            ControllerLayout::PlayStation
        );
        assert_eq!(
            ControllerLayout::from_name("PS5 DualSense"),
            ControllerLayout::PlayStation
        );
        assert_eq!(
            ControllerLayout::from_name("DualShock 4"),
            ControllerLayout::PlayStation
        );
        assert_eq!(
            ControllerLayout::from_name("Sony Controller"),
            ControllerLayout::PlayStation
        );
    }

    #[test]
    fn test_controller_layout_from_name_nintendo() {
        assert_eq!(
            ControllerLayout::from_name("Nintendo Switch Pro"),
            ControllerLayout::Nintendo
        );
        assert_eq!(
            ControllerLayout::from_name("Joy-Con"),
            ControllerLayout::Nintendo
        );
        assert_eq!(
            ControllerLayout::from_name("JoyCon L"),
            ControllerLayout::Nintendo
        );
        assert_eq!(
            ControllerLayout::from_name("Pro Controller"),
            ControllerLayout::Nintendo
        );
        assert_eq!(
            ControllerLayout::from_name("GameCube"),
            ControllerLayout::Nintendo
        );
        assert_eq!(
            ControllerLayout::from_name("Wii Remote"),
            ControllerLayout::Nintendo
        );
    }

    #[test]
    fn test_controller_layout_from_name_steam() {
        assert_eq!(
            ControllerLayout::from_name("Steam Controller"),
            ControllerLayout::Steam
        );
        assert_eq!(
            ControllerLayout::from_name("Valve Index"),
            ControllerLayout::Steam
        );
    }

    #[test]
    fn test_controller_layout_from_name_stadia() {
        assert_eq!(
            ControllerLayout::from_name("Stadia Controller"),
            ControllerLayout::Stadia
        );
        assert_eq!(
            ControllerLayout::from_name("Google Gamepad"),
            ControllerLayout::Stadia
        );
    }

    #[test]
    fn test_controller_layout_from_name_generic() {
        assert_eq!(
            ControllerLayout::from_name("Unknown Controller"),
            ControllerLayout::Generic
        );
        assert_eq!(
            ControllerLayout::from_name("8BitDo Pro 2"),
            ControllerLayout::Generic
        );
    }

    #[test]
    fn test_controller_layout_from_name_case_insensitive() {
        assert_eq!(ControllerLayout::from_name("XBOX"), ControllerLayout::Xbox);
        assert_eq!(
            ControllerLayout::from_name("playstation"),
            ControllerLayout::PlayStation
        );
    }

    #[test]
    fn test_controller_layout_button_name_playstation() {
        let layout = ControllerLayout::PlayStation;
        assert_eq!(layout.button_name(GamepadButton::South), "Cross");
        assert_eq!(layout.button_name(GamepadButton::East), "Circle");
        assert_eq!(layout.button_name(GamepadButton::West), "Square");
        assert_eq!(layout.button_name(GamepadButton::North), "Triangle");
    }

    #[test]
    fn test_controller_layout_button_name_nintendo() {
        let layout = ControllerLayout::Nintendo;
        assert_eq!(layout.button_name(GamepadButton::South), "B");
        assert_eq!(layout.button_name(GamepadButton::East), "A");
        assert_eq!(layout.button_name(GamepadButton::West), "Y");
        assert_eq!(layout.button_name(GamepadButton::North), "X");
    }

    #[test]
    fn test_controller_layout_button_name_xbox() {
        let layout = ControllerLayout::Xbox;
        assert_eq!(layout.button_name(GamepadButton::South), "A");
        assert_eq!(layout.button_name(GamepadButton::East), "B");
        assert_eq!(layout.button_name(GamepadButton::West), "X");
        assert_eq!(layout.button_name(GamepadButton::North), "Y");
    }

    #[test]
    fn test_controller_layout_button_name_triggers() {
        assert_eq!(
            ControllerLayout::Xbox.button_name(GamepadButton::LeftTrigger),
            "LB"
        );
        assert_eq!(
            ControllerLayout::Xbox.button_name(GamepadButton::RightTrigger),
            "RB"
        );
        assert_eq!(
            ControllerLayout::PlayStation.button_name(GamepadButton::LeftTrigger),
            "L1"
        );
        assert_eq!(
            ControllerLayout::Nintendo.button_name(GamepadButton::LeftTrigger),
            "L"
        );
    }

    #[test]
    fn test_controller_layout_button_name_system() {
        assert_eq!(
            ControllerLayout::PlayStation.button_name(GamepadButton::Select),
            "Share"
        );
        assert_eq!(
            ControllerLayout::PlayStation.button_name(GamepadButton::Start),
            "Options"
        );
        assert_eq!(
            ControllerLayout::Xbox.button_name(GamepadButton::Select),
            "View"
        );
        assert_eq!(
            ControllerLayout::Xbox.button_name(GamepadButton::Start),
            "Menu"
        );
    }

    // ========== ControllerConfig Tests ==========

    #[test]
    fn test_controller_config_default() {
        let config = ControllerConfig::default();
        assert_relative_eq!(config.deadzone, 0.15);
        assert_relative_eq!(config.left_stick_sensitivity, 1.0);
        assert_relative_eq!(config.right_stick_sensitivity, 1.0);
        assert!(config.vibration_enabled);
        assert_relative_eq!(config.vibration_intensity, 1.0);
        assert!(config.auto_detect_layout);
        assert!(!config.invert_left_x);
        assert!(!config.invert_left_y);
        assert!(!config.swap_sticks);
    }

    #[test]
    fn test_controller_config_effective_deadzone() {
        let mut config = ControllerConfig::default();

        // Normal deadzone
        config.deadzone = 0.2;
        assert_relative_eq!(config.effective_deadzone(), 0.2);

        // Clamped to min
        config.deadzone = 0.01;
        assert_relative_eq!(config.effective_deadzone(), config.min_deadzone);

        // Clamped to max
        config.deadzone = 0.9;
        assert_relative_eq!(config.effective_deadzone(), config.max_deadzone);
    }

    #[test]
    fn test_controller_config_effective_sensitivity() {
        let mut config = ControllerConfig::default();

        config.left_stick_sensitivity = 1.5;
        assert_relative_eq!(config.effective_left_sensitivity(), 1.5);

        config.right_stick_sensitivity = 2.5;
        assert_relative_eq!(config.effective_right_sensitivity(), 2.5);

        // Clamped values
        config.left_stick_sensitivity = 0.1;
        assert_relative_eq!(config.effective_left_sensitivity(), config.min_sensitivity);

        config.right_stick_sensitivity = 5.0;
        assert_relative_eq!(config.effective_right_sensitivity(), config.max_sensitivity);
    }

    #[test]
    fn test_controller_config_layout() {
        let mut config = ControllerConfig::default();
        config.current_layout = ControllerLayout::PlayStation;

        // No forced layout - use current
        assert_eq!(config.layout(), ControllerLayout::PlayStation);

        // Forced layout overrides
        config.forced_layout = Some(ControllerLayout::Nintendo);
        assert_eq!(config.layout(), ControllerLayout::Nintendo);
    }

    #[test]
    fn test_controller_config_apply_deadzone_left() {
        let config = ControllerConfig::default();

        // Within deadzone
        assert_relative_eq!(config.apply_deadzone_left(0.1), 0.0);

        // Outside deadzone
        let result = config.apply_deadzone_left(0.5);
        assert!(result > 0.0);

        // Negative values
        let result_neg = config.apply_deadzone_left(-0.5);
        assert!(result_neg < 0.0);
    }

    #[test]
    fn test_controller_config_apply_deadzone_right() {
        let config = ControllerConfig::default();

        // Within deadzone
        assert_relative_eq!(config.apply_deadzone_right(0.1), 0.0);

        // Outside deadzone
        let result = config.apply_deadzone_right(0.5);
        assert!(result > 0.0);
    }

    #[test]
    fn test_controller_config_apply_deadzone_2d() {
        let config = ControllerConfig::default();

        // Within deadzone
        let result = config.apply_deadzone_2d(0.05, 0.05, true);
        assert_eq!(result, Vec2::ZERO);

        // Outside deadzone
        let result = config.apply_deadzone_2d(0.5, 0.5, true);
        assert!(result.x > 0.0);
        assert!(result.y > 0.0);
    }

    #[test]
    fn test_controller_config_apply_inversion_left() {
        let mut config = ControllerConfig::default();
        let value = Vec2::new(0.5, 0.5);

        // No inversion
        let result = config.apply_inversion(value, true);
        assert_eq!(result, value);

        // Invert X
        config.invert_left_x = true;
        let result = config.apply_inversion(value, true);
        assert_relative_eq!(result.x, -0.5);
        assert_relative_eq!(result.y, 0.5);

        // Invert Y
        config.invert_left_x = false;
        config.invert_left_y = true;
        let result = config.apply_inversion(value, true);
        assert_relative_eq!(result.x, 0.5);
        assert_relative_eq!(result.y, -0.5);

        // Invert both
        config.invert_left_x = true;
        let result = config.apply_inversion(value, true);
        assert_relative_eq!(result.x, -0.5);
        assert_relative_eq!(result.y, -0.5);
    }

    #[test]
    fn test_controller_config_apply_inversion_right() {
        let mut config = ControllerConfig::default();
        let value = Vec2::new(0.5, 0.5);

        config.invert_right_x = true;
        let result = config.apply_inversion(value, false);
        assert_relative_eq!(result.x, -0.5);

        config.invert_right_y = true;
        let result = config.apply_inversion(value, false);
        assert_relative_eq!(result.y, -0.5);
    }

    #[test]
    fn test_controller_config_default_path() {
        let path = ControllerConfig::default_config_path();
        assert!(path.to_string_lossy().contains("controller"));
    }

    // ========== ConfigField Tests ==========

    #[test]
    fn test_config_field_equality() {
        assert_eq!(ConfigField::Deadzone, ConfigField::Deadzone);
        assert_ne!(ConfigField::Deadzone, ConfigField::Sensitivity);
    }

    #[test]
    fn test_config_field_variants() {
        let fields = [
            ConfigField::Deadzone,
            ConfigField::Sensitivity,
            ConfigField::Layout,
            ConfigField::Vibration,
            ConfigField::InvertAxis,
            ConfigField::SwapSticks,
            ConfigField::Timing,
        ];
        assert_eq!(fields.len(), 7);
    }

    // ========== ControllerConfigChanged Event Tests ==========

    #[test]
    fn test_controller_config_changed_event() {
        let event = ControllerConfigChanged {
            field: ConfigField::Deadzone,
        };
        assert_eq!(event.field, ConfigField::Deadzone);
    }
}
