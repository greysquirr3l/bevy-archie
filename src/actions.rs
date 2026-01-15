//! Input action mapping system.
//!
//! This module provides an abstraction layer over raw input,
//! allowing games to define logical actions that can be bound
//! to various input sources.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Predefined game actions that can be mapped to inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum GameAction {
    // Navigation
    /// Confirm/select action (A button, Enter key)
    Confirm,
    /// Cancel/back action (B button, Escape key)
    Cancel,
    /// Pause/menu action (Start button)
    Pause,
    /// Secondary menu action (Select button)
    Select,

    // Movement
    /// Move up
    Up,
    /// Move down
    Down,
    /// Move left
    Left,
    /// Move right
    Right,

    // Camera/View
    /// Look up
    LookUp,
    /// Look down
    LookDown,
    /// Look left
    LookLeft,
    /// Look right
    LookRight,

    // Actions
    /// Primary action (X button)
    Primary,
    /// Secondary action (Y button)
    Secondary,
    /// Left shoulder button
    LeftShoulder,
    /// Right shoulder button
    RightShoulder,
    /// Left trigger
    LeftTrigger,
    /// Right trigger
    RightTrigger,

    // UI Navigation
    /// Page left (in menus)
    PageLeft,
    /// Page right (in menus)
    PageRight,

    // Custom actions (use these for game-specific bindings)
    /// Custom action slot 1
    Custom1,
    /// Custom action slot 2
    Custom2,
    /// Custom action slot 3
    Custom3,
    /// Custom action slot 4
    Custom4,
}

impl GameAction {
    /// Get all actions as a slice.
    pub fn all() -> &'static [GameAction] {
        &[
            Self::Confirm,
            Self::Cancel,
            Self::Pause,
            Self::Select,
            Self::Up,
            Self::Down,
            Self::Left,
            Self::Right,
            Self::LookUp,
            Self::LookDown,
            Self::LookLeft,
            Self::LookRight,
            Self::Primary,
            Self::Secondary,
            Self::LeftShoulder,
            Self::RightShoulder,
            Self::LeftTrigger,
            Self::RightTrigger,
            Self::PageLeft,
            Self::PageRight,
            Self::Custom1,
            Self::Custom2,
            Self::Custom3,
            Self::Custom4,
        ]
    }

    /// Get the display name for this action.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Confirm => "Confirm",
            Self::Cancel => "Cancel",
            Self::Pause => "Pause",
            Self::Select => "Select",
            Self::Up => "Up",
            Self::Down => "Down",
            Self::Left => "Left",
            Self::Right => "Right",
            Self::LookUp => "Look Up",
            Self::LookDown => "Look Down",
            Self::LookLeft => "Look Left",
            Self::LookRight => "Look Right",
            Self::Primary => "Primary Action",
            Self::Secondary => "Secondary Action",
            Self::LeftShoulder => "Left Shoulder",
            Self::RightShoulder => "Right Shoulder",
            Self::LeftTrigger => "Left Trigger",
            Self::RightTrigger => "Right Trigger",
            Self::PageLeft => "Page Left",
            Self::PageRight => "Page Right",
            Self::Custom1 => "Custom 1",
            Self::Custom2 => "Custom 2",
            Self::Custom3 => "Custom 3",
            Self::Custom4 => "Custom 4",
        }
    }

    /// Whether this action can be remapped by the player.
    pub fn is_remappable(&self) -> bool {
        !matches!(self, Self::Pause) // Pause is usually not remappable
    }

    /// Whether this action requires a binding (cannot be unbound).
    pub fn is_required(&self) -> bool {
        matches!(self, Self::Confirm | Self::Cancel | Self::Pause)
    }
}

/// A binding source for an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputBinding {
    /// A gamepad button
    GamepadButton(GamepadButton),
    /// A gamepad axis (with direction)
    GamepadAxis(GamepadAxis, AxisDirection),
    /// A keyboard key
    Key(KeyCode),
    /// A mouse button
    MouseButton(MouseButton),
}

/// Direction for axis bindings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AxisDirection {
    /// Positive direction (right, up)
    Positive,
    /// Negative direction (left, down)
    Negative,
}

/// Resource containing action-to-input mappings.
#[derive(Debug, Clone, Resource, Serialize, Deserialize, Reflect)]
#[reflect(Resource)]
pub struct ActionMap {
    /// Gamepad button bindings
    #[reflect(ignore)]
    pub gamepad_bindings: HashMap<GameAction, Vec<GamepadButton>>,

    /// Gamepad axis bindings (action -> (axis, direction, threshold))
    #[reflect(ignore)]
    pub axis_bindings: HashMap<GameAction, Vec<(GamepadAxis, AxisDirection, f32)>>,

    /// Keyboard bindings
    #[reflect(ignore)]
    pub key_bindings: HashMap<GameAction, Vec<KeyCode>>,

    /// Mouse button bindings
    #[reflect(ignore)]
    pub mouse_bindings: HashMap<GameAction, Vec<MouseButton>>,
}

