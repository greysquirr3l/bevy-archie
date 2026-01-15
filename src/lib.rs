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

pub mod actions;
pub mod config;
pub mod detection;
pub mod icons;
pub mod plugin;
#[cfg(feature = "remapping")]
pub mod remapping;
pub mod virtual_cursor;
#[cfg(feature = "virtual_keyboard")]
pub mod virtual_keyboard;

pub mod prelude {
    //! Convenient imports for common use cases.

    pub use crate::actions::{ActionMap, ActionState, GameAction};
    pub use crate::config::{ControllerConfig, ControllerLayout};
    pub use crate::detection::{InputDevice, InputDeviceState};
    pub use crate::icons::{ControllerIconAssets, IconSize};
    pub use crate::plugin::ControllerPlugin;

    #[cfg(feature = "remapping")]
    pub use crate::remapping::{RemapButton, RemapEvent, RemappingState};

    pub use crate::virtual_cursor::{VirtualCursor, VirtualCursorClick, VirtualCursorState};

    #[cfg(feature = "virtual_keyboard")]
    pub use crate::virtual_keyboard::{
        VirtualKeyboard, VirtualKeyboardEvent, VirtualKeyboardState,
    };
}
