//! Touch-screen virtual joystick for mobile and touchpad controls.
//!
//! This module provides a virtual joystick that can be rendered on screen
//! and controlled via touch input, perfect for mobile games or touchpad controls.
//!
//! # Example
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use bevy_archie::touch_joystick::{TouchJoystick, TouchJoystickPlugin, TouchJoystickSettings};
//!
//! App::new()
//!     .add_plugins(DefaultPlugins)
//!     .add_plugins(TouchJoystickPlugin)
//!     .add_systems(Startup, setup)
//!     .run();
//!
//! fn setup(mut commands: Commands) {
//!     commands.spawn(TouchJoystick::left());
//! }
//! ```

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Plugin for touch-screen virtual joystick functionality.
pub struct TouchJoystickPlugin;

impl Plugin for TouchJoystickPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TouchJoystick>()
            .register_type::<TouchJoystickSettings>()
            .init_resource::<TouchJoystickSettings>()
            .add_message::<TouchJoystickEvent>()
            .add_systems(
                Update,
                (update_touch_joysticks, emit_joystick_events).chain_ignore_deferred(),
            );
    }
}

/// A virtual joystick component for touch input.
#[derive(Component, Debug, Clone, Reflect)]
pub struct TouchJoystick {
    /// Base position of the joystick (center of the base)
    pub base_position: Vec2,
    /// Current position of the knob relative to base
    pub knob_offset: Vec2,
    /// Maximum distance the knob can travel from the base
    pub radius: f32,
    /// Deadzone as a percentage of radius (0.0 to 1.0)
    pub deadzone: f32,
    /// Whether this joystick is currently being touched
    pub active: bool,
    /// The touch ID currently controlling this joystick
    pub touch_id: Option<u64>,
    /// Which side of the screen this joystick is on
    pub side: JoystickSide,
    /// Whether the base follows the initial touch
    pub floating: bool,
    /// Whether to snap back to center on release
    pub snap_to_center: bool,
}

impl Default for TouchJoystick {
    fn default() -> Self {
        Self {
            base_position: Vec2::ZERO,
            knob_offset: Vec2::ZERO,
            radius: 100.0,
            deadzone: 0.1,
            active: false,
            touch_id: None,
            side: JoystickSide::Left,
            floating: true,
            snap_to_center: true,
        }
    }
}

impl TouchJoystick {
    /// Create a left-side joystick (common for movement).
    #[must_use]
    pub fn left() -> Self {
        Self {
            side: JoystickSide::Left,
            ..Default::default()
        }
    }

    /// Create a right-side joystick (common for camera/aiming).
    #[must_use]
    pub fn right() -> Self {
        Self {
            side: JoystickSide::Right,
            ..Default::default()
        }
    }

    /// Create a joystick at a fixed position.
    #[must_use]
    pub fn fixed(position: Vec2) -> Self {
        Self {
            base_position: position,
            floating: false,
            ..Default::default()
        }
    }

    /// Set the radius.
    #[must_use]
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    /// Set the deadzone.
    #[must_use]
    pub fn with_deadzone(mut self, deadzone: f32) -> Self {
        self.deadzone = deadzone.clamp(0.0, 1.0);
        self
    }

    /// Get the normalized axis value (-1 to 1 for each component).
    #[must_use]
    pub fn axis(&self) -> Vec2 {
        if !self.active {
            return Vec2::ZERO;
        }

        let magnitude = self.knob_offset.length() / self.radius;
        if magnitude < self.deadzone {
            return Vec2::ZERO;
        }

        // Apply deadzone and normalize
        let adjusted_magnitude = (magnitude - self.deadzone) / (1.0 - self.deadzone);
        let direction = self.knob_offset.normalize_or_zero();
        direction * adjusted_magnitude.min(1.0)
    }

    /// Get the raw axis value without deadzone processing.
    #[must_use]
    pub fn raw_axis(&self) -> Vec2 {
        if !self.active {
            return Vec2::ZERO;
        }
        self.knob_offset / self.radius
    }

    /// Get the distance from center as a percentage (0 to 1).
    #[must_use]
    pub fn magnitude(&self) -> f32 {
        (self.knob_offset.length() / self.radius).min(1.0)
    }

    /// Get the angle of the joystick in radians (0 = right, PI/2 = up).
    #[must_use]
    pub fn angle(&self) -> f32 {
        self.knob_offset.y.atan2(self.knob_offset.x)
    }

    /// Check if the joystick is outside the deadzone.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.active && self.magnitude() > self.deadzone
    }
}

/// Which side of the screen the joystick should respond to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, Reflect)]
pub enum JoystickSide {
    /// Left half of the screen
    #[default]
    Left,
    /// Right half of the screen
    Right,
    /// Entire screen (only one joystick)
    Full,
    /// Custom zone (use with fixed position)
    Custom,
}