impl Default for ActionMap {
    fn default() -> Self {
        let mut map = Self {
            gamepad_bindings: HashMap::new(),
            axis_bindings: HashMap::new(),
            key_bindings: HashMap::new(),
            mouse_bindings: HashMap::new(),
        };

        // Default gamepad bindings
        map.bind_gamepad(GameAction::Confirm, GamepadButton::South);
        map.bind_gamepad(GameAction::Cancel, GamepadButton::East);
        map.bind_gamepad(GameAction::Pause, GamepadButton::Start);
        map.bind_gamepad(GameAction::Select, GamepadButton::Select);
        map.bind_gamepad(GameAction::Primary, GamepadButton::West);
        map.bind_gamepad(GameAction::Secondary, GamepadButton::North);
        map.bind_gamepad(GameAction::LeftShoulder, GamepadButton::LeftTrigger);
        map.bind_gamepad(GameAction::RightShoulder, GamepadButton::RightTrigger);
        map.bind_gamepad(GameAction::LeftTrigger, GamepadButton::LeftTrigger2);
        map.bind_gamepad(GameAction::RightTrigger, GamepadButton::RightTrigger2);
        map.bind_gamepad(GameAction::PageLeft, GamepadButton::LeftTrigger);
        map.bind_gamepad(GameAction::PageRight, GamepadButton::RightTrigger);

        // D-pad bindings
        map.bind_gamepad(GameAction::Up, GamepadButton::DPadUp);
        map.bind_gamepad(GameAction::Down, GamepadButton::DPadDown);
        map.bind_gamepad(GameAction::Left, GamepadButton::DPadLeft);
        map.bind_gamepad(GameAction::Right, GamepadButton::DPadRight);

        // Left stick axis bindings
        map.bind_axis(
            GameAction::Up,
            GamepadAxis::LeftStickY,
            AxisDirection::Positive,
            0.5,
        );
        map.bind_axis(
            GameAction::Down,
            GamepadAxis::LeftStickY,
            AxisDirection::Negative,
            0.5,
        );
        map.bind_axis(
            GameAction::Left,
            GamepadAxis::LeftStickX,
            AxisDirection::Negative,
            0.5,
        );
        map.bind_axis(
            GameAction::Right,
            GamepadAxis::LeftStickX,
            AxisDirection::Positive,
            0.5,
        );

        // Right stick for looking
        map.bind_axis(
            GameAction::LookUp,
            GamepadAxis::RightStickY,
            AxisDirection::Positive,
            0.5,
        );
        map.bind_axis(
            GameAction::LookDown,
            GamepadAxis::RightStickY,
            AxisDirection::Negative,
            0.5,
        );
        map.bind_axis(
            GameAction::LookLeft,
            GamepadAxis::RightStickX,
            AxisDirection::Negative,
            0.5,
        );
        map.bind_axis(
            GameAction::LookRight,
            GamepadAxis::RightStickX,
            AxisDirection::Positive,
            0.5,
        );

        // Default keyboard bindings
        map.bind_key(GameAction::Confirm, KeyCode::Enter);
        map.bind_key(GameAction::Confirm, KeyCode::Space);
        map.bind_key(GameAction::Cancel, KeyCode::Escape);
        map.bind_key(GameAction::Pause, KeyCode::Escape);
        map.bind_key(GameAction::Up, KeyCode::ArrowUp);
        map.bind_key(GameAction::Up, KeyCode::KeyW);
        map.bind_key(GameAction::Down, KeyCode::ArrowDown);
        map.bind_key(GameAction::Down, KeyCode::KeyS);
        map.bind_key(GameAction::Left, KeyCode::ArrowLeft);
        map.bind_key(GameAction::Left, KeyCode::KeyA);
        map.bind_key(GameAction::Right, KeyCode::ArrowRight);
        map.bind_key(GameAction::Right, KeyCode::KeyD);
        map.bind_key(GameAction::PageLeft, KeyCode::KeyQ);
        map.bind_key(GameAction::PageRight, KeyCode::KeyE);

        map
    }
}

impl ActionMap {
    /// Bind a gamepad button to an action.
    pub fn bind_gamepad(&mut self, action: GameAction, button: GamepadButton) {
        self.gamepad_bindings
            .entry(action)
            .or_default()
            .push(button);
    }

    /// Bind a gamepad axis to an action.
    pub fn bind_axis(
        &mut self,
        action: GameAction,
        axis: GamepadAxis,
        direction: AxisDirection,
        threshold: f32,
    ) {
        self.axis_bindings
            .entry(action)
            .or_default()
            .push((axis, direction, threshold));
    }

    /// Bind a keyboard key to an action.
    pub fn bind_key(&mut self, action: GameAction, key: KeyCode) {
        self.key_bindings.entry(action).or_default().push(key);
    }

    /// Bind a mouse button to an action.
    pub fn bind_mouse(&mut self, action: GameAction, button: MouseButton) {
        self.mouse_bindings.entry(action).or_default().push(button);
    }

