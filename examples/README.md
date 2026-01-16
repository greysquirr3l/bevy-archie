# Bevy-Archie Examples

This folder contains examples demonstrating bevy-archie's features.

## Running Examples

```bash
# Basic examples
cargo run --example <name>

# Examples requiring features
cargo run --example <name> --features <feature>
```

## Basic Examples

| Example | Description | Command |
| --------- | ------------- | --------- |
| **basic_input** | Simple input handling with device detection and action states | `cargo run --example basic_input` |
| **controller_icons** | Display controller-specific button icons that adapt to the connected controller type | `cargo run --example controller_icons` |
| **config_persistence** | Save and load controller settings to/from JSON files | `cargo run --example config_persistence` |
| **virtual_cursor** | Gamepad-controlled cursor for navigating mouse-based UIs | `cargo run --example virtual_cursor` |
| **remapping** | Runtime button remapping UI for player customization | `cargo run --example remapping --features remapping` |

## Hardware Integration Examples

These examples show how to integrate real controller hardware for advanced features like gyro and touchpad. They require the `motion-backends` feature and additional dependencies.

| Example | Controller | Features | Command |
| --------- | ------------ | ---------- | --------- |
| **ps5_dualsense_motion** | PS5 DualSense | Gyro + Touchpad | `cargo run --example ps5_dualsense_motion --features motion-backends` |
| **switch_pro_gyro** | Nintendo Switch Pro | Gyro | `cargo run --example switch_pro_gyro --features motion-backends` |
| **steam_touchpad** | Steam Deck / Steam Controller | Touchpad | `cargo run --example steam_touchpad --features motion-backends` |

### Hardware Example Dependencies

The hardware examples include commented-out code showing real integrations. To use them:

**PS5 DualSense** (via hidapi):

```toml
[dependencies]
hidapi = "2.6"
```

**Switch Pro Controller** (via SDL2):

```toml
[dependencies]
sdl2 = { version = "0.37", features = ["bundled"] }
```

**Steam Deck/Controller** (via Steam Input API):

```toml
[dependencies]
steamworks = "0.11"
```

See [Hardware Integration Guide](../docs/HARDWARE_INTEGRATION_GUIDE.md) for detailed instructions.

## Example Descriptions

### basic_input

Demonstrates the core input handling system:

- Detecting active input device (keyboard, mouse, gamepad)
- Reading action states (pressed, just_pressed, released)
- Responding to controller connection/disconnection

### controller_icons

Shows how to display the correct button icons for each controller type:

- Automatic layout detection (Xbox, PlayStation, Nintendo, Generic)
- Loading and displaying platform-specific icons
- Updating icons when controller type changes

### config_persistence

Demonstrates saving and loading controller settings:

- Persisting deadzone, sensitivity, and button mappings
- Platform-specific config paths
- Custom config file locations

### virtual_cursor

Implements a gamepad-controlled cursor for UI navigation:

- Smooth cursor movement with analog sticks
- Click events for UI interaction
- Cursor visibility and speed settings

### remapping

Provides runtime button remapping for players:

- UI for selecting actions to remap
- Capturing new button assignments
- Saving remapped controls

### ps5_dualsense_motion

Complete DualSense integration example:

- HID report parsing for USB and Bluetooth modes
- Gyroscope data injection (pitch, yaw, roll)
- Touchpad multi-touch with finger tracking
- Gesture detection (shake, tilt, swipe, pinch)

### switch_pro_gyro

Switch Pro Controller gyro integration:

- SDL2-based sensor access
- Cross-platform gyro support
- Alternative direct HID approach documented

### steam_touchpad

Steam Input API touchpad integration:

- Steam Deck central touchpad
- Steam Controller dual touchpads
- Haptic feedback support
