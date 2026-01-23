//! Controller profiles and auto-detection.
//!
//! This module provides controller-specific profiles that can be
//! automatically loaded based on detected hardware.

use bevy::prelude::*;
use std::collections::HashMap;

use crate::actions::ActionMap;
use crate::config::ControllerLayout;

/// Controller connection type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ConnectionType {
    /// USB wired connection.
    USB,
    /// Bluetooth wireless connection.
    Bluetooth,
    /// Unknown connection type.
    Unknown,
}

/// Controller-specific quirks or special handling requirements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ControllerQuirk {
    /// DualShock 4 over Bluetooth uses different HID report format than USB.
    DS4BluetoothReportDiffers,
    /// Some 8BitDo controllers report as Xbox but need special handling.
    EightBitDoXInputMode,
    /// Switch Pro needs handshake over USB before data is available.
    SwitchProUSBHandshake,
    /// Controller requires big-endian value interpretation (e.g., PS3 SIXAXIS).
    BigEndianValues,
}

/// Controller model/type identification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ControllerModel {
    /// Xbox 360 controller.
    Xbox360,
    /// Xbox One controller.
    XboxOne,
    /// Xbox Series X|S controller.
    XboxSeriesXS,
    /// `PlayStation` 3 `DualShock` 3 (SIXAXIS).
    PS3,
    /// `PlayStation` 4 `DualShock` 4.
    PS4,
    /// `PlayStation` 5 `DualSense`.
    PS5,
    /// Nintendo Switch Pro Controller.
    SwitchPro,
    /// Nintendo Switch Joy-Con (pair).
    SwitchJoyCon,
    /// Nintendo Switch 2 Pro Controller.
    Switch2Pro,
    /// Nintendo Switch 2 GameCube-style controller.
    Switch2GC,
    /// Steam Controller.
    Steam,
    /// Google Stadia Controller (Bluetooth mode).
    Stadia,
    /// Amazon Luna Controller.
    Luna,
    /// 8BitDo M30 (Genesis/Mega Drive style).
    EightBitDoM30,
    /// 8BitDo SN30 Pro (SNES style).
    EightBitDoSN30Pro,
    /// HORI Fighting Commander.
    HoriFightingCommander,
    /// Generic/unknown controller.
    Generic,
}

impl ControllerModel {
    /// Get the default layout for this controller model.
    #[must_use]
    pub const fn default_layout(self) -> ControllerLayout {
        match self {
            Self::Xbox360 | Self::XboxOne | Self::XboxSeriesXS | Self::Luna => {
                ControllerLayout::Xbox
            }
            Self::PS3 | Self::PS4 | Self::PS5 => ControllerLayout::PlayStation,
            Self::SwitchPro | Self::SwitchJoyCon | Self::Switch2Pro | Self::Switch2GC => {
                ControllerLayout::Nintendo
            }
            Self::EightBitDoM30 => ControllerLayout::Nintendo, // Genesis/MD layout similar to Nintendo
            Self::EightBitDoSN30Pro => ControllerLayout::Nintendo, // SNES-style
            Self::HoriFightingCommander => ControllerLayout::PlayStation,
            Self::Steam | Self::Stadia | Self::Generic => ControllerLayout::Xbox,
        }
    }

    /// Check if this controller supports advanced features.
    #[must_use]
    pub const fn supports_gyro(self) -> bool {
        matches!(
            self,
            Self::PS3
                | Self::PS4
                | Self::PS5
                | Self::SwitchPro
                | Self::SwitchJoyCon
                | Self::Switch2Pro
                | Self::Switch2GC
                | Self::Stadia
                | Self::Steam
        )
    }

    /// Check if this controller supports touchpad.
    #[must_use]
    pub const fn supports_touchpad(self) -> bool {
        matches!(self, Self::PS4 | Self::PS5 | Self::Steam)
    }

    /// Check if this controller supports pressure-sensitive buttons.
    #[must_use]
    pub const fn supports_pressure_buttons(self) -> bool {
        matches!(self, Self::PS3) // DualShock 3 has 12 pressure-sensitive buttons
    }

