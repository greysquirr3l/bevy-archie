//! Developer debugging tools for input visualization and testing.
//!
//! This module provides debugging utilities for visualizing controller
//! input, recording/playback, and automated testing.

use bevy::prelude::*;
use std::collections::VecDeque;

use crate::actions::GameAction;

/// Debug overlay state.
#[derive(Debug, Clone, Default, Resource)]
#[allow(clippy::struct_excessive_bools)]
pub struct InputDebugger {
    /// Whether debugging is enabled.
    pub enabled: bool,
    /// Show input history.
    pub show_history: bool,
    /// Show stick positions.
    pub show_sticks: bool,
    /// Show button states.
    pub show_buttons: bool,
    /// Show gyro data.
    pub show_gyro: bool,
    /// Input history size.
    pub history_size: usize,
}

impl InputDebugger {
    /// Create a new debugger.
    #[must_use]
    pub fn new() -> Self {
        Self {
            enabled: false,
            show_history: true,
            show_sticks: true,
            show_buttons: true,
            show_gyro: false,
            history_size: 20,
        }
    }

    /// Enable all debug features.
    pub fn enable_all(&mut self) {
        self.enabled = true;
        self.show_history = true;
        self.show_sticks = true;
        self.show_buttons = true;
        self.show_gyro = true;
    }
}

/// Input event for recording.
#[derive(Debug, Clone)]
pub struct RecordedInput {
    /// Action performed.
    pub action: GameAction,
    /// Timestamp.
    pub timestamp: f64,
    /// Button state (pressed/released).
    pub pressed: bool,
    /// Analog value (if applicable).
    pub analog_value: Option<f32>,
}

/// Input recording system.
#[derive(Debug, Clone, Default, Resource)]
pub struct InputRecorder {
    /// Whether recording is active.
    pub recording: bool,
    /// Recorded inputs.
    pub recorded: Vec<RecordedInput>,
    /// Recording start time.
    pub start_time: f64,
}

impl InputRecorder {
    /// Start recording.
    pub fn start(&mut self, time: f64) {
        self.recording = true;
        self.recorded.clear();
        self.start_time = time;
    }

    /// Stop recording.
    pub fn stop(&mut self) {
        self.recording = false;
    }

    /// Record an input.
    pub fn record(&mut self, input: RecordedInput) {
        if self.recording {
            self.recorded.push(input);
        }
    }

    /// Get recording duration.
    #[must_use]
    pub fn duration(&self, current_time: f64) -> f64 {
        current_time - self.start_time
    }
}

/// Input playback system.
#[derive(Debug, Clone, Default, Resource)]
pub struct InputPlayback {
    /// Whether playback is active.
    pub playing: bool,
    /// Inputs to play back.
    pub inputs: VecDeque<RecordedInput>,
    /// Playback start time.
    pub start_time: f64,
    /// Current playback index.
    pub current_index: usize,
}

impl InputPlayback {
    /// Start playback.
    pub fn start(&mut self, inputs: Vec<RecordedInput>, time: f64) {
        self.playing = true;
        self.inputs = inputs.into();
        self.start_time = time;
        self.current_index = 0;
    }

    /// Stop playback.
    pub fn stop(&mut self) {
        self.playing = false;
        self.inputs.clear();
    }

    /// Get next inputs to play.
    ///
    /// # Panics
    ///
    /// This method will not panic as the unwrap is guarded by the while condition.
    #[must_use]
    pub fn get_next(&mut self, current_time: f64) -> Vec<RecordedInput> {
        if !self.playing {
            return Vec::new();
        }

        let playback_time = current_time - self.start_time;
        let mut to_play = Vec::new();

        while let Some(input) = self.inputs.front() {
            if input.timestamp <= playback_time {
                to_play.push(self.inputs.pop_front().unwrap());
            } else {
                break;
            }
        }

        if self.inputs.is_empty() {
            self.playing = false;
        }

        to_play
    }
}

/// Command to toggle debug overlay.
#[derive(Debug, Clone, Message)]
pub struct ToggleInputDebug {
    /// Whether to enable or disable.
    pub enable: bool,
}

/// Command to start/stop recording.
#[derive(Debug, Clone, Message)]
pub struct RecordingCommand {
    /// Whether to start or stop.
    pub start: bool,
}

/// Command to start playback.
#[derive(Debug, Clone, Message)]
pub struct PlaybackCommand {
    /// Inputs to play back.
    pub inputs: Vec<RecordedInput>,
}

