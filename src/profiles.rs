//! Controller profiles and auto-detection.
//!
//! This module provides controller-specific profiles that can be
//! automatically loaded based on detected hardware.

use bevy::prelude::*;
use std::collections::HashMap;

use crate::actions::ActionMap;
use crate::config::ControllerLayout;

/// Controller model/type identification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ControllerModel {
    /// Xbox 360 controller.
    Xbox360,
    /// Xbox One controller.
    XboxOne,
    /// Xbox Series X|S controller.
    XboxSeriesXS,
    /// `PlayStation` 4 `DualShock` 4.
    PS4,
    /// `PlayStation` 5 `DualSense`.
    PS5,
    /// Nintendo Switch Pro Controller.
    SwitchPro,
    /// Nintendo Switch Joy-Con (pair).
    SwitchJoyCon,
    /// Steam Controller.
    Steam,
    /// Generic/unknown controller.
    Generic,
}

impl ControllerModel {
    /// Get the default layout for this controller model.
    #[must_use]
    pub const fn default_layout(self) -> ControllerLayout {
        match self {
            Self::Xbox360 | Self::XboxOne | Self::XboxSeriesXS => ControllerLayout::Xbox,
            Self::PS4 | Self::PS5 => ControllerLayout::PlayStation,
            Self::SwitchPro | Self::SwitchJoyCon => ControllerLayout::Nintendo,
            Self::Steam | Self::Generic => ControllerLayout::Xbox,
        }
    }

    /// Check if this controller supports advanced features.
    #[must_use]
    pub const fn supports_gyro(self) -> bool {
        matches!(
            self,
            Self::PS4 | Self::PS5 | Self::SwitchPro | Self::SwitchJoyCon
        )
    }

    /// Check if this controller supports touchpad.
    #[must_use]
    pub const fn supports_touchpad(self) -> bool {
        matches!(self, Self::PS4 | Self::PS5 | Self::Steam)
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
    fn identify(vendor_id: u16, product_id: u16) -> ControllerModel {
        match (vendor_id, product_id) {
            // Microsoft Xbox controllers
            (0x045e, 0x028e) => ControllerModel::Xbox360,
            (0x045e, 0x02d1) => ControllerModel::XboxOne,
            (0x045e, 0x0b13) => ControllerModel::XboxSeriesXS,
            // Sony PlayStation controllers
            (0x054c, 0x05c4 | 0x09cc) => ControllerModel::PS4, // PS4 and PS4 Slim
            (0x054c, 0x0ce6) => ControllerModel::PS5,
            // Nintendo Switch controllers
            (0x057e, 0x2009) => ControllerModel::SwitchPro,
            (0x057e, 0x2006 | 0x2007) => ControllerModel::SwitchJoyCon,
            // Valve Steam Controller
            (0x28de, 0x1142) => ControllerModel::Steam,
            _ => ControllerModel::Generic,
        }
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
        assert!(!ControllerModel::Steam.supports_gyro());
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
            ControllerModel::PS4,
            ControllerModel::PS5,
            ControllerModel::SwitchPro,
            ControllerModel::SwitchJoyCon,
            ControllerModel::Steam,
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
