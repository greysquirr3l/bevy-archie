//! Controller icon system.
//!
//! This module provides controller button icons that automatically
//! adapt to the current controller layout (Xbox, PlayStation, etc.).

use bevy::prelude::*;

use crate::config::ControllerLayout;

/// Icon size variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum IconSize {
    /// Small icons (32x32)
    Small,
    /// Medium icons (48x48)
    #[default]
    Medium,
    /// Large icons (64x64)
    Large,
}

impl IconSize {
    /// Get the pixel size for this icon size.
    pub fn pixels(&self) -> u32 {
        match self {
            Self::Small => 32,
            Self::Medium => 48,
            Self::Large => 64,
        }
    }

    /// Get the suffix for asset paths.
    pub fn suffix(&self) -> &'static str {
        match self {
            Self::Small => "_small",
            Self::Medium => "",
            Self::Large => "_large",
        }
    }
}

/// Button icon identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonIcon {
    // Face buttons
    FaceDown,  // A / Cross
    FaceRight, // B / Circle
    FaceLeft,  // X / Square
    FaceUp,    // Y / Triangle

    // Shoulder buttons
    LeftBumper,
    RightBumper,
    LeftTrigger,
    RightTrigger,

    // Stick buttons
    LeftStick,
    RightStick,
    LeftStickPress,
    RightStickPress,

    // D-pad
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    DPad, // Combined D-pad icon

    // System buttons
    Start,
    Select,
    Home,
}

impl ButtonIcon {
    /// Get the icon for a gamepad button type.
    pub fn from_button_type(button: GamepadButton) -> Option<Self> {
        match button {
            GamepadButton::South => Some(Self::FaceDown),
            GamepadButton::East => Some(Self::FaceRight),
            GamepadButton::West => Some(Self::FaceLeft),
            GamepadButton::North => Some(Self::FaceUp),
            GamepadButton::LeftTrigger => Some(Self::LeftBumper),
            GamepadButton::RightTrigger => Some(Self::RightBumper),
            GamepadButton::LeftTrigger2 => Some(Self::LeftTrigger),
            GamepadButton::RightTrigger2 => Some(Self::RightTrigger),
            GamepadButton::LeftThumb => Some(Self::LeftStickPress),
            GamepadButton::RightThumb => Some(Self::RightStickPress),
            GamepadButton::DPadUp => Some(Self::DPadUp),
            GamepadButton::DPadDown => Some(Self::DPadDown),
            GamepadButton::DPadLeft => Some(Self::DPadLeft),
            GamepadButton::DPadRight => Some(Self::DPadRight),
            GamepadButton::Start => Some(Self::Start),
            GamepadButton::Select => Some(Self::Select),
            GamepadButton::Mode => Some(Self::Home),
            _ => None,
        }
    }

