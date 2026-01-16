//! Nintendo Switch Pro Controller Gyro Integration Example
//!
//! This example demonstrates how to integrate Switch Pro Controller gyro data
//! using SDL2, which has built-in Switch Pro Controller support.
//!
//! ## Requirements
//!
//! Add to your Cargo.toml:
//! ```toml
//! [dependencies]
//! sdl2 = { version = "0.37", features = ["bundled"] }
//! ```
//!
//! ## Running
//!
//! ```bash
//! cargo run --example switch_pro_gyro --features motion-backends
//! ```
//!
//! ## How It Works
//!
//! 1. Uses SDL2's `GameController` API (supports Switch Pro via `SDL_HINT_JOYSTICK_HIDAPI_SWITCH_PRO`)
//! 2. Reads sensor data via `SDL_GameControllerGetSensorData`
//! 3. SDL handles all the low-level HID communication
//! 4. Injects calibrated gyro data into bevy-archie
//!
//! ## Why SDL2?
//!
//! - Cross-platform (Windows, macOS, Linux, Steam Deck)
//! - Handles Switch Pro pairing and authentication automatically
//! - Provides calibrated sensor data
//! - No need to parse raw HID reports

use bevy::prelude::*;
use bevy_archie::gyro::MotionGestureDetected;
use bevy_archie::prelude::*;

// Uncomment when sdl2 is available:
// use sdl2::controller::{GameController, Axis};
// use sdl2::sensor::SensorType;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ControllerPlugin::default())
        .add_systems(Startup, setup_sdl_controller)
        .add_systems(Update, inject_switch_gyro_data)
        .add_systems(Update, display_gyro_state)
        .run();
}

/// Resource to hold SDL2 controller handle
#[derive(Resource)]
struct SwitchController {
    // controller: GameController,
    _placeholder: u8,
}

/// Setup SDL2 and open Switch Pro Controller
fn setup_sdl_controller(mut commands: Commands) {
    /* UNCOMMENT TO USE REAL SDL2:

    let sdl_context = sdl2::init().expect("Failed to init SDL2");
    let game_controller = sdl_context
        .game_controller()
        .expect("Failed to get game controller subsystem");

    // Enable Switch Pro Controller support
    sdl2::hint::set("SDL_JOYSTICK_HIDAPI_SWITCH_PRO", "1");

    // Find and open first available controller
    let available = game_controller
        .num_joysticks()
        .expect("Failed to get joystick count");

    for id in 0..available {
        if game_controller.is_game_controller(id) {
            match game_controller.open(id) {
                Ok(controller) => {
                    info!("Opened controller: {}", controller.name());

                    // Enable gyro sensor
                    if controller.set_sensor_enabled(SensorType::Gyroscope, true).is_ok() {
                        info!("Gyro sensor enabled!");
                        commands.insert_resource(SwitchController { controller });
                        return;
                    }
                }
                Err(e) => warn!("Failed to open controller {}: {}", id, e),
            }
        }
    }

    END UNCOMMENT */

    warn!(
        "Switch Pro Controller integration requires SDL2. Add sdl2 to Cargo.toml and uncomment the code."
    );
    commands.insert_resource(SwitchController { _placeholder: 0 });
}

/// Read gyro data from Switch Pro Controller via SDL2
fn inject_switch_gyro_data(
    // switch_controller: Option<Res<SwitchController>>,
    mut gyro_query: Query<&mut GyroData>,
) {
    /* UNCOMMENT TO USE REAL SDL2:

    let Some(switch) = switch_controller else {
        return;
    };

    // Read gyro sensor data
    let mut gyro_data = [0.0f32; 3];
    if switch
        .controller
        .sensor_get_data(SensorType::Gyroscope, &mut gyro_data)
        .is_ok()
    {
        // SDL2 returns gyro in rad/s (x, y, z)
        // Switch Pro orientation: X=pitch, Y=yaw, Z=roll
        let pitch = gyro_data[0];
        let yaw = gyro_data[1];
        let roll = gyro_data[2];

        // Inject into bevy-archie
        for mut gyro in &mut gyro_query {
            gyro.set_raw(pitch, yaw, roll);
        }
    }

    END UNCOMMENT */

    // PLACEHOLDER: Simulate gyro data
    for mut gyro in &mut gyro_query {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f32();
        gyro.set_raw(
            (time * 0.7).sin() * 0.15,
            (time * 0.5).cos() * 0.12,
            (time * 0.3).sin() * 0.08,
        );
    }
}

