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

/// System to update gyro data (placeholder - needs platform-specific implementation).
pub fn update_gyro_data(
    mut gamepads: Query<(Entity, &Gamepad, Option<&mut GyroData>)>,
    mut commands: Commands,
) {
    for (entity, _gamepad, gyro) in &mut gamepads {
        // Note: Bevy 0.17 doesn't have built-in gyro support
        // This would need platform-specific implementation via SDL2 or gilrs
        // For now, add the component if missing
        if gyro.is_none() {
            commands.entity(entity).insert(GyroData::default());
        }
    }
}

/// System to update accelerometer data (placeholder).
pub fn update_accel_data(
    mut gamepads: Query<(Entity, &Gamepad, Option<&mut AccelData>)>,
    mut commands: Commands,
) {
    for (entity, _gamepad, accel) in &mut gamepads {
        if accel.is_none() {
            commands.entity(entity).insert(AccelData::default());
        }
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
