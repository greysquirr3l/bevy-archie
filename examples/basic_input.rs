//! Basic input example demonstrating controller detection and action mapping.

use bevy::prelude::*;
use bevy_archie::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ControllerPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (display_input_state, handle_actions))
        .run();
}

#[derive(Component)]
struct InputStateText;

#[derive(Component)]
struct ActionText;

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);

    // UI root
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                row_gap: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Bevy Archie - Controller Support Demo"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Input state display
            parent.spawn((
                Text::new("Input Device: Mouse"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                InputStateText,
            ));

            // Actions display
            parent.spawn((
                Text::new("Actions: None"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.8, 0.5)),
                ActionText,
            ));

            // Instructions
            parent.spawn((
                Text::new(
                    "Instructions:\n\
                     - Use keyboard, mouse, or gamepad\n\
                     - The active input device will be detected automatically\n\
                     - Press buttons to see action states",
                ),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
            ));
        });
}

fn display_input_state(
    input_state: Res<InputDeviceState>,
    config: Res<ControllerConfig>,
    mut query: Query<&mut Text, With<InputStateText>>,
) {
    if !input_state.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        let device_name = match input_state.active_device {
            InputDevice::Mouse => "Mouse".to_string(),
            InputDevice::Keyboard => "Keyboard".to_string(),
            InputDevice::Gamepad(_) => {
                format!("Gamepad ({:?})", config.layout())
            }
        };

        let gamepads_connected = input_state.connected_gamepads.len();

        **text = format!(
            "Input Device: {}\nConnected Gamepads: {}",
            device_name, gamepads_connected
        );
    }
}

fn handle_actions(actions: Res<ActionState>, mut query: Query<&mut Text, With<ActionText>>) {
    let mut active_actions = Vec::new();

    for action in GameAction::all() {
        if actions.pressed(*action) {
            active_actions.push(format!("{:?}", action));
        }
    }

    for mut text in query.iter_mut() {
        if active_actions.is_empty() {
            **text = "Actions: None".to_string();
        } else {
            **text = format!("Actions: {}", active_actions.join(", "));
        }
    }
}
