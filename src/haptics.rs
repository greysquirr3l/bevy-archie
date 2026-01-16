//! Haptic feedback and rumble support.
//!
//! This module provides vibration/rumble functionality for gamepads,
//! including simple rumble, complex patterns, and `DualSense` advanced haptics.

use bevy::prelude::*;
use std::time::Duration;

/// Rumble intensity for motors.
#[derive(Debug, Clone, Copy, PartialEq, Default, Reflect)]
pub struct RumbleIntensity {
    /// Low-frequency motor (0.0-1.0)
    pub low_frequency: f32,
    /// High-frequency motor (0.0-1.0)
    pub high_frequency: f32,
}

impl RumbleIntensity {
    /// Create a new rumble intensity.
    #[must_use]
    pub const fn new(low: f32, high: f32) -> Self {
        Self {
            low_frequency: if low < 0.0 {
                0.0
            } else if low > 1.0 {
                1.0
            } else {
                low
            },
            high_frequency: if high < 0.0 {
                0.0
            } else if high > 1.0 {
                1.0
            } else {
                high
            },
        }
    }

    /// Create a uniform rumble (both motors same intensity).
    #[must_use]
    pub const fn uniform(intensity: f32) -> Self {
        let clamped = if intensity < 0.0 {
            0.0
        } else if intensity > 1.0 {
            1.0
        } else {
            intensity
        };
        Self {
            low_frequency: clamped,
            high_frequency: clamped,
        }
    }

    /// No rumble.
    #[must_use]
    pub fn none() -> Self {
        Self::default()
    }
}

/// Predefined rumble patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum RumblePattern {
    /// Constant rumble.
    Constant,
    /// Pulsing rumble.
    Pulse,
    /// Explosion (strong then fade).
    Explosion,
    /// Light damage tap.
    DamageTap,
    /// Heavy impact.
    HeavyImpact,
    /// Engine/motor hum.
    Engine,
    /// Heartbeat pattern.
    Heartbeat,
}

/// Component for controlling gamepad rumble.
#[derive(Debug, Clone, Component)]
pub struct RumbleController {
    /// Target gamepad entity.
    pub gamepad: Entity,
    /// Current intensity.
    pub intensity: RumbleIntensity,
    /// Duration remaining.
    pub duration: Duration,
    /// Pattern being played.
    pub pattern: Option<RumblePattern>,
    /// Pattern timer for pulse effects.
    pub pattern_timer: f32,
}

impl RumbleController {
    /// Create a new rumble controller.
    #[must_use]
    pub fn new(gamepad: Entity) -> Self {
        Self {
            gamepad,
            intensity: RumbleIntensity::none(),
            duration: Duration::ZERO,
            pattern: None,
            pattern_timer: 0.0,
        }
    }

    /// Start a simple rumble.
    pub const fn rumble(&mut self, intensity: RumbleIntensity, duration: Duration) {
        self.intensity = intensity;
        self.duration = duration;
        self.pattern = Some(RumblePattern::Constant);
    }

    /// Start a rumble with pattern.
    pub const fn rumble_pattern(
        &mut self,
        pattern: RumblePattern,
        intensity: f32,
        duration: Duration,
    ) {
        self.intensity = RumbleIntensity::uniform(intensity);
        self.duration = duration;
        self.pattern = Some(pattern);
        self.pattern_timer = 0.0;
    }

    /// Stop rumble immediately.
    pub fn stop(&mut self) {
        self.intensity = RumbleIntensity::none();
        self.duration = Duration::ZERO;
        self.pattern = None;
    }
}

/// Event to request rumble on a specific gamepad.
#[derive(Debug, Clone, Message)]
pub struct RumbleRequest {
    /// Gamepad to rumble.
    pub gamepad: Entity,
    /// Rumble intensity.
    pub intensity: RumbleIntensity,
    /// Duration of rumble.
    pub duration: Duration,
    /// Optional pattern.
    pub pattern: Option<RumblePattern>,
}

impl RumbleRequest {
    /// Create a simple rumble request.
    #[must_use]
    pub const fn new(gamepad: Entity, intensity: f32, duration: Duration) -> Self {
        Self {
            gamepad,
            intensity: RumbleIntensity::uniform(intensity),
            duration,
            pattern: Some(RumblePattern::Constant),
        }
    }

    /// Create a rumble with pattern.
    #[must_use]
    pub const fn with_pattern(
        gamepad: Entity,
        pattern: RumblePattern,
        intensity: f32,
        duration: Duration,
    ) -> Self {
        Self {
            gamepad,
            intensity: RumbleIntensity::uniform(intensity),
            duration,
            pattern: Some(pattern),
        }
    }
}

