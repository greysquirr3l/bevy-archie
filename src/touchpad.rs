//! `PlayStation` controller touchpad support.
//!
//! This module provides touchpad input for PS4 `DualShock` 4 and PS5 `DualSense` controllers.

use bevy::prelude::*;

// ========== Touchpad Hardware Specifications ==========
// Sourced from PS4/PS5 HID descriptors and verified against Joypad OS implementation
// Reference: https://github.com/joypad-ai/joypad-os/blob/main/src/usb/usbd/descriptors/ps4_descriptors.h

/// `DualShock` 4 (PS4) touchpad resolution.
pub mod dualshock4_touchpad {
    /// Touchpad width in native resolution.
    pub const WIDTH: u16 = 1920;
    /// Touchpad height in native resolution.
    pub const HEIGHT: u16 = 943;
}

/// `DualSense` (PS5) touchpad resolution.
pub mod dualsense_touchpad {
    /// Touchpad width in native resolution (same as DS4).
    pub const WIDTH: u16 = 1920;
    /// Touchpad height in native resolution (taller than DS4).
    pub const HEIGHT: u16 = 1080;
}

/// Steam Controller touchpad resolution.
pub mod steam_touchpad {
    /// Steam touchpad is circular, but reported as rectangular.
    /// Width in native resolution.
    pub const WIDTH: u16 = 32767;
    /// Height in native resolution.
    pub const HEIGHT: u16 = 32767;
}

/// Normalize raw touchpad coordinates to 0.0-1.0 range.
///
/// # Arguments
/// * `x` - Raw X coordinate from hardware
/// * `y` - Raw Y coordinate from hardware  
/// * `max_x` - Maximum X value for this touchpad
/// * `max_y` - Maximum Y value for this touchpad
///
/// # Returns
/// Normalized (x, y) coordinates in 0.0-1.0 range.
#[must_use]
pub fn normalize_coords(x: u16, y: u16, max_x: u16, max_y: u16) -> (f32, f32) {
    (
        f32::from(x) / f32::from(max_x),
        f32::from(y) / f32::from(max_y),
    )
}

/// Convert normalized coordinates back to raw values for a specific touchpad.
#[must_use]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "intentional conversion to u16 touchpad coordinates"
)]
pub fn denormalize_coords(x: f32, y: f32, max_x: u16, max_y: u16) -> (u16, u16) {
    ((x * f32::from(max_x)) as u16, (y * f32::from(max_y)) as u16)
}

/// Touchpad finger data.
#[derive(Debug, Clone, Copy, Default, Reflect)]
pub struct TouchFinger {
    /// Normalized X position (0.0-1.0).
    pub x: f32,
    /// Normalized Y position (0.0-1.0).
    pub y: f32,
    /// Whether this finger is currently touching.
    pub active: bool,
    /// Unique finger ID.
    pub id: u8,
}

impl TouchFinger {
    /// Create a new touch finger.
    #[must_use]
    pub fn new(id: u8, x: f32, y: f32) -> Self {
        Self {
            x: x.clamp(0.0, 1.0),
            y: y.clamp(0.0, 1.0),
            active: true,
            id,
        }
    }

    /// Get position as Vec2.
    #[must_use]
    pub fn position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

/// Touchpad state for a gamepad.
#[derive(Debug, Clone, Component, Reflect)]
pub struct TouchpadData {
    /// First finger/touch point.
    pub finger1: TouchFinger,
    /// Second finger/touch point (for multi-touch).
    pub finger2: TouchFinger,
    /// Whether the touchpad button is pressed.
    pub button_pressed: bool,
    /// Previous frame's first finger position.
    pub prev_finger1: Vec2,
    /// Previous frame's second finger position.
    pub prev_finger2: Vec2,
}

impl Default for TouchpadData {
    fn default() -> Self {
        Self {
            finger1: TouchFinger::default(),
            finger2: TouchFinger::default(),
            button_pressed: false,
            prev_finger1: Vec2::ZERO,
            prev_finger2: Vec2::ZERO,
        }
    }
}

impl TouchpadData {
    /// Get the delta movement of finger 1.
    #[must_use]
    pub fn finger1_delta(&self) -> Vec2 {
        if !self.finger1.active {
            return Vec2::ZERO;
        }
        self.finger1.position() - self.prev_finger1
    }

