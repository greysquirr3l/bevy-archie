//! Virtual on-screen keyboard for controller text input.
//!
//! This module provides a controller-friendly on-screen keyboard
//! for entering text when a physical keyboard is not available.

use bevy::prelude::*;

/// The current state of the virtual keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, States, Hash)]
pub enum VirtualKeyboardState {
    /// Keyboard is hidden.
    #[default]
    Hidden,
    /// Keyboard is visible and accepting input.
    Visible,
}

/// Configuration for the virtual keyboard.
#[derive(Debug, Clone, Resource)]
pub struct VirtualKeyboardConfig {
    /// The characters in row 1 (top row).
    pub row1: String,
    /// The characters in row 2.
    pub row2: String,
    /// The characters in row 3.
    pub row3: String,
    /// The characters in the number row.
    pub numbers: String,
    /// Symbol characters (page 2).
    pub symbols1: String,
    pub symbols2: String,
    pub symbols3: String,
    /// Key width in pixels.
    pub key_width: f32,
    /// Key height in pixels.
    pub key_height: f32,
    /// Spacing between keys.
    pub key_spacing: f32,
    /// Background color.
    pub background_color: Color,
    /// Key color.
    pub key_color: Color,
    /// Key hover color.
    pub key_hover_color: Color,
    /// Key pressed color.
    pub key_pressed_color: Color,
    /// Text color.
    pub text_color: Color,
}

impl Default for VirtualKeyboardConfig {
    fn default() -> Self {
        Self {
            row1: "qwertyuiop".to_string(),
            row2: "asdfghjkl'".to_string(),
            row3: "zxcvbnm,.?".to_string(),
            numbers: "1234567890".to_string(),
            symbols1: "!@#$%^&*()".to_string(),
            symbols2: "`~_-+=:;'\"".to_string(),
            symbols3: "<>,.?/\\|".to_string(),
            key_width: 60.0,
            key_height: 50.0,
            key_spacing: 5.0,
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.9),
            key_color: Color::srgb(0.2, 0.2, 0.2),
            key_hover_color: Color::srgb(0.3, 0.3, 0.3),
            key_pressed_color: Color::srgb(0.1, 0.4, 0.8),
            text_color: Color::WHITE,
        }
    }
}

/// Resource tracking the virtual keyboard input state.
#[derive(Debug, Clone, Default, Resource)]
pub struct VirtualKeyboard {
    /// Current input buffer.
    pub buffer: String,
    /// Maximum length of input.
    pub max_length: Option<usize>,
    /// Cursor position in the buffer.
    pub cursor: usize,
    /// Whether shift is active.
    pub shift_active: bool,
    /// Current page (0 = letters, 1 = symbols).
    pub current_page: usize,
    /// Currently focused key index.
    pub focused_key: usize,
    /// Number of keys per row.
    pub keys_per_row: usize,
    /// Prompt text to display.
    pub prompt: String,
    /// Allowed characters (if None, all are allowed).
    pub allow: Option<String>,
    /// Excluded characters.
    pub exclude: Option<String>,
}

