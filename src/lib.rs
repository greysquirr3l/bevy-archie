// Allow clippy pedantic lints that are false positives or less critical for this game controller library
#![allow(clippy::needless_pass_by_value)] // Bevy systems require Res<T>, not &Res<T>
#![allow(clippy::struct_field_names)] // Field naming like `*_bindings` is intentional
#![allow(clippy::match_same_arms)] // Some match arms are intentionally kept separate for clarity

//! # Bevy Archie - Controller Support Module
//!
//! A comprehensive game controller support module for Bevy engine.
//!
//! ## Features
//!
//! - Input device detection (mouse, keyboard, gamepad)
//! - Controller icon system with automatic layout detection
//! - Input action mapping with customizable bindings
//! - Controller remapping at runtime
//! - Virtual keyboard for controller text input
//! - Configurable deadzones and sensitivity
//! - Haptic feedback and rumble patterns
//! - Input buffering and combo detection
//! - Multiplayer controller ownership
//! - Gyroscope and accelerometer support
//! - `PlayStation` touchpad support
//! - Action modifiers (hold, double-tap, long-press)
//! - Controller profiles and auto-detection
//! - Debug tools and input visualization
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use bevy_archie::prelude::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins(ControllerPlugin::default())
//!         .run();
//! }
//! ```

pub mod action_modifiers;
pub mod actions;
pub mod config;
pub mod debug;
pub mod detection;
pub mod gyro;
pub mod haptics;
pub mod icons;
pub mod input_buffer;
pub mod multiplayer;
pub mod plugin;
pub mod profiles;
#[cfg(feature = "remapping")]
pub mod remapping;
pub mod touchpad;
pub mod virtual_cursor;
#[cfg(feature = "virtual_keyboard")]
pub mod virtual_keyboard;

pub mod prelude {
    //! Convenient imports for common use cases.

    pub use crate::action_modifiers::{ActionModifier, ModifiedActionEvent, ModifierConfig};
    pub use crate::actions::{ActionMap, ActionState, GameAction};
    pub use crate::config::{ControllerConfig, ControllerLayout};
    pub use crate::debug::{InputDebugger, InputPlayback, InputRecorder};
    pub use crate::detection::{InputDevice, InputDeviceState};
    pub use crate::gyro::{AccelData, GyroData, MotionConfig, MotionGesture};
    pub use crate::haptics::{RumbleController, RumbleIntensity, RumblePattern, RumbleRequest};
    pub use crate::icons::{ControllerIconAssets, IconSize};
    pub use crate::input_buffer::{Combo, ComboRegistry, InputBuffer};
    pub use crate::multiplayer::{ControllerOwnership, Player, PlayerId};
    pub use crate::plugin::ControllerPlugin;
    pub use crate::profiles::{
        ControllerModel, ControllerProfile, DetectedController, ProfileRegistry,
    };
    pub use crate::touchpad::{TouchpadConfig, TouchpadData, TouchpadGesture};

    #[cfg(feature = "remapping")]
    pub use crate::remapping::{RemapButton, RemapEvent, RemappingState};

    pub use crate::virtual_cursor::{VirtualCursor, VirtualCursorClick, VirtualCursorState};

    #[cfg(feature = "virtual_keyboard")]
    pub use crate::virtual_keyboard::{
        VirtualKeyboard, VirtualKeyboardEvent, VirtualKeyboardState,
    };
}
