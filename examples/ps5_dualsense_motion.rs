//! PS5 `DualSense` Gyro and Touchpad Integration Example
//!
//! This example demonstrates how to integrate real `DualSense` controller
//! motion and touchpad data using the hidapi crate.
//!
//! ## Requirements
//!
//! Add to your Cargo.toml:
//! ```toml
//! [dependencies]
//! hidapi = "2.6"
//! ```
//!
//! ## Running
//!
//! ```bash
//! cargo run --example ps5_dualsense_motion --features motion-backends
//! ```
//!
//! ## How It Works
//!
//! 1. Opens HID connection to `DualSense` controller (USB or Bluetooth)
//! 2. Reads HID input reports (0x01 for USB, 0x31 for Bluetooth)
//! 3. Parses gyro data (bytes 16-21) and touchpad data (bytes 33+)
//! 4. Injects data into bevy-archie's `GyroData` and `TouchpadData` components
//! 5. Gesture detection happens automatically via bevy-archie systems
//!
//! ## `DualSense` HID Report Format
//!
//! USB Report (0x01, 64 bytes):
//! - Bytes 16-17: Gyro X (pitch) - i16 little-endian
//! - Bytes 18-19: Gyro Y (yaw) - i16 little-endian  
//! - Bytes 20-21: Gyro Z (roll) - i16 little-endian
//! - Bytes 22-23: Accel X - i16 little-endian
//! - Bytes 24-25: Accel Y - i16 little-endian
//! - Bytes 26-27: Accel Z - i16 little-endian
//! - Byte 33: Touchpad data header
//! - Bytes 34+: Finger 1 and 2 positions
//!
//! Bluetooth Report (0x31, 78 bytes): Same offsets + 2 bytes

use bevy::prelude::*;
use bevy_archie::gyro::MotionGestureDetected;
use bevy_archie::prelude::*;
use bevy_archie::touchpad::TouchpadGestureEvent;

// Uncomment when hidapi is available:
// use hidapi::{HidApi, HidDevice};
// use std::sync::{Arc, Mutex};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ControllerPlugin::default())
        // Add our custom gyro/touchpad injection system
        .add_systems(Update, inject_dualsense_data)
        .add_systems(Update, display_motion_data)
        .run();
}

/// System to read `DualSense` HID data and inject into bevy-archie components.
///
/// This is where you connect real hardware to the framework.
fn inject_dualsense_data(
    mut gyro_query: Query<&mut GyroData>,
    _accel_query: Query<&mut AccelData>,
    _touchpad_query: Query<&mut TouchpadData>,
) {
    // NOTE: This is a template. To use this example, you need to:
    // 1. Add hidapi = "2.6" to Cargo.toml
    // 2. Uncomment the hidapi code below
    // 3. Handle the HidDevice connection (open on startup, store in Resource)

    /* UNCOMMENT TO USE REAL HIDAPI:

    // Open DualSense controller (do this once at startup and store in Resource)
    // USB: VID 0x054c, PID 0x0ce6
    // Bluetooth: VID 0x054c, PID 0x0ce6
    let api = HidApi::new().expect("Failed to create HidApi");
    let device = api
        .open(0x054c, 0x0ce6)
        .expect("Failed to open DualSense controller");

    // Read HID report
    let mut buf = [0u8; 64];
    match device.read_timeout(&mut buf, 10) {
        Ok(size) if size > 0 => {
            // Check report ID
            let report_id = buf[0];
            let is_bluetooth = report_id == 0x31;
            let offset = if is_bluetooth { 2 } else { 0 }; // BT has 2 extra header bytes

            // Parse gyro data (raw sensor values, need calibration)
            // DualSense gyro is in deg/s, needs conversion to rad/s
            let gyro_x_raw = i16::from_le_bytes([buf[16 + offset], buf[17 + offset]]);
            let gyro_y_raw = i16::from_le_bytes([buf[18 + offset], buf[19 + offset]]);
            let gyro_z_raw = i16::from_le_bytes([buf[20 + offset], buf[21 + offset]]);

            // Convert to rad/s (DualSense gyro scale: ~0.001064 deg/s per LSB)
            let gyro_scale = 0.001064_f32.to_radians();
            let pitch = gyro_x_raw as f32 * gyro_scale;
            let yaw = gyro_y_raw as f32 * gyro_scale;
            let roll = gyro_z_raw as f32 * gyro_scale;

            // Parse accelerometer data
            let accel_x_raw = i16::from_le_bytes([buf[22 + offset], buf[23 + offset]]);
            let accel_y_raw = i16::from_le_bytes([buf[24 + offset], buf[25 + offset]]);
            let accel_z_raw = i16::from_le_bytes([buf[26 + offset], buf[27 + offset]]);

            // Convert to m/s² (DualSense accel scale: ~0.0001196 g per LSB)
            let accel_scale = 0.0001196_f32 * 9.81; // Convert g to m/s²
            let accel_x = accel_x_raw as f32 * accel_scale;
            let accel_y = accel_y_raw as f32 * accel_scale;
            let accel_z = accel_z_raw as f32 * accel_scale;

            // Inject gyro data
            for mut gyro in &mut gyro_query {
                gyro.set_raw(pitch, yaw, roll);
            }

            // Inject accelerometer data
            for mut accel in &mut accel_query {
                accel.set_raw(accel_x, accel_y, accel_z);
            }

            // Parse touchpad data (if present)
            let touchpad_byte = buf[33 + offset];
            let finger1_active = (touchpad_byte & 0x01) == 0; // Active when bit is 0
            let finger2_active = (touchpad_byte & 0x02) == 0;

            if finger1_active {
                // Finger 1: bytes 34-36
                let finger1_x_raw = u16::from_le_bytes([buf[34 + offset], buf[35 + offset]]) & 0x0FFF;
                let finger1_y_raw = u16::from_le_bytes([buf[36 + offset], buf[37 + offset]]) & 0x0FFF;

                // Normalize to 0.0-1.0 (DualSense touchpad: 1920x1080 resolution)
                let finger1_x = finger1_x_raw as f32 / 1920.0;
                let finger1_y = finger1_y_raw as f32 / 1080.0;

                for mut touchpad in &mut touchpad_query {
                    touchpad.set_finger(0, finger1_x, finger1_y, true);
                }
            } else {
                for mut touchpad in &mut touchpad_query {
                    touchpad.set_finger(0, 0.0, 0.0, false);
                }
            }

            if finger2_active {
                // Finger 2: bytes 38-40
                let finger2_x_raw = u16::from_le_bytes([buf[38 + offset], buf[39 + offset]]) & 0x0FFF;
                let finger2_y_raw = u16::from_le_bytes([buf[40 + offset], buf[41 + offset]]) & 0x0FFF;

                let finger2_x = finger2_x_raw as f32 / 1920.0;
                let finger2_y = finger2_y_raw as f32 / 1080.0;

                for mut touchpad in &mut touchpad_query {
                    touchpad.set_finger(1, finger2_x, finger2_y, true);
                }
            } else {
                for mut touchpad in &mut touchpad_query {
                    touchpad.set_finger(1, 0.0, 0.0, false);
                }
            }

            // Update touchpad frame (for delta calculations)
            for mut touchpad in &mut touchpad_query {
                touchpad.update_frame();
            }
        }
        _ => {} // No data or timeout
    }

    END UNCOMMENT */

    // PLACEHOLDER: Simulate some motion data for demo purposes
    // Replace this with real HID code above
    for mut gyro in &mut gyro_query {
        // Simulate a gentle rotation
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f32();
        gyro.set_raw((time * 0.5).sin() * 0.1, (time * 0.3).cos() * 0.1, 0.0);
    }
}