impl VirtualKeyboard {
    /// Create a new virtual keyboard with a prompt.
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            keys_per_row: 10,
            ..default()
        }
    }

    /// Set the maximum input length.
    pub fn with_max_length(mut self, length: usize) -> Self {
        self.max_length = Some(length);
        self
    }

    /// Set allowed characters.
    pub fn with_allow(mut self, chars: impl Into<String>) -> Self {
        self.allow = Some(chars.into());
        self
    }

    /// Set excluded characters.
    pub fn with_exclude(mut self, chars: impl Into<String>) -> Self {
        self.exclude = Some(chars.into());
        self
    }

    /// Set initial value.
    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.buffer = value.into();
        self.cursor = self.buffer.len();
        self
    }

    /// Check if a character is allowed.
    pub fn is_char_allowed(&self, c: char) -> bool {
        if let Some(ref allow) = self.allow {
            if !allow.contains(c) {
                return false;
            }
        }
        if let Some(ref exclude) = self.exclude {
            if exclude.contains(c) {
                return false;
            }
        }
        true
    }

    /// Add a character at the cursor position.
    pub fn add_char(&mut self, c: char) {
        if !self.is_char_allowed(c) {
            return;
        }

        if let Some(max) = self.max_length {
            if self.buffer.len() >= max {
                return;
            }
        }

        let c = if self.shift_active {
            c.to_uppercase().next().unwrap_or(c)
        } else {
            c
        };

        self.buffer.insert(self.cursor, c);
        self.cursor += 1;

        // Auto-disable shift after typing
        self.shift_active = false;
    }

    /// Remove the character before the cursor.
    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.buffer.remove(self.cursor);
        }
    }

    /// Move cursor left.
    pub fn cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// Move cursor right.
    pub fn cursor_right(&mut self) {
        if self.cursor < self.buffer.len() {
            self.cursor += 1;
        }
    }

    /// Toggle shift state.
    pub fn toggle_shift(&mut self) {
        self.shift_active = !self.shift_active;
    }

    /// Toggle between letter and symbol pages.
    pub fn toggle_page(&mut self) {
        self.current_page = if self.current_page == 0 { 1 } else { 0 };
    }

    /// Add a space.
    pub fn add_space(&mut self) {
        self.add_char(' ');
    }

    /// Clear the buffer.
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.cursor = 0;
    }

    /// Get the current value.
    pub fn value(&self) -> &str {
        &self.buffer
    }

    /// Move focus to adjacent key.
    pub fn move_focus(&mut self, direction: FocusDirection, total_keys: usize) {
        match direction {
            FocusDirection::Up => {
                if self.focused_key >= self.keys_per_row {
                    self.focused_key -= self.keys_per_row;
                }
            }
            FocusDirection::Down => {
                if self.focused_key + self.keys_per_row < total_keys {
                    self.focused_key += self.keys_per_row;
                }
            }
            FocusDirection::Left => {
                if self.focused_key > 0 {
                    self.focused_key -= 1;
                }
            }
            FocusDirection::Right => {
                if self.focused_key + 1 < total_keys {
                    self.focused_key += 1;
                }
            }
        }
    }
}

/// Direction for focus movement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Event to show the virtual keyboard.
#[derive(Debug, Clone, Event)]
pub struct ShowVirtualKeyboard {
    /// The keyboard configuration.
    pub keyboard: VirtualKeyboard,
}

/// Event to hide the virtual keyboard.
#[derive(Debug, Clone, Event)]
pub struct HideVirtualKeyboard;

/// Event fired when input is confirmed.
#[derive(Debug, Clone, Event)]
pub struct VirtualKeyboardEvent {
    /// The final input value.
    pub value: String,
    /// Whether input was confirmed (true) or cancelled (false).
    pub confirmed: bool,
}

/// Component marking an entity as a virtual keyboard key.
#[derive(Debug, Clone, Component)]
pub struct VirtualKey {
    /// The character this key types.
    pub character: char,
    /// The index of this key.
    pub index: usize,
}

/// Component marking the virtual keyboard root.
#[derive(Debug, Clone, Component)]
pub struct VirtualKeyboardRoot;

/// Component marking the input display.
#[derive(Debug, Clone, Component)]
pub struct VirtualKeyboardInput;

/// System to handle showing the virtual keyboard.
pub fn handle_show_keyboard(
    mut events: EventReader<ShowVirtualKeyboard>,
    mut keyboard: ResMut<VirtualKeyboard>,
    mut next_state: ResMut<NextState<VirtualKeyboardState>>,
) {
    for event in events.read() {
        *keyboard = event.keyboard.clone();
        next_state.set(VirtualKeyboardState::Visible);
    }
}

/// System to handle hiding the virtual keyboard.
pub fn handle_hide_keyboard(
    mut events: EventReader<HideVirtualKeyboard>,
    mut next_state: ResMut<NextState<VirtualKeyboardState>>,
) {
    for _ in events.read() {
        next_state.set(VirtualKeyboardState::Hidden);
    }
}

