#![allow(clippy::needless_pass_by_value)]

//! Example demonstrating controller configuration persistence.
//!
//! This example shows:
//! - Persisting config with Bevy's `SettingsPlugin`
//! - Modifying config at runtime
//! - Debounced and immediate saves
//! - Per-stick sensitivity settings
//! - X-axis inversion options
//!
//! `ControllerConfig` is a Bevy `SettingsGroup`, so the `SettingsPlugin` loads the
//! saved values on startup and writes them back out when the config changes. The
//! settings file lives at
//! `{preferences_dir}/com.bevy_archie.example.config_persistence/settings.toml`.

use std::time::Duration;

use bevy::prelude::*;
use bevy::settings::{SaveSettingsDeferred, SaveSettingsSync, SettingsPlugin};
use bevy::text::{FontSourceTemplate, LetterSpacing};
use bevy_archie::prelude::*;

/// Application id used to locate the settings file.
const APP_ID: &str = "com.bevy_archie.example.config_persistence";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ControllerPlugin::default())
        // `SettingsPlugin` loads `ControllerConfig` from disk and applies the saved
        // values over the defaults registered by `ControllerPlugin`.
        .add_plugins(SettingsPlugin::new(APP_ID))
        .add_systems(Startup, (report_config, setup))
        .add_systems(
            Update,
            (
                handle_config_changes,
                display_config_info,
                save_config_on_change,
            ),
        )
        .add_systems(Last, save_on_exit)
        .run();
}

fn report_config(config: Res<ControllerConfig>) {
    println!("Current config:");
    println!(
        "  Left stick sensitivity: {}",
        config.left_stick_sensitivity
    );
    println!(
        "  Right stick sensitivity: {}",
        config.right_stick_sensitivity
    );
    println!("  Invert left X: {}", config.invert_left_x);
    println!("  Invert right X: {}", config.invert_right_x);
}

fn setup(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        Camera2d
    });

    // Instructions
    commands.spawn_scene(bsn! {
        Text("Controller Config Example\n\n\
            D-Pad Up/Down: Adjust left stick sensitivity\n\
            D-Pad Left/Right: Adjust right stick sensitivity\n\
            L1: Toggle left stick X inversion\n\
            R1: Toggle right stick X inversion\n\
            Start: Save config\n\
            Select: Reset to defaults\n\n\
            Config is automatically saved on changes.")
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
        }
        TextFont {
            font: FontSourceTemplate::Generic(GenericFontFamily::SansSerif),
            font_size: FontSize::Px(18.0),
        }
        TextColor(Color::WHITE)
        LetterSpacing::Px(0.5)
    });
}

fn handle_config_changes(mut config: ResMut<ControllerConfig>, gamepads: Query<&Gamepad>) {
    for gamepad in gamepads.iter() {
        // Adjust left stick sensitivity
        if gamepad.just_pressed(GamepadButton::DPadUp) {
            config.left_stick_sensitivity = (config.left_stick_sensitivity + 0.1).min(3.0);
            println!(
                "Left stick sensitivity: {:.1}",
                config.left_stick_sensitivity
            );
        }
        if gamepad.just_pressed(GamepadButton::DPadDown) {
            config.left_stick_sensitivity = (config.left_stick_sensitivity - 0.1).max(0.2);
            println!(
                "Left stick sensitivity: {:.1}",
                config.left_stick_sensitivity
            );
        }

        // Adjust right stick sensitivity
        if gamepad.just_pressed(GamepadButton::DPadLeft) {
            config.right_stick_sensitivity = (config.right_stick_sensitivity - 0.1).max(0.2);
            println!(
                "Right stick sensitivity: {:.1}",
                config.right_stick_sensitivity
            );
        }
        if gamepad.just_pressed(GamepadButton::DPadRight) {
            config.right_stick_sensitivity = (config.right_stick_sensitivity + 0.1).min(3.0);
            println!(
                "Right stick sensitivity: {:.1}",
                config.right_stick_sensitivity
            );
        }

        // Toggle inversions
        if gamepad.just_pressed(GamepadButton::LeftTrigger) {
            config.invert_left_x = !config.invert_left_x;
            println!("Invert left X: {}", config.invert_left_x);
        }
        if gamepad.just_pressed(GamepadButton::RightTrigger) {
            config.invert_right_x = !config.invert_right_x;
            println!("Invert right X: {}", config.invert_right_x);
        }

        // Manual save
        if gamepad.just_pressed(GamepadButton::Start) {
            println!("Config saved successfully!");
        }

        // Reset to defaults
        if gamepad.just_pressed(GamepadButton::Select) {
            *config = ControllerConfig::default();
            println!("Config reset to defaults");
        }
    }
}

/// Debounced save: the write is scheduled a short moment after the last change.
fn save_config_on_change(config: Res<ControllerConfig>, mut commands: Commands) {
    if config.is_changed() && !config.is_added() {
        commands.queue(SaveSettingsDeferred(Duration::from_millis(250)));
    }
}

/// Immediate save on shutdown, so nothing is lost when the window is closed.
fn save_on_exit(mut commands: Commands) {
    commands.queue(SaveSettingsSync::IfChanged);
}

fn display_config_info(config: Res<ControllerConfig>, mut text_query: Query<&mut Text>) {
    if config.is_changed() {
        for mut text in &mut text_query {
            text.0 = format!(
                "Controller Config Example\n\n\
                D-Pad Up/Down: Adjust left stick sensitivity\n\
                D-Pad Left/Right: Adjust right stick sensitivity\n\
                L1: Toggle left stick X inversion\n\
                R1: Toggle right stick X inversion\n\
                Start: Save config\n\
                Select: Reset to defaults\n\n\
                Current Settings:\n\
                Left Stick Sensitivity: {:.1}\n\
                Right Stick Sensitivity: {:.1}\n\
                Invert Left X: {}\n\
                Invert Right X: {}\n\
                Deadzone: {:.2}",
                config.left_stick_sensitivity,
                config.right_stick_sensitivity,
                config.invert_left_x,
                config.invert_right_x,
                config.deadzone
            );
        }
    }
}
