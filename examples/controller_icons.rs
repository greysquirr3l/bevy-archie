//! Controller icons example showing how to display button prompts.

use bevy::prelude::*;
use bevy_archie::icons::ButtonIcon;
use bevy_archie::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ControllerPlugin::with_icon_path("icons/controller"))
        .add_systems(Startup, setup)
        .add_systems(Update, update_layout_display)
        .run();
}

#[derive(Component)]
struct LayoutText;

#[derive(Component)]
struct ButtonPrompt {
    #[allow(dead_code)]
    icon: ButtonIcon,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.2)),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Controller Icon Demo"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Layout indicator
            parent.spawn((
                Text::new("Layout: Xbox"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                LayoutText,
            ));

            // Button prompts row
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(30.0),
                    ..default()
                })
                .with_children(|row| {
                    spawn_button_prompt(row, ButtonIcon::FaceDown, "Confirm");
                    spawn_button_prompt(row, ButtonIcon::FaceRight, "Cancel");
                    spawn_button_prompt(row, ButtonIcon::FaceLeft, "Action");
                    spawn_button_prompt(row, ButtonIcon::FaceUp, "Special");
                });

            // Shoulder buttons row
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(30.0),
                    ..default()
                })
                .with_children(|row| {
                    spawn_button_prompt(row, ButtonIcon::LeftBumper, "Page Left");
                    spawn_button_prompt(row, ButtonIcon::RightBumper, "Page Right");
                    spawn_button_prompt(row, ButtonIcon::LeftTrigger, "Aim");
                    spawn_button_prompt(row, ButtonIcon::RightTrigger, "Fire");
                });

            // Instructions
            parent.spawn((
                Text::new(
                    "Press 1-4 to change layout:\n1=Xbox, 2=PlayStation, 3=Nintendo, 4=Generic",
                ),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
            ));
        });
}

fn spawn_button_prompt(parent: &mut ChildSpawnerCommands, icon: ButtonIcon, label: &str) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(5.0),
            ..default()
        })
        .with_children(|col: &mut ChildSpawnerCommands| {
            // Icon placeholder (in a real app, this would be an image)
            col.spawn((
                Node {
                    width: Val::Px(64.0),
                    height: Val::Px(64.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.3, 0.3, 0.4)),
                BorderRadius::all(Val::Px(8.0)),
                ButtonPrompt { icon },
            ))
            .with_children(|button: &mut ChildSpawnerCommands| {
                // Text fallback for the icon
                button.spawn((
                    Text::new(icon.label(ControllerLayout::Xbox)),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });

            // Label
            col.spawn((
                Text::new(label),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));
        });
}

#[allow(clippy::needless_pass_by_value)]
fn update_layout_display(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<ControllerConfig>,
    mut layout_query: Query<&mut Text, With<LayoutText>>,
) {
    let mut changed = false;

    if keyboard.just_pressed(KeyCode::Digit1) {
        config.forced_layout = Some(ControllerLayout::Xbox);
        changed = true;
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        config.forced_layout = Some(ControllerLayout::PlayStation);
        changed = true;
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        config.forced_layout = Some(ControllerLayout::Nintendo);
        changed = true;
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        config.forced_layout = Some(ControllerLayout::Generic);
        changed = true;
    }

    if changed {
        let layout = config.layout();
        for mut text in &mut layout_query {
            **text = format!("Layout: {layout:?}");
        }
    }
}
