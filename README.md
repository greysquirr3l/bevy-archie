# Bevy Archie - Controller Support Module

A comprehensive game controller support module for Bevy engine, inspired by the RenPy Controller GUI project.

## Features

- **Input Device Detection**: Automatically detect and switch between mouse, keyboard, and gamepad input
- **Controller Icon System**: Display appropriate button icons based on controller type (Xbox, PlayStation, Nintendo, Steam, Stadia, Generic)
- **Input Action Mapping**: Abstract input actions with customizable bindings
- **Controller Remapping**: Allow players to remap controller buttons at runtime
- **Virtual Keyboard**: On-screen keyboard for controller-friendly text input
- **Virtual Cursor**: Gamepad-controlled cursor for mouse-based UI navigation
- **Per-Stick Settings**: Independent sensitivity and inversion for left/right analog sticks
- **Configuration Persistence**: Save and load controller settings to/from JSON files
- **Deadzone Configuration**: Configurable stick deadzones and sensitivity
- **Focus Management**: Keyboard/controller-friendly UI focus navigation
- **Multi-controller Support**: Handle multiple connected controllers

## Supported Controllers

- **Xbox** - Xbox 360, Xbox One, Xbox Series X|S controllers
- **PlayStation** - PS3, PS4, PS5 (DualShock and DualSense)
- **Nintendo** - Switch Pro Controller, Joy-Cons
- **Steam** - Steam Controller, Steam Deck
- **Stadia** - Google Stadia Controller (Bluetooth mode)
- **Generic** - Any other standard gamepad

**Note**: Stadia controllers must be switched to Bluetooth mode (a permanent one-time operation that was available until Dec 31, 2025). In Bluetooth mode, they function as standard Xbox-style gamepads.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
bevy_archie = { path = "path/to/bevy-archie" }
# Or with specific features:
bevy_archie = { path = "path/to/bevy-archie", features = ["full"] }
```

## Quick Start

```rust
use bevy::prelude::*;
use bevy_archie::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ControllerPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn handle_input(
    input_state: Res<InputDeviceState>,
    actions: Res<ActionState>,
) {
    // Check which input device is active
    match input_state.active_device {
        InputDevice::Mouse => { /* Mouse logic */ }
        InputDevice::Keyboard => { /* Keyboard logic */ }
        InputDevice::Gamepad(_) => { /* Controller logic */ }
    }

    // Check action states
    if actions.just_pressed(GameAction::Confirm) {
        println!("Confirm pressed!");
    }
}
```

## Action System

Define your game actions and bind them to controller buttons:

```rust
use bevy_archie::prelude::*;

// Actions are predefined, but you can extend with custom actions
fn setup_actions(mut action_map: ResMut<ActionMap>) {
    // Rebind an action
    action_map.bind(GameAction::Confirm, GamepadButtonType::South);
    action_map.bind(GameAction::Cancel, GamepadButtonType::East);
    
    // Add keyboard bindings
    action_map.bind_key(GameAction::Confirm, KeyCode::Enter);
    action_map.bind_key(GameAction::Cancel, KeyCode::Escape);
}
```

## Controller Icons

Display controller-appropriate button icons in your UI:

```rust
fn spawn_button_prompt(
    mut commands: Commands,
    icon_assets: Res<ControllerIconAssets>,
    input_state: Res<InputDeviceState>,
) {
    let icon = icon_assets.get_icon(
        GamepadButtonType::South,
        input_state.controller_layout,
    );
    
    commands.spawn(ImageNode {
        image: icon,
        ..default()
    });
}
```

## Remapping

Enable controller remapping in your settings menu:

```rust
fn spawn_remap_ui(mut commands: Commands) {
    commands.spawn((
        RemapButton {
            action: GameAction::Confirm,
        },
        // ... UI components
    ));
}
```

## Configuration

```rust
use bevy_archie::prelude::*;

fn configure_controller(mut config: ResMut<ControllerConfig>) {
    // Stick deadzone (0.0 - 1.0)
    config.deadzone = 0.15;
    
    // Per-stick sensitivity multipliers
    config.left_stick_sensitivity = 1.0;
    config.right_stick_sensitivity = 1.5; // Faster cursor movement
    
    // Per-stick X-axis inversion
    config.invert_left_x = false;
    config.invert_right_x = true; // Inverted camera controls
    
    // Auto-detect controller layout
    config.auto_detect_layout = true;
    
    // Force a specific layout
    config.force_layout = Some(ControllerLayout::PlayStation);
}
```

## Virtual Cursor

Enable gamepad-controlled cursor for mouse-based UI:

```rust
use bevy::prelude::*;
use bevy_archie::prelude::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn virtual cursor (automatically shown when gamepad active)
    bevy_archie::virtual_cursor::spawn_virtual_cursor(
        &mut commands,
        &asset_server,
        None, // Uses default cursor.png
    );
}

fn handle_clicks(mut click_events: EventReader<VirtualCursorClick>) {
    for event in click_events.read() {
        println!("Cursor clicked at: {:?}", event.position);
    }
}
```

## Configuration Persistence

Save and load controller settings:

```rust
use bevy_archie::prelude::*;

// Load config on startup
fn load_config(mut config: ResMut<ControllerConfig>) {
    *config = ControllerConfig::load_or_default().unwrap();
}

// Save config
fn save_config(config: Res<ControllerConfig>) {
    config.save_default().unwrap();
}

// Custom path
fn save_to_custom_path(config: Res<ControllerConfig>) {
    config.save_to_file("my_config.json").unwrap();
}
```

Config files are saved to platform-specific directories:

- **Linux**: `~/.config/bevy_archie/controller.json`
- **macOS**: `~/Library/Application Support/bevy_archie/controller.json`
- **Windows**: `%APPDATA%\bevy_archie\controller.json`

## Supported Controller Layouts

- **Xbox**: Xbox 360, Xbox One, Xbox Series controllers
- **PlayStation**: DualShock 3/4, DualSense
- **Nintendo**: Joy-Con, Pro Controller, GameCube
- **Steam**: Steam Controller, Steam Deck
- **Stadia**: Google Stadia Controller (Bluetooth mode)
- **Generic**: Fallback for unrecognized controllers

## Examples

Run the examples to see features in action:

```bash
# Basic input handling
cargo run --example basic_input

# Controller icon display
cargo run --example controller_icons

# Button remapping UI
cargo run --example remapping

# Virtual cursor
cargo run --example virtual_cursor

# Config persistence
cargo run --example config_persistence
```

## Credits

Inspired by the [RenPy Controller GUI](https://feniksdev.com) by Feniks.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
