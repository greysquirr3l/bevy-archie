//! Steam Deck / Steam Controller Touchpad Integration
//!
//! This example shows how to integrate Steam Input API touchpad data
//! for Steam Deck and Steam Controller.
//!
//! ## Requirements
//!
//! For Steam Input API (recommended):
//! ```toml
//! [dependencies]
//! steamworks = "0.11"
//! ```
//!
//! For direct HID (advanced):
//! ```toml
//! [dependencies]
//! hidapi = "2.6"
//! ```
//!
//! ## Running
//!
//! ### Via Steam (recommended):
//! 1. Add your game to Steam as a non-Steam game
//! 2. Enable Steam Input in game properties
//! 3. Run via Steam
//! ```bash
//! cargo build --example steam_touchpad --features motion-backends
//! # Add target/debug/examples/steam_touchpad to Steam
//! ```
//!
//! ### Direct HID (advanced):
//! ```bash
//! cargo run --example steam_touchpad --features motion-backends
//! ```

use bevy::prelude::*;
use bevy_archie::prelude::*;
use bevy_archie::touchpad::TouchpadGestureEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ControllerPlugin::default())
        .add_systems(Startup, setup_steam_input)
        .add_systems(Update, inject_steam_touchpad_data)
        .add_systems(Update, display_touchpad_gestures)
        .run();
}

/// Resource to hold Steam Input context
#[derive(Resource)]
struct SteamInputContext {
    // client: steamworks::Client,
    _placeholder: u8,
}

/// Initialize Steam Input API
fn setup_steam_input(mut commands: Commands) {
    /* UNCOMMENT TO USE STEAMWORKS:

    match steamworks::Client::init() {
        Ok((client, single)) => {
            info!("Steam Input initialized!");

            // Enable Steam Input for controllers
            let input = client.input();
            input.init(false); // false = don't explicitly require controller focus

            commands.insert_resource(SteamInputContext { client });
        }
        Err(e) => {
            warn!("Failed to initialize Steam Input: {:?}", e);
            warn!("Make sure game is running through Steam client.");
        }
    }

    END UNCOMMENT */

    info!("Steam Input integration requires steamworks crate.");
    commands.insert_resource(SteamInputContext { _placeholder: 0 });
}

/// Read touchpad data via Steam Input API
fn inject_steam_touchpad_data(
    // steam: Option<Res<SteamInputContext>>,
    mut touchpad_query: Query<&mut TouchpadData>,
) {
    /* UNCOMMENT TO USE STEAMWORKS:

    let Some(steam) = steam else {
        return;
    };

    let input = steam.client.input();

    // Get all connected controllers
    let controllers = input.get_connected_controllers();

    for handle in controllers {
        // Get touchpad position (Steam Deck has 1 touchpad, Steam Controller has 2)
        // For Steam Deck touchpad:
        let motion_data = input.get_motion_data(handle);

        // Steam Input provides normalized touchpad coords (0.0-1.0)
        if let Some(touch_pos) = motion_data.touch_position {
            let x = touch_pos.x;
            let y = touch_pos.y;
            let active = motion_data.is_touching;

            for mut touchpad in &mut touchpad_query {
                touchpad.set_finger(0, x, y, active);
                touchpad.update_frame();
            }
        }

        // For Steam Controller, which has TWO circular touchpads:
        // Left pad is typically bound to d-pad or menu navigation
        // Right pad is used for camera/aiming
        // You can query them separately via action sets
    }

    END UNCOMMENT */

    // PLACEHOLDER: Simulate touchpad movement
    for mut touchpad in &mut touchpad_query {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f32();

        // Simulate a circular motion
        let x = ((time * 0.5).cos() + 1.0) * 0.5;
        let y = ((time * 0.5).sin() + 1.0) * 0.5;
        touchpad.set_finger(0, x, y, true);
        touchpad.update_frame();
    }
}

