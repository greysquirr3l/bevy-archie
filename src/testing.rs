//! Testing utilities for mocking controller input.
//!
//! This module provides tools for testing game logic that depends on
//! controller input without needing physical hardware.
//!
//! # Example
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use bevy_archie::testing::{MockInput, MockInputPlugin};
//!
//! // In your test setup
//! let mut app = App::new();
//! app.add_plugins(MockInputPlugin);
//!
//! // Press a button
//! app.world_mut().resource_mut::<MockInput>().press_key(KeyCode::Space);
//!
//! // Run your systems
//! app.update();
//!
//! // Verify behavior...
//! ```

use bevy::prelude::*;
use std::collections::HashSet;

/// A resource for mocking input state in tests.
///
/// This allows you to simulate button presses, releases, and axis values
/// without needing physical input devices.
#[derive(Resource, Debug, Default)]
pub struct MockInput {
    /// Currently pressed keyboard keys
    pressed_keys: HashSet<KeyCode>,
    /// Keys that were just pressed this frame
    just_pressed_keys: HashSet<KeyCode>,
    /// Keys that were just released this frame
    just_released_keys: HashSet<KeyCode>,

    /// Currently pressed mouse buttons
    pressed_mouse_buttons: HashSet<MouseButton>,
    /// Mouse buttons just pressed this frame
    just_pressed_mouse_buttons: HashSet<MouseButton>,
    /// Mouse buttons just released this frame
    just_released_mouse_buttons: HashSet<MouseButton>,

    /// Simulated gamepad axis values (axis index -> value)
    axis_values: Vec<f32>,
    /// Simulated gamepad button states
    pressed_gamepad_buttons: HashSet<GamepadButton>,
    /// Gamepad buttons just pressed this frame
    just_pressed_gamepad_buttons: HashSet<GamepadButton>,
    /// Gamepad buttons just released this frame
    just_released_gamepad_buttons: HashSet<GamepadButton>,

    /// Mouse position
    mouse_position: Vec2,
    /// Mouse delta since last frame
    mouse_delta: Vec2,
    /// Scroll wheel delta
    scroll_delta: Vec2,
}

