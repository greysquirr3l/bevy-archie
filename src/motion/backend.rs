//! Motion backend trait definitions.

// ========== Motion Sensor Calibration Constants ==========
// Sourced from hardware specifications and verified against Joypad OS implementation
// Reference: https://github.com/joypad-ai/joypad-os

/// `DualSense` (PS5) motion sensor calibration constants.
pub mod dualsense_calibration {
    /// Accelerometer range: ±8g at 16-bit resolution.
    pub const ACCEL_RANGE: f32 = 16384.0;
    /// Gyroscope range: ±2000 dps (degrees per second) at 16-bit resolution.
    pub const GYRO_RANGE: f32 = 1024.0;

    /// Convert raw `DualSense` accelerometer value to m/s².
    #[must_use]
    pub fn accel_to_ms2(raw: i16) -> f32 {
        const G: f32 = 9.81; // Standard gravity in m/s²
        (f32::from(raw) / ACCEL_RANGE) * 8.0 * G
    }

    /// Convert raw `DualSense` gyroscope value to rad/s.
    #[must_use]
    pub fn gyro_to_rads(raw: i16) -> f32 {
        const DEG_TO_RAD: f32 = std::f32::consts::PI / 180.0;
        (f32::from(raw) / GYRO_RANGE) * 2000.0 * DEG_TO_RAD
    }
}

/// `DualShock` 4 (PS4) motion sensor calibration constants.
pub mod dualshock4_calibration {
    /// Accelerometer range: ±8g at 16-bit resolution (same as `DualSense`).
    pub const ACCEL_RANGE: f32 = 16384.0;
    /// Gyroscope range: ±2000 dps at 16-bit resolution.
    pub const GYRO_RANGE: f32 = 1024.0;

    /// Convert raw `DualShock` 4 accelerometer value to m/s².
    #[must_use]
    pub fn accel_to_ms2(raw: i16) -> f32 {
        const G: f32 = 9.81;
        (f32::from(raw) / ACCEL_RANGE) * 8.0 * G
    }

    /// Convert raw `DualShock` 4 gyroscope value to rad/s.
    #[must_use]
    pub fn gyro_to_rads(raw: i16) -> f32 {
        const DEG_TO_RAD: f32 = std::f32::consts::PI / 180.0;
        (f32::from(raw) / GYRO_RANGE) * 2000.0 * DEG_TO_RAD
    }
}

/// `DualShock` 3 (PS3 SIXAXIS) motion sensor calibration constants.
pub mod dualshock3_calibration {
    /// SIXAXIS accelerometer midpoint value (big-endian).
    /// Note: PS3 uses big-endian 16-bit values.
    pub const SIXAXIS_MID: i16 = 0x0200;

    /// Approximate scale factor for PS3 accelerometer.
    pub const ACCEL_SCALE: f32 = 113.0;

    /// Convert raw `DualShock` 3 accelerometer value to m/s².
    /// Note: `DualShock` 3 values are big-endian and require byte swapping.
    #[must_use]
    pub fn accel_to_ms2(raw: i16) -> f32 {
        const G: f32 = 9.81;
        let centered = raw - SIXAXIS_MID;
        (f32::from(centered) / ACCEL_SCALE) * G
    }

    /// Convert raw big-endian bytes to i16.
    #[must_use]
    pub fn be_bytes_to_i16(bytes: [u8; 2]) -> i16 {
        i16::from_be_bytes(bytes)
    }
}

/// Nintendo Switch Pro Controller motion sensor calibration.
pub mod switch_calibration {
    /// Switch gyroscope sensitivity (from official SDK docs).
    pub const GYRO_SENSITIVITY: f32 = 13371.0;
    /// Switch accelerometer sensitivity.
    pub const ACCEL_SENSITIVITY: f32 = 4096.0;

    /// Convert raw Switch gyroscope value to rad/s.
    #[must_use]
    pub fn gyro_to_rads(raw: i16) -> f32 {
        const DEG_TO_RAD: f32 = std::f32::consts::PI / 180.0;
        (f32::from(raw) / GYRO_SENSITIVITY) * DEG_TO_RAD
    }

    /// Convert raw Switch accelerometer value to m/s².
    #[must_use]
    pub fn accel_to_ms2(raw: i16) -> f32 {
        const G: f32 = 9.81;
        (f32::from(raw) / ACCEL_SENSITIVITY) * G
    }
}

// ========== Motion Data Structures ==========

/// Data returned by a motion backend.
#[derive(Debug, Clone, Copy, Default)]
pub struct MotionData {
    /// Gyroscope pitch (angular velocity around X axis) in rad/s.
    pub gyro_pitch: f32,
    /// Gyroscope yaw (angular velocity around Y axis) in rad/s.
    pub gyro_yaw: f32,
    /// Gyroscope roll (angular velocity around Z axis) in rad/s.
    pub gyro_roll: f32,
    /// Accelerometer X in m/s².
    pub accel_x: f32,
    /// Accelerometer Y in m/s².
    pub accel_y: f32,
    /// Accelerometer Z in m/s².
    pub accel_z: f32,
}

/// Trait for motion control backends.
///
/// Implement this trait to provide gyroscope and accelerometer data
/// from a specific controller type or platform.
pub trait MotionBackend {
    /// Poll for new motion data.
    ///
    /// Returns `Some(MotionData)` if new data is available, `None` otherwise.
    fn poll(&mut self) -> Option<MotionData>;

    /// Check if the backend is connected to a controller.
    fn is_connected(&self) -> bool;

    /// Get the name of this backend.
    fn name(&self) -> &'static str;
}

/// Touchpad finger data.
#[derive(Debug, Clone, Copy, Default)]
pub struct TouchpadFinger {
    /// Whether this finger is touching.
    pub active: bool,
    /// X position (0.0 - 1.0).
    pub x: f32,
    /// Y position (0.0 - 1.0).
    pub y: f32,
    /// Finger ID for tracking.
    pub id: u8,
}

/// Data returned by a touchpad backend.
#[derive(Debug, Clone, Copy, Default)]
pub struct TouchpadData {
    /// First finger.
    pub finger1: TouchpadFinger,
    /// Second finger (if multi-touch supported).
    pub finger2: TouchpadFinger,
    /// Whether the touchpad button is pressed.
    pub button_pressed: bool,
}

/// Trait for touchpad backends.
///
/// Implement this trait to provide touchpad data from a specific controller.
pub trait TouchpadBackend {
    /// Poll for new touchpad data.
    fn poll(&mut self) -> Option<TouchpadData>;

    /// Check if the backend supports multi-touch.
    fn supports_multitouch(&self) -> bool;

    /// Get the name of this backend.
    fn name(&self) -> &'static str;
}