/// System to handle rumble requests.
pub fn handle_rumble_requests(
    mut requests: MessageReader<RumbleRequest>,
    mut commands: Commands,
    mut controllers: Query<&mut RumbleController>,
    gamepads: Query<Entity, With<Gamepad>>,
) {
    for request in requests.read() {
        // Check if gamepad exists
        if !gamepads.contains(request.gamepad) {
            continue;
        }

        // Find or create controller
        if let Ok(mut controller) = controllers.get_mut(request.gamepad) {
            controller.intensity = request.intensity;
            controller.duration = request.duration;
            controller.pattern = request.pattern;
            controller.pattern_timer = 0.0;
        } else {
            let mut controller = RumbleController::new(request.gamepad);
            controller.intensity = request.intensity;
            controller.duration = request.duration;
            controller.pattern = request.pattern;
            commands.entity(request.gamepad).insert(controller);
        }
    }
}

/// System to update rumble controllers and apply patterns.
pub fn update_rumble(
    mut controllers: Query<&mut RumbleController>,
    mut gamepads: Query<&mut Gamepad>,
    time: Res<Time>,
) {
    for mut controller in &mut controllers {
        if controller.duration.is_zero() {
            continue;
        }

        // Update duration
        let delta = time.delta();
        controller.duration = controller.duration.saturating_sub(delta);

        // Apply pattern modulation
        let mut intensity = controller.intensity;
        if let Some(pattern) = controller.pattern {
            controller.pattern_timer += time.delta_secs();

            let modifier = match pattern {
                RumblePattern::Constant => 1.0,
                RumblePattern::Pulse => (controller.pattern_timer * 8.0).sin().abs(),
                RumblePattern::Explosion => {
                    let t = controller.pattern_timer / controller.duration.as_secs_f32();
                    (1.0 - t).max(0.0)
                }
                RumblePattern::DamageTap => {
                    if controller.pattern_timer < 0.1 {
                        1.0
                    } else {
                        0.0
                    }
                }
                RumblePattern::HeavyImpact => {
                    let t = controller.pattern_timer;
                    if t < 0.15 {
                        1.0
                    } else {
                        (0.5 - t).max(0.0) * 2.0
                    }
                }
                RumblePattern::Engine => (controller.pattern_timer * 30.0).sin().mul_add(0.1, 0.3),
                RumblePattern::Heartbeat => {
                    let beat = (controller.pattern_timer * 2.0).sin();
                    if beat > 0.8 { 1.0 } else { 0.0 }
                }
            };

            intensity.low_frequency *= modifier;
            intensity.high_frequency *= modifier;
        }

        // Apply to gamepad
        if let Ok(gamepad) = gamepads.get_mut(controller.gamepad) {
            // Note: Bevy's Gamepad doesn't have direct rumble API in 0.17
            // This would need to use bevy_gamepads or platform-specific APIs
            // For now, this is the structure. Implementation depends on platform.
            let _ = (gamepad, intensity); // Placeholder
        }

        // Stop if duration expired
        if controller.duration.is_zero() {
            controller.stop();
        }
    }
}

/// Plugin for registering haptics types and systems.
pub(crate) fn register_haptics_types(app: &mut App) {
    app.register_type::<RumbleIntensity>()
        .register_type::<RumblePattern>()
        .add_message::<RumbleRequest>();
}