    /// Check if this controller supports adaptive triggers.
    #[must_use]
    pub const fn supports_adaptive_triggers(self) -> bool {
        matches!(self, Self::PS5)
    }
}

/// Component storing detected controller model.
#[derive(Debug, Clone, Copy, Component, Reflect)]
pub struct DetectedController {
    /// The detected model.
    pub model: ControllerModel,
    /// Vendor ID.
    pub vendor_id: u16,
    /// Product ID.
    pub product_id: u16,
}

impl DetectedController {
    /// Create a new detected controller.
    #[must_use]
    pub fn new(vendor_id: u16, product_id: u16) -> Self {
        let model = Self::identify(vendor_id, product_id);
        Self {
            model,
            vendor_id,
            product_id,
        }
    }

    /// Identify controller model from vendor/product IDs.
    ///
    /// VID/PID database compiled from:
    /// - USB-IF Vendor ID Database
    /// - Joypad OS controller registry
    /// - Community controller databases
    fn identify(vendor_id: u16, product_id: u16) -> ControllerModel {
        match (vendor_id, product_id) {
            // Microsoft Xbox controllers (VID: 0x045E)
            (0x045e, 0x028e) => ControllerModel::Xbox360,
            (0x045e, 0x02d1 | 0x02dd | 0x02e3 | 0x02ea | 0x0b00) => ControllerModel::XboxOne,
            (0x045e, 0x0b12 | 0x0b13) => ControllerModel::XboxSeriesXS,

            // Sony PlayStation controllers (VID: 0x054C)
            (0x054c, 0x0268) => ControllerModel::PS3, // DualShock 3 / SIXAXIS
            (0x054c, 0x05c4) => ControllerModel::PS4, // DualShock 4 (USB)
            (0x054c, 0x09cc) => ControllerModel::PS4, // DualShock 4 v2 (Bluetooth)
            (0x054c, 0x0ba0) => ControllerModel::PS4, // DualShock 4 USB Wireless Adapter
            (0x054c, 0x0ce6) => ControllerModel::PS5, // DualSense
            (0x054c, 0x0df2) => ControllerModel::PS5, // DualSense Edge

            // Nintendo Switch controllers (VID: 0x057E)
            (0x057e, 0x2009) => ControllerModel::SwitchPro,
            (0x057e, 0x2006) => ControllerModel::SwitchJoyCon, // Joy-Con Left
            (0x057e, 0x2007) => ControllerModel::SwitchJoyCon, // Joy-Con Right
            (0x057e, 0x2072) => ControllerModel::Switch2Pro,   // Switch 2 Pro Controller
            (0x057e, 0x2073) => ControllerModel::Switch2GC,    // Switch 2 GameCube-style

            // 8BitDo controllers (VID: 0x2DC8)
            (0x2dc8, 0x5006) => ControllerModel::EightBitDoM30, // M30 (Genesis/MD style)
            (0x2dc8, 0x6001 | 0x6101) => ControllerModel::EightBitDoSN30Pro, // SN30 Pro variants

            // HORI controllers (VID: 0x0F0D)
            (0x0f0d, 0x00c1) => ControllerModel::HoriFightingCommander,

            // Valve Steam Controller (VID: 0x28DE)
            (0x28de, 0x1142) => ControllerModel::Steam,

            // Google Stadia Controller (VID: 0x18D1) - Bluetooth mode only
            (0x18d1, 0x9400) => ControllerModel::Stadia,

            // Amazon Luna Controller (VID: 0x0171)
            (0x0171, 0x0419) => ControllerModel::Luna,

            // Unknown/Generic
            _ => ControllerModel::Generic,
        }
    }

    /// Get the connection type hint based on product ID patterns.
    ///
    /// Note: This is a heuristic based on common PID patterns.
    /// Real connection type detection requires platform-specific APIs.
    #[must_use]
    pub fn connection_type_hint(&self) -> ConnectionType {
        match (self.vendor_id, self.product_id) {
            // Sony PS4 Bluetooth PIDs
            (0x054c, 0x09cc | 0x0ba0) => ConnectionType::Bluetooth,
            // Most others USB by default
            _ => ConnectionType::Unknown,
        }
    }

