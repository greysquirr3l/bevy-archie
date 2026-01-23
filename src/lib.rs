// Allow clippy pedantic lints that are false positives or less critical for this game controller library
#![allow(clippy::needless_pass_by_value)] // Bevy systems require Res<T>, not &Res<T>
#![allow(clippy::struct_field_names)] // Field naming like `*_bindings` is intentional
#![allow(clippy::match_same_arms)] // Some match arms are intentionally kept separate for clarity
#![allow(clippy::multiple_crate_versions)] // Bevy dependency conflicts, not our issue
#![allow(clippy::missing_const_for_fn)] // Not all functions benefit from being const
#![allow(clippy::indexing_slicing)] // Used carefully in tests with known bounds
#![allow(clippy::unwrap_used)] // Used carefully in tests where panic is acceptable
#![allow(clippy::float_cmp)] // Test assertions need exact float comparison
#![allow(clippy::case_sensitive_file_extension_comparisons)] // Intentional for asset paths
#![allow(clippy::derive_partial_eq_without_eq)] // PartialEq is sufficient for these types
#![allow(clippy::suboptimal_flops)] // Premature optimization, readability preferred
#![allow(clippy::needless_pass_by_ref_mut)] // Some APIs need mut for future compatibility
#![allow(clippy::redundant_clone)] // False positives in some test scenarios
#![allow(clippy::option_if_let_else)] // Sometimes if-let is clearer than map_or
#![allow(clippy::useless_vec)] // Test code clarity
#![allow(clippy::cast_lossless)] // Test code where precision doesn't matter
#![allow(clippy::use_self)] // Type name repetition is clearer in some contexts
#![allow(clippy::redundant_closure_for_method_calls)] // Sometimes explicit closures are clearer
#![allow(clippy::unnecessary_map_or)] // map_or patterns are idiomatic
#![allow(clippy::field_reassign_with_default)] // Common test pattern
#![allow(clippy::imprecise_flops)] // Precision tradeoffs are intentional
#![allow(unused_must_use)] // Test code can ignore return values

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
//! App::new()
//!     .add_plugins(DefaultPlugins)
//!     .add_plugins(ControllerPlugin::default())
//!     .run();
//! ```

pub mod action_modifiers;
pub mod actions;
pub mod build_helpers;
pub mod chords;
pub mod conditions;
pub mod config;
pub mod constants;
pub mod debug;
pub mod detection;
pub mod gyro;
pub mod haptics;
pub mod icons;
pub mod input_buffer;
pub mod motion;
pub mod multiplayer;
pub mod networking;
pub mod plugin;
pub mod profiles;
#[cfg(feature = "remapping")]
pub mod remapping;
pub mod state_machine;
pub mod testing;
pub mod touch_joystick;
pub mod touchpad;
pub mod virtual_cursor;
pub mod virtual_input;
#[cfg(feature = "virtual_keyboard")]
pub mod virtual_keyboard;

pub mod prelude {
    //! Convenient imports for common use cases.

    pub use crate::action_modifiers::{ActionModifier, ModifiedActionEvent, ModifierConfig};
    pub use crate::actions::{ActionMap, ActionState, Actionlike, GameAction, InputBinding};
    pub use crate::chords::{ButtonChord, ChordBinding, ClashStrategy, ModifierKey};
    pub use crate::conditions::{
        ConditionContext, Conditionable, ConditionalBinding, InputCondition,
    };
    pub use crate::config::{ControllerConfig, ControllerLayout};
    pub use crate::debug::{InputDebugger, InputPlayback, InputRecorder};
    pub use crate::detection::{InputDevice, InputDeviceState};
    pub use crate::gyro::{AccelData, GyroData, MotionConfig, MotionGesture};
    pub use crate::haptics::{RumbleController, RumbleIntensity, RumblePattern, RumbleRequest};
    pub use crate::icons::{ControllerIconAssets, IconSize};
    pub use crate::input_buffer::{Combo, ComboRegistry, InputBuffer};
    pub use crate::multiplayer::{ControllerOwnership, Player, PlayerId};
    pub use crate::networking::{ActionDiff, ActionDiffBuffer, NetworkedInput};
    pub use crate::plugin::ControllerPlugin;
    pub use crate::profiles::{
        ControllerModel, ControllerProfile, DetectedController, ProfileRegistry,
    };
    pub use crate::state_machine::{
        InputStateMachine, StateMachineBuilder, StateTransitionEvent, TriggerType,
    };
    pub use crate::testing::{MockInput, MockInputPlugin};
    pub use crate::touch_joystick::{
        JoystickSide, TouchJoystick, TouchJoystickEvent, TouchJoystickPlugin, TouchJoystickSettings,
    };
    pub use crate::touchpad::{TouchpadConfig, TouchpadData, TouchpadGesture};
    pub use crate::virtual_input::{VirtualAxis, VirtualButton, VirtualDPad, VirtualDPad3D};

    #[cfg(feature = "remapping")]
    pub use crate::remapping::{RemapButton, RemapEvent, RemappingState, StartRemapEvent};

    pub use crate::virtual_cursor::{VirtualCursor, VirtualCursorClick, VirtualCursorState};

    #[cfg(feature = "virtual_keyboard")]
    pub use crate::virtual_keyboard::{
        VirtualKeyboard, VirtualKeyboardEvent, VirtualKeyboardState,
    };
}