    /// Get the delta movement of finger 2.
    #[must_use]
    pub fn finger2_delta(&self) -> Vec2 {
        if !self.finger2.active {
            return Vec2::ZERO;
        }
        self.finger2.position() - self.prev_finger2
    }

    /// Check if a swipe gesture is detected.
    #[must_use]
    pub fn is_swiping(&self, threshold: f32) -> bool {
        self.finger1_delta().length() > threshold
    }

    /// Check if a pinch gesture is detected (two fingers moving apart/together).
    #[must_use]
    pub fn is_pinching(&self) -> Option<f32> {
        if !self.finger1.active || !self.finger2.active {
            return None;
        }

        let current_dist = self.finger1.position().distance(self.finger2.position());
        let prev_dist = self.prev_finger1.distance(self.prev_finger2);
        let delta = current_dist - prev_dist;

        if delta.abs() > 0.01 {
            Some(delta) // Positive = pinch out, negative = pinch in
        } else {
            None
        }
    }

    /// Get the number of active fingers.
    #[must_use]
    pub fn active_fingers(&self) -> u8 {
        let mut count = 0;
        if self.finger1.active {
            count += 1;
        }
        if self.finger2.active {
            count += 1;
        }
        count
    }

    /// Set finger data from a platform-specific source.
    ///
    /// # Arguments
    /// * `finger_index` - 0 for finger1, 1 for finger2
    /// * `x` - Normalized X position (0.0-1.0)
    /// * `y` - Normalized Y position (0.0-1.0)
    /// * `active` - Whether the finger is touching
    pub fn set_finger(&mut self, finger_index: usize, x: f32, y: f32, active: bool) {
        let finger = if finger_index == 0 {
            &mut self.finger1
        } else {
            &mut self.finger2
        };
        finger.x = x.clamp(0.0, 1.0);
        finger.y = y.clamp(0.0, 1.0);
        finger.active = active;
    }

    /// Set finger data from raw hardware coordinates.
    ///
    /// This automatically normalizes the coordinates based on the touchpad type.
    ///
    /// # Arguments
    /// * `finger_index` - 0 for finger1, 1 for finger2
    /// * `raw_x` - Raw X coordinate from hardware
    /// * `raw_y` - Raw Y coordinate from hardware
    /// * `max_x` - Maximum X value for this touchpad (e.g., `dualshock4_touchpad::WIDTH`)
    /// * `max_y` - Maximum Y value for this touchpad (e.g., `dualshock4_touchpad::HEIGHT`)
    /// * `active` - Whether the finger is touching
    pub fn set_finger_raw(
        &mut self,
        finger_index: usize,
        raw_x: u16,
        raw_y: u16,
        max_x: u16,
        max_y: u16,
        active: bool,
    ) {
        let (x, y) = normalize_coords(raw_x, raw_y, max_x, max_y);
        self.set_finger(finger_index, x, y, active);
    }

    /// Update frame state - call this at the end of your custom system.
    ///
    /// This saves current finger positions to prev_ fields for delta calculation.
    pub fn update_frame(&mut self) {
        self.prev_finger1 = self.finger1.position();
        self.prev_finger2 = self.finger2.position();
    }

    /// Update from a motion backend's touchpad data.
    ///
    /// This integrates with the `motion::backend::TouchpadData` type to receive
    /// touchpad input from platform-specific backends like `DualSense` HID.
    ///
    /// # Example
    ///
    /// ```ignore
    /// if let Some(backend_data) = touchpad_backend.poll() {
    ///     touchpad.update_from_backend(&backend_data);
    /// }
    /// ```
    #[cfg(feature = "motion-backends")]
    pub fn update_from_backend(&mut self, data: &crate::motion::backend::TouchpadData) {
        // Update finger 1
        self.finger1.x = data.finger1.x.clamp(0.0, 1.0);
        self.finger1.y = data.finger1.y.clamp(0.0, 1.0);
        self.finger1.active = data.finger1.active;
        self.finger1.id = data.finger1.id;

        // Update finger 2
        self.finger2.x = data.finger2.x.clamp(0.0, 1.0);
        self.finger2.y = data.finger2.y.clamp(0.0, 1.0);
        self.finger2.active = data.finger2.active;
        self.finger2.id = data.finger2.id;

        // Update button state
        self.button_pressed = data.button_pressed;
    }
}

/// Touchpad gesture detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum TouchpadGesture {
    /// Single finger tap.
    Tap,
    /// Two finger tap.
    TwoFingerTap,
    /// Swipe left.
    SwipeLeft,
    /// Swipe right.
    SwipeRight,
    /// Swipe up.
    SwipeUp,
    /// Swipe down.
    SwipeDown,
    /// Pinch in (zoom out).
    PinchIn,
    /// Pinch out (zoom in).
    PinchOut,
}

