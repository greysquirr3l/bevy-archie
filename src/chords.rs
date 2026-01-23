//! Button chords and input clash detection.
//!
//! This module provides chord (button combination) support and
//! clash detection strategies for handling overlapping inputs.
//!
//! # Example
//!
//! ```rust,no_run
//! use bevy_archie::chords::{ButtonChord, ClashStrategy};
//! use bevy::prelude::*;
//!
//! // Create a chord that requires Ctrl+Shift+C
//! let chord = ButtonChord::from_keys(&[KeyCode::ControlLeft, KeyCode::ShiftLeft, KeyCode::KeyC]);
//! ```

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::virtual_input::VirtualButton;

/// A chord of buttons that must all be pressed simultaneously.
///
/// Chords are used to create complex input combinations like
/// Ctrl+Shift+C or LB+RB on a gamepad.
#[derive(Debug, Clone, Default, Reflect)]
pub struct ButtonChord {
    /// The buttons that make up this chord
    #[reflect(ignore)]
    buttons: Vec<VirtualButton>,
}

impl ButtonChord {
    /// Create a new empty chord.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a chord from an iterator of buttons.
    #[must_use]
    pub fn from_buttons(buttons: impl IntoIterator<Item = VirtualButton>) -> Self {
        Self {
            buttons: buttons.into_iter().collect(),
        }
    }

    /// Create a chord from keyboard keys.
    #[must_use]
    pub fn from_keys(keys: &[KeyCode]) -> Self {
        Self {
            buttons: keys.iter().map(|k| VirtualButton::Key(*k)).collect(),
        }
    }

    /// Create a chord from gamepad buttons.
    #[must_use]
    pub fn from_gamepad_buttons(buttons: &[GamepadButton]) -> Self {
        Self {
            buttons: buttons.iter().map(|b| VirtualButton::Gamepad(*b)).collect(),
        }
    }

    /// Add a button to this chord.
    #[must_use]
    pub fn with(mut self, button: impl Into<VirtualButton>) -> Self {
        self.buttons.push(button.into());
        self
    }

    /// Add multiple buttons to this chord.
    #[must_use]
    pub fn with_multiple(
        mut self,
        buttons: impl IntoIterator<Item = impl Into<VirtualButton>>,
    ) -> Self {
        self.buttons.extend(buttons.into_iter().map(Into::into));
        self
    }

    /// Get the buttons in this chord.
    #[must_use]
    pub fn buttons(&self) -> &[VirtualButton] {
        &self.buttons
    }

    /// Get the number of buttons in this chord.
    #[must_use]
    pub fn len(&self) -> usize {
        self.buttons.len()
    }

    /// Check if this chord is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.buttons.is_empty()
    }

    /// Check if all buttons in this chord are pressed.
    #[must_use]
    pub fn is_pressed(&self, keyboard: &ButtonInput<KeyCode>, gamepads: &Query<&Gamepad>) -> bool {
        if self.buttons.is_empty() {
            return false;
        }
        self.buttons
            .iter()
            .all(|b| b.is_pressed(keyboard, gamepads))
    }

    /// Check if all buttons are pressed, including mouse.
    #[must_use]
    pub fn is_pressed_with_mouse(
        &self,
        keyboard: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
        gamepads: &Query<&Gamepad>,
    ) -> bool {
        if self.buttons.is_empty() {
            return false;
        }
        self.buttons
            .iter()
            .all(|b| b.is_pressed_with_mouse(keyboard, mouse, gamepads))
    }

    /// Check if this chord clashes with another chord.
    ///
    /// Two chords clash if one is a subset of the other.
    #[must_use]
    pub fn clashes_with(&self, other: &Self) -> bool {
        if self.is_empty() || other.is_empty() {
            return false;
        }

        let self_set: HashSet<_> = self.buttons.iter().collect();
        let other_set: HashSet<_> = other.buttons.iter().collect();

        // Check if one is a subset of the other
        self_set.is_subset(&other_set) || other_set.is_subset(&self_set)
    }

    /// Decompose this chord into its basic input buttons.
    #[must_use]
    pub fn decompose(&self) -> Vec<VirtualButton> {
        self.buttons.clone()
    }
}