impl MockInput {
    /// Create a new mock input state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            axis_values: vec![0.0; 6], // Common axes: LeftX, LeftY, RightX, RightY, LeftTrigger, RightTrigger
            ..Default::default()
        }
    }

    /// Clear all "just pressed/released" states for the next frame.
    ///
    /// Call this at the start of each frame in your test loop.
    pub fn clear_just_states(&mut self) {
        self.just_pressed_keys.clear();
        self.just_released_keys.clear();
        self.just_pressed_mouse_buttons.clear();
        self.just_released_mouse_buttons.clear();
        self.just_pressed_gamepad_buttons.clear();
        self.just_released_gamepad_buttons.clear();
        self.mouse_delta = Vec2::ZERO;
        self.scroll_delta = Vec2::ZERO;
    }

    /// Reset all input state.
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    // === Keyboard ===

    /// Simulate pressing a key.
    pub fn press_key(&mut self, key: KeyCode) {
        if self.pressed_keys.insert(key) {
            self.just_pressed_keys.insert(key);
        }
    }

    /// Simulate releasing a key.
    pub fn release_key(&mut self, key: KeyCode) {
        if self.pressed_keys.remove(&key) {
            self.just_released_keys.insert(key);
        }
    }

    /// Check if a key is currently pressed.
    #[must_use]
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }

    /// Check if a key was just pressed this frame.
    #[must_use]
    pub fn is_key_just_pressed(&self, key: KeyCode) -> bool {
        self.just_pressed_keys.contains(&key)
    }

    /// Check if a key was just released this frame.
    #[must_use]
    pub fn is_key_just_released(&self, key: KeyCode) -> bool {
        self.just_released_keys.contains(&key)
    }

    // === Mouse ===

    /// Simulate pressing a mouse button.
    pub fn press_mouse(&mut self, button: MouseButton) {
        if self.pressed_mouse_buttons.insert(button) {
            self.just_pressed_mouse_buttons.insert(button);
        }
    }

    /// Simulate releasing a mouse button.
    pub fn release_mouse(&mut self, button: MouseButton) {
        if self.pressed_mouse_buttons.remove(&button) {
            self.just_released_mouse_buttons.insert(button);
        }
    }

    /// Check if a mouse button is pressed.
    #[must_use]
    pub fn is_mouse_pressed(&self, button: MouseButton) -> bool {
        self.pressed_mouse_buttons.contains(&button)
    }

    /// Check if a mouse button was just pressed.
    #[must_use]
    pub fn is_mouse_just_pressed(&self, button: MouseButton) -> bool {
        self.just_pressed_mouse_buttons.contains(&button)
    }

    /// Set the mouse position.
    pub fn set_mouse_position(&mut self, pos: Vec2) {
        self.mouse_delta = pos - self.mouse_position;
        self.mouse_position = pos;
    }

    /// Get the current mouse position.
    #[must_use]
    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }

    /// Get mouse movement delta.
    #[must_use]
    pub fn mouse_delta(&self) -> Vec2 {
        self.mouse_delta
    }

    /// Set scroll wheel delta.
    pub fn set_scroll(&mut self, delta: Vec2) {
        self.scroll_delta = delta;
    }

    /// Get scroll wheel delta.
    #[must_use]
    pub fn scroll_delta(&self) -> Vec2 {
        self.scroll_delta
    }

    // === Gamepad ===

    /// Simulate pressing a gamepad button.
    pub fn press_gamepad(&mut self, button: GamepadButton) {
        if self.pressed_gamepad_buttons.insert(button) {
            self.just_pressed_gamepad_buttons.insert(button);
        }
    }

    /// Simulate releasing a gamepad button.
    pub fn release_gamepad(&mut self, button: GamepadButton) {
        if self.pressed_gamepad_buttons.remove(&button) {
            self.just_released_gamepad_buttons.insert(button);
        }
    }

    /// Check if a gamepad button is pressed.
    #[must_use]
    pub fn is_gamepad_pressed(&self, button: GamepadButton) -> bool {
        self.pressed_gamepad_buttons.contains(&button)
    }

    /// Check if a gamepad button was just pressed.
    #[must_use]
    pub fn is_gamepad_just_pressed(&self, button: GamepadButton) -> bool {
        self.just_pressed_gamepad_buttons.contains(&button)
    }

    /// Set a gamepad axis value.
    pub fn set_axis(&mut self, axis: GamepadAxis, value: f32) {
        let index = axis_to_index(axis);
        if index < self.axis_values.len() {
            self.axis_values[index] = value.clamp(-1.0, 1.0);
        }
    }

    /// Get a gamepad axis value.
    #[must_use]
    pub fn get_axis(&self, axis: GamepadAxis) -> f32 {
        let index = axis_to_index(axis);
        self.axis_values.get(index).copied().unwrap_or(0.0)
    }

    /// Set the left stick position.
    pub fn set_left_stick(&mut self, x: f32, y: f32) {
        self.set_axis(GamepadAxis::LeftStickX, x);
        self.set_axis(GamepadAxis::LeftStickY, y);
    }

    /// Set the right stick position.
    pub fn set_right_stick(&mut self, x: f32, y: f32) {
        self.set_axis(GamepadAxis::RightStickX, x);
        self.set_axis(GamepadAxis::RightStickY, y);
    }

    /// Get the left stick as a Vec2.
    #[must_use]
    pub fn left_stick(&self) -> Vec2 {
        Vec2::new(
            self.get_axis(GamepadAxis::LeftStickX),
            self.get_axis(GamepadAxis::LeftStickY),
        )
    }

    /// Get the right stick as a Vec2.
    #[must_use]
    pub fn right_stick(&self) -> Vec2 {
        Vec2::new(
            self.get_axis(GamepadAxis::RightStickX),
            self.get_axis(GamepadAxis::RightStickY),
        )
    }
}

