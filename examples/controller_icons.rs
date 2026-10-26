//! Controller icons example showing how to display button prompts.

use bevy::prelude::*;
use bevy::text::{FontSourceTemplate, LetterSpacing};
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

#[derive(Component, Clone, Default)]
struct LayoutText;

#[derive(Component, Clone, Default)]
struct ButtonPrompt {
    #[allow(dead_code)]
    icon: ButtonIcon,
}

/// Scene for a single button prompt: an icon placeholder with a text fallback,
/// plus a label underneath.
fn button_prompt(icon: ButtonIcon, label: &str) -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(5.0),
        }
        Children [
            // Icon placeholder (in a real app, this would be an image)
            Node {
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(8.0)),
            }
            BackgroundColor(Color::srgb(0.3, 0.3, 0.4))
            ButtonPrompt { icon }
            Children [
                // Text fallback for the icon
                Text({ icon.label(ControllerLayout::Xbox) })
                TextFont {
                    font: FontSourceTemplate::Generic(GenericFontFamily::Monospace),
                    font_size: FontSize::Px(20.0),
                    weight: FontWeight::BOLD,
                }
                TextColor(Color::WHITE)
            ]
            --
            // Label
            Text(label)
            TextFont {
                font: FontSourceTemplate::Generic(GenericFontFamily::SansSerif),
                font_size: FontSize::Px(16.0),
            }
            TextColor(Color::srgb(0.8, 0.8, 0.8))
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
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(20.0),
        }
        BackgroundColor(Color::srgb(0.15, 0.15, 0.2))
        Children [
            // Title
            Text("Controller Icon Demo")
            TextFont {
                font: FontSourceTemplate::Generic(GenericFontFamily::SansSerif),
                font_size: FontSize::Px(32.0),
                weight: FontWeight::BOLD,
            }
            TextColor(Color::WHITE)
            ~LetterSpacing::Px(1.0)
            --
            // Layout indicator
            Text("Layout: Xbox")
            TextFont {
                font: FontSourceTemplate::Generic(GenericFontFamily::SansSerif),
                font_size: FontSize::Px(24.0),
            }
            TextColor(Color::srgb(0.7, 0.7, 0.7))
            LayoutText
            --
            // Button prompts row
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(30.0),
            }
            Children [
                @button_prompt(ButtonIcon::FaceDown, "Confirm")
                --
                @button_prompt(ButtonIcon::FaceRight, "Cancel")
                --
                @button_prompt(ButtonIcon::FaceLeft, "Action")
                --
                @button_prompt(ButtonIcon::FaceUp, "Special")
            ]
            --
            // Shoulder buttons row
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(30.0),
            }
            Children [
                @button_prompt(ButtonIcon::LeftBumper, "Page Left")
                --
                @button_prompt(ButtonIcon::RightBumper, "Page Right")
                --
                @button_prompt(ButtonIcon::LeftTrigger, "Aim")
                --
                @button_prompt(ButtonIcon::RightTrigger, "Fire")
            ]
            --
            // Instructions
            Text("Press 1-4 to change layout:\n1=Xbox, 2=PlayStation, 3=Nintendo, 4=Generic")
            TextFont {
                font: FontSourceTemplate::Generic(GenericFontFamily::SansSerif),
                font_size: FontSize::Px(18.0),
            }
            TextColor(Color::srgb(0.5, 0.5, 0.5))
        ]
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
