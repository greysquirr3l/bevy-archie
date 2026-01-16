#![allow(clippy::needless_pass_by_value)]

//! Example demonstrating controller configuration persistence.
//!
//! This example shows:
//! - Loading config from file on startup
//! - Modifying config at runtime
//! - Saving config back to file
//! - Per-stick sensitivity settings
//! - X-axis inversion options

use bevy::prelude::*;
use bevy_archie::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ControllerPlugin::default())
        .add_systems(Startup, (load_config, setup))
        .add_systems(
            Update,
            (
                handle_config_changes,
                display_config_info,
                save_config_on_change,
            ),
        )
        .run();
}

fn load_config(mut config: ResMut<ControllerConfig>) {
    // Try to load saved config, or use defaults
    match ControllerConfig::load_from_file("controller_config.ron") {
        Ok(loaded_config) => {
            println!("Loaded config from file");
            *config = loaded_config;
        }
        Err(e) => {
            println!("Using default config: {e}");
        }
    }

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
    commands.spawn(Camera2d);

    // Instructions
    commands.spawn((
        Text::new(
            "Controller Config Example\n\n\
            D-Pad Up/Down: Adjust left stick sensitivity\n\
            D-Pad Left/Right: Adjust right stick sensitivity\n\
            L1: Toggle left stick X inversion\n\
            R1: Toggle right stick X inversion\n\
            Start: Save config\n\
            Select: Reset to defaults\n\n\
            Config is automatically saved on changes.",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::WHITE),
    ));
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
            match config.save_default() {
                Ok(()) => println!("Config saved successfully!"),
                Err(e) => println!("Failed to save config: {e}"),
            }
        }

        // Reset to defaults
        if gamepad.just_pressed(GamepadButton::Select) {
            *config = ControllerConfig::default();
            println!("Config reset to defaults");
        }
    }
}

fn save_config_on_change(config: Res<ControllerConfig>) {
    if config.is_changed() && !config.is_added() {
        // Auto-save on any change
        if let Err(e) = config.save_default() {
            eprintln!("Auto-save failed: {e}");
        }
    }
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
