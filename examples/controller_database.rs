//! Example demonstrating the enhanced controller detection database.
//!
//! This example shows how to use the comprehensive VID/PID database
//! to detect specific controller models and their capabilities.
//!
//! Run with:
//! ```sh
//! cargo run --example controller_database
//! ```

use bevy::prelude::*;
use bevy_archie::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ControllerPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (display_controller_info, demonstrate_database))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Text::new(
            "Controller Detection Database Example\n\nConnect a controller to see its details...",
        ),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            ..default()
        },
    ));
}

fn display_controller_info(
    detected_query: Query<(&DetectedController, &Gamepad), Added<DetectedController>>,
    mut text_query: Query<&mut Text>,
) {
    for (detected, _gamepad) in &detected_query {
        let mut info = format!(
            "Controller Detected!\n\n\
             Model: {:?}\n\
             VID: 0x{:04X}\n\
             PID: 0x{:04X}\n\n",
            detected.model, detected.vendor_id, detected.product_id
        );

        // Display capabilities
        info.push_str("Capabilities:\n");
        if detected.model.supports_gyro() {
            info.push_str("  ✓ Gyroscope/Motion Control\n");
        }
        if detected.model.supports_touchpad() {
            info.push_str("  ✓ Touchpad\n");
        }
        if detected.model.supports_adaptive_triggers() {
            info.push_str("  ✓ Adaptive Triggers (DualSense)\n");
        }
        if detected.model.supports_pressure_buttons() {
            info.push_str("  ✓ Pressure-Sensitive Buttons (DualShock 3)\n");
        }

        // Display layout
        info.push_str(&format!(
            "\nButton Layout: {:?}\n",
            detected.model.default_layout()
        ));

        // Display connection type hint
        info.push_str(&format!(
            "Connection: {:?}\n",
            detected.connection_type_hint()
        ));

        // Display quirks
        let quirks = detected.quirks();
        if !quirks.is_empty() {
            info.push_str("\nSpecial Handling Required:\n");
            for quirk in quirks {
                info.push_str(&format!("  • {:?}\n", quirk));
            }
        }

        // Update the text display
        for mut text in &mut text_query {
            text.0 = info.clone();
        }

        println!("{}", info);
    }
}

fn demonstrate_database(_commands: Commands) {
    // This function demonstrates how to use the database for controller-specific logic
    // In a real game, you might do things like:
    //
    // if detected.model == ControllerModel::PS5 {
    //     // Enable DualSense-specific features
    //     setup_adaptive_triggers();
    //     enable_touchpad_gestures();
    // }
    //
    // if detected.model.supports_gyro() {
    //     // Enable motion controls
    //     setup_gyro_aiming();
    // }
    //
    // if detected.quirks().contains(&ControllerQuirk::BigEndianValues) {
    //     // Apply byte-swapping for PS3 controllers
    //     use_big_endian_conversion();
    // }
}

/// Example: Print all supported controller models
#[allow(dead_code)]
fn print_supported_controllers() {
    use bevy_archie::profiles::DetectedController;

    println!("Supported Controllers:\n");

    // Sony controllers
    let ps3 = DetectedController::new(0x054c, 0x0268);
    println!("✓ {} - {:?}", "PlayStation 3 DualShock 3", ps3.model);

    let ps4_usb = DetectedController::new(0x054c, 0x05c4);
    println!(
        "✓ {} - {:?}",
        "PlayStation 4 DualShock 4 (USB)", ps4_usb.model
    );

    let ps4_bt = DetectedController::new(0x054c, 0x09cc);
    println!(
        "✓ {} - {:?}",
        "PlayStation 4 DualShock 4 (BT)", ps4_bt.model
    );

    let ps5 = DetectedController::new(0x054c, 0x0ce6);
    println!("✓ {} - {:?}", "PlayStation 5 DualSense", ps5.model);

    // Microsoft controllers
    let xbox360 = DetectedController::new(0x045e, 0x028e);
    println!("✓ {} - {:?}", "Xbox 360", xbox360.model);

    let xboxone = DetectedController::new(0x045e, 0x02d1);
    println!("✓ {} - {:?}", "Xbox One", xboxone.model);

    let xboxseries = DetectedController::new(0x045e, 0x0b13);
    println!("✓ {} - {:?}", "Xbox Series X|S", xboxseries.model);

    // Nintendo controllers
    let switch_pro = DetectedController::new(0x057e, 0x2009);
    println!("✓ {} - {:?}", "Switch Pro Controller", switch_pro.model);

    let joycon_l = DetectedController::new(0x057e, 0x2006);
    println!("✓ {} - {:?}", "Switch Joy-Con Left", joycon_l.model);

    let switch2_pro = DetectedController::new(0x057e, 0x2072);
    println!("✓ {} - {:?}", "Switch 2 Pro Controller", switch2_pro.model);

    // Others
    let steam = DetectedController::new(0x28de, 0x1142);
    println!("✓ {} - {:?}", "Steam Controller", steam.model);

    let stadia = DetectedController::new(0x18d1, 0x9400);
    println!("✓ {} - {:?}", "Google Stadia (Bluetooth)", stadia.model);

    let eightbitdo_m30 = DetectedController::new(0x2dc8, 0x5006);
    println!("✓ {} - {:?}", "8BitDo M30", eightbitdo_m30.model);

    let eightbitdo_sn30 = DetectedController::new(0x2dc8, 0x6001);
    println!("✓ {} - {:?}", "8BitDo SN30 Pro", eightbitdo_sn30.model);

    println!("\nTotal: 15+ controllers with hardware-specific support");
}