/// Global settings for touch joysticks.
#[derive(Resource, Debug, Clone, Reflect)]
pub struct TouchJoystickSettings {
    /// Default radius for new joysticks
    pub default_radius: f32,
    /// Default deadzone
    pub default_deadzone: f32,
    /// Opacity of the joystick visuals (0.0 to 1.0)
    pub opacity: f32,
    /// Whether to show visual feedback
    pub show_visuals: bool,
    /// Color of the joystick base
    pub base_color: Color,
    /// Color of the joystick knob
    pub knob_color: Color,
    /// Margin from screen edge for floating joysticks
    pub screen_margin: f32,
}

impl Default for TouchJoystickSettings {
    fn default() -> Self {
        Self {
            default_radius: 100.0,
            default_deadzone: 0.1,
            opacity: 0.5,
            show_visuals: true,
            base_color: Color::srgba(0.3, 0.3, 0.3, 0.5),
            knob_color: Color::srgba(0.8, 0.8, 0.8, 0.7),
            screen_margin: 50.0,
        }
    }
}

/// Event emitted when a joystick value changes.
#[derive(Event, Message, Debug, Clone)]
pub struct TouchJoystickEvent {
    /// The entity of the joystick
    pub entity: Entity,
    /// The side of the joystick
    pub side: JoystickSide,
    /// The normalized axis value
    pub axis: Vec2,
    /// Whether the joystick is currently active
    pub active: bool,
    /// The raw offset from center
    pub raw_offset: Vec2,
}

/// System to update touch joysticks based on touch input.
fn update_touch_joysticks(
    touches: Res<Touches>,
    windows: Query<&Window>,
    mut joysticks: Query<&mut TouchJoystick>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let window_size = Vec2::new(window.width(), window.height());
    let half_width = window_size.x / 2.0;

    for mut joystick in &mut joysticks {
        // Check if our current touch is still active
        if let Some(touch_id) = joystick.touch_id {
            if let Some(touch) = touches.get_pressed(touch_id) {
                // Update knob position
                let touch_pos = touch.position();
                let offset = touch_pos - joystick.base_position;
                let clamped_offset = if offset.length() > joystick.radius {
                    offset.normalize() * joystick.radius
                } else {
                    offset
                };
                joystick.knob_offset = clamped_offset;
            } else {
                // Touch released
                joystick.active = false;
                joystick.touch_id = None;
                if joystick.snap_to_center {
                    joystick.knob_offset = Vec2::ZERO;
                }
            }
        } else {
            // Look for new touches
            for touch in touches.iter_just_pressed() {
                let touch_pos = touch.position();

                // Check if this touch is in our zone
                let in_zone = match joystick.side {
                    JoystickSide::Left => touch_pos.x < half_width,
                    JoystickSide::Right => touch_pos.x >= half_width,
                    JoystickSide::Full => true,
                    JoystickSide::Custom => {
                        let distance = (touch_pos - joystick.base_position).length();
                        distance <= joystick.radius * 2.0
                    }
                };

                if in_zone {
                    joystick.active = true;
                    joystick.touch_id = Some(touch.id());

                    if joystick.floating {
                        joystick.base_position = touch_pos;
                    }
                    joystick.knob_offset = Vec2::ZERO;
                    break;
                }
            }
        }
    }
}

/// System to emit joystick events.
fn emit_joystick_events(
    joysticks: Query<(Entity, &TouchJoystick), Changed<TouchJoystick>>,
    mut events: MessageWriter<TouchJoystickEvent>,
) {
    for (entity, joystick) in &joysticks {
        events.write(TouchJoystickEvent {
            entity,
            side: joystick.side,
            axis: joystick.axis(),
            active: joystick.is_active(),
            raw_offset: joystick.knob_offset,
        });
    }
}

/// Convenience component to render a basic joystick visual.
#[derive(Component, Debug, Clone)]
pub struct TouchJoystickVisual {
    /// Entity of the joystick this visual represents
    pub joystick_entity: Entity,
}

/// System to spawn visual elements for a joystick.
pub fn spawn_joystick_visual(
    commands: &mut Commands,
    settings: &TouchJoystickSettings,
    joystick_entity: Entity,
    joystick: &TouchJoystick,
) -> Entity {
    let base_entity = commands
        .spawn((
            Sprite {
                color: settings.base_color.with_alpha(settings.opacity),
                custom_size: Some(Vec2::splat(joystick.radius * 2.0)),
                ..default()
            },
            Transform::from_translation(joystick.base_position.extend(0.0)),
            TouchJoystickVisual { joystick_entity },
        ))
        .id();

    // Spawn knob as child
    commands.entity(base_entity).with_children(|parent| {
        parent.spawn((
            Sprite {
                color: settings.knob_color.with_alpha(settings.opacity),
                custom_size: Some(Vec2::splat(joystick.radius * 0.6)),
                ..default()
            },
            Transform::default(),
        ));
    });

    base_entity
}

