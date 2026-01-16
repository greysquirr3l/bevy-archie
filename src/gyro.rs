//! Gyroscope and accelerometer support.
//!
//! This module provides access to motion controls on modern gamepads
//! like PS4/PS5 DualShock/DualSense and Switch Pro Controller.

use bevy::prelude::*;

/// Gyroscope data from a gamepad.
#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
pub struct GyroData {
    /// Angular velocity around X axis (pitch) in rad/s.
    pub pitch: f32,
    /// Angular velocity around Y axis (yaw) in rad/s.
    pub yaw: f32,
    /// Angular velocity around Z axis (roll) in rad/s.
    pub roll: f32,
    /// Whether the data is valid/available.
    pub valid: bool,
}

impl GyroData {
    /// Create new gyro data.
    #[must_use]
    pub const fn new(pitch: f32, yaw: f32, roll: f32) -> Self {
        Self {
            pitch,
            yaw,
            roll,
            valid: true,
        }
    }

    /// Set raw gyro values from a platform-specific source.
    ///
    /// This marks the data as valid after setting values.
    pub fn set_raw(&mut self, pitch: f32, yaw: f32, roll: f32) {
        self.pitch = pitch;
        self.yaw = yaw;
        self.roll = roll;
        self.valid = true;
    }

    /// Get the magnitude of rotation.
    #[must_use]
    pub fn magnitude(&self) -> f32 {
        (self.pitch * self.pitch + self.yaw * self.yaw + self.roll * self.roll).sqrt()
    }

    /// Check if motion exceeds threshold.
    #[must_use]
    pub fn exceeds_threshold(&self, threshold: f32) -> bool {
        self.magnitude() > threshold
    }

    /// Update from a motion backend's data.
    ///
    /// This integrates with the `motion::backend::MotionData` type to receive
    /// motion input from platform-specific backends like `DualSense` HID.
    #[cfg(feature = "motion-backends")]
    pub fn update_from_backend(&mut self, data: &crate::motion::backend::MotionData) {
        self.set_raw(data.gyro_pitch, data.gyro_yaw, data.gyro_roll);
    }
}

/// Accelerometer data from a gamepad.
#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
pub struct AccelData {
    /// Acceleration in X direction (m/s²).
    pub x: f32,
    /// Acceleration in Y direction (m/s²).
    pub y: f32,
    /// Acceleration in Z direction (m/s²).
    pub z: f32,
    /// Whether the data is valid/available.
    pub valid: bool,
}

impl AccelData {
    /// Create new accelerometer data.
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            valid: true,
        }
    }

    /// Set raw accelerometer values from a platform-specific source.
    ///
    /// This marks the data as valid after setting values.
    pub fn set_raw(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.valid = true;
    }

    /// Get acceleration magnitude.
    #[must_use]
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Detect shake gesture.
    #[must_use]
    pub fn is_shaking(&self, threshold: f32) -> bool {
        // Subtract gravity (9.8 m/s²) and check if remaining acceleration is high
        let accel_without_gravity = self.magnitude() - 9.8;
        accel_without_gravity.abs() > threshold
    }

    /// Update from a motion backend's data.
    ///
    /// This integrates with the `motion::backend::MotionData` type to receive
    /// accelerometer input from platform-specific backends like `DualSense` HID.
    #[cfg(feature = "motion-backends")]
    pub fn update_from_backend(&mut self, data: &crate::motion::backend::MotionData) {
        self.set_raw(data.accel_x, data.accel_y, data.accel_z);
    }
}

/// Configuration for gyro/accel calibration.
#[derive(Debug, Clone, Resource)]
pub struct MotionConfig {
    /// Gyro sensitivity multiplier.
    pub gyro_sensitivity: f32,
    /// Gyro deadzone (rad/s).
    pub gyro_deadzone: f32,
    /// Accelerometer sensitivity.
    pub accel_sensitivity: f32,
    /// Whether motion controls are enabled.
    pub enabled: bool,
}

impl Default for MotionConfig {
    fn default() -> Self {
        Self {
            gyro_sensitivity: 1.0,
            gyro_deadzone: 0.01,
            accel_sensitivity: 1.0,
            enabled: true,
        }
    }
}

/// Gesture detection thresholds.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MotionGesture {
    /// Quick rotation.
    Flick,
    /// Sustained tilt.
    Tilt,
    /// Shake back and forth.
    Shake,
    /// Rotation around specific axis.
    Roll,
}

/// Event fired when a motion gesture is detected.
#[derive(Debug, Clone, Message)]
pub struct MotionGestureDetected {
    /// The gamepad that performed the gesture.
    pub gamepad: Entity,
    /// The detected gesture.
    pub gesture: MotionGesture,
    /// Intensity of the gesture.
    pub intensity: f32,
}