/// Convert gamepad axis to index in our internal array.
fn axis_to_index(axis: GamepadAxis) -> usize {
    match axis {
        GamepadAxis::LeftStickX => 0,
        GamepadAxis::LeftStickY => 1,
        GamepadAxis::RightStickX => 2,
        GamepadAxis::RightStickY => 3,
        GamepadAxis::LeftZ => 4,
        GamepadAxis::RightZ => 5,
        GamepadAxis::Other(_) => 0,
    }
}

/// A plugin that adds mock input support.
///
/// This plugin initializes the [`MockInput`] resource and adds
/// systems to synchronize it with Bevy's input resources.
pub struct MockInputPlugin;

impl Plugin for MockInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MockInput>()
            .add_systems(PreUpdate, sync_mock_input_to_bevy);
    }
}

/// Sync mock input state to Bevy's input resources.
fn sync_mock_input_to_bevy(
    mock: Res<MockInput>,
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
    mut mouse_buttons: ResMut<ButtonInput<MouseButton>>,
) {
    // Sync keyboard
    for key in &mock.just_pressed_keys {
        keyboard.press(*key);
    }
    for key in &mock.just_released_keys {
        keyboard.release(*key);
    }

    // Sync mouse buttons
    for button in &mock.just_pressed_mouse_buttons {
        mouse_buttons.press(*button);
    }
    for button in &mock.just_released_mouse_buttons {
        mouse_buttons.release(*button);
    }
}

/// Builder for creating complex mock input sequences.
#[derive(Debug, Default)]
pub struct MockInputSequence {
    frames: Vec<MockInputFrame>,
}

/// A single frame of mock input.
#[derive(Debug, Default)]
pub struct MockInputFrame {
    /// Keys to press
    pub press_keys: Vec<KeyCode>,
    /// Keys to release
    pub release_keys: Vec<KeyCode>,
    /// Gamepad buttons to press
    pub press_gamepad: Vec<GamepadButton>,
    /// Gamepad buttons to release
    pub release_gamepad: Vec<GamepadButton>,
    /// Axis values to set
    pub axis_values: Vec<(GamepadAxis, f32)>,
    /// Mouse buttons to press
    pub press_mouse: Vec<MouseButton>,
    /// Mouse buttons to release
    pub release_mouse: Vec<MouseButton>,
    /// Mouse position to set
    pub mouse_position: Option<Vec2>,
}

impl MockInputSequence {
    /// Create a new empty sequence.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a frame to the sequence.
    #[must_use]
    pub fn frame(mut self, frame: MockInputFrame) -> Self {
        self.frames.push(frame);
        self
    }

    /// Add a frame that presses a key.
    #[must_use]
    pub fn press_key(mut self, key: KeyCode) -> Self {
        self.frames.push(MockInputFrame {
            press_keys: vec![key],
            ..Default::default()
        });
        self
    }

    /// Add a frame that releases a key.
    #[must_use]
    pub fn release_key(mut self, key: KeyCode) -> Self {
        self.frames.push(MockInputFrame {
            release_keys: vec![key],
            ..Default::default()
        });
        self
    }

    /// Add an empty frame (wait frame).
    #[must_use]
    pub fn wait(mut self) -> Self {
        self.frames.push(MockInputFrame::default());
        self
    }

    /// Add multiple wait frames.
    #[must_use]
    pub fn wait_frames(mut self, count: usize) -> Self {
        for _ in 0..count {
            self.frames.push(MockInputFrame::default());
        }
        self
    }

    /// Get the number of frames in the sequence.
    #[must_use]
    pub fn len(&self) -> usize {
        self.frames.len()
    }

    /// Check if the sequence is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    /// Get the frames.
    #[must_use]
    pub fn frames(&self) -> &[MockInputFrame] {
        &self.frames
    }
}

/// Helper for running mock input sequences in tests.
pub struct MockInputRunner<'a> {
    mock: &'a mut MockInput,
    sequence: &'a MockInputSequence,
    current_frame: usize,
}

impl<'a> MockInputRunner<'a> {
    /// Create a new runner.
    pub fn new(mock: &'a mut MockInput, sequence: &'a MockInputSequence) -> Self {
        Self {
            mock,
            sequence,
            current_frame: 0,
        }
    }

