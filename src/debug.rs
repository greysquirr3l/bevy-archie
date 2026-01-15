//! Developer debugging tools for input visualization and testing.
//!
//! This module provides debugging utilities for visualizing controller
//! input, recording/playback, and automated testing.

use bevy::prelude::*;
use std::collections::VecDeque;

use crate::actions::GameAction;

/// Debug overlay state.
#[derive(Debug, Clone, Default, Resource)]
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
    if !debugger.enabled {
    }

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