/// System to update gyro data.
///
/// # Platform Support
///
/// Bevy 0.17 does not expose gyroscope data from controllers directly.
/// To use real gyro data, you need to:
///
/// 1. Access gilrs raw events to get gyro data from SDL2-supported controllers
/// 2. Or inject data manually via `GyroData::set_raw()` from a platform-specific source
///
/// This system initializes the `GyroData` component on gamepads that don't have it.
/// To inject real gyro data, implement a custom system that writes to `GyroData`.
///
/// # Example Custom Integration
///
/// ```ignore
/// fn custom_gyro_system(mut gamepads: Query<&mut GyroData>) {
///     // Get raw gyro data from your platform-specific source
///     let (pitch, yaw, roll) = get_controller_gyro_data();
///     
///     for mut gyro in &mut gamepads {
///         gyro.set_raw(pitch, yaw, roll);
///     }
/// }
/// ```
pub fn update_gyro_data(
    mut gamepads: Query<(Entity, &Gamepad, Option<&mut GyroData>)>,
    mut commands: Commands,
) {
    for (entity, _gamepad, gyro) in &mut gamepads {
        if gyro.is_none() {
            commands.entity(entity).insert(GyroData::default());
        }
        // Real gyro data must be injected by a platform-specific system.
        // The gesture detection systems will work once valid data is provided.
    }
}

/// System to update accelerometer data.
///
/// # Platform Support
///
/// Similar to gyro data, Bevy 0.17 does not expose accelerometer data directly.
/// Inject real data via `AccelData::set_raw()` from a platform-specific source.
pub fn update_accel_data(
    mut gamepads: Query<(Entity, &Gamepad, Option<&mut AccelData>)>,
    mut commands: Commands,
) {
    for (entity, _gamepad, accel) in &mut gamepads {
        if accel.is_none() {
            commands.entity(entity).insert(AccelData::default());
        }
        // Real accelerometer data must be injected by a platform-specific system.
    }
}

/// System to detect motion gestures.
pub fn detect_motion_gestures(
    gamepads: Query<(Entity, &GyroData, &AccelData)>,
    config: Res<MotionConfig>,
    mut gesture_events: MessageWriter<MotionGestureDetected>,
) {
    if !config.enabled {
        return;
    }

    for (entity, gyro, accel) in gamepads.iter() {
        if !gyro.valid || !accel.valid {
            continue;
        }

        // Detect flick (quick rotation)
        if gyro.magnitude() > 5.0 {
            gesture_events.write(MotionGestureDetected {
                gamepad: entity,
                gesture: MotionGesture::Flick,
                intensity: gyro.magnitude(),
            });
        }

        // Detect shake
        if accel.is_shaking(3.0) {
            gesture_events.write(MotionGestureDetected {
                gamepad: entity,
                gesture: MotionGesture::Shake,
                intensity: accel.magnitude(),
            });
        }
    }
}

/// Plugin for registering gyro types.
pub(crate) fn register_gyro_types(app: &mut App) {
    app.register_type::<GyroData>()
        .register_type::<AccelData>()
        .init_resource::<MotionConfig>()
        .add_message::<MotionGestureDetected>();
}

