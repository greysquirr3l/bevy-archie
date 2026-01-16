//! Integration tests for bevy_archie.

use bevy::prelude::*;
use bevy_archie::action_modifiers::ActionModifier;
use bevy_archie::gyro::MotionGesture;
use bevy_archie::haptics::{RumbleController, RumbleIntensity, RumblePattern};
use bevy_archie::icons::{ButtonIcon, IconSize};
use bevy_archie::input_buffer::InputBuffer;
use bevy_archie::multiplayer::{ControllerOwnership, Player, PlayerId};
use bevy_archie::prelude::*;
use bevy_archie::profiles::ControllerModel;
use bevy_archie::touchpad::TouchpadGesture;
use std::time::Duration;

#[test]
fn test_plugin_initialization() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::input::InputPlugin))
        .add_plugins(ControllerPlugin::default());

    app.update();

    // Check resources are initialized
    assert!(app.world().get_resource::<ControllerConfig>().is_some());
    assert!(app.world().get_resource::<InputDeviceState>().is_some());
    assert!(app.world().get_resource::<ActionMap>().is_some());
    assert!(app.world().get_resource::<ActionState>().is_some());
}

#[test]
fn test_action_state_updates() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::input::InputPlugin))
        .add_plugins(ControllerPlugin::default());

    app.update();

    // Just verify ActionState exists and has public methods
    let action_state = app.world().resource::<ActionState>();
    assert!(!action_state.pressed(GameAction::Confirm));
    assert_eq!(action_state.value(GameAction::LeftTrigger), 0.0);
}

#[test]
fn test_input_device_state_switch() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::input::InputPlugin))
        .add_plugins(ControllerPlugin::default());

    {
        let mut state = app.world_mut().resource_mut::<InputDeviceState>();
        state.active_device = InputDevice::Keyboard;
    }

    app.update();

    let state = app.world().resource::<InputDeviceState>();
    assert!(state.using_keyboard());
}

#[test]
fn test_controller_config_modifications() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::input::InputPlugin))
        .add_plugins(ControllerPlugin::default());

    {
        let mut config = app.world_mut().resource_mut::<ControllerConfig>();
        config.deadzone = 0.25;
        config.left_stick_sensitivity = 1.5;
    }

    app.update();

    let config = app.world().resource::<ControllerConfig>();
    assert_eq!(config.deadzone, 0.25);
    assert_eq!(config.left_stick_sensitivity, 1.5);
}

#[test]
fn test_action_bindings() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::input::InputPlugin))
        .add_plugins(ControllerPlugin::default());

    {
        let mut action_map = app.world_mut().resource_mut::<ActionMap>();
        action_map.bind_gamepad(GameAction::Custom1, GamepadButton::West);
    }

    app.update();

    let action_map = app.world().resource::<ActionMap>();
    assert_eq!(
        action_map.primary_gamepad_button(GameAction::Custom1),
        Some(GamepadButton::West)
    );
}

#[test]
fn test_multiplayer_controller_assignment() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::input::InputPlugin))
        .add_plugins(ControllerPlugin::default());

    let gamepad = app.world_mut().spawn_empty().id();

    {
        let mut ownership = app.world_mut().resource_mut::<ControllerOwnership>();
        ownership.assign(gamepad, PlayerId::new(0));
    }

    app.update();

    let ownership = app.world().resource::<ControllerOwnership>();
    assert_eq!(ownership.get_owner(gamepad), Some(PlayerId::new(0)));
    assert_eq!(ownership.get_gamepad(PlayerId::new(0)), Some(gamepad));
}

#[test]
fn test_icon_size_values() {
    assert_eq!(IconSize::Small.pixels(), 32);
    assert_eq!(IconSize::Medium.pixels(), 48);
    assert_eq!(IconSize::Large.pixels(), 64);

    assert_eq!(IconSize::Small.suffix(), "_small");
    assert_eq!(IconSize::Medium.suffix(), "");
    assert_eq!(IconSize::Large.suffix(), "_large");
}

