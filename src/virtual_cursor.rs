//! Virtual cursor for gamepad-controlled mouse emulation.
//!
//! This module provides a virtual cursor that can be controlled with gamepad
//! analog sticks, allowing gamepad users to interact with mouse-based UI.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::config::ControllerConfig;
use crate::detection::InputDeviceState;

/// Component marking an entity as the virtual cursor.
#[derive(Debug, Clone, Component)]
pub struct VirtualCursor {
    /// Current position of the cursor.
    pub position: Vec2,
    /// Speed multiplier for cursor movement.
    pub speed: f32,
    /// Whether the cursor is currently visible.
    pub visible: bool,
    /// Which stick controls the cursor (true = left, false = right).
    pub use_left_stick: bool,
}

impl Default for VirtualCursor {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            speed: 600.0, // Pixels per second
            visible: false,
            use_left_stick: false, // Use right stick by default
        }
    }
}

/// Click state for the virtual cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ClickState {
    /// Not clicking.
    #[default]
    Idle,
    /// Click just started this frame.
    JustPressed,
    /// Click is being held.
    Held,
    /// Click just released this frame.
    JustReleased,
}

/// Resource tracking virtual cursor state.
#[derive(Debug, Clone, Default, Resource)]
pub struct VirtualCursorState {
    /// Whether the virtual cursor is active.
    pub active: bool,
    /// Current cursor position in screen space.
    pub position: Vec2,
    /// Current click state.
    pub click_state: ClickState,
}

impl VirtualCursorState {
    /// Reset frame state (call at start of frame).
    pub fn reset_frame_state(&mut self) {
        // Transition JustPressed -> Held and JustReleased -> Idle
        self.click_state = match self.click_state {
            ClickState::JustPressed => ClickState::Held,
            ClickState::JustReleased => ClickState::Idle,
            other => other,
        };
    }

    /// Start a click.
    pub fn start_click(&mut self) {
        if self.click_state == ClickState::Idle {
            self.click_state = ClickState::JustPressed;
        }
    }

    /// End a click.
    pub fn end_click(&mut self) {
        if matches!(self.click_state, ClickState::Held | ClickState::JustPressed) {
            self.click_state = ClickState::JustReleased;
        }
    }

    /// Check if currently clicking.
    #[must_use]
    pub const fn is_clicking(&self) -> bool {
        matches!(self.click_state, ClickState::Held | ClickState::JustPressed)
    }

    /// Check if click just started.
    #[must_use]
    pub const fn just_clicked(&self) -> bool {
        matches!(self.click_state, ClickState::JustPressed)
    }

    /// Check if click just released.
    #[must_use]
    pub const fn just_released(&self) -> bool {
        matches!(self.click_state, ClickState::JustReleased)
    }
}

/// System to update virtual cursor position based on gamepad input.
pub fn update_virtual_cursor(
    time: Res<Time>,
    config: Res<ControllerConfig>,
    input_state: Res<InputDeviceState>,
    mut cursor_state: ResMut<VirtualCursorState>,
    gamepads: Query<&Gamepad>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut cursor_query: Query<(&mut Transform, &VirtualCursor)>,
) {
    // Only active when using gamepad
    if !input_state.using_gamepad() {
        cursor_state.active = false;
        return;
    }

    let Ok(window) = window_query.single() else {
        return;
    };

    // Get gamepad input
    let mut cursor_delta = Vec2::ZERO;
    for gamepad in gamepads.iter() {
        // Check if we should use this gamepad
        if let Some(_active_gamepad) = input_state.active_gamepad() {
            if cursor_query.is_empty() {
                continue;
            }

            for (_transform, virtual_cursor) in &mut cursor_query {
                // Get stick input based on configuration
                let (x_axis, y_axis) = if virtual_cursor.use_left_stick {
                    (GamepadAxis::LeftStickX, GamepadAxis::LeftStickY)
                } else {
                    (GamepadAxis::RightStickX, GamepadAxis::RightStickY)
                };

                if let (Some(x), Some(y)) = (gamepad.get(x_axis), gamepad.get(y_axis)) {
                    // Apply deadzone and sensitivity
                    let mut input = config.apply_deadzone_2d(x, y, virtual_cursor.use_left_stick);

                    // Apply inversion
                    input = config.apply_inversion(input, virtual_cursor.use_left_stick);

                    cursor_delta = input * virtual_cursor.speed * time.delta_secs();
                }
            }
        }
    }

    // Update cursor position
    if let Ok((mut transform, _)) = cursor_query.single_mut() {
        let new_pos = transform.translation.truncate() + cursor_delta;

        // Clamp to window bounds
        let half_width: f32 = window.width() / 2.0;
        let half_height: f32 = window.height() / 2.0;
        let clamped = Vec2::new(
            new_pos.x.clamp(-half_width, half_width),
            new_pos.y.clamp(-half_height, half_height),
        );

        transform.translation = clamped.extend(transform.translation.z);
        cursor_state.position = clamped;
        cursor_state.active = cursor_delta.length() > 0.01;
    }
}

