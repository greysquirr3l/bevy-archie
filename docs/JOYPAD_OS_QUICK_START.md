# Quick Start Guide: Using Joypad OS Enhancements

## Overview

This guide shows practical examples of using the enhanced controller database, calibration constants, and hardware specifications added from Joypad OS research.

## 1. Controller-Specific Features

### Detect and Enable Features Based on Controller Model

```rust
use bevy::prelude::*;
use bevy_archie::prelude::*;

fn setup_controller_features(
    query: Query<(&DetectedController, Entity), Added<DetectedController>>,
    mut commands: Commands,
) {
    for (detected, entity) in &query {
        println!("Detected: {:?} (VID: 0x{:04X}, PID: 0x{:04X})",
            detected.model,
            detected.vendor_id,
            detected.product_id
        );

        // Enable gyroscope if supported
        if detected.model.supports_gyro() {
            commands.entity(entity).insert((
                GyroData::default(),
                AccelData::default(),
            ));
            println!("  ✓ Gyroscope enabled");
        }

        // Enable touchpad if supported
        if detected.model.supports_touchpad() {
            commands.entity(entity).insert(TouchpadData::default());
            println!("  ✓ Touchpad enabled");
        }

        // Enable adaptive triggers (DualSense only)
        if detected.model.supports_adaptive_triggers() {
            setup_dualsense_triggers(entity);
            println!("  ✓ Adaptive triggers enabled");
        }

        // Handle PS3 pressure buttons
        if detected.model.supports_pressure_buttons() {
            setup_pressure_buttons(entity);
            println!("  ✓ Pressure-sensitive buttons enabled");
        }

        // Handle controller quirks
        for quirk in detected.quirks() {
            match quirk {
                ControllerQuirk::BigEndianValues => {
                    println!("  ⚠ Using big-endian byte conversion (PS3)");
                }
                ControllerQuirk::DS4BluetoothReportDiffers => {
                    println!("  ⚠ Using Bluetooth HID report format (DS4)");
                }
                ControllerQuirk::SwitchProUSBHandshake => {
                    println!("  ⚠ Performing USB handshake (Switch Pro)");
                }
                _ => {}
            }
        }
    }
}
```

## 2. Motion Sensor Calibration

### DualSense Motion Controls

```rust
use bevy::prelude::*;
use bevy_archie::motion::backend::dualsense_calibration;
use bevy_archie::prelude::*;

/// Example: Process raw DualSense motion sensor data
fn process_dualsense_motion(
    mut gyro_query: Query<(&mut GyroData, &DetectedController)>,
) {
    for (mut gyro, detected) in &mut gyro_query {
        if detected.model != ControllerModel::PS5 {
            continue;
        }

        // Simulated raw sensor data from HID report
        // In real implementation, you'd get this from dualsense-rs or HID API
        let raw_gyro_x: i16 = 0;     // Replace with actual HID data
        let raw_gyro_y: i16 = 0;
        let raw_gyro_z: i16 = 0;

        // Convert raw values to rad/s using calibration constants
        let pitch = dualsense_calibration::gyro_to_rads(raw_gyro_x);
        let yaw = dualsense_calibration::gyro_to_rads(raw_gyro_y);
        let roll = dualsense_calibration::gyro_to_rads(raw_gyro_z);

        gyro.set_raw(pitch, yaw, roll);
    }
}

/// Example: Process raw DualSense accelerometer data
fn process_dualsense_accel(
    mut accel_query: Query<(&mut AccelData, &DetectedController)>,
) {
    for (mut accel, detected) in &mut accel_query {
        if detected.model != ControllerModel::PS5 {
            continue;
        }

        // Simulated raw sensor data
        let raw_accel_x: i16 = 0;
        let raw_accel_y: i16 = 0;
        let raw_accel_z: i16 = 16384; // Gravity at rest

        // Convert to m/s²
        let accel_x = dualsense_calibration::accel_to_ms2(raw_accel_x);
        let accel_y = dualsense_calibration::accel_to_ms2(raw_accel_y);
        let accel_z = dualsense_calibration::accel_to_ms2(raw_accel_z);

        accel.set_raw(accel_x, accel_y, accel_z);
    }
}
```