    /// Get quirks for this controller.
    #[must_use]
    pub fn quirks(&self) -> Vec<ControllerQuirk> {
        let mut quirks = Vec::new();

        match self.model {
            ControllerModel::PS4 if self.product_id == 0x09cc => {
                quirks.push(ControllerQuirk::DS4BluetoothReportDiffers);
            }
            ControllerModel::PS3 => {
                quirks.push(ControllerQuirk::BigEndianValues);
            }
            ControllerModel::SwitchPro if self.connection_type_hint() == ConnectionType::USB => {
                quirks.push(ControllerQuirk::SwitchProUSBHandshake);
            }
            ControllerModel::EightBitDoM30 | ControllerModel::EightBitDoSN30Pro => {
                quirks.push(ControllerQuirk::EightBitDoXInputMode);
            }
            _ => {}
        }

        quirks
    }
}

/// A controller profile with custom settings.
#[derive(Debug, Clone, Resource)]
pub struct ControllerProfile {
    /// Profile name.
    pub name: String,
    /// Target controller model.
    pub model: ControllerModel,
    /// Custom action map for this profile.
    pub action_map: Option<ActionMap>,
    /// Layout override.
    pub layout: Option<ControllerLayout>,
}

impl ControllerProfile {
    /// Create a new profile.
    #[must_use]
    pub fn new(name: impl Into<String>, model: ControllerModel) -> Self {
        Self {
            name: name.into(),
            model,
            action_map: None,
            layout: None,
        }
    }

    /// Set custom action map.
    #[must_use]
    pub fn with_action_map(mut self, action_map: ActionMap) -> Self {
        self.action_map = Some(action_map);
        self
    }

    /// Set layout override.
    #[must_use]
    pub fn with_layout(mut self, layout: ControllerLayout) -> Self {
        self.layout = Some(layout);
        self
    }
}

/// Registry of controller profiles.
#[derive(Debug, Clone, Default, Resource)]
pub struct ProfileRegistry {
    /// Profiles mapped by controller model.
    pub profiles: HashMap<ControllerModel, ControllerProfile>,
    /// Whether to auto-load profiles.
    pub auto_load: bool,
}

impl ProfileRegistry {
    /// Register a profile.
    pub fn register(&mut self, profile: ControllerProfile) {
        self.profiles.insert(profile.model, profile);
    }

    /// Get a profile for a controller model.
    #[must_use]
    pub fn get(&self, model: ControllerModel) -> Option<&ControllerProfile> {
        self.profiles.get(&model)
    }
}

/// Event fired when a controller model is detected.
#[derive(Debug, Clone, Message)]
pub struct ControllerDetected {
    /// The gamepad entity.
    pub gamepad: Entity,
    /// Detected model.
    pub model: ControllerModel,
}

/// System to detect controller models.
pub fn detect_controller_models(
    mut gamepads: Query<(Entity, &Gamepad, Option<&Name>), Added<Gamepad>>,
    mut commands: Commands,
    mut detected_events: MessageWriter<ControllerDetected>,
) {
    for (entity, _gamepad, name) in &mut gamepads {
        // Note: Bevy 0.17 doesn't expose vendor/product IDs directly
        // This would need platform-specific implementation or gilrs integration
        // For now, try to detect from name
        let model = if let Some(name) = name {
            let name_lower = name.to_string().to_lowercase();
            if name_lower.contains("xbox") {
                if name_lower.contains("360") {
                    ControllerModel::Xbox360
                } else if name_lower.contains("series") {
                    ControllerModel::XboxSeriesXS
                } else {
                    ControllerModel::XboxOne
                }
            } else if name_lower.contains("playstation") || name_lower.contains("dualshock") {
                ControllerModel::PS4
            } else if name_lower.contains("dualsense") {
                ControllerModel::PS5
            } else if name_lower.contains("switch") {
                if name_lower.contains("pro") {
                    ControllerModel::SwitchPro
                } else {
                    ControllerModel::SwitchJoyCon
                }
            } else {
                ControllerModel::Generic
            }
        } else {
            ControllerModel::Generic
        };

        let detected = DetectedController {
            model,
            vendor_id: 0,
            product_id: 0,
        };

        commands.entity(entity).insert(detected);
        detected_events.write(ControllerDetected {
            gamepad: entity,
            model,
        });
    }
}