/// System to handle virtual cursor click input.
pub fn handle_virtual_cursor_clicks(
    mut cursor_state: ResMut<VirtualCursorState>,
    gamepads: Query<&Gamepad>,
) {
    cursor_state.reset_frame_state();

    for gamepad in gamepads.iter() {
        // A button to click
        if gamepad.just_pressed(GamepadButton::South) {
            cursor_state.start_click();
        }
        if gamepad.just_released(GamepadButton::South) {
            cursor_state.end_click();
        }
    }
}

/// System to show/hide virtual cursor based on input device.
pub fn toggle_virtual_cursor_visibility(
    input_state: Res<InputDeviceState>,
    mut cursor_query: Query<&mut Visibility, With<VirtualCursor>>,
) {
    let should_show = input_state.using_gamepad();

    for mut visibility in &mut cursor_query {
        *visibility = if should_show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

/// Event fired when the virtual cursor clicks.
#[derive(Debug, Clone, Message)]
pub struct VirtualCursorClick {
    /// Position where the click occurred.
    pub position: Vec2,
}

/// System to fire click events.
pub fn fire_virtual_cursor_events(
    cursor_state: Res<VirtualCursorState>,
    mut click_events: MessageWriter<VirtualCursorClick>,
) {
    if cursor_state.just_clicked() {
        click_events.write(VirtualCursorClick {
            position: cursor_state.position,
        });
    }
}

/// Helper function to spawn a virtual cursor entity.
#[must_use]
pub fn spawn_virtual_cursor(
    commands: &mut Commands,
    asset_server: &AssetServer,
    cursor_image: Option<Handle<Image>>,
) -> Entity {
    let image = cursor_image.unwrap_or_else(|| asset_server.load("cursor.png"));

    commands
        .spawn((
            VirtualCursor::default(),
            Sprite {
                image,
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 1000.0), // High Z to appear on top
            Visibility::Hidden,
        ))
        .id()
}

/// Plugin for registering virtual cursor types and systems.
pub(crate) fn register_virtual_cursor_types(app: &mut App) {
    app.init_resource::<VirtualCursorState>()
        .add_message::<VirtualCursorClick>();
}

/// Add virtual cursor systems to the app.
pub(crate) fn add_virtual_cursor_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            update_virtual_cursor,
            handle_virtual_cursor_clicks,
            toggle_virtual_cursor_visibility,
            fire_virtual_cursor_events,
        )
            .chain(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_cursor_default() {
        let cursor = VirtualCursor::default();
        assert_eq!(cursor.position, Vec2::ZERO);
        assert_eq!(cursor.speed, 600.0);
        assert!(!cursor.visible);
        assert!(!cursor.use_left_stick);
    }

    #[test]
    fn test_click_state_variants() {
        assert_ne!(ClickState::Idle, ClickState::JustPressed);
        assert_ne!(ClickState::Held, ClickState::JustReleased);
    }

    #[test]
    fn test_click_state_default() {
        let state = ClickState::default();
        assert_eq!(state, ClickState::Idle);
    }

    #[test]
    fn test_virtual_cursor_state_default() {
        let state = VirtualCursorState::default();
        assert!(!state.active);
        assert_eq!(state.position, Vec2::ZERO);
        assert_eq!(state.click_state, ClickState::Idle);
    }

    #[test]
    fn test_virtual_cursor_state_reset_frame_state() {
        let mut state = VirtualCursorState::default();

        state.click_state = ClickState::JustPressed;
        state.reset_frame_state();
        assert_eq!(state.click_state, ClickState::Held);

        state.click_state = ClickState::JustReleased;
        state.reset_frame_state();
        assert_eq!(state.click_state, ClickState::Idle);

        state.click_state = ClickState::Held;
        state.reset_frame_state();
        assert_eq!(state.click_state, ClickState::Held);
    }

    #[test]
    fn test_virtual_cursor_state_start_click() {
        let mut state = VirtualCursorState::default();
        state.start_click();
        assert_eq!(state.click_state, ClickState::JustPressed);

        // Should not change if already pressed
        state.click_state = ClickState::Held;
        state.start_click();
        assert_eq!(state.click_state, ClickState::Held);
    }

    #[test]
    fn test_virtual_cursor_state_end_click() {
        let mut state = VirtualCursorState::default();
        state.click_state = ClickState::Held;
        state.end_click();
        assert_eq!(state.click_state, ClickState::JustReleased);

        // Should not change if already idle
        state.click_state = ClickState::Idle;
        state.end_click();
        assert_eq!(state.click_state, ClickState::Idle);
    }

    #[test]
    fn test_virtual_cursor_click_event() {
        let event = VirtualCursorClick {
            position: Vec2::new(100.0, 200.0),
        };

        assert_eq!(event.position.x, 100.0);
        assert_eq!(event.position.y, 200.0);
    }
}
