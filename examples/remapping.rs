#![allow(clippy::needless_pass_by_value)]

//! Controller remapping example.

use bevy::prelude::*;
use bevy_archie::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ControllerPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_remap_ui, handle_remap_events))
        .run();
}

#[derive(Component)]
struct RemapActionButton(GameAction);

#[derive(Component)]
struct CurrentBindingText(GameAction);

#[derive(Component)]
struct StatusText;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(40.0)),
                row_gap: Val::Px(15.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Controller Remapping"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Status text
            parent.spawn((
                Text::new("Click an action to remap it, then press a button on your controller"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
                StatusText,
            ));

            // Remappable actions
            let actions = [
                GameAction::Confirm,
                GameAction::Cancel,
                GameAction::Primary,
                GameAction::Secondary,
                GameAction::LeftShoulder,
                GameAction::RightShoulder,
            ];

            for action in actions {
                spawn_remap_row(parent, action);
            }

            // Reset button
            parent
                .spawn((
                    Node {
                        margin: UiRect::top(Val::Px(20.0)),
                        padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        border_radius: BorderRadius::all(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.5, 0.2, 0.2)),
                    Button,
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("Reset to Defaults"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

fn spawn_remap_row(parent: &mut ChildSpawnerCommands, action: GameAction) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(20.0),
            ..default()
        })
        .with_children(|row: &mut ChildSpawnerCommands| {
            // Action name
            row.spawn((
                Text::new(action.display_name()),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    width: Val::Px(150.0),
                    ..default()
                },
            ));

            // Current binding display
            row.spawn((
                Text::new("[A]"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                CurrentBindingText(action),
                Node {
                    width: Val::Px(100.0),
                    ..default()
                },
            ));

            // Remap button
            row.spawn((
                Node {
                    padding: UiRect::axes(Val::Px(15.0), Val::Px(8.0)),
                    justify_content: JustifyContent::Center,
                    border_radius: BorderRadius::all(Val::Px(5.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.3, 0.5)),
                Button,
                RemapActionButton(action),
            ))
            .with_children(|btn: &mut ChildSpawnerCommands| {
                btn.spawn((
                    Text::new("Remap"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });
}

fn handle_remap_ui(
    mut remap_events: MessageWriter<StartRemapEvent>,
    action_map: Res<ActionMap>,
    config: Res<ControllerConfig>,
    interaction_query: Query<(&Interaction, &RemapActionButton), Changed<Interaction>>,
    mut binding_query: Query<(&mut Text, &CurrentBindingText)>,
) {
    // Handle remap button clicks
    for (interaction, remap_button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            remap_events.write(StartRemapEvent::new(remap_button.0));
        }
    }

    // Update current binding displays
    let layout = config.layout();
    for (mut text, binding_text) in &mut binding_query {
        if let Some(button) = action_map.primary_gamepad_button(binding_text.0) {
            let button_name = layout.button_name(button);
            **text = format!("[{button_name}]");
        } else {
            **text = "[None]".to_string();
        }
    }
}

fn handle_remap_events(
    mut events: MessageReader<RemapEvent>,
    mut status_query: Query<&mut Text, With<StatusText>>,
) {
    for event in events.read() {
        let message = match event {
            RemapEvent::Success { action, button } => {
                format!("Remapped {action:?} to {button:?}")
            }
            RemapEvent::Cancelled { action } => {
                format!("Cancelled remapping {action:?}")
            }
            RemapEvent::TimedOut { action } => {
                format!("Timed out remapping {action:?}")
            }
            RemapEvent::Conflict {
                action,
                conflicting_action,
                ..
            } => {
                format!("Conflict: {action:?} is already bound to {conflicting_action:?}")
            }
        };

        for mut text in &mut status_query {
            text.0.clone_from(&message);
        }
    }
}