/// Cardinal direction based on joystick angle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoystickDirection {
    /// No direction (in deadzone)
    None,
    /// Up
    Up,
    /// Down
    Down,
    /// Left
    Left,
    /// Right
    Right,
    /// Up-Left diagonal
    UpLeft,
    /// Up-Right diagonal
    UpRight,
    /// Down-Left diagonal
    DownLeft,
    /// Down-Right diagonal
    DownRight,
}

impl TouchJoystick {
    /// Get the cardinal direction of the joystick (8-way).
    #[must_use]
    pub fn direction_8way(&self) -> JoystickDirection {
        if !self.is_active() {
            return JoystickDirection::None;
        }

        let angle = self.angle();
        let pi = std::f32::consts::PI;

        // 8 directions, each covering 45 degrees
        if angle > -pi / 8.0 && angle <= pi / 8.0 {
            JoystickDirection::Right
        } else if angle > pi / 8.0 && angle <= 3.0 * pi / 8.0 {
            JoystickDirection::UpRight
        } else if angle > 3.0 * pi / 8.0 && angle <= 5.0 * pi / 8.0 {
            JoystickDirection::Up
        } else if angle > 5.0 * pi / 8.0 && angle <= 7.0 * pi / 8.0 {
            JoystickDirection::UpLeft
        } else if angle > 7.0 * pi / 8.0 || angle <= -7.0 * pi / 8.0 {
            JoystickDirection::Left
        } else if angle > -7.0 * pi / 8.0 && angle <= -5.0 * pi / 8.0 {
            JoystickDirection::DownLeft
        } else if angle > -5.0 * pi / 8.0 && angle <= -3.0 * pi / 8.0 {
            JoystickDirection::Down
        } else {
            JoystickDirection::DownRight
        }
    }

    /// Get the cardinal direction of the joystick (4-way).
    #[must_use]
    pub fn direction_4way(&self) -> JoystickDirection {
        if !self.is_active() {
            return JoystickDirection::None;
        }

        let angle = self.angle();
        let pi = std::f32::consts::PI;

        // 4 directions, each covering 90 degrees
        if angle > -pi / 4.0 && angle <= pi / 4.0 {
            JoystickDirection::Right
        } else if angle > pi / 4.0 && angle <= 3.0 * pi / 4.0 {
            JoystickDirection::Up
        } else if angle > 3.0 * pi / 4.0 || angle <= -3.0 * pi / 4.0 {
            JoystickDirection::Left
        } else {
            JoystickDirection::Down
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joystick_creation() {
        let left = TouchJoystick::left();
        assert_eq!(left.side, JoystickSide::Left);

        let right = TouchJoystick::right();
        assert_eq!(right.side, JoystickSide::Right);
    }

    #[test]
    fn test_joystick_axis() {
        let mut joystick = TouchJoystick::default();
        joystick.active = true;
        joystick.radius = 100.0;
        joystick.deadzone = 0.1;

        // Inside deadzone
        joystick.knob_offset = Vec2::new(5.0, 0.0);
        assert_eq!(joystick.axis(), Vec2::ZERO);

        // Outside deadzone
        joystick.knob_offset = Vec2::new(50.0, 0.0);
        let axis = joystick.axis();
        assert!(axis.x > 0.0);
        assert!((axis.y).abs() < 0.001);
    }

    #[test]
    fn test_joystick_direction() {
        let mut joystick = TouchJoystick::default();
        joystick.active = true;
        joystick.radius = 100.0;
        joystick.deadzone = 0.0;

        joystick.knob_offset = Vec2::new(50.0, 0.0);
        assert_eq!(joystick.direction_4way(), JoystickDirection::Right);

        joystick.knob_offset = Vec2::new(0.0, 50.0);
        assert_eq!(joystick.direction_4way(), JoystickDirection::Up);

        joystick.knob_offset = Vec2::new(-50.0, 0.0);
        assert_eq!(joystick.direction_4way(), JoystickDirection::Left);

        joystick.knob_offset = Vec2::new(0.0, -50.0);
        assert_eq!(joystick.direction_4way(), JoystickDirection::Down);
    }

    #[test]
    fn test_joystick_magnitude() {
        let mut joystick = TouchJoystick::default();
        joystick.active = true;
        joystick.radius = 100.0;

        joystick.knob_offset = Vec2::new(50.0, 0.0);
        assert!((joystick.magnitude() - 0.5).abs() < 0.001);

        joystick.knob_offset = Vec2::new(100.0, 0.0);
        assert!((joystick.magnitude() - 1.0).abs() < 0.001);

        // Clamped to 1.0
        joystick.knob_offset = Vec2::new(150.0, 0.0);
        assert!((joystick.magnitude() - 1.0).abs() < 0.001);
    }
}