/// System to handle debug commands.
pub fn handle_debug_commands(
    mut toggle_events: MessageReader<ToggleInputDebug>,
    mut record_events: MessageReader<RecordingCommand>,
    mut playback_events: MessageReader<PlaybackCommand>,
    mut debugger: ResMut<InputDebugger>,
    mut recorder: ResMut<InputRecorder>,
    mut playback: ResMut<InputPlayback>,
    time: Res<Time>,
) {
    for event in toggle_events.read() {
        debugger.enabled = event.enable;
    }

    for event in record_events.read() {
        if event.start {
            recorder.start(time.elapsed_secs_f64());
        } else {
            recorder.stop();
        }
    }

    for event in playback_events.read() {
        playback.start(event.inputs.clone(), time.elapsed_secs_f64());
    }
}

/// System to render debug overlay (would need egui or similar).
pub fn render_debug_overlay(debugger: Res<InputDebugger>, _gamepads: Query<&Gamepad>) {
    if !debugger.enabled {}

    // This would render debug information using egui or a custom UI system
    // For now, this is a placeholder
}

/// Plugin for registering debug types.
pub(crate) fn register_debug_types(app: &mut App) {
    app.init_resource::<InputDebugger>()
        .init_resource::<InputRecorder>()
        .init_resource::<InputPlayback>()
        .add_message::<ToggleInputDebug>()
        .add_message::<RecordingCommand>()
        .add_message::<PlaybackCommand>();
}