/// Display touchpad gestures as they're detected
fn display_touchpad_gestures(
    touchpad_query: Query<&TouchpadData>,
    mut gesture_events: MessageReader<TouchpadGestureEvent>,
) {
    // Display raw touchpad data
    for touchpad in &touchpad_query {
        if touchpad.finger1.active {
            let pos = touchpad.finger1.position();
            let delta = touchpad.finger1_delta();

            if delta.length() > 0.01 {
                debug!(
                    "Touchpad: pos=({:.3}, {:.3}), delta=({:.3}, {:.3})",
                    pos.x, pos.y, delta.x, delta.y
                );
            }
        }
    }

    // Display detected gestures
    for event in gesture_events.read() {
        info!(
            "🖐️ Touchpad Gesture: {:?} at ({:.2}, {:.2})",
            event.gesture, event.position.x, event.position.y
        );

        match event.gesture {
            TouchpadGesture::Tap => {
                info!("  → Quick tap detected! Use for selection/confirmation.");
            }
            TouchpadGesture::TwoFingerTap => {
                info!("  → Two-finger tap detected! Use for alternate actions.");
            }
            TouchpadGesture::SwipeLeft => {
                info!("  → Swipe left! Use for navigation or quick commands.");
            }
            TouchpadGesture::SwipeRight => {
                info!("  → Swipe right! Use for navigation or quick commands.");
            }
            TouchpadGesture::SwipeUp => {
                info!("  → Swipe up! Use for navigation or quick commands.");
            }
            TouchpadGesture::SwipeDown => {
                info!("  → Swipe down! Use for navigation or quick commands.");
            }
            TouchpadGesture::PinchIn => {
                info!("  → Pinch in (zoom out)!");
            }
            TouchpadGesture::PinchOut => {
                info!("  → Pinch out (zoom in)!");
            }
        }
    }
}

// ============================================================================
// Steam Deck Specific Features
// ============================================================================

/// Steam Deck has unique touchpad features:
/// - Large central touchpad (can be used as mouse replacement)
/// - Haptic feedback via Steam Input API
/// - Per-game touchpad bindings via Steam Input Configurator
///
/// Example action binding in `steam_input_manifest.vdf`:
/// ```vdf
/// "actions"
/// {
///     "camera_control"
///     {
///         "type" "analog"
///         "input_source" "touchpad"
///     }
///     "quick_menu"
///     {
///         "type" "digital"  
///         "input_source" "touchpad_tap"
///     }
/// }
/// ```
#[allow(dead_code, clippy::missing_const_for_fn)]
fn send_steam_deck_haptics(
    // steam: &SteamInputContext,
    intensity: f32,
    duration_ms: u32,
) {
    /* UNCOMMENT TO USE STEAMWORKS:

    let input = steam.client.input();
    let controllers = input.get_connected_controllers();

    for handle in controllers {
        // Trigger haptic pulse (intensity: 0.0-1.0)
        input.trigger_vibration(
            handle,
            (intensity * 65535.0) as u16, // Left motor
            (intensity * 65535.0) as u16, // Right motor
            duration_ms,
        );
    }

    END UNCOMMENT */

    let _ = (intensity, duration_ms); // Suppress warnings
}

// ============================================================================
// Alternative: Direct HID for Steam Controller (Advanced)
// ============================================================================

/// Steam Controller HID report format (for those who want maximum control)
///
/// USB Report (0x01, 64 bytes):
/// - Byte 8: Left pad touch (bit 7)
/// - Bytes 16-17: Left pad X - i16 little-endian (-32768 to 32767)
/// - Bytes 18-19: Left pad Y - i16 little-endian
/// - Byte 8: Right pad touch (bit 6)
/// - Bytes 20-21: Right pad X - i16 little-endian
/// - Bytes 22-23: Right pad Y - i16 little-endian
///
/// Note: Steam Controller uses proprietary wireless dongle or Bluetooth.
/// Direct HID requires handling the wireless protocol.
/// Steam Input API is STRONGLY recommended instead.
#[allow(dead_code, clippy::indexing_slicing, clippy::cast_lossless)]
fn parse_steam_controller_hid(report: &[u8]) -> Option<(f32, f32, bool)> {
    if report.len() < 24 {
        return None;
    }

    let touch_flags = report[8];
    let right_pad_touched = (touch_flags & 0x40) != 0;

    if right_pad_touched {
        let x_raw = i16::from_le_bytes([report[20], report[21]]);
        let y_raw = i16::from_le_bytes([report[22], report[23]]);

        // Normalize to 0.0-1.0
        let x = ((x_raw as f32 / 32768.0) + 1.0) * 0.5;
        let y = ((y_raw as f32 / 32768.0) + 1.0) * 0.5;

        Some((x, y, true))
    } else {
        Some((0.0, 0.0, false))
    }
}
