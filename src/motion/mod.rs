//! Motion control backends for gyroscope, accelerometer, and touchpad data.
//!
//! This module provides a backend-agnostic interface for motion controls.
//! Different controller types require different libraries to access motion data.
//!
//! # Architecture
//!
//! The motion system uses a trait-based design with optional backend implementations:
//!
//! - [`MotionBackend`] - Trait defining how to read gyro/accel/touchpad data
//! - [`StubBackend`] - No-op fallback (always available)
//! - [`DualSenseBackend`] - PS5 `DualSense` via `dualsense-rs` (feature: `dualsense`)
//!
//! # Feature Flags
//!
//! Enable platform-specific backends via Cargo features:
//!
//! ```toml
//! [dependencies]
//! bevy_archie = { version = "0.1", features = ["dualsense"] }
//! ```
//!
//! # Usage
//!
//! ```ignore
//! use bevy_archie::motion::{MotionBackend, MotionData};
//!
//! // The library will use the best available backend
//! // Or you can inject your own data:
//! fn custom_motion_system(mut gyro_query: Query<&mut GyroData>) {
//!     for mut gyro in &mut gyro_query {
//!         gyro.set_raw(pitch, yaw, roll);
//!     }
//! }
//! ```

pub mod backend;
mod stub;

#[cfg(feature = "dualsense")]
mod dualsense;

pub use backend::{
    MotionBackend, MotionData, TouchpadBackend, TouchpadData as BackendTouchpadData,
};
pub use stub::StubBackend;

#[cfg(feature = "dualsense")]
pub use dualsense::DualSenseBackend;

use bevy::prelude::*;

/// Resource holding the active motion backend.
#[derive(Resource)]
pub struct ActiveMotionBackend {
    backend: Box<dyn MotionBackend + Send + Sync>,
}

impl Default for ActiveMotionBackend {
    fn default() -> Self {
        Self {
            backend: Box::new(StubBackend::new()),
        }
    }
}

impl ActiveMotionBackend {
    /// Create with a specific backend.
    pub fn new<B: MotionBackend + Send + Sync + 'static>(backend: B) -> Self {
        Self {
            backend: Box::new(backend),
        }
    }

    /// Get motion data from the backend.
    pub fn poll(&mut self) -> Option<MotionData> {
        self.backend.poll()
    }

    /// Check if the backend is connected.
    pub fn is_connected(&self) -> bool {
        self.backend.is_connected()
    }
}

/// Resource holding the active touchpad backend.
#[derive(Resource)]
pub struct ActiveTouchpadBackend {
    backend: Box<dyn TouchpadBackend + Send + Sync>,
}

impl Default for ActiveTouchpadBackend {
    fn default() -> Self {
        Self {
            backend: Box::new(StubBackend::new()),
        }
    }
}

impl ActiveTouchpadBackend {
    /// Create with a specific backend.
    pub fn new<B: TouchpadBackend + Send + Sync + 'static>(backend: B) -> Self {
        Self {
            backend: Box::new(backend),
        }
    }

    /// Get touchpad data from the backend.
    pub fn poll(&mut self) -> Option<BackendTouchpadData> {
        self.backend.poll()
    }
}

/// System to update gyro/accel data from the active backend.
pub fn update_motion_from_backend(
    mut backend: ResMut<ActiveMotionBackend>,
    mut gyro_query: Query<&mut crate::gyro::GyroData>,
    mut accel_query: Query<&mut crate::gyro::AccelData>,
) {
    if let Some(data) = backend.poll() {
        // Update all gyro components with the backend data
        for mut gyro in &mut gyro_query {
            gyro.set_raw(data.gyro_pitch, data.gyro_yaw, data.gyro_roll);
        }

        // Update all accel components with the backend data
        for mut accel in &mut accel_query {
            accel.set_raw(data.accel_x, data.accel_y, data.accel_z);
        }
    }
}

/// System to update touchpad data from the active backend.
pub fn update_touchpad_from_backend(
    mut backend: ResMut<ActiveTouchpadBackend>,
    mut touchpad_query: Query<&mut crate::touchpad::TouchpadData>,
) {
    if let Some(data) = backend.poll() {
        for mut touchpad in &mut touchpad_query {
            // Update finger 1
            touchpad.set_finger(0, data.finger1.x, data.finger1.y, data.finger1.active);
            // Update finger 2
            touchpad.set_finger(1, data.finger2.x, data.finger2.y, data.finger2.active);
            // Button state must be set directly
            touchpad.button_pressed = data.button_pressed;
        }
    }
}

/// Register motion backend resources and systems.
#[allow(dead_code)] // Called from plugin when motion-backends feature is enabled
pub(crate) fn register_motion_backend(app: &mut App) {
    app.init_resource::<ActiveMotionBackend>()
        .init_resource::<ActiveTouchpadBackend>();
}

/// Add motion backend systems.
#[allow(dead_code)] // Called from plugin when motion-backends feature is enabled
pub(crate) fn add_motion_backend_systems(app: &mut App) {
    app.add_systems(
        Update,
        (update_motion_from_backend, update_touchpad_from_backend),
    );
}
