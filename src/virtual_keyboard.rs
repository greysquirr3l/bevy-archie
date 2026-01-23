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
    #[must_use]
    pub fn with_max_length(mut self, length: usize) -> Self {
        self.max_length = Some(length);
        self
    }

    /// Set allowed characters.
    #[must_use]
    pub fn with_allow(mut self, chars: impl Into<String>) -> Self {
        self.allow = Some(chars.into());
        self
    }

    /// Set excluded characters.
    #[must_use]
    pub fn with_exclude(mut self, chars: impl Into<String>) -> Self {
        self.exclude = Some(chars.into());
        self
    }

    /// Set initial value.
    #[must_use]
    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.buffer = value.into();
        self.cursor = self.buffer.len();
        self
    }

    /// Check if a character is allowed.
    #[must_use]
    pub fn is_char_allowed(&self, c: char) -> bool {
        if let Some(ref allow) = self.allow
            && !allow.contains(c)
        {
            return false;
        }
        if let Some(ref exclude) = self.exclude
            && exclude.contains(c)
        {
            return false;
        }
        true
    }

    /// Add a character at the cursor position.
    pub fn add_char(&mut self, c: char) {
        if !self.is_char_allowed(c) {
            return;
        }

        if let Some(max) = self.max_length
            && self.buffer.len() >= max
        {
            return;
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
        self.current_page = usize::from(self.current_page == 0);
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
    #[must_use]
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
#[derive(Debug, Clone, Message)]
pub struct ShowVirtualKeyboard {
    /// The keyboard configuration.
    pub keyboard: VirtualKeyboard,
}

/// Event to hide the virtual keyboard.
#[derive(Debug, Clone, Message)]
pub struct HideVirtualKeyboard;

/// Event fired when input is confirmed.
#[derive(Debug, Clone, Message)]
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
    mut events: MessageReader<ShowVirtualKeyboard>,
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
    mut events: MessageReader<HideVirtualKeyboard>,
    mut next_state: ResMut<NextState<VirtualKeyboardState>>,
) {
    for _ in events.read() {
        next_state.set(VirtualKeyboardState::Hidden);
    }
}

/// System to handle keyboard input from controller.
pub fn handle_keyboard_input(
    mut keyboard: ResMut<VirtualKeyboard>,
    mut keyboard_events: MessageWriter<VirtualKeyboardEvent>,
    mut hide_events: MessageWriter<HideVirtualKeyboard>,
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
        .add_message::<ShowVirtualKeyboard>()
        .add_message::<HideVirtualKeyboard>()
        .add_message::<VirtualKeyboardEvent>()
        .add_systems(Update, (handle_show_keyboard, handle_hide_keyboard))
        .add_systems(
            Update,
            handle_keyboard_input.run_if(in_state(VirtualKeyboardState::Visible)),
        );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_keyboard_state_variants() {
        assert_eq!(
            VirtualKeyboardState::default(),
            VirtualKeyboardState::Hidden
        );
        assert_ne!(VirtualKeyboardState::Hidden, VirtualKeyboardState::Visible);
    }

    #[test]
    fn test_virtual_keyboard_config_default() {
        let config = VirtualKeyboardConfig::default();
        assert_eq!(config.row1, "qwertyuiop");
        assert_eq!(config.row2, "asdfghjkl'");
        assert_eq!(config.row3, "zxcvbnm,.?");
        assert_eq!(config.numbers, "1234567890");
        assert_eq!(config.key_width, 60.0);
        assert_eq!(config.key_height, 50.0);
        assert_eq!(config.key_spacing, 5.0);
    }

    #[test]
    fn test_virtual_keyboard_new() {
        let kb = VirtualKeyboard::new("Enter name:");
        assert_eq!(kb.prompt, "Enter name:");
        assert_eq!(kb.keys_per_row, 10);
        assert_eq!(kb.buffer, "");
        assert_eq!(kb.cursor, 0);
        assert!(!kb.shift_active);
        assert_eq!(kb.current_page, 0);
    }

    #[test]
    fn test_virtual_keyboard_with_max_length() {
        let kb = VirtualKeyboard::new("Test").with_max_length(20);
        assert_eq!(kb.max_length, Some(20));
    }

    #[test]
    fn test_virtual_keyboard_with_allow() {
        let kb = VirtualKeyboard::new("Test").with_allow("abc123");
        assert_eq!(kb.allow, Some("abc123".to_string()));
    }

    #[test]
    fn test_virtual_keyboard_with_exclude() {
        let kb = VirtualKeyboard::new("Test").with_exclude("!@#");
        assert_eq!(kb.exclude, Some("!@#".to_string()));
    }

    #[test]
    fn test_virtual_keyboard_with_value() {
        let kb = VirtualKeyboard::new("Test").with_value("initial");
        assert_eq!(kb.buffer, "initial");
        assert_eq!(kb.cursor, 7);
    }

    #[test]
    fn test_virtual_keyboard_is_char_allowed() {
        let mut kb = VirtualKeyboard::new("Test");
        assert!(kb.is_char_allowed('a'));

        kb.allow = Some("abc".to_string());
        assert!(kb.is_char_allowed('a'));
        assert!(!kb.is_char_allowed('z'));

        kb.allow = None;
        kb.exclude = Some("xyz".to_string());
        assert!(kb.is_char_allowed('a'));
        assert!(!kb.is_char_allowed('x'));
    }

    #[test]
    fn test_virtual_keyboard_add_char() {
        let mut kb = VirtualKeyboard::new("Test");
        kb.add_char('h');
        kb.add_char('i');
        assert_eq!(kb.buffer, "hi");
        assert_eq!(kb.cursor, 2);
    }

    #[test]
    fn test_virtual_keyboard_add_char_with_max_length() {
        let mut kb = VirtualKeyboard::new("Test").with_max_length(3);
        kb.add_char('a');
        kb.add_char('b');
        kb.add_char('c');
        kb.add_char('d'); // Should be ignored
        assert_eq!(kb.buffer, "abc");
    }

    #[test]
    fn test_virtual_keyboard_add_char_with_shift() {
        let mut kb = VirtualKeyboard::new("Test");
        kb.shift_active = true;
        kb.add_char('h');
        assert_eq!(kb.buffer, "H");
        assert!(!kb.shift_active); // shift auto-disables
    }

    #[test]
    fn test_virtual_keyboard_backspace() {
        let mut kb = VirtualKeyboard::new("Test").with_value("Hello");
        kb.backspace();
        assert_eq!(kb.buffer, "Hell");
        assert_eq!(kb.cursor, 4);

        // Backspace at start does nothing
        kb.cursor = 0;
        kb.backspace();
        assert_eq!(kb.buffer, "Hell");
    }

    #[test]
    fn test_virtual_keyboard_cursor_movement() {
        let mut kb = VirtualKeyboard::new("Test").with_value("Hello");

        kb.cursor_left();
        assert_eq!(kb.cursor, 4);

        kb.cursor_left();
        assert_eq!(kb.cursor, 3);

        kb.cursor_right();
        assert_eq!(kb.cursor, 4);

        // At start
        kb.cursor = 0;
        kb.cursor_left();
        assert_eq!(kb.cursor, 0);

        // At end
        kb.cursor = kb.buffer.len();
        kb.cursor_right();
        assert_eq!(kb.cursor, kb.buffer.len());
    }

    #[test]
    fn test_virtual_keyboard_toggle_shift() {
        let mut kb = VirtualKeyboard::new("Test");
        assert!(!kb.shift_active);

        kb.toggle_shift();
        assert!(kb.shift_active);

        kb.toggle_shift();
        assert!(!kb.shift_active);
    }

    #[test]
    fn test_virtual_keyboard_toggle_page() {
        let mut kb = VirtualKeyboard::new("Test");
        assert_eq!(kb.current_page, 0);

        kb.toggle_page();
        assert_eq!(kb.current_page, 1);

        kb.toggle_page();
        assert_eq!(kb.current_page, 0);
    }

    #[test]
    fn test_virtual_keyboard_add_space() {
        let mut kb = VirtualKeyboard::new("Test");
        kb.add_char('a');
        kb.add_space();
        kb.add_char('b');
        assert_eq!(kb.buffer, "a b");
    }

    #[test]
    fn test_virtual_keyboard_clear() {
        let mut kb = VirtualKeyboard::new("Test").with_value("Hello");
        kb.clear();
        assert_eq!(kb.buffer, "");
        assert_eq!(kb.cursor, 0);
    }

    #[test]
    fn test_virtual_keyboard_move_focus() {
        let mut kb = VirtualKeyboard::new("Test");
        kb.keys_per_row = 10;
        kb.focused_key = 0;

        // Down
        kb.move_focus(FocusDirection::Down, 40);
        assert_eq!(kb.focused_key, 10);

        // Right
        kb.move_focus(FocusDirection::Right, 40);
        assert_eq!(kb.focused_key, 11);

        // Up
        kb.move_focus(FocusDirection::Up, 40);
        assert_eq!(kb.focused_key, 1);

        // Left
        kb.move_focus(FocusDirection::Left, 40);
        assert_eq!(kb.focused_key, 0);

        // At boundary - left at 0
        kb.move_focus(FocusDirection::Left, 40);
        assert_eq!(kb.focused_key, 0);
    }

    #[test]
    fn test_focus_direction_variants() {
        let all_directions = [
            FocusDirection::Up,
            FocusDirection::Down,
            FocusDirection::Left,
            FocusDirection::Right,
        ];

        // Ensure all are unique
        for (i, &dir1) in all_directions.iter().enumerate() {
            for (j, &dir2) in all_directions.iter().enumerate() {
                if i != j {
                    assert_ne!(dir1, dir2);
                }
            }
        }
    }

    // ========== Additional VirtualKeyboard Tests ==========

    #[test]
    fn test_virtual_keyboard_value() {
        let kb = VirtualKeyboard::new("Test").with_value("Hello World");
        assert_eq!(kb.value(), "Hello World");
    }

    #[test]
    fn test_virtual_keyboard_insert_in_middle() {
        let mut kb = VirtualKeyboard::new("Test").with_value("Hllo");
        kb.cursor = 1;
        kb.add_char('e');
        assert_eq!(kb.buffer, "Hello");
    }

    #[test]
    fn test_virtual_keyboard_add_char_not_allowed() {
        let mut kb = VirtualKeyboard::new("Test").with_allow("abc");
        kb.add_char('z');
        assert_eq!(kb.buffer, "");
    }

    #[test]
    fn test_virtual_keyboard_add_char_excluded() {
        let mut kb = VirtualKeyboard::new("Test").with_exclude("xyz");
        kb.add_char('x');
        assert_eq!(kb.buffer, "");
        kb.add_char('a');
        assert_eq!(kb.buffer, "a");
    }

    #[test]
    fn test_virtual_keyboard_focus_boundary_right() {
        let mut kb = VirtualKeyboard::new("Test");
        kb.keys_per_row = 10;
        kb.focused_key = 39; // Last key
        kb.move_focus(FocusDirection::Right, 40);
        assert_eq!(kb.focused_key, 39); // Should stay at boundary
    }

    #[test]
    fn test_virtual_keyboard_focus_boundary_up() {
        let mut kb = VirtualKeyboard::new("Test");
        kb.keys_per_row = 10;
        kb.focused_key = 5; // First row
        kb.move_focus(FocusDirection::Up, 40);
        assert_eq!(kb.focused_key, 5); // Should stay
    }

    #[test]
    fn test_virtual_keyboard_focus_boundary_down() {
        let mut kb = VirtualKeyboard::new("Test");
        kb.keys_per_row = 10;
        kb.focused_key = 35; // Last row
        kb.move_focus(FocusDirection::Down, 40);
        assert_eq!(kb.focused_key, 35); // Should stay
    }

    // ========== Event Tests ==========

    #[test]
    fn test_show_virtual_keyboard_event() {
        let kb = VirtualKeyboard::new("Enter name:");
        let event = ShowVirtualKeyboard { keyboard: kb };
        assert_eq!(event.keyboard.prompt, "Enter name:");
    }

    #[test]
    fn test_virtual_keyboard_event_confirmed() {
        let event = VirtualKeyboardEvent {
            value: "TestValue".to_string(),
            confirmed: true,
        };
        assert!(event.confirmed);
        assert_eq!(event.value, "TestValue");
    }

    #[test]
    fn test_virtual_keyboard_event_cancelled() {
        let event = VirtualKeyboardEvent {
            value: String::new(),
            confirmed: false,
        };
        assert!(!event.confirmed);
    }

    // ========== VirtualKeyboardConfig Tests ==========

    #[test]
    fn test_virtual_keyboard_config_symbols() {
        let config = VirtualKeyboardConfig::default();
        assert_eq!(config.symbols1, "!@#$%^&*()");
        assert_eq!(config.symbols2, "`~_-+=:;'\"");
        assert_eq!(config.symbols3, "<>,.?/\\|");
    }

    #[test]
    fn test_virtual_keyboard_config_colors() {
        let config = VirtualKeyboardConfig::default();
        // Just ensure colors are set
        assert_ne!(config.key_color, config.key_hover_color);
        assert_ne!(config.key_color, config.key_pressed_color);
    }
}