    /// Create a new runner starting at a specific frame index.
    pub fn with_index(
        mock: &'a mut MockInput,
        sequence: &'a MockInputSequence,
        index: usize,
    ) -> Self {
        Self {
            mock,
            sequence,
            current_frame: index,
        }
    }

    /// Apply the next frame of input.
    ///
    /// Returns false when the sequence is complete.
    pub fn next_frame(&mut self) -> bool {
        if self.current_frame >= self.sequence.len() {
            return false;
        }

        self.mock.clear_just_states();

        let frame = &self.sequence.frames[self.current_frame];

        for key in &frame.press_keys {
            self.mock.press_key(*key);
        }
        for key in &frame.release_keys {
            self.mock.release_key(*key);
        }
        for button in &frame.press_gamepad {
            self.mock.press_gamepad(*button);
        }
        for button in &frame.release_gamepad {
            self.mock.release_gamepad(*button);
        }
        for (axis, value) in &frame.axis_values {
            self.mock.set_axis(*axis, *value);
        }
        for button in &frame.press_mouse {
            self.mock.press_mouse(*button);
        }
        for button in &frame.release_mouse {
            self.mock.release_mouse(*button);
        }
        if let Some(pos) = frame.mouse_position {
            self.mock.set_mouse_position(pos);
        }

        self.current_frame += 1;
        true
    }

    /// Check if the sequence is complete.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.current_frame >= self.sequence.len()
    }

    /// Reset to the beginning.
    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.mock.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_input_keyboard() {
        let mut mock = MockInput::new();

        mock.press_key(KeyCode::Space);
        assert!(mock.is_key_pressed(KeyCode::Space));
        assert!(mock.is_key_just_pressed(KeyCode::Space));

        mock.clear_just_states();
        assert!(mock.is_key_pressed(KeyCode::Space));
        assert!(!mock.is_key_just_pressed(KeyCode::Space));

        mock.release_key(KeyCode::Space);
        assert!(!mock.is_key_pressed(KeyCode::Space));
        assert!(mock.is_key_just_released(KeyCode::Space));
    }

    #[test]
    fn test_mock_input_gamepad_axis() {
        let mut mock = MockInput::new();

        mock.set_left_stick(0.5, -0.7);
        let stick = mock.left_stick();
        assert!((stick.x - 0.5).abs() < 0.001);
        assert!((stick.y - (-0.7)).abs() < 0.001);
    }

    #[test]
    fn test_mock_input_sequence() {
        let sequence = MockInputSequence::new()
            .press_key(KeyCode::Space)
            .wait()
            .release_key(KeyCode::Space);

        assert_eq!(sequence.len(), 3);

        let mut mock = MockInput::new();

        // Frame 1: Press Space
        {
            let mut runner = MockInputRunner::new(&mut mock, &sequence);
            assert!(runner.next_frame());
        }
        assert!(mock.is_key_pressed(KeyCode::Space));

        // Frame 2: Wait
        {
            let mut runner = MockInputRunner::with_index(&mut mock, &sequence, 1);
            assert!(runner.next_frame());
        }
        assert!(mock.is_key_pressed(KeyCode::Space));

        // Frame 3: Release Space
        {
            let mut runner = MockInputRunner::with_index(&mut mock, &sequence, 2);
            assert!(runner.next_frame());
        }
        assert!(!mock.is_key_pressed(KeyCode::Space));

        // Sequence complete
        {
            let mut runner = MockInputRunner::with_index(&mut mock, &sequence, 3);
            assert!(!runner.next_frame());
            assert!(runner.is_complete());
        }
    }

    #[test]
    fn test_mock_input_mouse() {
        let mut mock = MockInput::new();

        mock.set_mouse_position(Vec2::new(100.0, 200.0));
        assert_eq!(mock.mouse_position(), Vec2::new(100.0, 200.0));

        mock.set_mouse_position(Vec2::new(150.0, 250.0));
        assert_eq!(mock.mouse_delta(), Vec2::new(50.0, 50.0));
    }
}
