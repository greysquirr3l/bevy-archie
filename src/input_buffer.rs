//! Input buffering and combo detection.
//!
//! This module provides input buffering for fighting games and action games,
//! allowing detection of input sequences and combos.

use bevy::prelude::*;
use std::time::Duration;

use crate::actions::GameAction;

/// Maximum size of input buffer.
const MAX_BUFFER_SIZE: usize = 32;

/// A buffered input entry.
#[derive(Debug, Clone)]
pub struct BufferedInput {
    /// The action that was pressed.
    pub action: GameAction,
    /// Time when it was pressed.
    pub timestamp: f64,
    /// Whether it's still being held.
    pub held: bool,
}

/// Input buffer resource for storing recent inputs.
#[derive(Debug, Clone, Default, Resource)]
pub struct InputBuffer {
    /// Ring buffer of recent inputs.
    pub inputs: Vec<BufferedInput>,
    /// Buffer window duration.
    pub window: Duration,
    /// Current game time.
    pub current_time: f64,
}

impl InputBuffer {
    /// Create a new input buffer with specified window.
    #[must_use]
    pub fn new(window: Duration) -> Self {
        Self {
            inputs: Vec::with_capacity(MAX_BUFFER_SIZE),
            window,
            current_time: 0.0,
        }
    }

    /// Add an input to the buffer.
    pub fn push(&mut self, action: GameAction, held: bool) {
        let input = BufferedInput {
            action,
            timestamp: self.current_time,
            held,
        };

        self.inputs.push(input);

        // Limit buffer size
        if self.inputs.len() > MAX_BUFFER_SIZE {
            self.inputs.remove(0);
        }

        // Clean old inputs
        self.clean_old_inputs();
    }

    /// Clean inputs outside the window.
    fn clean_old_inputs(&mut self) {
        let cutoff = self.current_time - self.window.as_secs_f64();
        self.inputs.retain(|input| input.timestamp >= cutoff);
    }

    /// Check if a sequence of actions was performed.
    #[must_use]
    pub fn check_sequence(&self, sequence: &[GameAction], window: Duration) -> bool {
        if sequence.is_empty() || sequence.len() > self.inputs.len() {
            return false;
        }

        let window_secs = window.as_secs_f64();
        let mut seq_idx = 0;

        for input in self.inputs.iter().rev() {
            if input.action == sequence[seq_idx] {
                seq_idx += 1;
                if seq_idx == sequence.len() {
                    // Check if all within window
                    let first_time = self.inputs[self.inputs.len() - seq_idx].timestamp;
                    let last_time = input.timestamp;
                    return (last_time - first_time) <= window_secs;
                }
            }
        }

        false
    }

    /// Get the last N actions.
    #[must_use]
    pub fn last_actions(&self, count: usize) -> Vec<GameAction> {
        self.inputs
            .iter()
            .rev()
            .take(count)
            .map(|input| input.action)
            .collect()
    }

    /// Check for a specific action in the buffer.
    #[must_use]
    pub fn has_action(&self, action: GameAction, within: Duration) -> bool {
        let cutoff = self.current_time - within.as_secs_f64();
        self.inputs
            .iter()
            .rev()
            .any(|input| input.action == action && input.timestamp >= cutoff)
    }

    /// Clear the buffer.
    pub fn clear(&mut self) {
        self.inputs.clear();
    }
}

/// Combo definition.
#[derive(Debug, Clone)]
pub struct Combo {
    /// Name of the combo.
    pub name: String,
    /// Sequence of actions required.
    pub sequence: Vec<GameAction>,
    /// Maximum time between inputs.
    pub window: Duration,
    /// Whether this combo is enabled.
    pub enabled: bool,
}

impl Combo {
    /// Create a new combo.
    #[must_use]
    pub fn new(name: impl Into<String>, sequence: Vec<GameAction>) -> Self {
        Self {
            name: name.into(),
            sequence,
            window: Duration::from_millis(500),
            enabled: true,
        }
    }

    /// Set the window duration.
    #[must_use]
    pub fn with_window(mut self, window: Duration) -> Self {
        self.window = window;
        self
    }

    /// Check if this combo matches the buffer.
    #[must_use]
    pub fn check(&self, buffer: &InputBuffer) -> bool {
        if !self.enabled {
            return false;
        }
        buffer.check_sequence(&self.sequence, self.window)
    }
}

/// Resource for managing combo definitions.
#[derive(Debug, Clone, Default, Resource)]
pub struct ComboRegistry {
    /// Registered combos.
    pub combos: Vec<Combo>,
}

impl ComboRegistry {
    /// Add a combo to the registry.
    pub fn register(&mut self, combo: Combo) {
        self.combos.push(combo);
    }

    /// Check all combos against buffer.
    #[must_use]
    pub fn check_combos(&self, buffer: &InputBuffer) -> Vec<String> {
        self.combos
            .iter()
            .filter(|combo| combo.check(buffer))
            .map(|combo| combo.name.clone())
            .collect()
    }
}

/// Event fired when a combo is detected.
#[derive(Debug, Clone, Message)]
pub struct ComboDetected {
    /// Name of the detected combo.
    pub combo: String,
    /// The gamepad that performed it.
    pub gamepad: Option<Entity>,
}

/// System to update input buffer with new inputs.
pub fn update_input_buffer(
    mut buffer: ResMut<InputBuffer>,
    _keyboard: Res<ButtonInput<KeyCode>>,
    _mouse: Res<ButtonInput<MouseButton>>,
    _gamepads: Query<&Gamepad>,
    time: Res<Time>,
) {
    buffer.current_time = time.elapsed_secs_f64();

    // This would integrate with the action system to detect which actions were pressed
    // For now, this is a placeholder structure
}

/// System to detect combos.
pub fn detect_combos(
    buffer: Res<InputBuffer>,
    registry: Res<ComboRegistry>,
    mut combo_events: MessageWriter<ComboDetected>,
) {
    if buffer.is_changed() {
        for combo_name in registry.check_combos(&buffer) {
            combo_events.write(ComboDetected {
                combo: combo_name,
                gamepad: None,
            });
        }
    }
}

/// Plugin for registering input buffer types.
pub(crate) fn register_input_buffer_types(app: &mut App) {
    app.init_resource::<InputBuffer>()
        .init_resource::<ComboRegistry>()
        .add_message::<ComboDetected>();
}

/// Add input buffer systems to the app.
pub(crate) fn add_input_buffer_systems(app: &mut App) {
    app.add_systems(Update, (update_input_buffer, detect_combos).chain());
}
