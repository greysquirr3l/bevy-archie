//! Virtual input composites for creating complex input bindings.
//!
//! This module provides virtual axis and D-pad types that compose
//! multiple button inputs into unified axis values, similar to
//! leafwing-input-manager's approach.
//!
//! # Example
//!
//! ```rust,no_run
//! use bevy_archie::virtual_input::{VirtualAxis, VirtualDPad};
//! use bevy::prelude::*;
//!
//! // Create a virtual axis from two keys
//! let horizontal = VirtualAxis::from_keys(KeyCode::KeyA, KeyCode::KeyD);
//!
//! // Create a virtual D-pad from WASD
//! let movement = VirtualDPad::wasd();
//!
//! // Create a virtual D-pad from arrow keys
//! let arrows = VirtualDPad::arrow_keys();
//! ```

use bevy::prelude::*;

/// A virtual axis that combines two button inputs into a single axis value.
///
/// The axis value ranges from -1.0 to 1.0:
/// - Negative button pressed: -1.0
/// - Positive button pressed: 1.0
/// - Both or neither pressed: 0.0
#[derive(Debug, Clone, Reflect)]
pub struct VirtualAxis {
    /// The button that produces negative values (-1.0)
    pub negative: VirtualButton,
    /// The button that produces positive values (+1.0)
    pub positive: VirtualButton,
}

impl VirtualAxis {
    /// Create a new virtual axis from two buttons.
    #[must_use]
    pub const fn new(negative: VirtualButton, positive: VirtualButton) -> Self {
        Self { negative, positive }
    }

    /// Create a virtual axis from two keyboard keys.
    #[must_use]
    pub const fn from_keys(negative: KeyCode, positive: KeyCode) -> Self {
        Self {
            negative: VirtualButton::Key(negative),
            positive: VirtualButton::Key(positive),
        }
    }

    /// Create a virtual axis from two gamepad buttons.
    #[must_use]
    pub const fn from_gamepad(negative: GamepadButton, positive: GamepadButton) -> Self {
        Self {
            negative: VirtualButton::Gamepad(negative),
            positive: VirtualButton::Gamepad(positive),
        }
    }

    /// Horizontal arrow keys (Left = negative, Right = positive).
    #[must_use]
    pub const fn horizontal_arrow_keys() -> Self {
        Self::from_keys(KeyCode::ArrowLeft, KeyCode::ArrowRight)
    }

    /// Vertical arrow keys (Down = negative, Up = positive).
    #[must_use]
    pub const fn vertical_arrow_keys() -> Self {
        Self::from_keys(KeyCode::ArrowDown, KeyCode::ArrowUp)
    }

    /// A/D keys for horizontal movement.
    #[must_use]
    pub const fn ad() -> Self {
        Self::from_keys(KeyCode::KeyA, KeyCode::KeyD)
    }

    /// W/S keys for vertical movement.
    #[must_use]
    pub const fn ws() -> Self {
        Self::from_keys(KeyCode::KeyS, KeyCode::KeyW)
    }

    /// Horizontal D-pad (Left = negative, Right = positive).
    #[must_use]
    pub const fn horizontal_dpad() -> Self {
        Self::from_gamepad(GamepadButton::DPadLeft, GamepadButton::DPadRight)
    }

    /// Vertical D-pad (Down = negative, Up = positive).
    #[must_use]
    pub const fn vertical_dpad() -> Self {
        Self::from_gamepad(GamepadButton::DPadDown, GamepadButton::DPadUp)
    }

    /// Horizontal face buttons (West = negative, East = positive).
    #[must_use]
    pub const fn horizontal_face_buttons() -> Self {
        Self::from_gamepad(GamepadButton::West, GamepadButton::East)
    }

    /// Vertical face buttons (South = negative, North = positive).
    #[must_use]
    pub const fn vertical_face_buttons() -> Self {
        Self::from_gamepad(GamepadButton::South, GamepadButton::North)
    }

    /// Get the axis value based on current input state.
    #[must_use]
    pub fn value(&self, keyboard: &ButtonInput<KeyCode>, gamepads: &Query<&Gamepad>) -> f32 {
        let neg = self.negative.is_pressed(keyboard, gamepads);
        let pos = self.positive.is_pressed(keyboard, gamepads);

        match (neg, pos) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        }
    }
}

/// A virtual D-pad that combines four button inputs into a 2D axis value.
///
/// The output is a `Vec2` where:
/// - X ranges from -1.0 (left) to 1.0 (right)
/// - Y ranges from -1.0 (down) to 1.0 (up)
#[derive(Debug, Clone, Reflect)]
pub struct VirtualDPad {
    /// Button for up direction (+Y)
    pub up: VirtualButton,
    /// Button for down direction (-Y)
    pub down: VirtualButton,
    /// Button for left direction (-X)
    pub left: VirtualButton,
    /// Button for right direction (+X)
    pub right: VirtualButton,
}