impl PartialEq for ButtonChord {
    fn eq(&self, other: &Self) -> bool {
        if self.buttons.len() != other.buttons.len() {
            return false;
        }
        let self_set: HashSet<_> = self.buttons.iter().collect();
        let other_set: HashSet<_> = other.buttons.iter().collect();
        self_set == other_set
    }
}

impl Eq for ButtonChord {}

impl std::hash::Hash for ButtonChord {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Sort buttons for consistent hashing regardless of order
        let mut sorted: Vec<_> = self.buttons.iter().collect();
        sorted.sort_by_key(|b| format!("{b:?}"));
        for button in sorted {
            button.hash(state);
        }
    }
}

/// Strategy for resolving input clashes.
///
/// When multiple actions are bound to overlapping inputs (e.g., "A" and "Ctrl+A"),
/// this strategy determines which actions are triggered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, Reflect)]
pub enum ClashStrategy {
    /// Prioritize the longest chord (most buttons).
    ///
    /// If Ctrl+Shift+A is pressed and there are bindings for:
    /// - A
    /// - Ctrl+A
    /// - Ctrl+Shift+A
    ///
    /// Only Ctrl+Shift+A will trigger.
    #[default]
    PrioritizeLongest,

    /// Trigger all matching actions.
    ///
    /// All three bindings (A, Ctrl+A, Ctrl+Shift+A) would trigger.
    UseAll,

    /// Prioritize the first registered action.
    ///
    /// Whichever action was registered first will trigger.
    PrioritizeFirst,

    /// Only trigger exact matches (no chord subset matching).
    ///
    /// Only Ctrl+Shift+A would trigger; A and Ctrl+A would not.
    ExactOnly,
}

/// A chord binding that associates a chord with an action value.
#[derive(Debug, Clone)]
pub struct ChordBinding<A> {
    /// The chord to match
    pub chord: ButtonChord,
    /// The action to trigger
    pub action: A,
    /// Priority for clash resolution (higher = more priority)
    pub priority: i32,
}

impl<A> ChordBinding<A> {
    /// Create a new chord binding.
    pub fn new(chord: ButtonChord, action: A) -> Self {
        Self {
            chord,
            action,
            priority: 0,
        }
    }

    /// Create a binding with a specific priority.
    pub fn with_priority(chord: ButtonChord, action: A, priority: i32) -> Self {
        Self {
            chord,
            action,
            priority,
        }
    }
}

/// Resolves clashes between multiple pressed chords.
pub fn resolve_clashes<A: Clone>(
    pressed_bindings: &[ChordBinding<A>],
    strategy: ClashStrategy,
) -> Vec<A> {
    if pressed_bindings.is_empty() {
        return Vec::new();
    }

    match strategy {
        ClashStrategy::UseAll => pressed_bindings.iter().map(|b| b.action.clone()).collect(),
        ClashStrategy::PrioritizeLongest => {
            // Group by chord length, take the longest
            let max_len = pressed_bindings
                .iter()
                .map(|b| b.chord.len())
                .max()
                .unwrap_or(0);
            pressed_bindings
                .iter()
                .filter(|b| b.chord.len() == max_len)
                .map(|b| b.action.clone())
                .collect()
        }
        ClashStrategy::PrioritizeFirst => {
            // Just take the first one
            pressed_bindings
                .first()
                .map(|b| b.action.clone())
                .into_iter()
                .collect()
        }
        ClashStrategy::ExactOnly => {
            // Only include chords that don't clash with longer chords
            let mut result = Vec::new();
            for binding in pressed_bindings {
                let is_subset_of_longer = pressed_bindings.iter().any(|other| {
                    other.chord.len() > binding.chord.len()
                        && binding.chord.clashes_with(&other.chord)
                });
                if !is_subset_of_longer {
                    result.push(binding.action.clone());
                }
            }
            result
        }
    }
}