/// Event fired when a touchpad gesture is detected.
#[derive(Debug, Clone, Message)]
pub struct TouchpadGestureEvent {
    /// The gamepad that performed the gesture.
    pub gamepad: Entity,
    /// The detected gesture.
    pub gesture: TouchpadGesture,
    /// Position where gesture occurred (if applicable).
    pub position: Vec2,
    /// Intensity/magnitude of gesture.
    pub intensity: f32,
}

/// Configuration for touchpad sensitivity and gestures.
#[derive(Debug, Clone, Resource)]
pub struct TouchpadConfig {
    /// Swipe detection threshold.
    pub swipe_threshold: f32,
    /// Tap detection time window.
    pub tap_time_window: f32,
    /// Whether touchpad is enabled.
    pub enabled: bool,
}

impl Default for TouchpadConfig {
    fn default() -> Self {
        Self {
            swipe_threshold: 0.15,
            tap_time_window: 0.2,
            enabled: true,
        }
    }
}

/// System to update touchpad data.
///
/// # Platform Support
///
/// Bevy 0.17 does not expose touchpad data from controllers directly.
/// This is primarily available on PS4/PS5 DualShock/DualSense controllers.
///
/// To use real touchpad data, you need to:
///
/// 1. Access gilrs raw events to get touchpad data from SDL2-supported controllers
/// 2. Or inject data manually via `TouchpadData::set_finger()` from a platform-specific source
///
/// This system initializes the `TouchpadData` component on gamepads that don't have it.
///
/// # Example Custom Integration
///
/// ```ignore
/// fn custom_touchpad_system(mut gamepads: Query<&mut TouchpadData>) {
///     // Get raw touchpad data from your platform-specific source
///     let (f1_x, f1_y, f1_pressed) = get_touchpad_finger1();
///     let (f2_x, f2_y, f2_pressed) = get_touchpad_finger2();
///     
///     for mut touchpad in &mut gamepads {
///         touchpad.set_finger(0, f1_x, f1_y, f1_pressed);
///         touchpad.set_finger(1, f2_x, f2_y, f2_pressed);
///         touchpad.update_frame();
///     }
/// }
/// ```
pub fn update_touchpad_data(
    mut gamepads: Query<(Entity, &Gamepad, Option<&mut TouchpadData>)>,
    mut commands: Commands,
) {
    for (entity, _gamepad, touchpad) in &mut gamepads {
        if touchpad.is_none() {
            commands.entity(entity).insert(TouchpadData::default());
        }
        // Real touchpad data must be injected by a platform-specific system.
        // The gesture detection systems will work once valid data is provided.
    }
}

/// System to detect touchpad gestures.
pub fn detect_touchpad_gestures(
    mut gamepads: Query<(Entity, &mut TouchpadData)>,
    config: Res<TouchpadConfig>,
    mut gesture_events: MessageWriter<TouchpadGestureEvent>,
) {
    if !config.enabled {
        return;
    }

    for (entity, mut touchpad) in &mut gamepads {
        // Detect swipes
        let delta = touchpad.finger1_delta();
        if delta.length() > config.swipe_threshold {
            let gesture = if delta.x.abs() > delta.y.abs() {
                if delta.x > 0.0 {
                    TouchpadGesture::SwipeRight
                } else {
                    TouchpadGesture::SwipeLeft
                }
            } else if delta.y > 0.0 {
                TouchpadGesture::SwipeDown
            } else {
                TouchpadGesture::SwipeUp
            };

            gesture_events.write(TouchpadGestureEvent {
                gamepad: entity,
                gesture,
                position: touchpad.finger1.position(),
                intensity: delta.length(),
            });
        }

        // Detect pinch
        if let Some(pinch_delta) = touchpad.is_pinching() {
            let gesture = if pinch_delta > 0.0 {
                TouchpadGesture::PinchOut
            } else {
                TouchpadGesture::PinchIn
            };

            gesture_events.write(TouchpadGestureEvent {
                gamepad: entity,
                gesture,
                position: (touchpad.finger1.position() + touchpad.finger2.position()) / 2.0,
                intensity: pinch_delta.abs(),
            });
        }

        // Update previous positions
        touchpad.prev_finger1 = touchpad.finger1.position();
        touchpad.prev_finger2 = touchpad.finger2.position();
    }
}

