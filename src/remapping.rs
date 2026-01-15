//! Controller remapping system.
//!
//! This module allows players to remap controller buttons at runtime.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::actions::{ActionMap, GameAction};

/// The current state of the remapping system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, States, Hash)]
pub enum RemappingState {
    /// Not currently remapping.
    #[default]
    Inactive,
    /// Waiting for the player to press a button.
    WaitingForInput,
}

/// Resource tracking the current remapping operation.
#[derive(Debug, Clone, Default, Resource)]
pub struct RemappingContext {
    /// The action being remapped.
    pub action: Option<GameAction>,
    /// Timeout timer for the remap operation.
    pub timeout: f32,
    /// Maximum time to wait for input.
    pub max_timeout: f32,
}

impl RemappingContext {
    /// Start a new remapping operation.
    pub fn start(&mut self, action: GameAction, timeout: f32) {
        self.action = Some(action);
        self.timeout = timeout;
        self.max_timeout = timeout;
    }

    /// Cancel the current remapping operation.
    pub fn cancel(&mut self) {
        self.action = None;
        self.timeout = 0.0;
    }

    /// Check if remapping is active.
    pub fn is_active(&self) -> bool {
        self.action.is_some()
    }

    /// Get the remaining time as a percentage (0.0 - 1.0).
    pub fn time_remaining_percent(&self) -> f32 {
        if self.max_timeout > 0.0 {
            self.timeout / self.max_timeout
        } else {
            0.0
        }
    }
}

/// Event to start remapping an action.
#[derive(Debug, Clone, Event)]
pub struct StartRemapEvent {
    /// The action to remap.
    pub action: GameAction,
    /// Timeout in seconds (default: 5.0).
    pub timeout: f32,
}

impl StartRemapEvent {
    /// Create a new remap event with default timeout.
    pub fn new(action: GameAction) -> Self {
        Self {
            action,
            timeout: 5.0,
        }
    }

    /// Create a new remap event with custom timeout.
    pub fn with_timeout(action: GameAction, timeout: f32) -> Self {
        Self { action, timeout }
    }
}

/// Event fired when remapping completes.
#[derive(Debug, Clone, Event)]
pub enum RemapEvent {
    /// Remapping was successful.
    Success {
        /// The action that was remapped.
        action: GameAction,
        /// The new button binding.
        button: GamepadButton,
    },
    /// Remapping was cancelled.
    Cancelled {
        /// The action that was being remapped.
        action: GameAction,
    },
    /// Remapping timed out.
    TimedOut {
        /// The action that was being remapped.
        action: GameAction,
    },
    /// The button is already bound to another action.
    Conflict {
        /// The action being remapped.
        action: GameAction,
        /// The conflicting action.
        conflicting_action: GameAction,
        /// The button that caused the conflict.
        button: GamepadButton,
    },
}

/// Saved controller bindings for persistence.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Resource)]
pub struct SavedBindings {
    /// Custom gamepad button bindings.
    pub gamepad: std::collections::HashMap<GameAction, Vec<GamepadButton>>,
}

impl SavedBindings {
    /// Apply saved bindings to an action map.
    pub fn apply_to(&self, action_map: &mut ActionMap) {
        for (action, buttons) in &self.gamepad {
            action_map.clear_gamepad_bindings(*action);
            for button in buttons {
                action_map.bind_gamepad(*action, *button);
            }
        }
    }

    /// Save current bindings from an action map.
    pub fn save_from(&mut self, action_map: &ActionMap) {
        self.gamepad = action_map.gamepad_bindings.clone();
    }
}

/// Component for a remap button UI element.
#[derive(Debug, Clone, Component)]
pub struct RemapButton {
    /// The action this button remaps.
    pub action: GameAction,
}

/// System to handle starting a remap operation.
pub fn handle_start_remap(
    mut events: EventReader<StartRemapEvent>,
    mut context: ResMut<RemappingContext>,
    mut next_state: ResMut<NextState<RemappingState>>,
) {
    for event in events.read() {
        context.start(event.action, event.timeout);
        next_state.set(RemappingState::WaitingForInput);
    }
}

/// System to handle input during remapping.
pub fn handle_remap_input(
    mut context: ResMut<RemappingContext>,
    mut action_map: ResMut<ActionMap>,
    mut remap_events: EventWriter<RemapEvent>,
    mut next_state: ResMut<NextState<RemappingState>>,
    time: Res<Time>,
    gamepads: Query<&Gamepad>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !context.is_active() {
        return;
    }

    let action = context.action.unwrap();

    // Check for cancel (Escape key or B button)
    if keyboard.just_pressed(KeyCode::Escape) {
        remap_events.write(RemapEvent::Cancelled { action });
        context.cancel();
        next_state.set(RemappingState::Inactive);
        return;
    }

    for gamepad in gamepads.iter() {
        if gamepad.just_pressed(GamepadButton::East) {
            remap_events.write(RemapEvent::Cancelled { action });
            context.cancel();
            next_state.set(RemappingState::Inactive);
            return;
        }
    }

    // Check for button press to remap
    for gamepad in gamepads.iter() {
        let buttons_to_check = [
            GamepadButton::South,
            GamepadButton::North,
            GamepadButton::West,
            GamepadButton::LeftTrigger,
            GamepadButton::RightTrigger,
            GamepadButton::LeftTrigger2,
            GamepadButton::RightTrigger2,
            GamepadButton::LeftThumb,
            GamepadButton::RightThumb,
            GamepadButton::DPadUp,
            GamepadButton::DPadDown,
            GamepadButton::DPadLeft,
            GamepadButton::DPadRight,
            GamepadButton::Select,
        ];

        for button in buttons_to_check {
            if gamepad.just_pressed(button) {
                // Check for conflicts
                let mut conflict = None;
                for other_action in GameAction::all() {
                    if *other_action != action {
                        if let Some(buttons) = action_map.gamepad_bindings.get(other_action) {
                            if buttons.contains(&button) {
                                conflict = Some(*other_action);
                                break;
                            }
                        }
                    }
                }

                if let Some(conflicting_action) = conflict {
                    remap_events.write(RemapEvent::Conflict {
                        action,
                        conflicting_action,
                        button,
                    });
                } else {
                    // Apply the new binding
                    action_map.clear_gamepad_bindings(action);
                    action_map.bind_gamepad(action, button);

                    remap_events.write(RemapEvent::Success { action, button });
                    context.cancel();
                    next_state.set(RemappingState::Inactive);
                }
                return;
            }
        }
    }

    // Update timeout
    context.timeout -= time.delta_secs();
    if context.timeout <= 0.0 {
        remap_events.write(RemapEvent::TimedOut { action });
        context.cancel();
        next_state.set(RemappingState::Inactive);
    }
}

/// System to reset bindings to defaults.
pub fn reset_bindings_to_default(mut action_map: ResMut<ActionMap>) {
    *action_map = ActionMap::default();
}

/// Add remapping systems to the app.
pub(crate) fn add_remapping_systems(app: &mut App) {
    app.init_state::<RemappingState>()
        .init_resource::<RemappingContext>()
        .init_resource::<SavedBindings>()
        .add_event::<StartRemapEvent>()
        .add_event::<RemapEvent>()
        .add_systems(
            Update,
            (handle_start_remap, handle_remap_input)
                .chain()
                .run_if(in_state(RemappingState::WaitingForInput).or(on_event::<StartRemapEvent>)),
        );
}
