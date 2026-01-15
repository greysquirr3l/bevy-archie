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
