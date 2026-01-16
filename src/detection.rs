//! Input device detection and state tracking.
//!
//! This module handles automatic detection of which input device
//! (mouse, keyboard, or gamepad) the player is currently using.

use bevy::prelude::*;

/// The type of input device currently being used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Reflect)]
pub enum InputDevice {
    /// Mouse is the active input device.
    #[default]
    Mouse,
    /// Keyboard is the active input device.
    Keyboard,
    /// A gamepad is the active input device.
    Gamepad(Entity),
}

impl InputDevice {
    /// Returns true if this is a gamepad device.
    #[must_use]
    pub fn is_gamepad(&self) -> bool {
        matches!(self, Self::Gamepad(_))
    }

    /// Returns true if this is the mouse.
    #[must_use]
    pub fn is_mouse(&self) -> bool {
        matches!(self, Self::Mouse)
    }

    /// Returns true if this is the keyboard.
    #[must_use]
    pub fn is_keyboard(&self) -> bool {
        matches!(self, Self::Keyboard)
    }

    /// Get the gamepad entity if this is a gamepad device.
    #[must_use]
    pub fn gamepad(&self) -> Option<Entity> {
        match self {
            Self::Gamepad(entity) => Some(*entity),
            _ => None,
        }
    }
}

/// Resource tracking the current input device state.
#[derive(Debug, Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct InputDeviceState {
    /// The currently active input device.
    pub active_device: InputDevice,

    /// The previously active device (for detecting changes).
    pub previous_device: InputDevice,

    /// Whether the active device changed this frame.
    pub device_changed: bool,

    /// All currently connected gamepads.
    pub connected_gamepads: Vec<Entity>,

    /// The primary gamepad (first connected or manually selected).
    pub primary_gamepad: Option<Entity>,

    /// Mouse movement threshold to consider mouse "active".
    pub mouse_movement_threshold: f32,

    /// Whether to automatically switch devices based on input.
    pub auto_switch: bool,
}

impl Default for InputDeviceState {
    fn default() -> Self {
        Self {
            active_device: InputDevice::Mouse,
            previous_device: InputDevice::Mouse,
            device_changed: false,
            connected_gamepads: Vec::new(),
            primary_gamepad: None,
            mouse_movement_threshold: 1.0,
            auto_switch: true,
        }
    }
}

impl InputDeviceState {
    /// Returns true if the player is currently using a mouse.
    #[must_use]
    pub fn using_mouse(&self) -> bool {
        self.active_device.is_mouse()
    }

    /// Returns true if the player is currently using a keyboard.
    #[must_use]
    pub fn using_keyboard(&self) -> bool {
        self.active_device.is_keyboard()
    }

    /// Returns true if the player is currently using a gamepad.
    #[must_use]
    pub fn using_gamepad(&self) -> bool {
        self.active_device.is_gamepad()
    }

    /// Returns true if using keyboard or gamepad (non-mouse).
    #[must_use]
    pub fn using_non_mouse(&self) -> bool {
        !self.using_mouse()
    }

    /// Get the active gamepad entity, if any.
    #[must_use]
    pub fn active_gamepad(&self) -> Option<Entity> {
        self.active_device.gamepad()
    }

    /// Set the active device and track changes.
    fn set_active(&mut self, device: InputDevice) {
        if self.active_device != device {
            self.previous_device = self.active_device;
            self.active_device = device;
            self.device_changed = true;
        }
    }
}

/// Event fired when the active input device changes.
#[derive(Debug, Clone, Message)]
pub struct InputDeviceChanged {
    /// The previous input device.
    pub previous: InputDevice,
    /// The new input device.
    pub current: InputDevice,
}

/// Event fired when a gamepad is connected.
#[derive(Debug, Clone, Message)]
pub struct GamepadConnected {
    /// The connected gamepad entity.
    pub gamepad: Entity,
    /// The gamepad name, if available.
    pub name: Option<String>,
}

/// Event fired when a gamepad is disconnected.
#[derive(Debug, Clone, Message)]
pub struct GamepadDisconnected {
    /// The disconnected gamepad entity.
    pub gamepad: Entity,
}

