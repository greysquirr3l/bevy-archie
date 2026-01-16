//! Action modifiers for advanced input detection.
//!
//! This module provides hold, double-tap, long-press, and other
//! input modifiers for the action system.

use bevy::prelude::*;

use crate::actions::GameAction;

/// Action modifier types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ActionModifier {
    /// Tap (quick press and release).
    Tap,
    /// Hold for duration.
    Hold,
    /// Double tap.
    DoubleTap,
    /// Long press.
    LongPress,
    /// Released (action released event).
    Released,
}

/// State for tracking action modifiers.
#[derive(Debug, Clone, Default, Resource)]
pub struct ActionModifierState {
    /// Currently held actions with timestamps.
    pub held_actions: Vec<(GameAction, f64)>,
    /// Recent taps for double-tap detection.
    pub recent_taps: Vec<(GameAction, f64)>,
    /// Configuration.
    pub config: ModifierConfig,
}

/// Configuration for action modifiers.
#[derive(Debug, Clone, Reflect)]
pub struct ModifierConfig {
    /// Duration for hold detection (seconds).
    pub hold_duration: f32,
    /// Duration for long press detection (seconds).
    pub long_press_duration: f32,
    /// Maximum time between double taps (seconds).
    pub double_tap_window: f32,
    /// Maximum time for a tap (seconds).
    pub tap_duration: f32,
}

impl Default for ModifierConfig {
    fn default() -> Self {
        Self {
            hold_duration: 0.2,
            long_press_duration: 0.8,
            double_tap_window: 0.3,
            tap_duration: 0.2,
        }
    }
}

/// Event fired when a modified action is detected.
#[derive(Debug, Clone, Message)]
pub struct ModifiedActionEvent {
    /// The base action.
    pub action: GameAction,
    /// The modifier applied.
    pub modifier: ActionModifier,
    /// Gamepad that triggered it (if applicable).
    pub gamepad: Option<Entity>,
    /// Duration held (for Hold/LongPress).
    pub duration: f32,
}

impl ActionModifierState {
    /// Record an action press.
    pub fn record_press(&mut self, action: GameAction, time: f64) {
        self.held_actions.push((action, time));
    }

    /// Record an action release and check for modifiers.
    #[must_use]
    pub fn record_release(&mut self, action: GameAction, time: f64) -> Vec<ActionModifier> {
        let mut detected = Vec::new();

        // Find the held action
        if let Some(idx) = self.held_actions.iter().position(|(a, _)| *a == action) {
            let (_, press_time) = self.held_actions.remove(idx);
            #[allow(clippy::cast_possible_truncation)]
            let duration = (time - press_time) as f32;

            // Check for tap
            if duration <= self.config.tap_duration {
                // Check for double tap
                if let Some(tap_idx) = self.recent_taps.iter().position(|(a, t)| {
                    *a == action && (time - t) < f64::from(self.config.double_tap_window)
                }) {
                    self.recent_taps.remove(tap_idx);
                    detected.push(ActionModifier::DoubleTap);
                } else {
                    self.recent_taps.push((action, time));
                    detected.push(ActionModifier::Tap);
                }
            }
            // Check for long press
            else if duration >= self.config.long_press_duration {
                detected.push(ActionModifier::LongPress);
            }
            // Check for hold
            else if duration >= self.config.hold_duration {
                detected.push(ActionModifier::Hold);
            }

            detected.push(ActionModifier::Released);
        }

        // Clean old taps
        self.recent_taps
            .retain(|(_, t)| (time - t) < f64::from(self.config.double_tap_window));

        detected
    }

    /// Check for held actions that exceeded long press duration.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn check_long_press(&mut self, time: f64) -> Vec<GameAction> {
        self.held_actions
            .iter()
            .filter(|(_, press_time)| (time - press_time) as f32 >= self.config.long_press_duration)
            .map(|(action, _)| *action)
            .collect()
    }
}

/// System to detect action modifiers.
pub fn detect_action_modifiers(
    mut modifier_state: ResMut<ActionModifierState>,
    _keyboard: Res<ButtonInput<KeyCode>>,
    _mouse: Res<ButtonInput<MouseButton>>,
    _gamepads: Query<&Gamepad>,
    time: Res<Time>,
    mut modifier_events: MessageWriter<ModifiedActionEvent>,
) {
    let current_time = time.elapsed_secs_f64();

    // This would integrate with the action system to detect which actions were pressed/released
    // For now, this is a placeholder structure

    // Check for long presses
    for action in modifier_state.check_long_press(current_time) {
        modifier_events.write(ModifiedActionEvent {
            action,
            modifier: ActionModifier::LongPress,
            gamepad: None,
            duration: modifier_state.config.long_press_duration,
        });
    }
}

