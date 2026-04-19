//! PS5 `DualSense` motion backend using `dualsense-rs`.
//!
//! This backend provides gyroscope, accelerometer, and touchpad data
//! from `PlayStation` 5 `DualSense` controllers via USB HID.
//!
//! # Requirements
//!
//! - Enable the `dualsense` feature in Cargo.toml
//! - Controller must be connected via USB (Bluetooth support varies)
//!
//! # Architecture
//!
//! The `dualsense-rs` crate uses a callback-based API that requires `'static`
//! closures. This module provides a wrapper that bridges this to bevy-archie's
//! polling-based backend interface.
//!
//! # Note
//!
//! Due to the callback-based nature of `dualsense-rs` and its requirement for
//! `'static` closures, a full implementation requires either:
//! - Using `std::sync::LazyLock` (stable in Rust 1.80+) for global state
//! - Using `Box::leak` to create static references
//!
//! This is a reference implementation showing the intended architecture.
//! See [`DualSenseBackend::new`] documentation for the implementation pattern.

use std::sync::{Arc, Mutex};

use super::backend::{MotionBackend, MotionData, TouchpadBackend, TouchpadData, TouchpadFinger};

/// Motion data received from `DualSense`, protected for thread-safe access.
#[derive(Debug, Default)]
struct DualSenseState {
    gyro_pitch: f32,
    gyro_yaw: f32,
    gyro_roll: f32,
    accel_x: f32,
    accel_y: f32,
    accel_z: f32,
    touch1_active: bool,
    touch1_x: f32,
    touch1_y: f32,
    touch1_id: u8,
    touch2_active: bool,
    touch2_x: f32,
    touch2_y: f32,
    touch2_id: u8,
    connected: bool,
}

/// PS5 `DualSense` motion backend.
///
/// This backend communicates with a `DualSense` controller via HID
/// and provides motion sensor and touchpad data.
///
/// # Implementation Notes
///
/// The `dualsense-rs` crate requires `'static` callbacks for its event handlers.
/// This implementation uses `Arc<Mutex<>>` for shared state between the callback
/// thread and the Bevy game thread.
///
/// Available callback methods in `dualsense-rs`:
/// - `on_gyro_x_changed`, `on_gyro_y_changed`, `on_gyro_z_changed` - Gyroscope (i16)
/// - `on_accel_x_changed`, `on_accel_y_changed`, `on_accel_z_changed` - Accelerometer (i16)
/// - `on_touchpad1_x_changed`, `on_touchpad1_y_changed` - Touch point 1 position (u16)
/// - `on_touchpoint2_x_changed`, `on_touchpoint2_y_changed` - Touch point 2 position (u16)
/// - `on_touchpad1_pressed`, `on_touchpoint2_changed` - Touch active state (bool)
/// - `on_touchpoint1_id_changed`, `on_touchpoint2_id_changed` - Touch ID (u8)
pub struct DualSenseBackend {
    state: Arc<Mutex<DualSenseState>>,
}

impl DualSenseBackend {
    /// Try to create a new `DualSense` backend.
    ///
    /// Returns `None` if no `DualSense` controller is connected or the feature is disabled.
    ///
    /// # Feature Requirements
    ///
    /// Requires the `dualsense` feature to be enabled:
    /// ```toml
    /// [dependencies]
    /// bevy_archie = { version = "0.1", features = ["dualsense"] }
    /// ```
    ///
    /// # Implementation Pattern
    ///
    /// A full implementation using `once_cell` would look like:
    /// ```ignore
    /// use dualsense_rs::DualSense;
    /// use once_cell::sync::Lazy;
    /// use std::sync::{Arc, Mutex};
    ///
    /// // Global state accessible from 'static callbacks
    /// static DUALSENSE_STATE: Lazy<Arc<Mutex<DualSenseState>>> =
    ///     Lazy::new(|| Arc::new(Mutex::new(DualSenseState::default())));
    ///
    /// pub fn new() -> Option<Self> {
    ///     let mut controller = DualSense::default();
    ///     
    ///     // Static closures can reference global state
    ///     controller.on_gyro_x_changed(&|val| {
    ///         if let Ok(mut s) = DUALSENSE_STATE.lock() {
    ///             s.gyro_pitch = (val as f32) / 1000.0;
    ///         }
    ///     });
    ///     controller.on_gyro_y_changed(&|val| {
    ///         if let Ok(mut s) = DUALSENSE_STATE.lock() {
    ///             s.gyro_yaw = (val as f32) / 1000.0;
    ///         }
    ///     });
    ///     controller.on_gyro_z_changed(&|val| {
    ///         if let Ok(mut s) = DUALSENSE_STATE.lock() {
    ///             s.gyro_roll = (val as f32) / 1000.0;
    ///         }
    ///     });
    ///     // ... set up accel and touchpad callbacks similarly ...
    ///     
    ///     let _handle = controller.run();
    ///     if let Ok(mut s) = DUALSENSE_STATE.lock() {
    ///         s.connected = true;
    ///     }
    ///     Some(Self { state: DUALSENSE_STATE.clone() })
    /// }
    /// ```
    ///
    /// # Current Status
    ///
    /// This placeholder returns `None` and logs a warning. To use `DualSense`
    /// motion controls, implement the pattern above using `once_cell` or
    /// `lazy_static` for global state management.
    #[cfg(feature = "dualsense")]
    #[must_use]
    pub fn new() -> Option<Self> {
        // Full implementation requires 'static callbacks with global state.
        // See documentation above for the implementation pattern.
        log::warn!(
            "DualSense backend requires global state for 'static callbacks. \
             See DualSenseBackend::new() docs for implementation pattern using once_cell."
        );
        None
    }

    /// Create a backend without the dualsense feature (always returns None).
    #[cfg(not(feature = "dualsense"))]
    #[must_use]
    pub fn new() -> Option<Self> {
        None
    }
}

impl MotionBackend for DualSenseBackend {
    fn poll(&mut self) -> Option<MotionData> {
        self.state.lock().ok().map(|s| MotionData {
            gyro_pitch: s.gyro_pitch,
            gyro_yaw: s.gyro_yaw,
            gyro_roll: s.gyro_roll,
            accel_x: s.accel_x,
            accel_y: s.accel_y,
            accel_z: s.accel_z,
        })
    }

    fn is_connected(&self) -> bool {
        self.state.lock().map(|s| s.connected).unwrap_or(false)
    }

    fn name(&self) -> &'static str {
        "dualsense"
    }
}

impl TouchpadBackend for DualSenseBackend {
    fn poll(&mut self) -> Option<TouchpadData> {
        self.state.lock().ok().map(|s| TouchpadData {
            finger1: TouchpadFinger {
                active: s.touch1_active,
                x: s.touch1_x,
                y: s.touch1_y,
                id: s.touch1_id,
            },
            finger2: TouchpadFinger {
                active: s.touch2_active,
                x: s.touch2_x,
                y: s.touch2_y,
                id: s.touch2_id,
            },
            button_pressed: false,
        })
    }

    fn supports_multitouch(&self) -> bool {
        true
    }

    fn name(&self) -> &'static str {
        "dualsense"
    }
}
