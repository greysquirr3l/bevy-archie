//! Stub backend providing no motion/touchpad data.
//!
//! This is the fallback backend when no platform-specific backend is available.
//! It always returns `None` from poll operations.

use super::backend::{MotionBackend, MotionData, TouchpadBackend, TouchpadData};

/// Stub backend that provides no data.
///
/// Use this as a fallback or for testing.
#[derive(Debug, Clone, Default)]
pub struct StubBackend {
    _private: (),
}

impl StubBackend {
    /// Create a new stub backend.
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }
}

impl MotionBackend for StubBackend {
    fn poll(&mut self) -> Option<MotionData> {
        None
    }

    fn is_connected(&self) -> bool {
        false
    }

    fn name(&self) -> &'static str {
        "stub"
    }
}

impl TouchpadBackend for StubBackend {
    fn poll(&mut self) -> Option<TouchpadData> {
        None
    }

    fn supports_multitouch(&self) -> bool {
        false
    }

    fn name(&self) -> &'static str {
        "stub"
    }
}
