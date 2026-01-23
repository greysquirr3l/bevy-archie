//! Hardware constants and specifications for controller support.
//!
//! This module contains calibration constants, hardware specifications,
//! and conventions sourced from hardware datasheets and community research.

// ========== Analog Stick Conventions ==========
// Reference: USB HID Usage Tables, DirectInput, Joypad OS
// https://github.com/joypad-ai/joypad-os/blob/main/src/core/input_event.h

/// Standard analog stick value conventions.
///
/// Most controllers and HID implementations follow this convention:
/// - **X-axis**: 0 = full left, 128 = center, 255 = full right
/// - **Y-axis**: 0 = full up, 128 = center, 255 = full down
///
/// This is the USB HID standard and matches `DirectInput`, GP2040-CE, and most
/// controller firmware implementations.
///
/// **Important**: Some game engines (including Bevy/gilrs) may invert the Y-axis
/// in their API layer to match typical 3D coordinate systems where +Y is up.
/// This module documents the **hardware convention**, not the API convention.
pub mod analog_stick {
    /// Minimum raw value (left/up).
    pub const MIN: u8 = 0;
    /// Center/neutral raw value.
    pub const CENTER: u8 = 128;
    /// Maximum raw value (right/down).
    pub const MAX: u8 = 255;

    /// Minimum normalized value.
    pub const MIN_NORMALIZED: f32 = -1.0;
    /// Center normalized value.
    pub const CENTER_NORMALIZED: f32 = 0.0;
    /// Maximum normalized value.
    pub const MAX_NORMALIZED: f32 = 1.0;

    /// Default deadzone radius (as proportion of full range).
    pub const DEFAULT_DEADZONE: f32 = 0.15;

    /// Convert raw 8-bit value to normalized -1.0 to 1.0 range.
    #[must_use]
    pub fn normalize(raw: u8) -> f32 {
        (f32::from(raw) - f32::from(CENTER)) / f32::from(CENTER)
    }

    /// Convert normalized value back to raw 8-bit.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn denormalize(normalized: f32) -> u8 {
        ((normalized * f32::from(CENTER)) + f32::from(CENTER)) as u8
    }

    /// Apply circular deadzone to X/Y pair.
    #[must_use]
    pub fn apply_deadzone(x: f32, y: f32, deadzone: f32) -> (f32, f32) {
        let magnitude = (x * x + y * y).sqrt();
        if magnitude < deadzone {
            (0.0, 0.0)
        } else {
            let scale = (magnitude - deadzone) / (1.0 - deadzone);
            (x * scale / magnitude, y * scale / magnitude)
        }
    }
}

/// Trigger value conventions.
///
/// Analog triggers (L2/R2, LT/RT) typically use:
/// - 0 = fully released
/// - 255 = fully pressed
pub mod trigger {
    /// Minimum raw value (released).
    pub const MIN: u8 = 0;
    /// Maximum raw value (fully pressed).
    pub const MAX: u8 = 255;

    /// Convert raw trigger value to normalized 0.0-1.0 range.
    #[must_use]
    pub fn normalize(raw: u8) -> f32 {
        f32::from(raw) / f32::from(MAX)
    }

    /// Convert normalized value back to raw.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn denormalize(normalized: f32) -> u8 {
        (normalized * f32::from(MAX)) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_analog_stick_normalize_center() {
        assert_relative_eq!(analog_stick::normalize(128), 0.0);
    }

    #[test]
    fn test_analog_stick_normalize_min() {
        assert_relative_eq!(analog_stick::normalize(0), -1.0);
    }

    #[test]
    fn test_analog_stick_normalize_max() {
        assert_relative_eq!(analog_stick::normalize(255), 0.9921875); // ~1.0
    }

    #[test]
    fn test_analog_stick_denormalize_center() {
        assert_eq!(analog_stick::denormalize(0.0), 128);
    }

    #[test]
    fn test_analog_stick_denormalize_min() {
        assert_eq!(analog_stick::denormalize(-1.0), 0);
    }

    #[test]
    fn test_analog_stick_denormalize_max() {
        assert_eq!(analog_stick::denormalize(1.0), 255);
    }

    #[test]
    fn test_analog_stick_deadzone_inside() {
        let (x, y) = analog_stick::apply_deadzone(0.1, 0.1, 0.15);
        assert_relative_eq!(x, 0.0);
        assert_relative_eq!(y, 0.0);
    }

    #[test]
    fn test_analog_stick_deadzone_outside() {
        let (x, y) = analog_stick::apply_deadzone(0.5, 0.0, 0.15);
        assert!(x > 0.0);
        assert_relative_eq!(y, 0.0);
    }

    #[test]
    fn test_trigger_normalize_min() {
        assert_relative_eq!(trigger::normalize(0), 0.0);
    }

    #[test]
    fn test_trigger_normalize_max() {
        assert_relative_eq!(trigger::normalize(255), 1.0);
    }

    #[test]
    fn test_trigger_normalize_half() {
        assert_relative_eq!(trigger::normalize(127), 0.498, epsilon = 0.01);
    }

    #[test]
    fn test_trigger_denormalize_min() {
        assert_eq!(trigger::denormalize(0.0), 0);
    }

    #[test]
    fn test_trigger_denormalize_max() {
        assert_eq!(trigger::denormalize(1.0), 255);
    }

    #[test]
    fn test_trigger_denormalize_half() {
        assert_eq!(trigger::denormalize(0.5), 127);
    }
}