/// A modified key that acts as a button.
///
/// This is useful for treating modifier keys (Ctrl, Shift, Alt) as buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum ModifierKey {
    /// Control key (either left or right)
    Control,
    /// Shift key (either left or right)
    Shift,
    /// Alt key (either left or right)
    Alt,
    /// Super/Windows/Command key (either left or right)
    Super,
}

impl ModifierKey {
    /// Check if this modifier is pressed.
    #[must_use]
    pub fn is_pressed(self, keyboard: &ButtonInput<KeyCode>) -> bool {
        match self {
            Self::Control => {
                keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight)
            }
            Self::Shift => {
                keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight)
            }
            Self::Alt => keyboard.pressed(KeyCode::AltLeft) || keyboard.pressed(KeyCode::AltRight),
            Self::Super => {
                keyboard.pressed(KeyCode::SuperLeft) || keyboard.pressed(KeyCode::SuperRight)
            }
        }
    }
}

/// Register chord types with the app.
pub(crate) fn register_chord_types(app: &mut App) {
    app.register_type::<ButtonChord>()
        .register_type::<ClashStrategy>()
        .register_type::<ModifierKey>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chord_creation() {
        let chord = ButtonChord::from_keys(&[KeyCode::ControlLeft, KeyCode::KeyC]);
        assert_eq!(chord.len(), 2);
        assert!(!chord.is_empty());
    }

    #[test]
    fn test_chord_with_builder() {
        let chord = ButtonChord::new()
            .with(KeyCode::ControlLeft)
            .with(KeyCode::ShiftLeft)
            .with(KeyCode::KeyC);
        assert_eq!(chord.len(), 3);
    }

    #[test]
    fn test_chord_equality() {
        let chord1 = ButtonChord::from_keys(&[KeyCode::KeyA, KeyCode::KeyB]);
        let chord2 = ButtonChord::from_keys(&[KeyCode::KeyB, KeyCode::KeyA]);
        assert_eq!(chord1, chord2); // Order independent
    }

    #[test]
    fn test_chord_clash_detection() {
        let short = ButtonChord::from_keys(&[KeyCode::KeyA]);
        let long = ButtonChord::from_keys(&[KeyCode::ControlLeft, KeyCode::KeyA]);
        let unrelated = ButtonChord::from_keys(&[KeyCode::KeyB]);

        assert!(short.clashes_with(&long)); // A is subset of Ctrl+A
        assert!(long.clashes_with(&short));
        assert!(!short.clashes_with(&unrelated));
    }

    #[test]
    fn test_clash_strategy_prioritize_longest() {
        let bindings = vec![
            ChordBinding::new(ButtonChord::from_keys(&[KeyCode::KeyA]), "A"),
            ChordBinding::new(
                ButtonChord::from_keys(&[KeyCode::ControlLeft, KeyCode::KeyA]),
                "Ctrl+A",
            ),
        ];

        let result = resolve_clashes(&bindings, ClashStrategy::PrioritizeLongest);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "Ctrl+A");
    }

    #[test]
    fn test_clash_strategy_use_all() {
        let bindings = vec![
            ChordBinding::new(ButtonChord::from_keys(&[KeyCode::KeyA]), "A"),
            ChordBinding::new(
                ButtonChord::from_keys(&[KeyCode::ControlLeft, KeyCode::KeyA]),
                "Ctrl+A",
            ),
        ];

        let result = resolve_clashes(&bindings, ClashStrategy::UseAll);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_modifier_key() {
        // Can't test is_pressed without a World, but can test enum variants
        assert_ne!(ModifierKey::Control, ModifierKey::Shift);
        assert_ne!(ModifierKey::Alt, ModifierKey::Super);
    }
}
