//! Motion backend trait definitions.

/// Data returned by a motion backend.
#[derive(Debug, Clone, Copy, Default)]
pub struct MotionData {
    /// Gyroscope pitch (angular velocity around X axis) in rad/s.
    pub gyro_pitch: f32,
    /// Gyroscope yaw (angular velocity around Y axis) in rad/s.
    pub gyro_yaw: f32,
    /// Gyroscope roll (angular velocity around Z axis) in rad/s.
    pub gyro_roll: f32,
    /// Accelerometer X in m/s².
    pub accel_x: f32,
    /// Accelerometer Y in m/s².
    pub accel_y: f32,
    /// Accelerometer Z in m/s².
    pub accel_z: f32,
}

/// Trait for motion control backends.
///
/// Implement this trait to provide gyroscope and accelerometer data
/// from a specific controller type or platform.
pub trait MotionBackend {
    /// Poll for new motion data.
    ///
    /// Returns `Some(MotionData)` if new data is available, `None` otherwise.
    fn poll(&mut self) -> Option<MotionData>;

    /// Check if the backend is connected to a controller.
    fn is_connected(&self) -> bool;

    /// Get the name of this backend.
    fn name(&self) -> &'static str;
}

/// Touchpad finger data.
#[derive(Debug, Clone, Copy, Default)]
pub struct TouchpadFinger {
    /// Whether this finger is touching.
    pub active: bool,
    /// X position (0.0 - 1.0).
    pub x: f32,
    /// Y position (0.0 - 1.0).
    pub y: f32,
    /// Finger ID for tracking.
    pub id: u8,
}

/// Data returned by a touchpad backend.
#[derive(Debug, Clone, Copy, Default)]
pub struct TouchpadData {
    /// First finger.
    pub finger1: TouchpadFinger,
    /// Second finger (if multi-touch supported).
    pub finger2: TouchpadFinger,
    /// Whether the touchpad button is pressed.
    pub button_pressed: bool,
}

/// Trait for touchpad backends.
///
/// Implement this trait to provide touchpad data from a specific controller.
pub trait TouchpadBackend {
    /// Poll for new touchpad data.
    fn poll(&mut self) -> Option<TouchpadData>;

    /// Check if the backend supports multi-touch.
    fn supports_multitouch(&self) -> bool;

    /// Get the name of this backend.
    fn name(&self) -> &'static str;
}