#[test]
fn test_button_icon_filename_generation() {
    let icon = ButtonIcon::FaceDown;

    // Xbox layout
    assert_eq!(
        icon.filename(ControllerLayout::Xbox, IconSize::Medium),
        "xbox_a.png"
    );
    assert_eq!(
        icon.filename(ControllerLayout::Xbox, IconSize::Small),
        "xbox_a_small.png"
    );

    // PlayStation layout
    assert_eq!(
        icon.filename(ControllerLayout::PlayStation, IconSize::Medium),
        "ps_cross.png"
    );

    // Nintendo layout
    assert_eq!(
        icon.filename(ControllerLayout::Nintendo, IconSize::Medium),
        "switch_b.png"
    );
}

#[test]
fn test_button_icon_labels() {
    let icon = ButtonIcon::FaceDown;

    assert_eq!(icon.label(ControllerLayout::Xbox), "A");
    assert_eq!(icon.label(ControllerLayout::PlayStation), "✕");
    assert_eq!(icon.label(ControllerLayout::Nintendo), "B");
}

#[test]
fn test_rumble_pattern_assignment() {
    let gamepad = Entity::from_bits(1);
    let mut controller = RumbleController::new(gamepad);

    assert_eq!(controller.gamepad, gamepad);
    assert_eq!(controller.duration, Duration::ZERO);

    controller.rumble(RumbleIntensity::uniform(0.75), Duration::from_millis(500));

    assert_eq!(controller.pattern, Some(RumblePattern::Constant));
    assert!(controller.duration > Duration::ZERO);
}

#[test]
fn test_player_id_creation() {
    let player1 = PlayerId::new(0);
    let player2 = PlayerId::new(1);

    assert_eq!(player1.id(), 0);
    assert_eq!(player2.id(), 1);
    assert_ne!(player1, player2);
}

#[test]
fn test_player_component() {
    let player = Player::new(0);
    assert_eq!(player.id, PlayerId::new(0));
    assert!(player.active);

    let p1 = Player::one();
    assert_eq!(p1.id, PlayerId::new(0));

    let p2 = Player::two();
    assert_eq!(p2.id, PlayerId::new(1));
}

#[test]
fn test_motion_gesture_variants() {
    let gestures = [
        MotionGesture::Flick,
        MotionGesture::Shake,
        MotionGesture::Tilt,
        MotionGesture::Roll,
    ];

    // Ensure variants are distinct
    for (i, g1) in gestures.iter().enumerate() {
        for (j, g2) in gestures.iter().enumerate() {
            if i == j {
                assert_eq!(g1, g2);
            } else {
                assert_ne!(g1, g2);
            }
        }
    }
}

#[test]
fn test_touchpad_gesture_variants() {
    let gestures = [
        TouchpadGesture::Tap,
        TouchpadGesture::TwoFingerTap,
        TouchpadGesture::SwipeLeft,
        TouchpadGesture::SwipeRight,
        TouchpadGesture::SwipeUp,
        TouchpadGesture::SwipeDown,
        TouchpadGesture::PinchIn,
        TouchpadGesture::PinchOut,
    ];

    assert_eq!(gestures.len(), 8);
}

#[test]
fn test_controller_model_detection() {
    let model = ControllerModel::PS5;
    assert!(model.supports_touchpad());
    assert!(model.supports_gyro());
    assert!(model.supports_adaptive_triggers());

    let xbox = ControllerModel::XboxOne;
    assert!(!xbox.supports_touchpad());
    assert!(!xbox.supports_gyro());
    assert!(!xbox.supports_adaptive_triggers());
}

#[test]
fn test_action_modifier_states() {
    let modifiers = [
        ActionModifier::Tap,
        ActionModifier::Hold,
        ActionModifier::DoubleTap,
        ActionModifier::LongPress,
        ActionModifier::Released,
    ];

    assert_eq!(modifiers.len(), 5);

    // Check each is distinct
    for (i, m1) in modifiers.iter().enumerate() {
        for (j, m2) in modifiers.iter().enumerate() {
            if i == j {
                assert_eq!(m1, m2);
            } else {
                assert_ne!(m1, m2);
            }
        }
    }
}

#[test]
fn test_input_buffer_capacity() {
    let buffer = InputBuffer::new(Duration::from_secs(1));
    assert_eq!(buffer.inputs.capacity(), 32);
    assert_eq!(buffer.inputs.len(), 0);
    assert!(buffer.inputs.is_empty());
}