/// Add debug systems to the app.
pub(crate) fn add_debug_systems(app: &mut App) {
    app.add_systems(Update, (handle_debug_commands, render_debug_overlay));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_debugger_new() {
        let debugger = InputDebugger::new();
        assert!(!debugger.enabled);
        assert!(debugger.show_history);
        assert!(debugger.show_sticks);
        assert!(debugger.show_buttons);
        assert!(!debugger.show_gyro);
        assert_eq!(debugger.history_size, 20);
    }

    #[test]
    fn test_input_debugger_default() {
        let debugger = InputDebugger::default();
        assert!(!debugger.enabled);
    }

    #[test]
    fn test_input_debugger_enable_all() {
        let mut debugger = InputDebugger::new();
        debugger.enable_all();

        assert!(debugger.enabled);
        assert!(debugger.show_history);
        assert!(debugger.show_sticks);
        assert!(debugger.show_buttons);
        assert!(debugger.show_gyro);
    }

    #[test]
    fn test_recorded_input_creation() {
        let input = RecordedInput {
            action: GameAction::Primary,
            timestamp: 1.5,
            pressed: true,
            analog_value: Some(0.5),
        };

        assert_eq!(input.timestamp, 1.5);
        assert!(input.pressed);
        assert_eq!(input.analog_value, Some(0.5));
    }

    #[test]
    fn test_input_recorder_default() {
        let recorder = InputRecorder::default();
        assert!(!recorder.recording);
        assert_eq!(recorder.recorded.len(), 0);
    }

    #[test]
    fn test_input_playback_default() {
        let playback = InputPlayback::default();
        assert!(!playback.playing);
        assert_eq!(playback.inputs.len(), 0);
        assert_eq!(playback.current_index, 0);
        assert_eq!(playback.start_time, 0.0);
    }

    #[test]
    fn test_toggle_input_debug() {
        let event = ToggleInputDebug { enable: true };
        assert!(event.enable);

        let event2 = ToggleInputDebug { enable: false };
        assert!(!event2.enable);
    }

    #[test]
    fn test_recording_command_creation() {
        let start = RecordingCommand { start: true };
        assert!(start.start);

        let stop = RecordingCommand { start: false };
        assert!(!stop.start);
    }

    #[test]
    fn test_playback_command_creation() {
        let empty = PlaybackCommand { inputs: vec![] };
        assert_eq!(empty.inputs.len(), 0);

        let with_input = PlaybackCommand {
            inputs: vec![RecordedInput {
                timestamp: 0.0,
                action: GameAction::Primary,
                pressed: true,
                analog_value: None,
            }],
        };
        assert_eq!(with_input.inputs.len(), 1);
    }

    // ========== InputRecorder Additional Tests ==========

    #[test]
    fn test_input_recorder_start() {
        let mut recorder = InputRecorder::default();
        recorder.start(10.5);

        assert!(recorder.recording);
        assert_eq!(recorder.start_time, 10.5);
        assert!(recorder.recorded.is_empty());
    }

    #[test]
    fn test_input_recorder_stop() {
        let mut recorder = InputRecorder::default();
        recorder.start(0.0);
        recorder.stop();

        assert!(!recorder.recording);
    }

    #[test]
    fn test_input_recorder_record() {
        let mut recorder = InputRecorder::default();
        recorder.start(0.0);

        let input = RecordedInput {
            action: GameAction::Primary,
            timestamp: 1.0,
            pressed: true,
            analog_value: None,
        };
        recorder.record(input);

        assert_eq!(recorder.recorded.len(), 1);
    }

    #[test]
    fn test_input_recorder_record_not_recording() {
        let mut recorder = InputRecorder::default();
        // Not started

        let input = RecordedInput {
            action: GameAction::Primary,
            timestamp: 1.0,
            pressed: true,
            analog_value: None,
        };
        recorder.record(input);

        // Should not be recorded
        assert!(recorder.recorded.is_empty());
    }

    #[test]
    fn test_input_recorder_duration() {
        let mut recorder = InputRecorder::default();
        recorder.start(10.0);

        let duration = recorder.duration(15.0);
        assert!((duration - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_input_recorder_start_clears_previous() {
        let mut recorder = InputRecorder::default();
        recorder.start(0.0);

        let input = RecordedInput {
            action: GameAction::Secondary,
            timestamp: 1.0,
            pressed: false,
            analog_value: Some(0.7),
        };
        recorder.record(input);
        assert_eq!(recorder.recorded.len(), 1);

        // Start again - should clear
        recorder.start(5.0);
        assert!(recorder.recorded.is_empty());
    }

    // ========== InputPlayback Additional Tests ==========

    #[test]
    fn test_input_playback_start() {
        let mut playback = InputPlayback::default();
        let inputs = vec![
            RecordedInput {
                action: GameAction::Primary,
                timestamp: 0.5,
                pressed: true,
                analog_value: None,
            },
            RecordedInput {
                action: GameAction::Secondary,
                timestamp: 1.0,
                pressed: true,
                analog_value: None,
            },
        ];

        playback.start(inputs, 10.0);

        assert!(playback.playing);
        assert_eq!(playback.start_time, 10.0);
        assert_eq!(playback.inputs.len(), 2);
        assert_eq!(playback.current_index, 0);
    }

    #[test]
    fn test_input_playback_stop() {
        let mut playback = InputPlayback::default();
        playback.start(vec![], 0.0);
        playback.playing = true;

        playback.stop();

        assert!(!playback.playing);
        assert!(playback.inputs.is_empty());
    }

    #[test]
    fn test_input_playback_get_next_not_playing() {
        let mut playback = InputPlayback::default();

        let result = playback.get_next(5.0);
        assert!(result.is_empty());
    }

    #[test]
    fn test_input_playback_get_next_returns_due_inputs() {
        let mut playback = InputPlayback::default();
        let inputs = vec![
            RecordedInput {
                action: GameAction::Primary,
                timestamp: 0.5,
                pressed: true,
                analog_value: None,
            },
            RecordedInput {
                action: GameAction::Secondary,
                timestamp: 1.0,
                pressed: true,
                analog_value: None,
            },
            RecordedInput {
                action: GameAction::Up,
                timestamp: 2.0,
                pressed: true,
                analog_value: None,
            },
        ];

        playback.start(inputs, 10.0);

        // At time 10.7 (playback_time = 0.7), should get first input (0.5)
        let result = playback.get_next(10.7);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].timestamp, 0.5);

        // At time 11.1 (playback_time = 1.1), should get second input (1.0)
        let result = playback.get_next(11.1);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].timestamp, 1.0);

        // At time 15.0 (playback_time = 5.0), should get third input and stop
        let result = playback.get_next(15.0);
        assert_eq!(result.len(), 1);
        assert!(!playback.playing); // Should auto-stop when empty
    }

    #[test]
    fn test_input_playback_stops_when_empty() {
        let mut playback = InputPlayback::default();
        let inputs = vec![RecordedInput {
            action: GameAction::Primary,
            timestamp: 0.1,
            pressed: true,
            analog_value: None,
        }];

        playback.start(inputs, 0.0);
        assert!(playback.playing);

        // Get the only input
        let _ = playback.get_next(1.0);

        // Should have stopped
        assert!(!playback.playing);
    }

    // ========== RecordedInput Tests ==========

    #[test]
    fn test_recorded_input_with_analog() {
        let input = RecordedInput {
            action: GameAction::Up,
            timestamp: 2.5,
            pressed: false,
            analog_value: Some(0.75),
        };

        assert_eq!(input.analog_value, Some(0.75));
        assert!(!input.pressed);
    }

    #[test]
    fn test_recorded_input_without_analog() {
        let input = RecordedInput {
            action: GameAction::Primary,
            timestamp: 0.0,
            pressed: true,
            analog_value: None,
        };

        assert!(input.analog_value.is_none());
    }
}