impl VirtualDPad {
    /// Create a new virtual D-pad from four buttons.
    #[must_use]
    pub const fn new(
        up: VirtualButton,
        down: VirtualButton,
        left: VirtualButton,
        right: VirtualButton,
    ) -> Self {
        Self {
            up,
            down,
            left,
            right,
        }
    }

    /// Create a virtual D-pad from four keyboard keys.
    #[must_use]
    pub const fn from_keys(up: KeyCode, down: KeyCode, left: KeyCode, right: KeyCode) -> Self {
        Self {
            up: VirtualButton::Key(up),
            down: VirtualButton::Key(down),
            left: VirtualButton::Key(left),
            right: VirtualButton::Key(right),
        }
    }

    /// Create a virtual D-pad from four gamepad buttons.
    #[must_use]
    pub const fn from_gamepad(
        up: GamepadButton,
        down: GamepadButton,
        left: GamepadButton,
        right: GamepadButton,
    ) -> Self {
        Self {
            up: VirtualButton::Gamepad(up),
            down: VirtualButton::Gamepad(down),
            left: VirtualButton::Gamepad(left),
            right: VirtualButton::Gamepad(right),
        }
    }

    /// WASD keys for movement.
    #[must_use]
    pub const fn wasd() -> Self {
        Self::from_keys(KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD)
    }

    /// Arrow keys for movement.
    #[must_use]
    pub const fn arrow_keys() -> Self {
        Self::from_keys(
            KeyCode::ArrowUp,
            KeyCode::ArrowDown,
            KeyCode::ArrowLeft,
            KeyCode::ArrowRight,
        )
    }

    /// Gamepad D-pad.
    #[must_use]
    pub const fn dpad() -> Self {
        Self::from_gamepad(
            GamepadButton::DPadUp,
            GamepadButton::DPadDown,
            GamepadButton::DPadLeft,
            GamepadButton::DPadRight,
        )
    }

    /// Gamepad face buttons (North/South/West/East).
    #[must_use]
    pub const fn face_buttons() -> Self {
        Self::from_gamepad(
            GamepadButton::North,
            GamepadButton::South,
            GamepadButton::West,
            GamepadButton::East,
        )
    }

    /// Get the 2D axis value based on current input state.
    #[must_use]
    pub fn axis_pair(&self, keyboard: &ButtonInput<KeyCode>, gamepads: &Query<&Gamepad>) -> Vec2 {
        let up = self.up.is_pressed(keyboard, gamepads);
        let down = self.down.is_pressed(keyboard, gamepads);
        let left = self.left.is_pressed(keyboard, gamepads);
        let right = self.right.is_pressed(keyboard, gamepads);

        let x = match (left, right) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        };

        let y = match (down, up) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        };

        Vec2::new(x, y)
    }

    /// Get the normalized 2D axis value (unit length when diagonal).
    #[must_use]
    pub fn axis_pair_normalized(
        &self,
        keyboard: &ButtonInput<KeyCode>,
        gamepads: &Query<&Gamepad>,
    ) -> Vec2 {
        let value = self.axis_pair(keyboard, gamepads);
        if value.length_squared() > 0.0 {
            value.normalize()
        } else {
            Vec2::ZERO
        }
    }
}

/// A virtual 3D D-pad for 6-directional input.
#[derive(Debug, Clone, Reflect)]
pub struct VirtualDPad3D {
    /// Button for up direction (+Y)
    pub up: VirtualButton,
    /// Button for down direction (-Y)
    pub down: VirtualButton,
    /// Button for left direction (-X)
    pub left: VirtualButton,
    /// Button for right direction (+X)
    pub right: VirtualButton,
    /// Button for forward direction (+Z)
    pub forward: VirtualButton,
    /// Button for backward direction (-Z)
    pub backward: VirtualButton,
}

impl VirtualDPad3D {
    /// Create a new 3D virtual D-pad.
    #[must_use]
    pub const fn new(
        up: VirtualButton,
        down: VirtualButton,
        left: VirtualButton,
        right: VirtualButton,
        forward: VirtualButton,
        backward: VirtualButton,
    ) -> Self {
        Self {
            up,
            down,
            left,
            right,
            forward,
            backward,
        }
    }