/// Add gyro systems to the app.
pub(crate) fn add_gyro_systems(app: &mut App) {
    app.add_systems(
        Update,
        (update_gyro_data, update_accel_data, detect_motion_gestures).chain(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_gyro_data_new() {
        let gyro = GyroData::new(1.0, 2.0, 3.0);
        assert_relative_eq!(gyro.pitch, 1.0);
        assert_relative_eq!(gyro.yaw, 2.0);
        assert_relative_eq!(gyro.roll, 3.0);
        assert!(gyro.valid);
    }

    #[test]
    fn test_gyro_data_magnitude() {
        let gyro = GyroData::new(3.0, 4.0, 0.0);
        assert_relative_eq!(gyro.magnitude(), 5.0);
    }

    #[test]
    fn test_gyro_data_exceeds_threshold() {
        let gyro = GyroData::new(3.0, 4.0, 0.0);
        assert!(gyro.exceeds_threshold(4.0));
        assert!(!gyro.exceeds_threshold(6.0));
    }

    #[test]
    fn test_gyro_data_default() {
        let gyro = GyroData::default();
        assert_relative_eq!(gyro.pitch, 0.0);
        assert_relative_eq!(gyro.yaw, 0.0);
        assert_relative_eq!(gyro.roll, 0.0);
        assert!(!gyro.valid);
    }

    #[test]
    fn test_accel_data_new() {
        let accel = AccelData::new(1.0, 2.0, 3.0);
        assert_relative_eq!(accel.x, 1.0);
        assert_relative_eq!(accel.y, 2.0);
        assert_relative_eq!(accel.z, 3.0);
        assert!(accel.valid);
    }

    #[test]
    fn test_accel_data_magnitude() {
        let accel = AccelData::new(3.0, 4.0, 0.0);
        assert_relative_eq!(accel.magnitude(), 5.0);
    }

    #[test]
    fn test_accel_data_is_shaking() {
        let accel = AccelData::new(0.0, 20.0, 0.0); // High acceleration
        assert!(accel.is_shaking(5.0));

        let still = AccelData::new(0.0, 9.8, 0.0); // Just gravity
        assert!(!still.is_shaking(5.0));
    }

    #[test]
    fn test_accel_data_default() {
        let accel = AccelData::default();
        assert_relative_eq!(accel.x, 0.0);
        assert_relative_eq!(accel.y, 0.0);
        assert_relative_eq!(accel.z, 0.0);
        assert!(!accel.valid);
    }

    #[test]
    fn test_motion_gesture_variants() {
        assert_ne!(MotionGesture::Shake, MotionGesture::Tilt);
        assert_ne!(MotionGesture::Roll, MotionGesture::Flick);
    }

    #[test]
    fn test_motion_config_default() {
        let config = MotionConfig::default();
        assert!(config.gyro_sensitivity > 0.0);
        assert!(config.accel_sensitivity > 0.0);
        assert!(config.gyro_deadzone > 0.0);
        assert!(config.enabled);
    }

    #[test]
    fn test_motion_gesture_detected_event() {
        let gamepad = Entity::from_bits(77);
        let event = MotionGestureDetected {
            gamepad,
            gesture: MotionGesture::Shake,
            intensity: 0.8,
        };

        assert_eq!(event.gamepad, gamepad);
        assert_eq!(event.gesture, MotionGesture::Shake);
        assert_relative_eq!(event.intensity, 0.8);
    }

    // ========== Additional Tests ==========

    #[test]
    fn test_gyro_data_zero_magnitude() {
        let gyro = GyroData::new(0.0, 0.0, 0.0);
        assert_relative_eq!(gyro.magnitude(), 0.0);
    }

    #[test]
    fn test_gyro_data_invalid() {
        let mut gyro = GyroData::new(1.0, 2.0, 3.0);
        gyro.valid = false;
        assert!(!gyro.valid);
    }

    #[test]
    fn test_accel_data_zero_magnitude() {
        let accel = AccelData::new(0.0, 0.0, 0.0);
        assert_relative_eq!(accel.magnitude(), 0.0);
    }

    #[test]
    fn test_accel_data_invalid() {
        let mut accel = AccelData::new(1.0, 2.0, 3.0);
        accel.valid = false;
        assert!(!accel.valid);
    }

    #[test]
    fn test_motion_gesture_all_variants() {
        let gestures = [
            MotionGesture::Flick,
            MotionGesture::Tilt,
            MotionGesture::Shake,
            MotionGesture::Roll,
        ];

        // Check all are unique
        for (i, &g1) in gestures.iter().enumerate() {
            for (j, &g2) in gestures.iter().enumerate() {
                if i != j {
                    assert_ne!(g1, g2);
                }
            }
        }
    }

    #[test]
    fn test_motion_config_custom_values() {
        let config = MotionConfig {
            gyro_sensitivity: 2.0,
            gyro_deadzone: 0.05,
            accel_sensitivity: 1.5,
            enabled: false,
        };

        assert_relative_eq!(config.gyro_sensitivity, 2.0);
        assert_relative_eq!(config.gyro_deadzone, 0.05);
        assert_relative_eq!(config.accel_sensitivity, 1.5);
        assert!(!config.enabled);
    }

    #[test]
    fn test_gyro_data_different_thresholds() {
        let gyro = GyroData::new(5.0, 0.0, 0.0);
        assert_relative_eq!(gyro.magnitude(), 5.0);

        assert!(gyro.exceeds_threshold(4.0));
        assert!(!gyro.exceeds_threshold(5.0)); // Equal to threshold should not exceed
        let accel = AccelData::new(0.0, 9.8, 0.0);
        assert_relative_eq!(accel.magnitude(), 9.8, epsilon = 0.1);
        assert!(!accel.is_shaking(1.0));
    }

    #[test]
    fn test_accel_data_strong_shake() {
        // Strong acceleration in addition to gravity
        let accel = AccelData::new(10.0, 15.0, 5.0);
        assert!(accel.is_shaking(5.0));
    }

    #[test]
    fn test_motion_gesture_detected_different_gestures() {
        let gamepad = Entity::from_bits(1);

        let flick = MotionGestureDetected {
            gamepad,
            gesture: MotionGesture::Flick,
            intensity: 1.0,
        };

        let shake = MotionGestureDetected {
            gamepad,
            gesture: MotionGesture::Shake,
            intensity: 0.5,
        };

        assert_ne!(flick.gesture, shake.gesture);
    }
}
