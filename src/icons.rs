//! Controller icon system.
//!
//! This module provides controller button icons that automatically
//! adapt to the current controller layout (Xbox, `PlayStation`, etc.).

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
    #[must_use]
    pub const fn pixels(self) -> u32 {
        match self {
            Self::Small => 32,
            Self::Medium => 48,
            Self::Large => 64,
        }
    }

    /// Get the suffix for asset paths.
    #[must_use]
    pub const fn suffix(self) -> &'static str {
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
    #[must_use]
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
    #[must_use]
    pub fn filename(self, layout: ControllerLayout, size: IconSize) -> String {
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
    #[must_use]
    pub const fn label(self, layout: ControllerLayout) -> &'static str {
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
            (ControllerLayout::PlayStation | ControllerLayout::Stadia, Self::LeftBumper) => "L1",
            (ControllerLayout::PlayStation | ControllerLayout::Stadia, Self::RightBumper) => "R1",
            (ControllerLayout::PlayStation | ControllerLayout::Stadia, Self::LeftTrigger) => "L2",
            (ControllerLayout::PlayStation | ControllerLayout::Stadia, Self::RightTrigger) => "R2",

            (ControllerLayout::Nintendo, Self::LeftBumper) => "L",
            (ControllerLayout::Nintendo, Self::RightBumper) => "R",
            (ControllerLayout::Nintendo, Self::LeftTrigger) => "ZL",
            (ControllerLayout::Nintendo, Self::RightTrigger) => "ZR",

            (ControllerLayout::Xbox, Self::LeftBumper) => "LB",
            (ControllerLayout::Xbox, Self::RightBumper) => "RB",
            (ControllerLayout::Xbox, Self::LeftTrigger) => "LT",
            (ControllerLayout::Xbox, Self::RightTrigger) => "RT",

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
    #[must_use]
    pub fn new(base_path: impl Into<String>) -> Self {
        Self {
            base_path: base_path.into(),
            icons: std::collections::HashMap::new(),
        }
    }

    /// Get or load an icon for a button.
    #[must_use]
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
    #[must_use]
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
    asset_server: Option<Res<AssetServer>>,
    mut query: Query<(&ControllerIconDisplay, &mut ImageNode), Changed<ControllerIconDisplay>>,
) {
    // Skip if asset server is not available (e.g., in tests without asset plugin)
    let Some(asset_server) = asset_server else {
        return;
    };

    let layout = config.layout();

    for (display, mut image) in &mut query {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_size_pixels() {
        assert_eq!(IconSize::Small.pixels(), 32);
        assert_eq!(IconSize::Medium.pixels(), 48);
        assert_eq!(IconSize::Large.pixels(), 64);
    }

    #[test]
    fn test_icon_size_suffix() {
        assert_eq!(IconSize::Small.suffix(), "_small");
        assert_eq!(IconSize::Medium.suffix(), "");
        assert_eq!(IconSize::Large.suffix(), "_large");
    }

    #[test]
    fn test_icon_size_default() {
        assert_eq!(IconSize::default(), IconSize::Medium);
    }

    #[test]
    fn test_button_icon_from_button_type() {
        assert_eq!(
            ButtonIcon::from_button_type(GamepadButton::South),
            Some(ButtonIcon::FaceDown)
        );
        assert_eq!(
            ButtonIcon::from_button_type(GamepadButton::East),
            Some(ButtonIcon::FaceRight)
        );
        assert_eq!(
            ButtonIcon::from_button_type(GamepadButton::West),
            Some(ButtonIcon::FaceLeft)
        );
        assert_eq!(
            ButtonIcon::from_button_type(GamepadButton::North),
            Some(ButtonIcon::FaceUp)
        );
        assert_eq!(
            ButtonIcon::from_button_type(GamepadButton::LeftTrigger),
            Some(ButtonIcon::LeftBumper)
        );
        assert_eq!(
            ButtonIcon::from_button_type(GamepadButton::DPadUp),
            Some(ButtonIcon::DPadUp)
        );
    }

    #[test]
    fn test_button_icon_filename_xbox() {
        assert_eq!(
            ButtonIcon::FaceDown.filename(ControllerLayout::Xbox, IconSize::Medium),
            "xbox_a.png"
        );
        assert_eq!(
            ButtonIcon::FaceRight.filename(ControllerLayout::Xbox, IconSize::Small),
            "xbox_b_small.png"
        );
        assert_eq!(
            ButtonIcon::LeftBumper.filename(ControllerLayout::Xbox, IconSize::Large),
            "xbox_lb_large.png"
        );
    }

    #[test]
    fn test_button_icon_filename_playstation() {
        assert_eq!(
            ButtonIcon::FaceDown.filename(ControllerLayout::PlayStation, IconSize::Medium),
            "ps_cross.png"
        );
        assert_eq!(
            ButtonIcon::FaceRight.filename(ControllerLayout::PlayStation, IconSize::Medium),
            "ps_circle.png"
        );
        assert_eq!(
            ButtonIcon::FaceLeft.filename(ControllerLayout::PlayStation, IconSize::Medium),
            "ps_square.png"
        );
        assert_eq!(
            ButtonIcon::FaceUp.filename(ControllerLayout::PlayStation, IconSize::Medium),
            "ps_triangle.png"
        );
    }

    #[test]
    fn test_button_icon_filename_nintendo() {
        assert_eq!(
            ButtonIcon::FaceDown.filename(ControllerLayout::Nintendo, IconSize::Medium),
            "switch_b.png"
        );
        assert_eq!(
            ButtonIcon::FaceRight.filename(ControllerLayout::Nintendo, IconSize::Medium),
            "switch_a.png"
        );
        assert_eq!(
            ButtonIcon::LeftTrigger.filename(ControllerLayout::Nintendo, IconSize::Medium),
            "switch_zl.png"
        );
    }

    #[test]
    fn test_button_icon_filename_common() {
        assert_eq!(
            ButtonIcon::LeftStick.filename(ControllerLayout::Xbox, IconSize::Medium),
            "left_stick.png"
        );
        assert_eq!(
            ButtonIcon::DPad.filename(ControllerLayout::PlayStation, IconSize::Medium),
            "dpad.png"
        );
    }

    #[test]
    fn test_button_icon_label_xbox() {
        assert_eq!(ButtonIcon::FaceDown.label(ControllerLayout::Xbox), "A");
        assert_eq!(ButtonIcon::FaceRight.label(ControllerLayout::Xbox), "B");
        assert_eq!(ButtonIcon::LeftBumper.label(ControllerLayout::Xbox), "LB");
        assert_eq!(ButtonIcon::LeftTrigger.label(ControllerLayout::Xbox), "LT");
    }

    #[test]
    fn test_button_icon_label_playstation() {
        assert_eq!(
            ButtonIcon::FaceDown.label(ControllerLayout::PlayStation),
            "✕"
        );
        assert_eq!(
            ButtonIcon::FaceRight.label(ControllerLayout::PlayStation),
            "○"
        );
        assert_eq!(
            ButtonIcon::FaceLeft.label(ControllerLayout::PlayStation),
            "□"
        );
        assert_eq!(ButtonIcon::FaceUp.label(ControllerLayout::PlayStation), "△");
        assert_eq!(
            ButtonIcon::Start.label(ControllerLayout::PlayStation),
            "Options"
        );
        assert_eq!(
            ButtonIcon::Select.label(ControllerLayout::PlayStation),
            "Share"
        );
    }

    #[test]
    fn test_button_icon_label_nintendo() {
        assert_eq!(ButtonIcon::FaceDown.label(ControllerLayout::Nintendo), "B");
        assert_eq!(ButtonIcon::FaceRight.label(ControllerLayout::Nintendo), "A");
        assert_eq!(ButtonIcon::FaceLeft.label(ControllerLayout::Nintendo), "Y");
        assert_eq!(ButtonIcon::FaceUp.label(ControllerLayout::Nintendo), "X");
        assert_eq!(
            ButtonIcon::LeftTrigger.label(ControllerLayout::Nintendo),
            "ZL"
        );
        assert_eq!(ButtonIcon::Start.label(ControllerLayout::Nintendo), "+");
        assert_eq!(ButtonIcon::Select.label(ControllerLayout::Nintendo), "-");
    }

    #[test]
    fn test_controller_icon_assets_new() {
        let assets = ControllerIconAssets::new("assets/icons");
        assert_eq!(assets.base_path, "assets/icons");
    }

    #[test]
    fn test_controller_icon_display_default() {
        let display = ControllerIconDisplay::default();
        assert_eq!(display.icon, ButtonIcon::FaceDown);
        assert_eq!(display.size, IconSize::Medium);
        assert!(display.auto_update);
    }

    #[test]
    fn test_button_icon_filename_all_layouts() {
        // Test Xbox layout
        assert_eq!(
            ButtonIcon::FaceDown.filename(ControllerLayout::Xbox, IconSize::Medium),
            "xbox_a.png"
        );
        assert_eq!(
            ButtonIcon::LeftBumper.filename(ControllerLayout::Xbox, IconSize::Small),
            "xbox_lb_small.png"
        );
        assert_eq!(
            ButtonIcon::LeftTrigger.filename(ControllerLayout::Xbox, IconSize::Large),
            "xbox_lt_large.png"
        );

        // Test PlayStation layout
        assert_eq!(
            ButtonIcon::FaceDown.filename(ControllerLayout::PlayStation, IconSize::Medium),
            "ps_cross.png"
        );
        assert_eq!(
            ButtonIcon::FaceRight.filename(ControllerLayout::PlayStation, IconSize::Medium),
            "ps_circle.png"
        );
        assert_eq!(
            ButtonIcon::Start.filename(ControllerLayout::PlayStation, IconSize::Medium),
            "ps_options.png"
        );

        // Test Nintendo layout
        assert_eq!(
            ButtonIcon::FaceDown.filename(ControllerLayout::Nintendo, IconSize::Medium),
            "switch_b.png"
        );
        assert_eq!(
            ButtonIcon::Start.filename(ControllerLayout::Nintendo, IconSize::Medium),
            "switch_plus.png"
        );

        // Test Stadia layout
        assert_eq!(
            ButtonIcon::Home.filename(ControllerLayout::Stadia, IconSize::Medium),
            "stadia_home.png"
        );
    }

    #[test]
    fn test_button_icon_label_all_variants() {
        // Test shoulder buttons
        assert_eq!(ButtonIcon::LeftBumper.label(ControllerLayout::Xbox), "LB");
        assert_eq!(ButtonIcon::RightBumper.label(ControllerLayout::Xbox), "RB");
        assert_eq!(
            ButtonIcon::LeftBumper.label(ControllerLayout::PlayStation),
            "L1"
        );
        assert_eq!(
            ButtonIcon::LeftBumper.label(ControllerLayout::Nintendo),
            "L"
        );

        // Test triggers
        assert_eq!(ButtonIcon::LeftTrigger.label(ControllerLayout::Xbox), "LT");
        assert_eq!(
            ButtonIcon::LeftTrigger.label(ControllerLayout::Nintendo),
            "ZL"
        );
        assert_eq!(
            ButtonIcon::RightTrigger.label(ControllerLayout::Nintendo),
            "ZR"
        );

        // Test sticks
        assert_eq!(ButtonIcon::LeftStick.label(ControllerLayout::Xbox), "LS");
        assert_eq!(ButtonIcon::RightStick.label(ControllerLayout::Xbox), "RS");
        assert_eq!(
            ButtonIcon::LeftStickPress.label(ControllerLayout::PlayStation),
            "L3"
        );

        // Test D-pad
        assert_eq!(ButtonIcon::DPadUp.label(ControllerLayout::Xbox), "↑");
        assert_eq!(ButtonIcon::DPadDown.label(ControllerLayout::Xbox), "↓");
        assert_eq!(ButtonIcon::DPadLeft.label(ControllerLayout::Xbox), "←");
        assert_eq!(ButtonIcon::DPadRight.label(ControllerLayout::Xbox), "→");
        assert_eq!(ButtonIcon::DPad.label(ControllerLayout::Xbox), "D-Pad");

        // Test system buttons
        assert_eq!(ButtonIcon::Start.label(ControllerLayout::Xbox), "Menu");
        assert_eq!(ButtonIcon::Select.label(ControllerLayout::Xbox), "View");
        assert_eq!(ButtonIcon::Home.label(ControllerLayout::Xbox), "Home");
    }

    #[test]
    fn test_button_icon_all_variants() {
        let all_icons = [
            ButtonIcon::FaceDown,
            ButtonIcon::FaceRight,
            ButtonIcon::FaceLeft,
            ButtonIcon::FaceUp,
            ButtonIcon::LeftBumper,
            ButtonIcon::RightBumper,
            ButtonIcon::LeftTrigger,
            ButtonIcon::RightTrigger,
            ButtonIcon::LeftStick,
            ButtonIcon::RightStick,
            ButtonIcon::LeftStickPress,
            ButtonIcon::RightStickPress,
            ButtonIcon::DPadUp,
            ButtonIcon::DPadDown,
            ButtonIcon::DPadLeft,
            ButtonIcon::DPadRight,
            ButtonIcon::DPad,
            ButtonIcon::Start,
            ButtonIcon::Select,
            ButtonIcon::Home,
        ];

        // Verify all generate valid filenames
        for &icon in &all_icons {
            let filename = icon.filename(ControllerLayout::Xbox, IconSize::Medium);
            assert!(filename.ends_with(".png"));
            assert!(!filename.is_empty());
        }
    }

    #[test]
    fn test_controller_icon_assets_with_custom_path() {
        let assets = ControllerIconAssets::new("custom/path/icons");
        assert_eq!(assets.base_path, "custom/path/icons");
    }
}
