#![allow(clippy::needless_pass_by_value)]

//! Controller remapping example.

use bevy::prelude::*;
use bevy::text::{FontSourceTemplate, LetterSpacing};
use bevy::ui_widgets::{Activate, Button};
use bevy_archie::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ControllerPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_remap_ui, handle_remap_events))
        .run();
}

#[derive(Component, Clone, Default)]
struct RemapActionButton(GameAction);

#[derive(Component, Clone, Default)]
struct CurrentBindingText(GameAction);

#[derive(Component, Clone, Default)]
struct StatusText;

/// Scene for one remappable action: name, current binding, and a "Remap" button.
fn remap_row(action: GameAction) -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(20.0),
        }
        Children [
            // Action name
            Text({ action.display_name() })
            TextFont {
                font: FontSourceTemplate::Generic(GenericFontFamily::SansSerif),
                font_size: FontSize::Px(20.0),
                weight: FontWeight::SEMIBOLD,
            }
            TextColor(Color::WHITE)
            Node {
                width: Val::Px(150.0),
            }
            --
            // Current binding display
            Text("[A]")
            TextFont {
                font: FontSourceTemplate::Generic(GenericFontFamily::Monospace),
                font_size: FontSize::Px(18.0),
            }
            TextColor(Color::srgb(0.7, 0.7, 0.7))
            CurrentBindingText({ action })
            Node {
                width: Val::Px(100.0),
            }
            --
            // Remap button
            Node {
                padding: UiRect::axes(Val::Px(15.0), Val::Px(8.0)),
                justify_content: JustifyContent::Center,
                border_radius: BorderRadius::all(Val::Px(5.0)),
            }
            BackgroundColor(Color::srgb(0.2, 0.3, 0.5))
            Button
            RemapActionButton({ action })
            on(|trigger: On<Activate>,
               actions: Query<&RemapActionButton>,
               mut events: MessageWriter<StartRemapEvent>| {
                if let Ok(action) = actions.get(trigger.entity) {
                    events.write(StartRemapEvent::new(action.0));
                }
            })
            Children [
                Text("Remap")
                TextFont {
                    font: FontSourceTemplate::Generic(GenericFontFamily::SansSerif),
                    font_size: FontSize::Px(14.0),
                }
                TextColor(Color::WHITE)
            ]
        ]
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        Camera2d
    });

    commands.spawn_scene(bsn! {
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(40.0)),
            row_gap: Val::Px(15.0),
        }
        BackgroundColor(Color::srgb(0.1, 0.1, 0.15))
        Children [
            // Title
            Text("Controller Remapping")
            TextFont {
                font: FontSourceTemplate::Generic(GenericFontFamily::SansSerif),
                font_size: FontSize::Px(32.0),
                weight: FontWeight::BOLD,
            }
            TextColor(Color::WHITE)
            ~LetterSpacing::Px(1.0)
            --
            // Status text
            Text("Click an action to remap it, then press a button on your controller")
            TextFont {
                font: FontSourceTemplate::Generic(GenericFontFamily::SansSerif),
                font_size: FontSize::Px(18.0),
            }
            TextColor(Color::srgb(0.6, 0.6, 0.6))
            StatusText
            --
            // Remappable actions
            @remap_row(GameAction::Confirm)
            --
            @remap_row(GameAction::Cancel)
            --
            @remap_row(GameAction::Primary)
            --
            @remap_row(GameAction::Secondary)
            --
            @remap_row(GameAction::LeftShoulder)
            --
            @remap_row(GameAction::RightShoulder)
            --
            // Reset button
            Node {
                margin: UiRect::top(Val::Px(20.0)),
                padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                border_radius: BorderRadius::all(Val::Px(5.0)),
            }
            BackgroundColor(Color::srgb(0.5, 0.2, 0.2))
            Button
            Children [
                Text("Reset to Defaults")
                TextFont {
                    font: FontSourceTemplate::Generic(GenericFontFamily::SansSerif),
                    font_size: FontSize::Px(16.0),
                    weight: FontWeight::BOLD,
                }
                TextColor(Color::WHITE)
            ]
        ]
    });
}

fn handle_remap_ui(
    action_map: Res<ActionMap>,
    config: Res<ControllerConfig>,
    mut binding_query: Query<(&mut Text, &CurrentBindingText)>,
) {
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
