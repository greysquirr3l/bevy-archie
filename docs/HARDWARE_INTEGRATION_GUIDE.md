# Hardware Integration Guide

This guide explains how to integrate real controller hardware for advanced features like gyro and touchpad support.

## Overview

Bevy-archie provides the **framework** for gyro and touchpad features:

- ✅ Data structures (`GyroData`, `AccelData`, `TouchpadData`)
- ✅ Gesture detection systems (`MotionGesture`, `TouchpadGesture`)
- ✅ Event systems for application logic
- ❌ Hardware drivers (you need to provide these)

**Your job**: Read data from controller hardware and inject it into the provided components.

## Quick Start

1. **Choose your controller**:
   - PS5 DualSense: Has both gyro and touchpad
   - Switch Pro: Has gyro only
   - Steam Deck/Controller: Has touchpad only

2. **Pick integration approach**:
   - **hidapi** (direct HID): Maximum control, more complexity
   - **SDL2**: Cross-platform, handles pairing/auth automatically
   - **Steam Input API**: Best for Steam Deck/Steam releases
   - **Platform APIs**: CoreMotion (iOS), Windows.Gaming.Input (Windows)

3. **Copy example code**:
   - See [examples/ps5_dualsense_motion.rs](../examples/ps5_dualsense_motion.rs)
   - See [examples/switch_pro_gyro.rs](../examples/switch_pro_gyro.rs)
   - See [examples/steam_touchpad.rs](../examples/steam_touchpad.rs)

## Integration Patterns

### Pattern 1: Direct HID (hidapi)

**Best for**: Full control, custom hardware, specific controller models

```toml
[dependencies]
hidapi = "2.6"
```

```rust
use bevy::prelude::*;
use bevy_archie::prelude::*;
use hidapi::HidApi;

fn inject_gyro_system(mut gyro_query: Query<&mut GyroData>) {
    // 1. Open HID device (do this once, store in Resource)
    let api = HidApi::new().expect("Failed to init HID");
    let device = api.open(0x054c, 0x0ce6).expect("No DualSense found");
    
    // 2. Read HID report
    let mut buf = [0u8; 64];
    if device.read(&mut buf).is_ok() {
        // 3. Parse sensor data from report
        let gyro_x_raw = i16::from_le_bytes([buf[16], buf[17]]);
        let gyro_y_raw = i16::from_le_bytes([buf[18], buf[19]]);
        let gyro_z_raw = i16::from_le_bytes([buf[20], buf[21]]);
        
        // 4. Convert to physical units (rad/s)
        let scale = 0.001064_f32.to_radians();
        let pitch = gyro_x_raw as f32 * scale;
        let yaw = gyro_y_raw as f32 * scale;
        let roll = gyro_z_raw as f32 * scale;
        
        // 5. Inject into bevy-archie
        for mut gyro in &mut gyro_query {
            gyro.set_raw(pitch, yaw, roll);
        }
    }
}
```

**Pros**:

- Maximum control over hardware
- No extra dependencies beyond hidapi
- Works on all platforms

**Cons**:

- Need to parse HID reports manually
- Must handle Bluetooth pairing yourself
- Different report formats for USB vs Bluetooth

### Pattern 2: SDL2

**Best for**: Cross-platform games, Switch Pro Controller, Steam Deck

```toml
[dependencies]
sdl2 = { version = "0.37", features = ["bundled"] }
```

```rust
use sdl2::controller::{GameController, Axis};
use sdl2::sensor::SensorType;

fn inject_gyro_sdl(controller: &GameController, mut gyro_query: Query<&mut GyroData>) {
    // SDL2 handles HID parsing and Bluetooth auth for you
    let mut data = [0.0f32; 3];
    if controller.sensor_get_data(SensorType::Gyroscope, &mut data).is_ok() {
        // SDL2 returns calibrated rad/s values
        for mut gyro in &mut gyro_query {
            gyro.set_raw(data[0], data[1], data[2]);
        }
    }
}
```

**Pros**:

- Handles Bluetooth pairing/auth automatically
- Cross-platform (Windows, macOS, Linux, Steam Deck)
- Returns calibrated sensor data

**Cons**:

- Large dependency (SDL2)
- May conflict with other input systems
- Less control over hardware details

### Pattern 3: Steam Input API

**Best for**: Steam releases, Steam Deck games

```toml
[dependencies]
steamworks = "0.11"
```

```rust
use steamworks::{Client, Controller};

fn inject_touchpad_steam(
    client: &Client,
    controllers: &[Controller],
    mut touchpad_query: Query<&mut TouchpadData>
) {
    let input = client.input();
    
    for &handle in controllers {
        let motion = input.get_motion_data(handle);
        
        if let Some(pos) = motion.touch_position {
            for mut touchpad in &mut touchpad_query {
                touchpad.set_finger(0, pos.x, pos.y, motion.is_touching);
                touchpad.update_frame();
            }
        }
    }
}
```

**Pros**:

- Best integration with Steam features
- Respects user's Steam Input settings
- Built-in configurator UI

**Cons**:

- Only works when launched through Steam
- Requires Steam client running
- Not portable to non-Steam platforms

## Controller-Specific Details

### PS5 DualSense

**Hardware**: Sony DualSense (VID: 0x054c, PID: 0x0ce6)

**Features**:

- ✅ Gyroscope (3-axis, ~2000 deg/s range)
- ✅ Accelerometer (3-axis, ±8g range)
- ✅ Touchpad (1920x1080, 2-finger multi-touch)
- ✅ Haptics (dual motor + voice coil)

**HID Report Format**:

```text
USB Report (0x01, 64 bytes):
  16-17: Gyro Pitch (i16 LE)
  18-19: Gyro Yaw (i16 LE)
  20-21: Gyro Roll (i16 LE)
  22-23: Accel X (i16 LE)
  24-25: Accel Y (i16 LE)
  26-27: Accel Z (i16 LE)
  33: Touchpad header
  34+: Finger positions

Bluetooth Report (0x31, 78 bytes):
  Same offsets + 2 byte header
```

**Calibration**:

- Gyro: Multiply by `0.001064 * (π/180)` for rad/s
- Accel: Multiply by `0.0001196 * 9.81` for m/s²
- Touchpad: Divide by 1920 (X) and 1080 (Y) for normalized 0-1

**Example**: [ps5_dualsense_motion.rs](../examples/ps5_dualsense_motion.rs)

### Nintendo Switch Pro Controller

**Hardware**: Nintendo Switch Pro (VID: 0x057e, PID: 0x2009)

**Features**:

- ✅ Gyroscope (3-axis, ~2000 deg/s range)
- ✅ Accelerometer (3-axis, ±8g range)
- ❌ No touchpad

**HID Report Format**:

```text
Report (0x30, 49 bytes):
  13-14: Gyro Pitch (i16 LE)
  15-16: Gyro Roll (i16 LE)
  17-18: Gyro Yaw (i16 LE)
  19-24: Second sample (333Hz)
  25-30: Third sample (333Hz)
```

**Calibration**:

- Gyro: Multiply by `(π/180) / 13371.0` for rad/s
- Switch sends 3 samples per frame (1000Hz / 3 = 333Hz each)

**Notes**:

- Bluetooth requires pairing handshake
- SDL2 handles this automatically
- Direct HID requires implementing handshake protocol

**Example**: [switch_pro_gyro.rs](../examples/switch_pro_gyro.rs)

### Steam Deck / Steam Controller

**Hardware**:

- Steam Deck (integrated touchpad)
- Steam Controller (dual circular touchpads)

**Features**:

- ✅ Touchpad (capacitive, variable resolution)
- ✅ Gyroscope (Steam Deck only)
- ✅ Haptic feedback

**Integration**: Use Steam Input API (strongly recommended)

**Notes**:

- Direct HID for Steam Controller requires proprietary wireless protocol
- Steam Input API handles all complexity
- Respects user's controller configuration

**Example**: [steam_touchpad.rs](../examples/steam_touchpad.rs)

## Data Injection API

### Gyro Data

```rust
// Inject raw gyro readings (rad/s)
gyro.set_raw(pitch, yaw, roll);

// Framework automatically:
// - Calculates magnitude
// - Applies smoothing
// - Detects gestures (Shake, Tilt, Flick)
// - Emits MotionGestureDetected events
```

### Accelerometer Data

```rust
// Inject raw accelerometer readings (m/s²)
accel.set_raw(x, y, z);

// Framework automatically:
// - Calculates magnitude
// - Detects orientation
```

### Touchpad Data

```rust
// Set finger position (normalized 0.0-1.0)
touchpad.set_finger(finger_index, x, y, is_touching);

// IMPORTANT: Call update_frame() after all fingers updated
touchpad.update_frame();

// Framework automatically:
// - Calculates deltas
// - Tracks velocity
// - Detects gestures (Tap, Swipe, Pinch, etc.)
// - Emits TouchpadGestureEvent
```

## Testing Without Hardware

Use placeholder data for development:

```rust
fn simulate_gyro(mut gyro_query: Query<&mut GyroData>) {
    let time = get_elapsed_time();
    for mut gyro in &mut gyro_query {
        // Simulate gentle rotation
        gyro.set_raw(
            (time * 0.5).sin() * 0.1,
            (time * 0.3).cos() * 0.1,
            0.0
        );
    }
}
```

The gesture detection will work with simulated data, allowing you to test application logic before hardware integration.

## Troubleshooting

### "No HID device found"

- Check USB connection
- Verify VID/PID in code matches your controller
- On Linux: Check udev rules for permissions
- On macOS: Grant Input Monitoring permission

### "Gyro values are wrong"

- Check sensor orientation (axes may be different than expected)
- Verify calibration scale factor
- Some controllers need bias removal (subtract at-rest value)

### "Bluetooth doesn't work"

- Pair controller via OS Bluetooth settings first
- Use SDL2 for automatic auth handling
- Direct HID requires implementing pairing protocol

### "Touchpad coordinates are inverted"

- Some controllers have Y-axis inverted
- Apply: `y = 1.0 - y` if needed

### "Gestures not detected"

- Call `touchpad.update_frame()` after updating fingers
- Check that `active` flag is set correctly
- Verify normalized coordinates (0.0-1.0 range)

## Platform-Specific Notes

### Linux

Add udev rules for HID access:

```bash
# /etc/udev/rules.d/99-hidapi.rules
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="054c", MODE="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="054c", MODE="0666"
```

### macOS

Request Input Monitoring permission in Info.plist:

```xml
<key>NSInputMonitoringUsageDescription</key>
<string>This app needs to access game controllers</string>
```

### Windows

No special setup required. HID devices work out of the box.

## Further Resources

- [DualSense HID Report Spec](https://controllers.fandom.com/wiki/Sony_DualSense)
- [Switch Pro Controller Protocol](https://github.com/dekuNukem/Nintendo_Switch_Reverse_Engineering)
- [SDL2 Game Controller Database](https://github.com/gabomdq/SDL_GameControllerDB)
- [Steam Input Documentation](https://partner.steamgames.com/doc/features/steam_controller)

## Need Help?

- Check the examples: Each shows a complete working integration
- Read the API docs: `cargo doc --open --features motion-backends`
- File an issue: Include controller model and OS version