/// Plugin for registering touchpad types.
pub(crate) fn register_touchpad_types(app: &mut App) {
    app.register_type::<TouchFinger>()
        .register_type::<TouchpadData>()
        .register_type::<TouchpadGesture>()
        .init_resource::<TouchpadConfig>()
        .add_message::<TouchpadGestureEvent>();
}

/// Add touchpad systems to the app.
pub(crate) fn add_touchpad_systems(app: &mut App) {
    app.add_systems(
        Update,
        (update_touchpad_data, detect_touchpad_gestures).chain(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    // ========== TouchFinger Tests ==========

    #[test]
    fn test_touch_finger_new() {
        let finger = TouchFinger::new(1, 0.5, 0.7);
        assert_eq!(finger.id, 1);
        assert_relative_eq!(finger.x, 0.5);
        assert_relative_eq!(finger.y, 0.7);
        assert!(finger.active);
    }

    #[test]
    fn test_touch_finger_new_clamps_values() {
        let finger = TouchFinger::new(0, -0.5, 1.5);
        assert_relative_eq!(finger.x, 0.0);
        assert_relative_eq!(finger.y, 1.0);
    }

    #[test]
    fn test_touch_finger_position() {
        let finger = TouchFinger::new(0, 0.3, 0.8);
        let pos = finger.position();
        assert_relative_eq!(pos.x, 0.3);
        assert_relative_eq!(pos.y, 0.8);
    }

    #[test]
    fn test_touch_finger_default() {
        let finger = TouchFinger::default();
        assert_eq!(finger.id, 0);
        assert_relative_eq!(finger.x, 0.0);
        assert_relative_eq!(finger.y, 0.0);
        assert!(!finger.active);
    }

    // ========== TouchpadData Tests ==========

    #[test]
    fn test_touchpad_data_default() {
        let data = TouchpadData::default();
        assert!(!data.finger1.active);
        assert!(!data.finger2.active);
        assert!(!data.button_pressed);
        assert_eq!(data.prev_finger1, Vec2::ZERO);
        assert_eq!(data.prev_finger2, Vec2::ZERO);
    }

    #[test]
    fn test_touchpad_data_finger1_delta_inactive() {
        let data = TouchpadData::default();
        assert_eq!(data.finger1_delta(), Vec2::ZERO);
    }

    #[test]
    fn test_touchpad_data_finger1_delta_active() {
        let mut data = TouchpadData::default();
        data.finger1 = TouchFinger::new(0, 0.5, 0.5);
        data.prev_finger1 = Vec2::new(0.3, 0.3);

        let delta = data.finger1_delta();
        assert_relative_eq!(delta.x, 0.2, epsilon = 0.001);
        assert_relative_eq!(delta.y, 0.2, epsilon = 0.001);
    }

    #[test]
    fn test_touchpad_data_finger2_delta_inactive() {
        let data = TouchpadData::default();
        assert_eq!(data.finger2_delta(), Vec2::ZERO);
    }

    #[test]
    fn test_touchpad_data_finger2_delta_active() {
        let mut data = TouchpadData::default();
        data.finger2 = TouchFinger::new(1, 0.8, 0.6);
        data.prev_finger2 = Vec2::new(0.4, 0.2);

        let delta = data.finger2_delta();
        assert_relative_eq!(delta.x, 0.4, epsilon = 0.001);
        assert_relative_eq!(delta.y, 0.4, epsilon = 0.001);
    }

    #[test]
    fn test_touchpad_data_is_swiping_true() {
        let mut data = TouchpadData::default();
        data.finger1 = TouchFinger::new(0, 0.8, 0.5);
        data.prev_finger1 = Vec2::new(0.3, 0.5);

        assert!(data.is_swiping(0.1));
    }

    #[test]
    fn test_touchpad_data_is_swiping_false() {
        let mut data = TouchpadData::default();
        data.finger1 = TouchFinger::new(0, 0.35, 0.5);
        data.prev_finger1 = Vec2::new(0.3, 0.5);

        assert!(!data.is_swiping(0.1));
    }

    #[test]
    fn test_touchpad_data_is_pinching_none_when_single_finger() {
        let mut data = TouchpadData::default();
        data.finger1 = TouchFinger::new(0, 0.5, 0.5);
        // finger2 not active

        assert!(data.is_pinching().is_none());
    }

    #[test]
    fn test_touchpad_data_is_pinching_out() {
        let mut data = TouchpadData::default();
        data.finger1 = TouchFinger::new(0, 0.2, 0.5);
        data.finger2 = TouchFinger::new(1, 0.8, 0.5);
        data.prev_finger1 = Vec2::new(0.3, 0.5);
        data.prev_finger2 = Vec2::new(0.7, 0.5);

        let pinch = data.is_pinching();
        assert!(pinch.is_some());
        assert!(pinch.unwrap() > 0.0); // Pinch out
    }

    #[test]
    fn test_touchpad_data_is_pinching_in() {
        let mut data = TouchpadData::default();
        data.finger1 = TouchFinger::new(0, 0.4, 0.5);
        data.finger2 = TouchFinger::new(1, 0.6, 0.5);
        data.prev_finger1 = Vec2::new(0.2, 0.5);
        data.prev_finger2 = Vec2::new(0.8, 0.5);

        let pinch = data.is_pinching();
        assert!(pinch.is_some());
        assert!(pinch.unwrap() < 0.0); // Pinch in
    }

    #[test]
    fn test_touchpad_data_is_pinching_none_when_no_movement() {
        let mut data = TouchpadData::default();
        data.finger1 = TouchFinger::new(0, 0.3, 0.5);
        data.finger2 = TouchFinger::new(1, 0.7, 0.5);
        data.prev_finger1 = Vec2::new(0.3, 0.5);
        data.prev_finger2 = Vec2::new(0.7, 0.5);

        assert!(data.is_pinching().is_none());
    }

    #[test]
    fn test_touchpad_data_active_fingers_none() {
        let data = TouchpadData::default();
        assert_eq!(data.active_fingers(), 0);
    }

    #[test]
    fn test_touchpad_data_active_fingers_one() {
        let mut data = TouchpadData::default();
        data.finger1 = TouchFinger::new(0, 0.5, 0.5);
        assert_eq!(data.active_fingers(), 1);
    }

    #[test]
    fn test_touchpad_data_active_fingers_two() {
        let mut data = TouchpadData::default();
        data.finger1 = TouchFinger::new(0, 0.5, 0.5);
        data.finger2 = TouchFinger::new(1, 0.7, 0.7);
        assert_eq!(data.active_fingers(), 2);
    }

    // ========== TouchpadGesture Tests ==========

    #[test]
    fn test_touchpad_gesture_equality() {
        assert_eq!(TouchpadGesture::Tap, TouchpadGesture::Tap);
        assert_ne!(TouchpadGesture::Tap, TouchpadGesture::TwoFingerTap);
    }

    #[test]
    fn test_touchpad_gesture_variants() {
        let gestures = [
            TouchpadGesture::Tap,
            TouchpadGesture::TwoFingerTap,
            TouchpadGesture::SwipeLeft,
            TouchpadGesture::SwipeRight,
            TouchpadGesture::SwipeUp,
            TouchpadGesture::SwipeDown,
            TouchpadGesture::PinchIn,
            TouchpadGesture::PinchOut,
        ];
        assert_eq!(gestures.len(), 8);
    }

    // ========== TouchpadConfig Tests ==========

    #[test]
    fn test_touchpad_config_default() {
        let config = TouchpadConfig::default();
        assert_relative_eq!(config.swipe_threshold, 0.15);
        assert_relative_eq!(config.tap_time_window, 0.2);
        assert!(config.enabled);
    }

    #[test]
    fn test_touchpad_config_custom() {
        let config = TouchpadConfig {
            swipe_threshold: 0.25,
            tap_time_window: 0.3,
            enabled: false,
        };
        assert_relative_eq!(config.swipe_threshold, 0.25);
        assert_relative_eq!(config.tap_time_window, 0.3);
        assert!(!config.enabled);
    }

    // ========== TouchpadGestureEvent Tests ==========

    #[test]
    fn test_touchpad_gesture_event_creation() {
        let event = TouchpadGestureEvent {
            gamepad: Entity::PLACEHOLDER,
            gesture: TouchpadGesture::SwipeRight,
            position: Vec2::new(0.5, 0.5),
            intensity: 0.3,
        };
        assert_eq!(event.gesture, TouchpadGesture::SwipeRight);
        assert_relative_eq!(event.intensity, 0.3);
    }
}