/// System to handle keyboard input from controller.
pub fn handle_keyboard_input(
    mut keyboard: ResMut<VirtualKeyboard>,
    mut keyboard_events: EventWriter<VirtualKeyboardEvent>,
    mut hide_events: EventWriter<HideVirtualKeyboard>,
    gamepads: Query<&Gamepad>,
    config: Res<VirtualKeyboardConfig>,
) {
    for gamepad in gamepads.iter() {
        // D-pad navigation
        if gamepad.just_pressed(GamepadButton::DPadUp) {
            keyboard.move_focus(FocusDirection::Up, 40); // Approximate total keys
        }
        if gamepad.just_pressed(GamepadButton::DPadDown) {
            keyboard.move_focus(FocusDirection::Down, 40);
        }
        if gamepad.just_pressed(GamepadButton::DPadLeft) {
            keyboard.move_focus(FocusDirection::Left, 40);
        }
        if gamepad.just_pressed(GamepadButton::DPadRight) {
            keyboard.move_focus(FocusDirection::Right, 40);
        }

        // Confirm key press (A button)
        if gamepad.just_pressed(GamepadButton::South) {
            // Get the character at focused_key and add it
            let rows = if keyboard.current_page == 0 {
                vec![&config.numbers, &config.row1, &config.row2, &config.row3]
            } else {
                vec![
                    &config.numbers,
                    &config.symbols1,
                    &config.symbols2,
                    &config.symbols3,
                ]
            };

            let mut current_index = 0;
            for row in rows {
                for c in row.chars() {
                    if current_index == keyboard.focused_key {
                        keyboard.add_char(c);
                        break;
                    }
                    current_index += 1;
                }
            }
        }

        // Backspace (X button)
        if gamepad.just_pressed(GamepadButton::West) {
            keyboard.backspace();
        }

        // Space (Y button)
        if gamepad.just_pressed(GamepadButton::North) {
            keyboard.add_space();
        }

        // Cancel (B button)
        if gamepad.just_pressed(GamepadButton::East) {
            keyboard_events.write(VirtualKeyboardEvent {
                value: keyboard.buffer.clone(),
                confirmed: false,
            });
            hide_events.write(HideVirtualKeyboard);
        }

        // Confirm input (Start button)
        if gamepad.just_pressed(GamepadButton::Start) {
            keyboard_events.write(VirtualKeyboardEvent {
                value: keyboard.buffer.clone(),
                confirmed: true,
            });
            hide_events.write(HideVirtualKeyboard);
        }

        // Toggle shift (Left trigger)
        if gamepad.just_pressed(GamepadButton::LeftTrigger2) {
            keyboard.toggle_shift();
        }

        // Toggle page (Left stick press)
        if gamepad.just_pressed(GamepadButton::LeftThumb) {
            keyboard.toggle_page();
        }

        // Cursor movement (bumpers)
        if gamepad.just_pressed(GamepadButton::LeftTrigger) {
            keyboard.cursor_left();
        }
        if gamepad.just_pressed(GamepadButton::RightTrigger) {
            keyboard.cursor_right();
        }
    }
}

/// Add virtual keyboard systems to the app.
pub(crate) fn add_virtual_keyboard_systems(app: &mut App) {
    app.init_state::<VirtualKeyboardState>()
        .init_resource::<VirtualKeyboard>()
        .init_resource::<VirtualKeyboardConfig>()
        .add_event::<ShowVirtualKeyboard>()
        .add_event::<HideVirtualKeyboard>()
        .add_event::<VirtualKeyboardEvent>()
        .add_systems(
            Update,
            (handle_show_keyboard, handle_hide_keyboard)
                .run_if(on_event::<ShowVirtualKeyboard>.or(on_event::<HideVirtualKeyboard>)),
        )
        .add_systems(
            Update,
            handle_keyboard_input.run_if(in_state(VirtualKeyboardState::Visible)),
        );
}