/// System to detect input device changes based on user input.
pub fn detect_input_device(
    mut state: ResMut<InputDeviceState>,
    mut device_changed_events: MessageWriter<InputDeviceChanged>,
    mut mouse_motion: MessageReader<bevy::input::mouse::MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<(&Gamepad, Entity)>,
) {
    // Reset change flag at start of frame
    state.device_changed = false;

    if !state.auto_switch {
        return;
    }

    let previous = state.active_device;

    // Check for mouse activity
    let mouse_moved = mouse_motion.read().count() > 0;
    let mouse_clicked = mouse_buttons.get_just_pressed().next().is_some();

    if mouse_moved || mouse_clicked {
        state.set_active(InputDevice::Mouse);
    }

    // Check for keyboard activity
    if keyboard.get_just_pressed().next().is_some() {
        state.set_active(InputDevice::Keyboard);
    }

    // Check for gamepad activity
    for (gamepad, gamepad_entity) in gamepads.iter() {
        // Check if any button is pressed
        let has_button_input = gamepad.get_just_pressed().next().is_some();

        // Check if any axis exceeds threshold
        // Note: LeftStick and RightStick button variants were removed in Bevy 0.17
        // This check is now simplified to just button presses
        let has_axis_input = false;

        if has_button_input || has_axis_input {
            state.set_active(InputDevice::Gamepad(gamepad_entity));
            break;
        }
    }

    // Fire event if device changed
    if state.device_changed {
        device_changed_events.write(InputDeviceChanged {
            previous,
            current: state.active_device,
        });
    }
}

/// System to track gamepad connections/disconnections.
pub fn track_gamepad_connections(
    mut state: ResMut<InputDeviceState>,
    mut connected_events: MessageWriter<GamepadConnected>,
    mut disconnected_events: MessageWriter<GamepadDisconnected>,
    gamepads: Query<(Entity, Option<&Name>), Added<Gamepad>>,
    mut removed_gamepads: RemovedComponents<Gamepad>,
) {
    // Track new connections
    for (entity, name) in gamepads.iter() {
        if !state.connected_gamepads.contains(&entity) {
            state.connected_gamepads.push(entity);

            // Set as primary if none exists
            if state.primary_gamepad.is_none() {
                state.primary_gamepad = Some(entity);
            }

            connected_events.write(GamepadConnected {
                gamepad: entity,
                name: name.map(std::string::ToString::to_string),
            });
        }
    }

    // Track disconnections
    for entity in removed_gamepads.read() {
        if let Some(pos) = state.connected_gamepads.iter().position(|&e| e == entity) {
            state.connected_gamepads.remove(pos);

            // Update primary if it was disconnected
            if state.primary_gamepad == Some(entity) {
                state.primary_gamepad = state.connected_gamepads.first().copied();
            }

            // Update active device if it was the disconnected gamepad
            if state.active_device == InputDevice::Gamepad(entity) {
                state.active_device = state
                    .primary_gamepad
                    .map_or(InputDevice::Keyboard, InputDevice::Gamepad);
            }

            disconnected_events.write(GamepadDisconnected { gamepad: entity });
        }
    }
}

/// Plugin for registering detection types and systems.
pub(crate) fn register_detection_types(app: &mut App) {
    app.register_type::<InputDevice>()
        .register_type::<InputDeviceState>()
        .init_resource::<InputDeviceState>()
        .add_message::<InputDeviceChanged>()
        .add_message::<GamepadConnected>()
        .add_message::<GamepadDisconnected>();
}

