#![allow(
    clippy::needless_pass_by_value,
    clippy::explicit_iter_loop,
    clippy::uninlined_format_args,
    clippy::indexing_slicing,
    clippy::default_constructed_unit_structs,
    unused_must_use
)]

//! Example demonstrating virtual cursor for gamepad-controlled UI.
//!
//! This example shows:
//! - Virtual cursor appearing when using gamepad
//! - Cursor movement with analog stick
//! - Clicking with A button
//! - Configurable sensitivity and speed

use bevy::prelude::*;
use bevy_archie::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ControllerPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_cursor_clicks, show_cursor_position))
        .run();
}

/// Marker for UI buttons.
#[derive(Component)]
struct ClickableButton;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut controller_config: ResMut<ControllerConfig>,
) {
    // Set up camera
    commands.spawn(Camera2d::default());

    // Customize controller config for faster cursor
    controller_config.right_stick_sensitivity = 1.5;

    // Spawn the virtual cursor
    bevy_archie::virtual_cursor::spawn_virtual_cursor(
        &mut commands,
        &asset_server,
        None, // Will use default "cursor.png"
    );

    // Create some UI buttons for demonstration
    let button_size = 150.0;
    let positions = [
        Vec2::new(-200.0, 100.0),
        Vec2::new(0.0, 100.0),
        Vec2::new(200.0, 100.0),
        Vec2::new(-200.0, -100.0),
        Vec2::new(0.0, -100.0),
        Vec2::new(200.0, -100.0),
    ];

    let colors = [
        Color::srgb(1.0, 0.3, 0.3),
        Color::srgb(0.3, 1.0, 0.3),
        Color::srgb(0.3, 0.3, 1.0),
        Color::srgb(1.0, 1.0, 0.3),
        Color::srgb(1.0, 0.3, 1.0),
        Color::srgb(0.3, 1.0, 1.0),
    ];

    for (i, &pos) in positions.iter().enumerate() {
        commands.spawn((
            ClickableButton,
            Sprite {
                color: colors[i],
                custom_size: Some(Vec2::splat(button_size)),
                ..default()
            },
            Transform::from_translation(pos.extend(0.0)),
        ));
    }

    // Add instructions text
    commands.spawn((
        Text::new(
            "Move cursor with RIGHT STICK\nClick with A button\nSwitch to mouse to hide cursor",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
    ));
}

fn handle_cursor_clicks(
    mut click_events: MessageReader<VirtualCursorClick>,
    mut buttons: Query<(&mut Sprite, &Transform), With<ClickableButton>>,
) {
    for event in click_events.read() {
        println!("Virtual cursor clicked at: {:?}", event.position);

        // Check if we clicked on any button
        for (mut sprite, transform) in buttons.iter_mut() {
            let button_pos = transform.translation.truncate();
            let size = sprite.custom_size.unwrap_or(Vec2::splat(100.0));
            let half_size = size / 2.0;

            let in_bounds = event.position.x >= button_pos.x - half_size.x
                && event.position.x <= button_pos.x + half_size.x
                && event.position.y >= button_pos.y - half_size.y
                && event.position.y <= button_pos.y + half_size.y;

            if in_bounds {
                // Flash the button when clicked
                sprite.color = Color::WHITE;
                println!("Button clicked at: {:?}", button_pos);
            }
        }
    }
}

fn show_cursor_position(cursor_state: Res<VirtualCursorState>, mut text_query: Query<&mut Text>) {
    if cursor_state.is_changed() {
        for mut text in text_query.iter_mut() {
            text.0 = format!(
                "Move cursor with RIGHT STICK\nClick with A button\nSwitch to mouse to hide cursor\n\nCursor: ({:.0}, {:.0})\nActive: {}",
                cursor_state.position.x, cursor_state.position.y, cursor_state.active
            );
        }
    }
}