    /// Get the asset filename for this icon on a specific layout.
    pub fn filename(&self, layout: ControllerLayout, size: IconSize) -> String {
        let base = match (layout, self) {
            // Face buttons vary by platform
            (ControllerLayout::PlayStation, Self::FaceDown) => "ps_cross",
            (ControllerLayout::PlayStation, Self::FaceRight) => "ps_circle",
            (ControllerLayout::PlayStation, Self::FaceLeft) => "ps_square",
            (ControllerLayout::PlayStation, Self::FaceUp) => "ps_triangle",

            (ControllerLayout::Nintendo, Self::FaceDown) => "switch_b",
            (ControllerLayout::Nintendo, Self::FaceRight) => "switch_a",
            (ControllerLayout::Nintendo, Self::FaceLeft) => "switch_y",
            (ControllerLayout::Nintendo, Self::FaceUp) => "switch_x",

            (ControllerLayout::Stadia, Self::FaceDown) => "stadia_a",
            (ControllerLayout::Stadia, Self::FaceRight) => "stadia_b",
            (ControllerLayout::Stadia, Self::FaceLeft) => "stadia_x",
            (ControllerLayout::Stadia, Self::FaceUp) => "stadia_y",

            (_, Self::FaceDown) => "xbox_a",
            (_, Self::FaceRight) => "xbox_b",
            (_, Self::FaceLeft) => "xbox_x",
            (_, Self::FaceUp) => "xbox_y",

            // Shoulder buttons
            (ControllerLayout::PlayStation, Self::LeftBumper) => "ps_l1",
            (ControllerLayout::PlayStation, Self::RightBumper) => "ps_r1",
            (ControllerLayout::PlayStation, Self::LeftTrigger) => "ps_l2",
            (ControllerLayout::PlayStation, Self::RightTrigger) => "ps_r2",

            (ControllerLayout::Nintendo, Self::LeftBumper) => "switch_l",
            (ControllerLayout::Nintendo, Self::RightBumper) => "switch_r",
            (ControllerLayout::Nintendo, Self::LeftTrigger) => "switch_zl",
            (ControllerLayout::Nintendo, Self::RightTrigger) => "switch_zr",

            (ControllerLayout::Stadia, Self::LeftBumper) => "stadia_l1",
            (ControllerLayout::Stadia, Self::RightBumper) => "stadia_r1",
            (ControllerLayout::Stadia, Self::LeftTrigger) => "stadia_l2",
            (ControllerLayout::Stadia, Self::RightTrigger) => "stadia_r2",

            (_, Self::LeftBumper) => "xbox_lb",
            (_, Self::RightBumper) => "xbox_rb",
            (_, Self::LeftTrigger) => "xbox_lt",
            (_, Self::RightTrigger) => "xbox_rt",

            // Sticks (same across platforms)
            (_, Self::LeftStick) => "left_stick",
            (_, Self::RightStick) => "right_stick",
            (_, Self::LeftStickPress) => "left_stick_press",
            (_, Self::RightStickPress) => "right_stick_press",

            // D-pad (same across platforms)
            (_, Self::DPadUp) => "dpad_up",
            (_, Self::DPadDown) => "dpad_down",
            (_, Self::DPadLeft) => "dpad_left",
            (_, Self::DPadRight) => "dpad_right",
            (_, Self::DPad) => "dpad",

            // System buttons
            (ControllerLayout::PlayStation, Self::Start) => "ps_options",
            (ControllerLayout::PlayStation, Self::Select) => "ps_share",
            (ControllerLayout::Nintendo, Self::Start) => "switch_plus",
            (ControllerLayout::Nintendo, Self::Select) => "switch_minus",
            (ControllerLayout::Stadia, Self::Start) => "stadia_menu",
            (ControllerLayout::Stadia, Self::Select) => "stadia_options",
            (ControllerLayout::Stadia, Self::Home) => "stadia_home",
            (_, Self::Start) => "xbox_menu",
            (_, Self::Select) => "xbox_view",
            (_, Self::Home) => "home",
        };

        format!("{}{}.png", base, size.suffix())
    }

    /// Get the text label for this button on a specific layout.
    pub fn label(&self, layout: ControllerLayout) -> &'static str {
        match (layout, self) {
            // Face buttons
            (ControllerLayout::PlayStation, Self::FaceDown) => "✕",
            (ControllerLayout::PlayStation, Self::FaceRight) => "○",
            (ControllerLayout::PlayStation, Self::FaceLeft) => "□",
            (ControllerLayout::PlayStation, Self::FaceUp) => "△",

            (ControllerLayout::Nintendo, Self::FaceDown) => "B",
            (ControllerLayout::Nintendo, Self::FaceRight) => "A",
            (ControllerLayout::Nintendo, Self::FaceLeft) => "Y",
            (ControllerLayout::Nintendo, Self::FaceUp) => "X",

            // Stadia uses Xbox-style labels
            (ControllerLayout::Stadia, Self::FaceDown) => "A",
            (ControllerLayout::Stadia, Self::FaceRight) => "B",
            (ControllerLayout::Stadia, Self::FaceLeft) => "X",
            (ControllerLayout::Stadia, Self::FaceUp) => "Y",

            (_, Self::FaceDown) => "A",
            (_, Self::FaceRight) => "B",
            (_, Self::FaceLeft) => "X",
            (_, Self::FaceUp) => "Y",

            // Shoulder buttons
            (ControllerLayout::PlayStation, Self::LeftBumper) => "L1",
            (ControllerLayout::PlayStation, Self::RightBumper) => "R1",
            (ControllerLayout::PlayStation, Self::LeftTrigger) => "L2",
            (ControllerLayout::PlayStation, Self::RightTrigger) => "R2",

            (ControllerLayout::Nintendo, Self::LeftBumper) => "L",
            (ControllerLayout::Nintendo, Self::RightBumper) => "R",
            (ControllerLayout::Nintendo, Self::LeftTrigger) => "ZL",
            (ControllerLayout::Nintendo, Self::RightTrigger) => "ZR",

            (ControllerLayout::Xbox, Self::LeftBumper) => "LB",
            (ControllerLayout::Xbox, Self::RightBumper) => "RB",
            (ControllerLayout::Xbox, Self::LeftTrigger) => "LT",
            (ControllerLayout::Xbox, Self::RightTrigger) => "RT",

            (ControllerLayout::Stadia, Self::LeftBumper) => "L1",
            (ControllerLayout::Stadia, Self::RightBumper) => "R1",
            (ControllerLayout::Stadia, Self::LeftTrigger) => "L2",
            (ControllerLayout::Stadia, Self::RightTrigger) => "R2",

            (_, Self::LeftBumper) => "L1",
            (_, Self::RightBumper) => "R1",
            (_, Self::LeftTrigger) => "L2",
            (_, Self::RightTrigger) => "R2",

            // Sticks
            (_, Self::LeftStick) => "LS",
            (_, Self::RightStick) => "RS",
            (_, Self::LeftStickPress) => "L3",
            (_, Self::RightStickPress) => "R3",

            // D-pad
            (_, Self::DPadUp) => "↑",
            (_, Self::DPadDown) => "↓",
            (_, Self::DPadLeft) => "←",
            (_, Self::DPadRight) => "→",
            (_, Self::DPad) => "D-Pad",

            // System
            (ControllerLayout::PlayStation, Self::Start) => "Options",
            (ControllerLayout::PlayStation, Self::Select) => "Share",
            (ControllerLayout::Nintendo, Self::Start) => "+",
            (ControllerLayout::Nintendo, Self::Select) => "-",
            (ControllerLayout::Stadia, Self::Start) => "Menu",
            (ControllerLayout::Stadia, Self::Select) => "Options",
            (_, Self::Start) => "Menu",
            (_, Self::Select) => "View",
            (_, Self::Home) => "Home",
        }
    }
}