/// Display motion and touchpad data to console
fn display_motion_data(
    gyro_query: Query<&GyroData>,
    touchpad_query: Query<&TouchpadData>,
    mut gesture_events: MessageReader<MotionGestureDetected>,
    mut touchpad_events: MessageReader<TouchpadGestureEvent>,
) {
    // Display gyro data
    for gyro in &gyro_query {
        if gyro.valid {
            info!(
                "Gyro: pitch={:.3}, yaw={:.3}, roll={:.3}, mag={:.3}",
                gyro.pitch,
                gyro.yaw,
                gyro.roll,
                gyro.magnitude()
            );
        }
    }

    // Display motion gestures
    for event in gesture_events.read() {
        info!("Motion Gesture: {:?}", event.gesture);
    }

    // Display touchpad data
    for touchpad in &touchpad_query {
        if touchpad.finger1.active {
            let pos = touchpad.finger1.position();
            let delta = touchpad.finger1_delta();
            info!(
                "Finger 1: pos=({:.3}, {:.3}), delta=({:.3}, {:.3})",
                pos.x, pos.y, delta.x, delta.y
            );
        }
        if touchpad.finger2.active {
            let pos = touchpad.finger2.position();
            info!("Finger 2: pos=({:.3}, {:.3})", pos.x, pos.y);
        }
    }

    // Display touchpad gestures
    for event in touchpad_events.read() {
        info!(
            "Touchpad Gesture: {:?} at {:?}",
            event.gesture, event.position
        );
    }
}

// ============================================================================
// Production-Ready Resource Pattern
// ============================================================================

/// Store the HID device connection as a Bevy Resource.
/// This is the recommended pattern for production use.
#[allow(dead_code)]
#[derive(Resource)]
struct DualSenseDevice {
    // device: Arc<Mutex<HidDevice>>,
    is_bluetooth: bool,
}

#[allow(dead_code)]
impl DualSenseDevice {
    /// Open a `DualSense` controller connection.
    #[allow(clippy::missing_const_for_fn)]
    fn new() -> Option<Self> {
        // In real code:
        // let api = HidApi::new().ok()?;
        // let device = api.open(0x054c, 0x0ce6).ok()?;
        // Some(Self {
        //     device: Arc::new(Mutex::new(device)),
        //     is_bluetooth: detect_bluetooth_mode(&device),
        // })
        None
    }
}

/// System to setup `DualSense` device at startup
#[allow(dead_code)]
fn setup_dualsense(mut commands: Commands) {
    if let Some(device) = DualSenseDevice::new() {
        commands.insert_resource(device);
        info!("DualSense controller connected!");
    } else {
        warn!("No DualSense controller found");
    }
}