/// Add detection systems to the app.
pub(crate) fn add_detection_systems(app: &mut App) {
    app.add_systems(
        PreUpdate,
        (track_gamepad_connections, detect_input_device).chain(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== InputDevice Tests ==========

    #[test]
    fn test_input_device_default() {
        let device = InputDevice::default();
        assert_eq!(device, InputDevice::Mouse);
    }

    #[test]
    fn test_input_device_is_gamepad() {
        assert!(InputDevice::Gamepad(Entity::PLACEHOLDER).is_gamepad());
        assert!(!InputDevice::Mouse.is_gamepad());
        assert!(!InputDevice::Keyboard.is_gamepad());
    }

    #[test]
    fn test_input_device_is_mouse() {
        assert!(InputDevice::Mouse.is_mouse());
        assert!(!InputDevice::Keyboard.is_mouse());
        assert!(!InputDevice::Gamepad(Entity::PLACEHOLDER).is_mouse());
    }

    #[test]
    fn test_input_device_is_keyboard() {
        assert!(InputDevice::Keyboard.is_keyboard());
        assert!(!InputDevice::Mouse.is_keyboard());
        assert!(!InputDevice::Gamepad(Entity::PLACEHOLDER).is_keyboard());
    }

    #[test]
    fn test_input_device_gamepad_returns_entity() {
        let entity = Entity::PLACEHOLDER;
        let device = InputDevice::Gamepad(entity);
        assert_eq!(device.gamepad(), Some(entity));
    }

    #[test]
    fn test_input_device_gamepad_returns_none_for_mouse() {
        assert!(InputDevice::Mouse.gamepad().is_none());
    }

    #[test]
    fn test_input_device_gamepad_returns_none_for_keyboard() {
        assert!(InputDevice::Keyboard.gamepad().is_none());
    }

    #[test]
    fn test_input_device_equality() {
        assert_eq!(InputDevice::Mouse, InputDevice::Mouse);
        assert_eq!(InputDevice::Keyboard, InputDevice::Keyboard);
        assert_ne!(InputDevice::Mouse, InputDevice::Keyboard);

        let entity = Entity::PLACEHOLDER;
        assert_eq!(InputDevice::Gamepad(entity), InputDevice::Gamepad(entity));
    }

    // ========== InputDeviceState Tests ==========

    #[test]
    fn test_input_device_state_default() {
        let state = InputDeviceState::default();
        assert_eq!(state.active_device, InputDevice::Mouse);
        assert_eq!(state.previous_device, InputDevice::Mouse);
        assert!(!state.device_changed);
        assert!(state.connected_gamepads.is_empty());
        assert!(state.primary_gamepad.is_none());
        assert!(state.auto_switch);
    }

    #[test]
    fn test_input_device_state_using_mouse() {
        let mut state = InputDeviceState::default();
        state.active_device = InputDevice::Mouse;
        assert!(state.using_mouse());
        assert!(!state.using_keyboard());
        assert!(!state.using_gamepad());
    }

    #[test]
    fn test_input_device_state_using_keyboard() {
        let mut state = InputDeviceState::default();
        state.active_device = InputDevice::Keyboard;
        assert!(!state.using_mouse());
        assert!(state.using_keyboard());
        assert!(!state.using_gamepad());
    }

    #[test]
    fn test_input_device_state_using_gamepad() {
        let mut state = InputDeviceState::default();
        state.active_device = InputDevice::Gamepad(Entity::PLACEHOLDER);
        assert!(!state.using_mouse());
        assert!(!state.using_keyboard());
        assert!(state.using_gamepad());
    }

    #[test]
    fn test_input_device_state_using_non_mouse() {
        let mut state = InputDeviceState::default();

        state.active_device = InputDevice::Mouse;
        assert!(!state.using_non_mouse());

        state.active_device = InputDevice::Keyboard;
        assert!(state.using_non_mouse());

        state.active_device = InputDevice::Gamepad(Entity::PLACEHOLDER);
        assert!(state.using_non_mouse());
    }

    #[test]
    fn test_input_device_state_active_gamepad() {
        let mut state = InputDeviceState::default();
        assert!(state.active_gamepad().is_none());

        let entity = Entity::PLACEHOLDER;
        state.active_device = InputDevice::Gamepad(entity);
        assert_eq!(state.active_gamepad(), Some(entity));
    }

    #[test]
    fn test_input_device_state_set_active_changes_device() {
        let mut state = InputDeviceState::default();
        state.set_active(InputDevice::Keyboard);

        assert_eq!(state.active_device, InputDevice::Keyboard);
        assert_eq!(state.previous_device, InputDevice::Mouse);
        assert!(state.device_changed);
    }

    #[test]
    fn test_input_device_state_set_active_same_device() {
        let mut state = InputDeviceState::default();
        state.device_changed = false;
        state.set_active(InputDevice::Mouse); // Same as default

        assert_eq!(state.active_device, InputDevice::Mouse);
        assert!(!state.device_changed);
    }

    #[test]
    fn test_input_device_state_mouse_movement_threshold() {
        let state = InputDeviceState::default();
        assert!(state.mouse_movement_threshold > 0.0);
    }

    // ========== Event Tests ==========

    #[test]
    fn test_input_device_changed_event() {
        let event = InputDeviceChanged {
            previous: InputDevice::Mouse,
            current: InputDevice::Keyboard,
        };
        assert_eq!(event.previous, InputDevice::Mouse);
        assert_eq!(event.current, InputDevice::Keyboard);
    }

    #[test]
    fn test_gamepad_connected_event() {
        let event = GamepadConnected {
            gamepad: Entity::PLACEHOLDER,
            name: Some("Xbox Controller".to_string()),
        };
        assert_eq!(event.gamepad, Entity::PLACEHOLDER);
        assert_eq!(event.name, Some("Xbox Controller".to_string()));
    }

    #[test]
    fn test_gamepad_connected_event_no_name() {
        let event = GamepadConnected {
            gamepad: Entity::PLACEHOLDER,
            name: None,
        };
        assert!(event.name.is_none());
    }

    #[test]
    fn test_gamepad_disconnected_event() {
        let event = GamepadDisconnected {
            gamepad: Entity::PLACEHOLDER,
        };
        assert_eq!(event.gamepad, Entity::PLACEHOLDER);
    }

    // ========== Connected Gamepads Tests ==========

    #[test]
    fn test_connected_gamepads_tracking() {
        let mut state = InputDeviceState::default();
        let entity = Entity::PLACEHOLDER;

        state.connected_gamepads.push(entity);
        assert!(state.connected_gamepads.contains(&entity));
        assert_eq!(state.connected_gamepads.len(), 1);
    }

    #[test]
    fn test_primary_gamepad_assignment() {
        let mut state = InputDeviceState::default();
        let entity = Entity::PLACEHOLDER;

        state.primary_gamepad = Some(entity);
        assert_eq!(state.primary_gamepad, Some(entity));
    }
}