/// Resource containing loaded controller icon assets.
#[derive(Debug, Default, Resource)]
pub struct ControllerIconAssets {
    /// Base path for icon assets.
    pub base_path: String,

    /// Cached icon handles.
    icons: std::collections::HashMap<(ButtonIcon, ControllerLayout, IconSize), Handle<Image>>,
}

impl ControllerIconAssets {
    /// Create a new icon assets resource with a base path.
    pub fn new(base_path: impl Into<String>) -> Self {
        Self {
            base_path: base_path.into(),
            icons: std::collections::HashMap::new(),
        }
    }

    /// Get or load an icon for a button.
    pub fn get_icon(
        &mut self,
        icon: ButtonIcon,
        layout: ControllerLayout,
        size: IconSize,
        asset_server: &AssetServer,
    ) -> Handle<Image> {
        let key = (icon, layout, size);

        if let Some(handle) = self.icons.get(&key) {
            return handle.clone();
        }

        let path = format!("{}/{}", self.base_path, icon.filename(layout, size));
        let handle = asset_server.load(&path);
        self.icons.insert(key, handle.clone());
        handle
    }

    /// Get an icon for a gamepad button type.
    pub fn get_button_icon(
        &mut self,
        button: GamepadButton,
        layout: ControllerLayout,
        size: IconSize,
        asset_server: &AssetServer,
    ) -> Option<Handle<Image>> {
        ButtonIcon::from_button_type(button)
            .map(|icon| self.get_icon(icon, layout, size, asset_server))
    }
}

/// Component for displaying a controller button icon.
#[derive(Debug, Clone, Component)]
pub struct ControllerIconDisplay {
    /// The button icon to display.
    pub icon: ButtonIcon,
    /// The icon size.
    pub size: IconSize,
    /// Whether to auto-update when layout changes.
    pub auto_update: bool,
}

impl Default for ControllerIconDisplay {
    fn default() -> Self {
        Self {
            icon: ButtonIcon::FaceDown,
            size: IconSize::Medium,
            auto_update: true,
        }
    }
}

/// System to update icon displays when layout changes.
pub fn update_icon_displays(
    mut icons: ResMut<ControllerIconAssets>,
    config: Res<crate::config::ControllerConfig>,
    asset_server: Res<AssetServer>,
    mut query: Query<(&ControllerIconDisplay, &mut ImageNode), Changed<ControllerIconDisplay>>,
) {
    let layout = config.layout();

    for (display, mut image) in query.iter_mut() {
        if display.auto_update {
            let handle = icons.get_icon(display.icon, layout, display.size, &asset_server);
            image.image = handle;
        }
    }
}

/// Plugin for registering icon types.
pub(crate) fn register_icon_types(app: &mut App) {
    app.init_resource::<ControllerIconAssets>();
}

/// Add icon systems to the app.
#[cfg(feature = "icons")]
pub(crate) fn add_icon_systems(app: &mut App) {
    app.add_systems(Update, update_icon_displays);
}

#[cfg(not(feature = "icons"))]
pub(crate) fn add_icon_systems(_app: &mut App) {}