/// System to auto-load profiles when controllers are detected.
pub fn auto_load_profiles(
    mut detected_events: MessageReader<ControllerDetected>,
    registry: Res<ProfileRegistry>,
    mut action_map: ResMut<ActionMap>,
) {
    if !registry.auto_load {
        return;
    }

    for event in detected_events.read() {
        if let Some(profile) = registry.get(event.model) {
            // Apply profile settings
            if let Some(ref profile_map) = profile.action_map {
                // Merge or replace action map
                *action_map = profile_map.clone();
            }
        }
    }
}

/// Plugin for registering profile types.
pub(crate) fn register_profile_types(app: &mut App) {
    app.register_type::<ControllerModel>()
        .register_type::<DetectedController>()
        .init_resource::<ProfileRegistry>()
        .add_message::<ControllerDetected>();
}

/// Add profile systems to the app.
pub(crate) fn add_profile_systems(app: &mut App) {
    app.add_systems(
        Update,
        (detect_controller_models, auto_load_profiles).chain(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_controller_model_variants() {
        assert_ne!(ControllerModel::Xbox360, ControllerModel::XboxOne);
        assert_ne!(ControllerModel::PS4, ControllerModel::PS5);
        assert_ne!(ControllerModel::SwitchPro, ControllerModel::SwitchJoyCon);
    }

    #[test]
    fn test_controller_model_default_layout() {
        assert_eq!(
            ControllerModel::Xbox360.default_layout(),
            ControllerLayout::Xbox
        );
        assert_eq!(
            ControllerModel::XboxOne.default_layout(),
            ControllerLayout::Xbox
        );
        assert_eq!(
            ControllerModel::PS4.default_layout(),
            ControllerLayout::PlayStation
        );
        assert_eq!(
            ControllerModel::PS5.default_layout(),
            ControllerLayout::PlayStation
        );
        assert_eq!(
            ControllerModel::SwitchPro.default_layout(),
            ControllerLayout::Nintendo
        );
        assert_eq!(
            ControllerModel::Steam.default_layout(),
            ControllerLayout::Xbox
        );
    }

    #[test]
    fn test_controller_model_supports_gyro() {
        assert!(ControllerModel::PS4.supports_gyro());
        assert!(ControllerModel::PS5.supports_gyro());
        assert!(ControllerModel::SwitchPro.supports_gyro());
        assert!(!ControllerModel::Xbox360.supports_gyro());
        assert!(!ControllerModel::XboxOne.supports_gyro());
    }

    #[test]
    fn test_controller_model_supports_touchpad() {
        assert!(ControllerModel::PS4.supports_touchpad());
        assert!(ControllerModel::PS5.supports_touchpad());
        assert!(ControllerModel::Steam.supports_touchpad());
        assert!(!ControllerModel::Xbox360.supports_touchpad());
        assert!(!ControllerModel::SwitchPro.supports_touchpad());
    }

    #[test]
    fn test_controller_model_supports_adaptive_triggers() {
        assert!(ControllerModel::PS5.supports_adaptive_triggers());
        assert!(!ControllerModel::PS4.supports_adaptive_triggers());
        assert!(!ControllerModel::Xbox360.supports_adaptive_triggers());
        assert!(!ControllerModel::SwitchPro.supports_adaptive_triggers());
    }

    #[test]
    fn test_detected_controller_creation() {
        let detected = DetectedController {
            model: ControllerModel::PS5,
            vendor_id: 0x054C,
            product_id: 0x0CE6,
        };

        assert_eq!(detected.model, ControllerModel::PS5);
        assert_eq!(detected.vendor_id, 0x054C);
        assert_eq!(detected.product_id, 0x0CE6);
    }

    #[test]
    fn test_controller_profile_creation() {
        let profile = ControllerProfile {
            name: "Custom Xbox".to_string(),
            model: ControllerModel::XboxOne,
            action_map: Some(ActionMap::default()),
            layout: Some(ControllerLayout::Xbox),
        };

        assert_eq!(profile.name, "Custom Xbox");
        assert_eq!(profile.model, ControllerModel::XboxOne);
        assert!(profile.action_map.is_some());
        assert_eq!(profile.layout, Some(ControllerLayout::Xbox));
    }

    #[test]
    fn test_profile_registry_default() {
        let registry = ProfileRegistry::default();
        assert_eq!(registry.profiles.len(), 0);
        assert!(!registry.auto_load);
    }

    #[test]
    fn test_controller_detected_event() {
        let gamepad = Entity::from_bits(99);
        let event = ControllerDetected {
            gamepad,
            model: ControllerModel::PS4,
        };

        assert_eq!(event.gamepad, gamepad);
        assert_eq!(event.model, ControllerModel::PS4);
    }

    #[test]
    fn test_controller_model_default_layout_all() {
        assert_eq!(
            ControllerModel::Xbox360.default_layout(),
            ControllerLayout::Xbox
        );
        assert_eq!(
            ControllerModel::XboxOne.default_layout(),
            ControllerLayout::Xbox
        );
        assert_eq!(
            ControllerModel::XboxSeriesXS.default_layout(),
            ControllerLayout::Xbox
        );
        assert_eq!(
            ControllerModel::PS4.default_layout(),
            ControllerLayout::PlayStation
        );
        assert_eq!(
            ControllerModel::PS5.default_layout(),
            ControllerLayout::PlayStation
        );
        assert_eq!(
            ControllerModel::SwitchPro.default_layout(),
            ControllerLayout::Nintendo
        );
        assert_eq!(
            ControllerModel::SwitchJoyCon.default_layout(),
            ControllerLayout::Nintendo
        );
        assert_eq!(
            ControllerModel::Steam.default_layout(),
            ControllerLayout::Xbox
        );
        assert_eq!(
            ControllerModel::Stadia.default_layout(),
            ControllerLayout::Xbox
        );
        assert_eq!(
            ControllerModel::Generic.default_layout(),
            ControllerLayout::Xbox
        );
    }

    #[test]
    fn test_controller_model_supports_gyro_all() {
        assert!(!ControllerModel::Xbox360.supports_gyro());
        assert!(!ControllerModel::XboxOne.supports_gyro());
        assert!(!ControllerModel::XboxSeriesXS.supports_gyro());
        assert!(ControllerModel::PS4.supports_gyro());
        assert!(ControllerModel::PS5.supports_gyro());
        assert!(ControllerModel::SwitchPro.supports_gyro());
        assert!(ControllerModel::SwitchJoyCon.supports_gyro());
        assert!(ControllerModel::Steam.supports_gyro());
        assert!(ControllerModel::Stadia.supports_gyro());
        assert!(!ControllerModel::Generic.supports_gyro());
    }

    #[test]
    fn test_controller_model_supports_touchpad_all() {
        assert!(!ControllerModel::Xbox360.supports_touchpad());
        assert!(!ControllerModel::XboxOne.supports_touchpad());
        assert!(!ControllerModel::XboxSeriesXS.supports_touchpad());
        assert!(ControllerModel::PS4.supports_touchpad());
        assert!(ControllerModel::PS5.supports_touchpad());
        assert!(!ControllerModel::SwitchPro.supports_touchpad());
        assert!(!ControllerModel::SwitchJoyCon.supports_touchpad());
        assert!(ControllerModel::Steam.supports_touchpad());
        assert!(!ControllerModel::Stadia.supports_touchpad());
        assert!(!ControllerModel::Generic.supports_touchpad());
    }

    #[test]
    fn test_controller_model_supports_adaptive_triggers_all() {
        assert!(!ControllerModel::Xbox360.supports_adaptive_triggers());
        assert!(!ControllerModel::XboxOne.supports_adaptive_triggers());
        assert!(!ControllerModel::XboxSeriesXS.supports_adaptive_triggers());
        assert!(!ControllerModel::PS4.supports_adaptive_triggers());
        assert!(ControllerModel::PS5.supports_adaptive_triggers());
        assert!(!ControllerModel::SwitchPro.supports_adaptive_triggers());
        assert!(!ControllerModel::SwitchJoyCon.supports_adaptive_triggers());
        assert!(!ControllerModel::Steam.supports_adaptive_triggers());
        assert!(!ControllerModel::Stadia.supports_adaptive_triggers());
        assert!(!ControllerModel::Generic.supports_adaptive_triggers());
    }

    #[test]
    fn test_profile_registry_register() {
        let mut registry = ProfileRegistry::default();
        let profile = ControllerProfile {
            name: "Test Profile".to_string(),
            model: ControllerModel::PS5,
            action_map: None,
            layout: None,
        };

        registry.register(profile.clone());
        assert_eq!(registry.profiles.len(), 1);
        assert!(registry.profiles.contains_key(&ControllerModel::PS5));
    }

    #[test]
    fn test_profile_registry_get() {
        let mut registry = ProfileRegistry::default();
        let profile = ControllerProfile {
            name: "PS5 Profile".to_string(),
            model: ControllerModel::PS5,
            action_map: None,
            layout: Some(ControllerLayout::PlayStation),
        };

        registry.register(profile);

        let retrieved = registry.get(ControllerModel::PS5);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "PS5 Profile");
    }

    #[test]
    fn test_detected_controller_check_methods() {
        let detected_ps5 = DetectedController {
            model: ControllerModel::PS5,
            vendor_id: 0x054C,
            product_id: 0x0CE6,
        };

        assert!(detected_ps5.model.supports_gyro());
        assert!(detected_ps5.model.supports_touchpad());
        assert!(detected_ps5.model.supports_adaptive_triggers());

        let detected_xbox = DetectedController {
            model: ControllerModel::XboxOne,
            vendor_id: 0x045E,
            product_id: 0x02DD,
        };

        assert!(!detected_xbox.model.supports_gyro());
        assert!(!detected_xbox.model.supports_touchpad());
        assert!(!detected_xbox.model.supports_adaptive_triggers());
    }

    #[test]
    fn test_controller_model_all_variants() {
        let all_models = [
            ControllerModel::Xbox360,
            ControllerModel::XboxOne,
            ControllerModel::XboxSeriesXS,
            ControllerModel::PS3,
            ControllerModel::PS4,
            ControllerModel::PS5,
            ControllerModel::SwitchPro,
            ControllerModel::SwitchJoyCon,
            ControllerModel::Switch2Pro,
            ControllerModel::Switch2GC,
            ControllerModel::Steam,
            ControllerModel::Stadia,
            ControllerModel::Luna,
            ControllerModel::EightBitDoM30,
            ControllerModel::EightBitDoSN30Pro,
            ControllerModel::HoriFightingCommander,
            ControllerModel::Generic,
        ];

        // Ensure all are unique
        for (i, &model1) in all_models.iter().enumerate() {
            for (j, &model2) in all_models.iter().enumerate() {
                if i != j {
                    assert_ne!(model1, model2);
                }
            }
        }
    }

    // ========== New Controller Detection Tests ==========

    #[test]
    fn test_detected_controller_identify_ps3() {
        let detected = DetectedController::new(0x054c, 0x0268);
        assert_eq!(detected.model, ControllerModel::PS3);
    }

    #[test]
    fn test_detected_controller_identify_switch2_pro() {
        let detected = DetectedController::new(0x057e, 0x2072);
        assert_eq!(detected.model, ControllerModel::Switch2Pro);
    }

    #[test]
    fn test_detected_controller_identify_switch2_gc() {
        let detected = DetectedController::new(0x057e, 0x2073);
        assert_eq!(detected.model, ControllerModel::Switch2GC);
    }

    #[test]
    fn test_detected_controller_identify_8bitdo_m30() {
        let detected = DetectedController::new(0x2dc8, 0x5006);
        assert_eq!(detected.model, ControllerModel::EightBitDoM30);
    }

    #[test]
    fn test_detected_controller_identify_8bitdo_sn30pro() {
        let detected = DetectedController::new(0x2dc8, 0x6001);
        assert_eq!(detected.model, ControllerModel::EightBitDoSN30Pro);
    }

    #[test]
    fn test_detected_controller_identify_hori_fighting_commander() {
        let detected = DetectedController::new(0x0f0d, 0x00c1);
        assert_eq!(detected.model, ControllerModel::HoriFightingCommander);
    }

    #[test]
    fn test_detected_controller_identify_luna() {
        let detected = DetectedController::new(0x0171, 0x0419);
        assert_eq!(detected.model, ControllerModel::Luna);
    }

    #[test]
    fn test_connection_type_hint_bluetooth() {
        let detected = DetectedController::new(0x054c, 0x09cc);
        assert_eq!(detected.connection_type_hint(), ConnectionType::Bluetooth);
    }

    #[test]
    fn test_connection_type_hint_unknown() {
        let detected = DetectedController::new(0x045e, 0x028e);
        assert_eq!(detected.connection_type_hint(), ConnectionType::Unknown);
    }

    #[test]
    fn test_quirks_ps4_bluetooth() {
        let detected = DetectedController::new(0x054c, 0x09cc);
        let quirks = detected.quirks();
        assert!(quirks.contains(&ControllerQuirk::DS4BluetoothReportDiffers));
    }

    #[test]
    fn test_quirks_ps3() {
        let detected = DetectedController::new(0x054c, 0x0268);
        let quirks = detected.quirks();
        assert!(quirks.contains(&ControllerQuirk::BigEndianValues));
    }

    #[test]
    fn test_quirks_8bitdo() {
        let detected = DetectedController::new(0x2dc8, 0x5006);
        let quirks = detected.quirks();
        assert!(quirks.contains(&ControllerQuirk::EightBitDoXInputMode));
    }

    #[test]
    fn test_supports_pressure_buttons() {
        assert!(ControllerModel::PS3.supports_pressure_buttons());
        assert!(!ControllerModel::PS4.supports_pressure_buttons());
        assert!(!ControllerModel::Xbox360.supports_pressure_buttons());
    }

    // ========== Additional DetectedController Tests ==========

    #[test]
    fn test_detected_controller_new() {
        let detected = DetectedController::new(0x054c, 0x0ce6);
        assert_eq!(detected.model, ControllerModel::PS5);
        assert_eq!(detected.vendor_id, 0x054c);
        assert_eq!(detected.product_id, 0x0ce6);
    }

    #[test]
    fn test_detected_controller_identify_xbox360() {
        let detected = DetectedController::new(0x045e, 0x028e);
        assert_eq!(detected.model, ControllerModel::Xbox360);
    }

    #[test]
    fn test_detected_controller_identify_xboxone() {
        let detected = DetectedController::new(0x045e, 0x02d1);
        assert_eq!(detected.model, ControllerModel::XboxOne);
    }

    #[test]
    fn test_detected_controller_identify_xbox_series() {
        let detected = DetectedController::new(0x045e, 0x0b13);
        assert_eq!(detected.model, ControllerModel::XboxSeriesXS);
    }

    #[test]
    fn test_detected_controller_identify_ps4() {
        let detected = DetectedController::new(0x054c, 0x05c4);
        assert_eq!(detected.model, ControllerModel::PS4);
    }

    #[test]
    fn test_detected_controller_identify_ps4_slim() {
        let detected = DetectedController::new(0x054c, 0x09cc);
        assert_eq!(detected.model, ControllerModel::PS4);
    }

    #[test]
    fn test_detected_controller_identify_switch_pro() {
        let detected = DetectedController::new(0x057e, 0x2009);
        assert_eq!(detected.model, ControllerModel::SwitchPro);
    }

    #[test]
    fn test_detected_controller_identify_joycon_left() {
        let detected = DetectedController::new(0x057e, 0x2006);
        assert_eq!(detected.model, ControllerModel::SwitchJoyCon);
    }

    #[test]
    fn test_detected_controller_identify_joycon_right() {
        let detected = DetectedController::new(0x057e, 0x2007);
        assert_eq!(detected.model, ControllerModel::SwitchJoyCon);
    }

    #[test]
    fn test_detected_controller_identify_steam() {
        let detected = DetectedController::new(0x28de, 0x1142);
        assert_eq!(detected.model, ControllerModel::Steam);
    }

    #[test]
    fn test_detected_controller_identify_stadia() {
        let detected = DetectedController::new(0x18d1, 0x9400);
        assert_eq!(detected.model, ControllerModel::Stadia);
    }

    #[test]
    fn test_detected_controller_identify_generic() {
        let detected = DetectedController::new(0x1234, 0x5678);
        assert_eq!(detected.model, ControllerModel::Generic);
    }

    // ========== Additional ControllerProfile Tests ==========

    #[test]
    fn test_controller_profile_new() {
        let profile = ControllerProfile::new("My Profile", ControllerModel::PS5);
        assert_eq!(profile.name, "My Profile");
        assert_eq!(profile.model, ControllerModel::PS5);
        assert!(profile.action_map.is_none());
        assert!(profile.layout.is_none());
    }

    #[test]
    fn test_controller_profile_with_action_map() {
        let profile = ControllerProfile::new("Test", ControllerModel::XboxOne)
            .with_action_map(ActionMap::default());

        assert!(profile.action_map.is_some());
    }

    #[test]
    fn test_controller_profile_with_layout() {
        let profile = ControllerProfile::new("Test", ControllerModel::XboxOne)
            .with_layout(ControllerLayout::PlayStation);

        assert_eq!(profile.layout, Some(ControllerLayout::PlayStation));
    }

    #[test]
    fn test_controller_profile_builder_chain() {
        let profile = ControllerProfile::new("Full Profile", ControllerModel::PS4)
            .with_action_map(ActionMap::default())
            .with_layout(ControllerLayout::PlayStation);

        assert!(profile.action_map.is_some());
        assert_eq!(profile.layout, Some(ControllerLayout::PlayStation));
    }

    // ========== ProfileRegistry Additional Tests ==========

    #[test]
    fn test_profile_registry_get_nonexistent() {
        let registry = ProfileRegistry::default();
        let result = registry.get(ControllerModel::PS5);
        assert!(result.is_none());
    }

    #[test]
    fn test_profile_registry_multiple_profiles() {
        let mut registry = ProfileRegistry::default();

        registry.register(ControllerProfile::new("PS4", ControllerModel::PS4));
        registry.register(ControllerProfile::new("PS5", ControllerModel::PS5));
        registry.register(ControllerProfile::new("Xbox", ControllerModel::XboxOne));

        assert_eq!(registry.profiles.len(), 3);
        assert!(registry.get(ControllerModel::PS4).is_some());
        assert!(registry.get(ControllerModel::PS5).is_some());
        assert!(registry.get(ControllerModel::XboxOne).is_some());
    }

    #[test]
    fn test_profile_registry_replace_profile() {
        let mut registry = ProfileRegistry::default();

        registry.register(ControllerProfile::new("First", ControllerModel::PS5));
        registry.register(ControllerProfile::new("Second", ControllerModel::PS5));

        // Second should replace first
        assert_eq!(registry.profiles.len(), 1);
        let profile = registry.get(ControllerModel::PS5).unwrap();
        assert_eq!(profile.name, "Second");
    }

    #[test]
    fn test_profile_registry_auto_load_flag() {
        let mut registry = ProfileRegistry::default();
        assert!(!registry.auto_load);

        registry.auto_load = true;
        assert!(registry.auto_load);
    }
}