/// Plugin for registering action modifier types.
pub(crate) fn register_action_modifier_types(app: &mut App) {
    app.register_type::<ActionModifier>()
        .register_type::<ModifierConfig>()
        .init_resource::<ActionModifierState>()
        .add_message::<ModifiedActionEvent>();
}

/// Add action modifier systems to the app.
pub(crate) fn add_action_modifier_systems(app: &mut App) {
    app.add_systems(Update, detect_action_modifiers);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_modifier_variants() {
        assert_ne!(ActionModifier::Tap, ActionModifier::Hold);
        assert_ne!(ActionModifier::DoubleTap, ActionModifier::LongPress);
        assert_ne!(ActionModifier::Released, ActionModifier::Tap);
    }

    #[test]
    fn test_modifier_config_default() {
        let config = ModifierConfig::default();
        assert!(config.hold_duration > 0.0);
        assert!(config.long_press_duration > 0.0);
        assert!(config.double_tap_window > 0.0);
        assert!(config.tap_duration > 0.0);
        assert!(config.long_press_duration > config.hold_duration);
    }

    #[test]
    fn test_action_modifier_state_default() {
        let state = ActionModifierState::default();
        assert_eq!(state.held_actions.len(), 0);
        assert_eq!(state.recent_taps.len(), 0);
    }

    #[test]
    fn test_action_modifier_state_record_press() {
        let mut state = ActionModifierState::default();
        state.record_press(GameAction::Confirm, 1.0);
        state.record_press(GameAction::Cancel, 1.5);

        assert_eq!(state.held_actions.len(), 2);
        assert_eq!(state.held_actions[0].0, GameAction::Confirm);
        assert_eq!(state.held_actions[1].0, GameAction::Cancel);
    }

    #[test]
    fn test_modified_action_event_creation() {
        let event = ModifiedActionEvent {
            action: GameAction::Primary,
            modifier: ActionModifier::DoubleTap,
            gamepad: Some(Entity::from_bits(42)),
            duration: 0.15,
        };

        assert_eq!(event.modifier, ActionModifier::DoubleTap);
        assert_eq!(event.gamepad, Some(Entity::from_bits(42)));
        assert_eq!(event.duration, 0.15);
    }

    #[test]
    fn test_action_modifier_state_record_release_tap() {
        let mut state = ActionModifierState::default();
        state.record_press(GameAction::Primary, 1.0);
        let modifiers = state.record_release(GameAction::Primary, 1.15); // 0.15s - within tap duration

        assert_eq!(modifiers.len(), 2); // Tap + Released
        assert!(modifiers.contains(&ActionModifier::Tap));
        assert!(modifiers.contains(&ActionModifier::Released));
        assert_eq!(state.recent_taps.len(), 1);
    }

    #[test]
    fn test_action_modifier_state_record_release_double_tap() {
        let mut state = ActionModifierState::default();

        // First tap
        state.record_press(GameAction::Primary, 1.0);
        state.record_release(GameAction::Primary, 1.15);

        // Second tap within window
        state.record_press(GameAction::Primary, 1.2);
        let modifiers = state.record_release(GameAction::Primary, 1.35);

        assert_eq!(modifiers.len(), 2); // DoubleTap + Released
        assert!(modifiers.contains(&ActionModifier::DoubleTap));
        assert!(modifiers.contains(&ActionModifier::Released));
        state.record_press(GameAction::Primary, 1.0);
        let modifiers = state.record_release(GameAction::Primary, 1.25); // 0.25s - longer than tap, matches hold

        assert!(modifiers.contains(&ActionModifier::Hold));
    }

    #[test]
    fn test_action_modifier_state_record_release_long_press() {
        let mut state = ActionModifierState::default();
        state.record_press(GameAction::Primary, 1.0);
        let modifiers = state.record_release(GameAction::Primary, 1.9); // 0.9s - exceeds long_press_duration

        assert!(modifiers.contains(&ActionModifier::LongPress));
    }

    #[test]
    fn test_action_modifier_state_check_long_press() {
        let mut state = ActionModifierState::default();
        state.record_press(GameAction::Primary, 1.0);
        state.record_press(GameAction::Secondary, 1.2);

        let long_presses = state.check_long_press(2.0); // 0.8s default long press duration

        // Primary has been held for 1.0s - should be detected
        assert!(long_presses.contains(&GameAction::Primary));
    }

    #[test]
    fn test_action_modifier_all_variants() {
        let all_modifiers = [
            ActionModifier::Tap,
            ActionModifier::Hold,
            ActionModifier::DoubleTap,
            ActionModifier::LongPress,
            ActionModifier::Released,
        ];

        // Ensure all are unique
        for (i, &mod1) in all_modifiers.iter().enumerate() {
            for (j, &mod2) in all_modifiers.iter().enumerate() {
                if i != j {
                    assert_ne!(mod1, mod2);
                }
            }
        }
    }
}