### DualShock 3 (PS3) Motion Controls

```rust
use bevy_archie::motion::backend::dualshock3_calibration;

/// Example: Handle PS3 SIXAXIS big-endian values
fn process_ps3_motion(raw_bytes: &[u8]) {
    // PS3 uses big-endian 16-bit values
    let accel_x_bytes = [raw_bytes[0], raw_bytes[1]];
    let accel_y_bytes = [raw_bytes[2], raw_bytes[3]];
    let accel_z_bytes = [raw_bytes[4], raw_bytes[5]];

    // Convert from big-endian
    let raw_x = dualshock3_calibration::be_bytes_to_i16(accel_x_bytes);
    let raw_y = dualshock3_calibration::be_bytes_to_i16(accel_y_bytes);
    let raw_z = dualshock3_calibration::be_bytes_to_i16(accel_z_bytes);

    // Convert to m/s²
    let accel_x = dualshock3_calibration::accel_to_ms2(raw_x);
    let accel_y = dualshock3_calibration::accel_to_ms2(raw_y);
    let accel_z = dualshock3_calibration::accel_to_ms2(raw_z);

    println!("PS3 Accelerometer: X={:.2}, Y={:.2}, Z={:.2} m/s²",
        accel_x, accel_y, accel_z);
}
```

### Switch Pro Controller Motion

```rust
use bevy_archie::motion::backend::switch_calibration;

fn process_switch_motion(raw_gyro: i16, raw_accel: i16) {
    let gyro_rads = switch_calibration::gyro_to_rads(raw_gyro);
    let accel_ms2 = switch_calibration::accel_to_ms2(raw_accel);

    println!("Switch Motion - Gyro: {:.2} rad/s, Accel: {:.2} m/s²",
        gyro_rads, accel_ms2);
}
```

## 3. Touchpad Integration

### DualSense Touchpad

```rust
use bevy::prelude::*;
use bevy_archie::touchpad::{dualsense_touchpad, TouchpadData};

/// Example: Process raw DualSense touchpad data
fn process_dualsense_touchpad(
    mut touchpad_query: Query<(&mut TouchpadData, &DetectedController)>,
) {
    for (mut touchpad, detected) in &mut touchpad_query {
        if detected.model != ControllerModel::PS5 {
            continue;
        }

        // Simulated raw touchpad data from HID report
        // Finger 1: center of touchpad
        let finger1_raw_x: u16 = 960;  // Center X
        let finger1_raw_y: u16 = 540;  // Center Y
        let finger1_active = true;

        // Finger 2: not touching
        let finger2_active = false;

        // Set finger data with automatic normalization
        touchpad.set_finger_raw(
            0,
            finger1_raw_x,
            finger1_raw_y,
            dualsense_touchpad::WIDTH,
            dualsense_touchpad::HEIGHT,
            finger1_active,
        );

        touchpad.set_finger_raw(
            1,
            0,
            0,
            dualsense_touchpad::WIDTH,
            dualsense_touchpad::HEIGHT,
            finger2_active,
        );

        // Update frame state for delta calculation
        touchpad.update_frame();

        // Check for gestures
        if touchpad.is_swiping(0.15) {
            println!("Swipe detected!");
        }
    }
}
```

### DualShock 4 Touchpad

```rust
use bevy_archie::touchpad::{dualshock4_touchpad, normalize_coords};

fn process_ds4_touchpad() {
    // Raw coordinates from HID report
    let raw_x: u16 = 1920;  // Right edge
    let raw_y: u16 = 471;   // Center height

    // Normalize to 0.0-1.0 range
    let (norm_x, norm_y) = normalize_coords(
        raw_x,
        raw_y,
        dualshock4_touchpad::WIDTH,
        dualshock4_touchpad::HEIGHT,
    );

    println!("DS4 Touchpad: ({:.2}, {:.2})", norm_x, norm_y);
    // Output: "DS4 Touchpad: (1.00, 0.50)"
}
```