/// Display gyro state and detected gestures
fn display_gyro_state(
    gyro_query: Query<&GyroData>,
    mut gesture_events: MessageReader<MotionGestureDetected>,
) {
    for gyro in &gyro_query {
        if gyro.valid {
            let magnitude = gyro.magnitude();
            if magnitude > 0.1 {
                info!(
                    "Gyro active: pitch={:.3}, yaw={:.3}, roll={:.3}, mag={:.3}",
                    gyro.pitch, gyro.yaw, gyro.roll, magnitude
                );
            }
        }
    }

    for event in gesture_events.read() {
        info!("🎮 Motion Gesture Detected: {:?}", event.gesture);
        match event.gesture {
            MotionGesture::Shake => info!("  → Shake detected! Use for actions like reload."),
            MotionGesture::Tilt => {
                info!("  → Tilt detected! Use for steering or aiming.");
            }
            MotionGesture::Flick => {
                info!("  → Flick detected! Use for quick actions.");
            }
            MotionGesture::Roll => {
                info!("  → Roll detected! Use for barrel rolls.");
            }
        }
    }
}

// ============================================================================
// Alternative: Direct HID Approach (more control, more complexity)
// ============================================================================

/// For advanced users who need more control, you can parse HID reports directly.
/// Switch Pro Controller HID report format (USB mode):
///
/// Report ID: 0x30 (Full report)
/// - Bytes 13-14: Gyro X (pitch) - i16 little-endian
/// - Bytes 15-16: Gyro Y (roll) - i16 little-endian  
/// - Bytes 17-18: Gyro Z (yaw) - i16 little-endian
/// - Bytes 19-24: Second gyro sample (Switch sends 3 samples per frame)
/// - Bytes 25-30: Third gyro sample
///
/// Gyro scale: 13371 LSB per deg/s → multiply by (π/180 / 13371) for rad/s
///
/// Example with hidapi:
/// ```rust,ignore
/// let gyro_x_raw = i16::from_le_bytes([buf[13], buf[14]]);
/// let gyro_scale = (std::f32::consts::PI / 180.0) / 13371.0;
/// let pitch = gyro_x_raw as f32 * gyro_scale;
/// ```
///
/// Note: Switch Pro uses Bluetooth authentication. You'll need to:
/// 1. Pair the controller via OS Bluetooth settings
/// 2. Handle the pairing handshake if using raw HID
/// 3. OR use SDL2 which handles this for you
#[allow(
    dead_code,
    clippy::indexing_slicing,
    clippy::similar_names,
    clippy::cast_lossless
)]
fn parse_switch_pro_hid_report(report: &[u8]) -> Option<(f32, f32, f32)> {
    if report.len() < 31 || report[0] != 0x30 {
        return None;
    }

    // Parse first gyro sample (Switch sends 3 samples at ~333Hz each)
    let gyro_x_raw = i16::from_le_bytes([report[13], report[14]]);
    let gyro_y_raw = i16::from_le_bytes([report[15], report[16]]);
    let gyro_z_raw = i16::from_le_bytes([report[17], report[18]]);

    // Convert to rad/s
    let gyro_scale = (std::f32::consts::PI / 180.0) / 13371.0;
    let pitch = gyro_x_raw as f32 * gyro_scale;
    let roll = gyro_y_raw as f32 * gyro_scale;
    let yaw = gyro_z_raw as f32 * gyro_scale;

    Some((pitch, yaw, roll))
}