    /// Clear all bindings for an action.
    pub fn clear_bindings(&mut self, action: GameAction) {
        self.gamepad_bindings.remove(&action);
        self.axis_bindings.remove(&action);
        self.key_bindings.remove(&action);
        self.mouse_bindings.remove(&action);
    }

    /// Clear only gamepad bindings for an action.
    pub fn clear_gamepad_bindings(&mut self, action: GameAction) {
        self.gamepad_bindings.remove(&action);
        self.axis_bindings.remove(&action);
    }

    /// Get the primary gamepad button for an action (for icon display).
    pub fn primary_gamepad_button(&self, action: GameAction) -> Option<GamepadButton> {
        self.gamepad_bindings
            .get(&action)
            .and_then(|buttons| buttons.first().copied())
    }
}

/// Resource tracking the current state of all actions.
#[derive(Debug, Clone, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct ActionState {
    /// Actions that are currently pressed.
    #[reflect(ignore)]
    pressed: HashMap<GameAction, bool>,

    /// Actions that were just pressed this frame.
    #[reflect(ignore)]
    just_pressed: HashMap<GameAction, bool>,

    /// Actions that were just released this frame.
    #[reflect(ignore)]
    just_released: HashMap<GameAction, bool>,

    /// Analog values for actions (0.0 - 1.0).
    #[reflect(ignore)]
    values: HashMap<GameAction, f32>,
}

impl ActionState {
    /// Check if an action is currently pressed.
    pub fn pressed(&self, action: GameAction) -> bool {
        self.pressed.get(&action).copied().unwrap_or(false)
    }

    /// Check if an action was just pressed this frame.
    pub fn just_pressed(&self, action: GameAction) -> bool {
        self.just_pressed.get(&action).copied().unwrap_or(false)
    }

    /// Check if an action was just released this frame.
    pub fn just_released(&self, action: GameAction) -> bool {
        self.just_released.get(&action).copied().unwrap_or(false)
    }

    /// Get the analog value of an action (0.0 - 1.0).
    pub fn value(&self, action: GameAction) -> f32 {
        self.values.get(&action).copied().unwrap_or(0.0)
    }

    /// Reset just_pressed and just_released flags.
    pub(crate) fn reset_frame_state(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    /// Set an action's pressed state.
    pub(crate) fn set_pressed(&mut self, action: GameAction, pressed: bool) {
        let was_pressed = self.pressed.get(&action).copied().unwrap_or(false);

        if pressed && !was_pressed {
            self.just_pressed.insert(action, true);
        } else if !pressed && was_pressed {
            self.just_released.insert(action, true);
        }

        self.pressed.insert(action, pressed);
    }

    /// Set an action's analog value.
    pub(crate) fn set_value(&mut self, action: GameAction, value: f32) {
        self.values.insert(action, value.clamp(0.0, 1.0));
    }
}

/// System to update action states from input.
pub fn update_action_state(
    mut state: ResMut<ActionState>,
    action_map: Res<ActionMap>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    gamepads: Query<&Gamepad>,
) {
    // Reset frame state
    state.reset_frame_state();

    // Check all actions
    for action in GameAction::all() {
        let mut pressed = false;
        let mut value = 0.0f32;

        // Check keyboard bindings
        if let Some(keys) = action_map.key_bindings.get(action) {
            for key in keys {
                if keyboard.pressed(*key) {
                    pressed = true;
                    value = 1.0;
                    break;
                }
            }
        }

        // Check mouse bindings
        if !pressed {
            if let Some(buttons) = action_map.mouse_bindings.get(action) {
                for button in buttons {
                    if mouse_buttons.pressed(*button) {
                        pressed = true;
                        value = 1.0;
                        break;
                    }
                }
            }
        }

        // Check gamepad bindings
        if !pressed {
            for gamepad in gamepads.iter() {
                // Check button bindings
                if let Some(buttons) = action_map.gamepad_bindings.get(action) {
                    for button_type in buttons {
                        if gamepad.pressed(*button_type) {
                            pressed = true;
                            value = 1.0;
                            break;
                        }
                    }
                }

                // Check axis bindings
                if !pressed {
                    if let Some(axes) = action_map.axis_bindings.get(action) {
                        for (axis_type, direction, threshold) in axes {
                            if let Some(axis_value) = gamepad.get(*axis_type) {
                                let check_value = match direction {
                                    AxisDirection::Positive => axis_value,
                                    AxisDirection::Negative => -axis_value,
                                };

                                if check_value > *threshold {
                                    pressed = true;
                                    value = value.max(check_value);
                                }
                            }
                        }
                    }
                }

                if pressed {
                    break;
                }
            }
        }

        state.set_pressed(*action, pressed);
        state.set_value(*action, value);
    }
}

/// Plugin for registering action types and systems.
pub(crate) fn register_action_types(app: &mut App) {
    app.register_type::<GameAction>()
        .register_type::<ActionMap>()
        .register_type::<ActionState>()
        .init_resource::<ActionMap>()
        .init_resource::<ActionState>();
}

/// Add action systems to the app.
pub(crate) fn add_action_systems(app: &mut App) {
    app.add_systems(PreUpdate, update_action_state);
}