    /// Get the 3D axis value.
    #[must_use]
    pub fn axis_triple(&self, keyboard: &ButtonInput<KeyCode>, gamepads: &Query<&Gamepad>) -> Vec3 {
        let up = self.up.is_pressed(keyboard, gamepads);
        let down = self.down.is_pressed(keyboard, gamepads);
        let left = self.left.is_pressed(keyboard, gamepads);
        let right = self.right.is_pressed(keyboard, gamepads);
        let forward = self.forward.is_pressed(keyboard, gamepads);
        let backward = self.backward.is_pressed(keyboard, gamepads);

        let x = match (left, right) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        };

        let y = match (down, up) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        };

        let z = match (backward, forward) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        };

        Vec3::new(x, y, z)
    }
}

/// A virtual button that can be either a keyboard key or gamepad button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum VirtualButton {
    /// A keyboard key
    Key(KeyCode),
    /// A gamepad button
    Gamepad(GamepadButton),
    /// A mouse button
    Mouse(MouseButton),
}

impl VirtualButton {
    /// Check if this virtual button is currently pressed.
    #[must_use]
    pub fn is_pressed(&self, keyboard: &ButtonInput<KeyCode>, gamepads: &Query<&Gamepad>) -> bool {
        match self {
            Self::Key(key) => keyboard.pressed(*key),
            Self::Gamepad(button) => gamepads.iter().any(|gamepad| gamepad.pressed(*button)),
            Self::Mouse(_) => false, // Mouse handled separately
        }
    }

    /// Check if this virtual button is pressed, including mouse buttons.
    #[must_use]
    pub fn is_pressed_with_mouse(
        &self,
        keyboard: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
        gamepads: &Query<&Gamepad>,
    ) -> bool {
        match self {
            Self::Key(key) => keyboard.pressed(*key),
            Self::Gamepad(button) => gamepads.iter().any(|gamepad| gamepad.pressed(*button)),
            Self::Mouse(button) => mouse.pressed(*button),
        }
    }
}

impl From<KeyCode> for VirtualButton {
    fn from(key: KeyCode) -> Self {
        Self::Key(key)
    }
}

impl From<GamepadButton> for VirtualButton {
    fn from(button: GamepadButton) -> Self {
        Self::Gamepad(button)
    }
}

impl From<MouseButton> for VirtualButton {
    fn from(button: MouseButton) -> Self {
        Self::Mouse(button)
    }
}

/// Register virtual input types with the app.
pub(crate) fn register_virtual_input_types(app: &mut App) {
    app.register_type::<VirtualAxis>()
        .register_type::<VirtualDPad>()
        .register_type::<VirtualDPad3D>()
        .register_type::<VirtualButton>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_axis_wasd() {
        let axis = VirtualAxis::ad();
        assert!(matches!(axis.negative, VirtualButton::Key(KeyCode::KeyA)));
        assert!(matches!(axis.positive, VirtualButton::Key(KeyCode::KeyD)));
    }

    #[test]
    fn test_virtual_dpad_wasd() {
        let dpad = VirtualDPad::wasd();
        assert!(matches!(dpad.up, VirtualButton::Key(KeyCode::KeyW)));
        assert!(matches!(dpad.down, VirtualButton::Key(KeyCode::KeyS)));
        assert!(matches!(dpad.left, VirtualButton::Key(KeyCode::KeyA)));
        assert!(matches!(dpad.right, VirtualButton::Key(KeyCode::KeyD)));
    }

    #[test]
    fn test_virtual_dpad_arrow_keys() {
        let dpad = VirtualDPad::arrow_keys();
        assert!(matches!(dpad.up, VirtualButton::Key(KeyCode::ArrowUp)));
        assert!(matches!(dpad.down, VirtualButton::Key(KeyCode::ArrowDown)));
        assert!(matches!(dpad.left, VirtualButton::Key(KeyCode::ArrowLeft)));
        assert!(matches!(
            dpad.right,
            VirtualButton::Key(KeyCode::ArrowRight)
        ));
    }

    #[test]
    fn test_virtual_dpad_gamepad() {
        let dpad = VirtualDPad::dpad();
        assert!(matches!(
            dpad.up,
            VirtualButton::Gamepad(GamepadButton::DPadUp)
        ));
        assert!(matches!(
            dpad.down,
            VirtualButton::Gamepad(GamepadButton::DPadDown)
        ));
    }

    #[test]
    fn test_virtual_button_from() {
        let key: VirtualButton = KeyCode::Space.into();
        assert!(matches!(key, VirtualButton::Key(KeyCode::Space)));

        let button: VirtualButton = GamepadButton::South.into();
        assert!(matches!(
            button,
            VirtualButton::Gamepad(GamepadButton::South)
        ));

        let mouse: VirtualButton = MouseButton::Left.into();
        assert!(matches!(mouse, VirtualButton::Mouse(MouseButton::Left)));
    }
}
