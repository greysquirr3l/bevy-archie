//! `PlayStation` controller touchpad support.
//!
//! This module provides touchpad input for PS4 `DualShock` 4 and PS5 `DualSense` controllers.

use bevy::prelude::*;

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

/// System to update touchpad data (placeholder - needs platform implementation).
pub fn update_touchpad_data(
    mut gamepads: Query<(Entity, &Gamepad, Option<&mut TouchpadData>)>,
    mut commands: Commands,
) {
    for (entity, _gamepad, touchpad) in &mut gamepads {
        // Note: Bevy 0.17 doesn't have built-in touchpad support
        // This would need platform-specific implementation via SDL2 or custom gamepad backend
        // For now, add the component if missing
        if touchpad.is_none() {
            commands.entity(entity).insert(TouchpadData::default());
        }
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