## 4. Analog Stick Processing

### Using Hardware Constants

```rust
use bevy_archie::constants::analog_stick;

fn process_analog_input(raw_x: u8, raw_y: u8) {
    // Normalize raw 0-255 values to -1.0 to 1.0
    let norm_x = analog_stick::normalize(raw_x);
    let norm_y = analog_stick::normalize(raw_y);

    println!("Raw: ({}, {}) -> Normalized: ({:.2}, {:.2})",
        raw_x, raw_y, norm_x, norm_y);

    // Apply circular deadzone
    let (final_x, final_y) = analog_stick::apply_deadzone(
        norm_x,
        norm_y,
        analog_stick::DEFAULT_DEADZONE,
    );

    println!("After deadzone: ({:.2}, {:.2})", final_x, final_y);
}

// Example usage:
fn example() {
    // Stick at rest (should be filtered by deadzone)
    process_analog_input(128, 128);
    // Raw: (128, 128) -> Normalized: (0.00, 0.00)
    // After deadzone: (0.00, 0.00)

    // Stick pushed right
    process_analog_input(255, 128);
    // Raw: (255, 128) -> Normalized: (0.99, 0.00)
    // After deadzone: (0.99, 0.00)
}
```

### Trigger Processing

```rust
use bevy_archie::constants::trigger;

fn process_trigger_input(raw_trigger: u8) {
    let normalized = trigger::normalize(raw_trigger);
    println!("Trigger: {} -> {:.2}", raw_trigger, normalized);

    // Use normalized value for game logic
    if normalized > 0.5 {
        println!("  Trigger pressed more than 50%");
    }
}
```

## 5. Complete Integration Example

```rust
use bevy::prelude::*;
use bevy_archie::prelude::*;
use bevy_archie::motion::backend::dualsense_calibration;
use bevy_archie::touchpad::dualsense_touchpad;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ControllerPlugin::default())
        .add_systems(Update, (
            setup_controller_features,
            process_dualsense_motion,
            process_dualsense_touchpad,
        ))
        .run();
}

fn setup_controller_features(
    query: Query<(&DetectedController, Entity), Added<DetectedController>>,
    mut commands: Commands,
) {
    for (detected, entity) in &query {
        match detected.model {
            ControllerModel::PS5 => {
                // Enable all DualSense features
                commands.entity(entity).insert((
                    GyroData::default(),
                    AccelData::default(),
                    TouchpadData::default(),
                ));
                println!("DualSense features enabled");
            }
            ControllerModel::PS4 => {
                commands.entity(entity).insert((
                    GyroData::default(),
                    AccelData::default(),
                    TouchpadData::default(),
                ));
                println!("DualShock 4 features enabled");
            }
            ControllerModel::PS3 => {
                commands.entity(entity).insert((
                    GyroData::default(),
                    AccelData::default(),
                ));
                println!("DualShock 3 features enabled (big-endian mode)");
            }
            ControllerModel::SwitchPro | ControllerModel::Switch2Pro => {
                commands.entity(entity).insert((
                    GyroData::default(),
                    AccelData::default(),
                ));
                println!("Switch Pro features enabled");
            }
            _ => {
                println!("Standard controller (no motion support)");
            }
        }
    }
}
```

## 6. Hardware Integration Guide Reference

For implementing actual HID drivers to feed data into these systems, see:

- `docs/HARDWARE_INTEGRATION_GUIDE.md`
- `examples/ps5_dualsense_motion.rs`
- `examples/switch_pro_gyro.rs`

The calibration constants provided make it easy to convert raw HID report data into the expected units (rad/s for gyro, m/s² for accelerometer, normalized 0.0-1.0 for touchpad).

## Summary

The Joypad OS enhancements provide:

1. **Automatic controller detection** with 20+ models
2. **Calibration constants** for accurate motion sensor data
3. **Touchpad specifications** for proper coordinate normalization
4. **Analog stick conventions** documented and implemented
5. **Controller-specific quirks** handling

All enhancements are **opt-in** and **backward-compatible**. Use what you need!
