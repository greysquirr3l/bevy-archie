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