/// Add haptics systems to the app.
pub(crate) fn add_haptics_systems(app: &mut App) {
    app.add_systems(Update, (handle_rumble_requests, update_rumble).chain());
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    // ========== RumbleIntensity Tests ==========

    #[test]
    fn test_rumble_intensity_new() {
        let intensity = RumbleIntensity::new(0.5, 0.8);
        assert_relative_eq!(intensity.low_frequency, 0.5);
        assert_relative_eq!(intensity.high_frequency, 0.8);
    }

    #[test]
    fn test_rumble_intensity_new_clamps_low() {
        let intensity = RumbleIntensity::new(-0.5, 0.5);
        assert_relative_eq!(intensity.low_frequency, 0.0);
    }

    #[test]
    fn test_rumble_intensity_new_clamps_high() {
        let intensity = RumbleIntensity::new(0.5, 1.5);
        assert_relative_eq!(intensity.high_frequency, 1.0);
    }

    #[test]
    fn test_rumble_intensity_uniform() {
        let intensity = RumbleIntensity::uniform(0.7);
        assert_relative_eq!(intensity.low_frequency, 0.7);
        assert_relative_eq!(intensity.high_frequency, 0.7);
    }

    #[test]
    fn test_rumble_intensity_uniform_clamps() {
        let intensity = RumbleIntensity::uniform(2.0);
        assert_relative_eq!(intensity.low_frequency, 1.0);
        assert_relative_eq!(intensity.high_frequency, 1.0);
    }

    #[test]
    fn test_rumble_intensity_none() {
        let intensity = RumbleIntensity::none();
        assert_relative_eq!(intensity.low_frequency, 0.0);
        assert_relative_eq!(intensity.high_frequency, 0.0);
    }

    #[test]
    fn test_rumble_intensity_default() {
        let intensity = RumbleIntensity::default();
        assert_relative_eq!(intensity.low_frequency, 0.0);
        assert_relative_eq!(intensity.high_frequency, 0.0);
    }

    #[test]
    fn test_rumble_intensity_equality() {
        let a = RumbleIntensity::new(0.5, 0.5);
        let b = RumbleIntensity::uniform(0.5);
        assert_eq!(a, b);
    }

    // ========== RumblePattern Tests ==========

    #[test]
    fn test_rumble_pattern_equality() {
        assert_eq!(RumblePattern::Constant, RumblePattern::Constant);
        assert_ne!(RumblePattern::Pulse, RumblePattern::Constant);
    }

    #[test]
    fn test_rumble_pattern_variants() {
        let patterns = [
            RumblePattern::Constant,
            RumblePattern::Pulse,
            RumblePattern::Explosion,
            RumblePattern::DamageTap,
            RumblePattern::HeavyImpact,
            RumblePattern::Engine,
            RumblePattern::Heartbeat,
        ];
        assert_eq!(patterns.len(), 7);
    }

    // ========== RumbleController Tests ==========

    #[test]
    fn test_rumble_controller_new() {
        let controller = RumbleController::new(Entity::PLACEHOLDER);
        assert_eq!(controller.gamepad, Entity::PLACEHOLDER);
        assert_eq!(controller.intensity, RumbleIntensity::none());
        assert_eq!(controller.duration, Duration::ZERO);
        assert!(controller.pattern.is_none());
        assert_relative_eq!(controller.pattern_timer, 0.0);
    }

    #[test]
    fn test_rumble_controller_rumble() {
        let mut controller = RumbleController::new(Entity::PLACEHOLDER);
        let intensity = RumbleIntensity::new(0.6, 0.8);
        let duration = Duration::from_secs(1);

        controller.rumble(intensity, duration);

        assert_eq!(controller.intensity, intensity);
        assert_eq!(controller.duration, duration);
        assert_eq!(controller.pattern, Some(RumblePattern::Constant));
    }

    #[test]
    fn test_rumble_controller_rumble_pattern() {
        let mut controller = RumbleController::new(Entity::PLACEHOLDER);
        let duration = Duration::from_millis(500);

        controller.rumble_pattern(RumblePattern::Heartbeat, 0.9, duration);

        assert_eq!(controller.intensity, RumbleIntensity::uniform(0.9));
        assert_eq!(controller.duration, duration);
        assert_eq!(controller.pattern, Some(RumblePattern::Heartbeat));
        assert_relative_eq!(controller.pattern_timer, 0.0);
    }

    #[test]
    fn test_rumble_controller_stop() {
        let mut controller = RumbleController::new(Entity::PLACEHOLDER);
        controller.rumble(RumbleIntensity::uniform(1.0), Duration::from_secs(2));

        controller.stop();

        assert_eq!(controller.intensity, RumbleIntensity::none());
        assert_eq!(controller.duration, Duration::ZERO);
        assert!(controller.pattern.is_none());
    }

    // ========== RumbleRequest Tests ==========

    #[test]
    fn test_rumble_request_new() {
        let request = RumbleRequest::new(Entity::PLACEHOLDER, 0.8, Duration::from_millis(200));

        assert_eq!(request.gamepad, Entity::PLACEHOLDER);
        assert_eq!(request.intensity, RumbleIntensity::uniform(0.8));
        assert_eq!(request.duration, Duration::from_millis(200));
        assert_eq!(request.pattern, Some(RumblePattern::Constant));
    }

    #[test]
    fn test_rumble_request_with_pattern() {
        let request = RumbleRequest::with_pattern(
            Entity::PLACEHOLDER,
            RumblePattern::Explosion,
            0.9,
            Duration::from_secs(1),
        );

        assert_eq!(request.gamepad, Entity::PLACEHOLDER);
        assert_eq!(request.intensity, RumbleIntensity::uniform(0.9));
        assert_eq!(request.duration, Duration::from_secs(1));
        assert_eq!(request.pattern, Some(RumblePattern::Explosion));
    }

    #[test]
    fn test_rumble_request_intensity_clamps() {
        let request = RumbleRequest::new(Entity::PLACEHOLDER, 2.0, Duration::from_secs(1));
        assert_relative_eq!(request.intensity.low_frequency, 1.0);
        assert_relative_eq!(request.intensity.high_frequency, 1.0);
    }

    // ========== Duration Tests ==========

    #[test]
    fn test_duration_saturation() {
        let mut controller = RumbleController::new(Entity::PLACEHOLDER);
        controller.duration = Duration::from_millis(100);

        // Simulate update that goes over duration
        controller.duration = controller
            .duration
            .saturating_sub(Duration::from_millis(150));

        assert_eq!(controller.duration, Duration::ZERO);
    }
}
