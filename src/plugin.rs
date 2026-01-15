//! Main controller plugin.
//!
//! This module provides the main plugin that ties together all
//! controller support functionality.

use bevy::prelude::*;

/// The main controller support plugin.
///
/// Add this plugin to your app to enable controller support:
///
/// ```rust,no_run
/// use bevy::prelude::*;
/// use bevy_archie::prelude::*;
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugins(ControllerPlugin::default())
///         .run();
/// }
/// ```
#[derive(Debug, Clone, Default)]
pub struct ControllerPlugin {
    /// Base path for controller icon assets.
    pub icon_base_path: Option<String>,
}

impl ControllerPlugin {
    /// Create a new controller plugin with custom icon path.
    pub fn with_icon_path(icon_path: impl Into<String>) -> Self {
        Self {
            icon_base_path: Some(icon_path.into()),
        }
    }
}

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        // Register types
        crate::config::register_config_types(app);
        crate::detection::register_detection_types(app);
        crate::actions::register_action_types(app);
        crate::icons::register_icon_types(app);
        crate::virtual_cursor::register_virtual_cursor_types(app);

        // Set up icon path if provided
        if let Some(path) = &self.icon_base_path {
            app.insert_resource(crate::icons::ControllerIconAssets::new(path.clone()));
        }

        // Add systems
        crate::detection::add_detection_systems(app);
        crate::actions::add_action_systems(app);
        crate::icons::add_icon_systems(app);
        crate::virtual_cursor::add_virtual_cursor_systems(app);

        // Add feature-gated systems
        #[cfg(feature = "remapping")]
        crate::remapping::add_remapping_systems(app);

        #[cfg(feature = "virtual_keyboard")]
        crate::virtual_keyboard::add_virtual_keyboard_systems(app);
    }
}

/// System set for controller input processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum ControllerSystemSet {
    /// Device detection runs first.
    Detection,
    /// Action state updates.
    Actions,
    /// UI updates based on input state.
    UI,
}
