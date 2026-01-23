# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.6] - 2026-01-23

### Added

- **Actionlike Trait**: New trait for defining custom action types, inspired by leafwing-input-manager
  - Implement `Actionlike` on your own enum types for type-safe action handling
  - Automatic reflection support via Bevy's `Reflect` trait
- **Virtual Input Composites** (`src/virtual_input.rs`): Combine multiple inputs into unified axes
  - `VirtualAxis` - Combine two buttons into a -1.0 to 1.0 axis
  - `VirtualDPad` - Combine four buttons into a 2D vector
  - `VirtualDPad3D` - Combine six buttons into a 3D vector
  - `VirtualButton` - Combine multiple buttons with OR/AND logic
- **Button Chords & Combos** (`src/chords.rs`): Advanced chord and combo detection
  - `ButtonChord` - Detect simultaneous button presses
  - `ClashStrategy` - Configure how overlapping chords are resolved
  - `ModifierKey` - Shift/Ctrl/Alt modifier support for keyboard
  - Configurable timing windows for chord detection
- **Input Mocking for Tests** (`src/testing.rs`): Comprehensive testing utilities
  - `MockInput` - Builder pattern for simulating input states
  - `MockInputPlugin` - Drop-in plugin for test environments
  - `MockInputSequence` - Script input sequences for automated testing
- **Touch Joystick** (`src/touch_joystick.rs`): Mobile-style virtual joysticks
  - Configurable dead zones and sensitivity
  - Fixed or floating anchor modes
  - Multi-touch support for dual-stick layouts
- **Conditional Input Bindings** (`src/conditions.rs`): Context-aware action triggering
  - `InputCondition` trait with `Always`, `InState`, `WhenResource`, `Custom` variants
  - Chain conditions with `and()`, `or()`, `negated()` methods
  - State-based binding activation (menu vs gameplay)
- **Network Input Synchronization** (`src/networking.rs`): Multiplayer input sync
  - `ActionDiff` - Efficient delta encoding for input state changes
  - `ActionDiffBuffer` - Rollback-friendly input history
  - Serialization helpers for netcode integration
- **Build Script Helpers** (`src/build_helpers.rs`): Asset pipeline utilities
  - `ControllerIconConfig` - Configure icon asset discovery
  - `generate_icon_manifest()` - Generate icon manifests at build time
  - Platform-specific asset organization
- **Input State Machine** (`src/state_machine.rs`): FSM for input-driven state transitions
  - `InputStateMachine` - Define states and transitions based on actions
  - `StateMachineBuilder` - Fluent API for state machine construction
  - Automatic transition detection and state change events

### Fixed

- Fixed 47+ clippy pedantic warnings across all new modules
- Added `#[must_use]` attributes to all builder methods and pure functions
- Implemented `std::ops::Not` trait for `InputCondition` (use `!condition` instead of `condition.not()`)
- Added proper `# Errors` and `# Panics` documentation sections

## [0.1.5] - 2026-01-23

### Added

- **Enhanced Controller Database**: Expanded VID/PID detection database with 30+ controller entries
  - PlayStation 3 DualShock 3 controllers (wired and Bluetooth)
  - Nintendo Switch 2 Pro Controller and GameCube-style controller
  - 8BitDo M30 and SN30 Pro controllers
  - HORI Fighting Commander and HORIPAD series
  - Additional Amazon Luna controller VIDs
- **Motion Sensor Calibration**: Hardware-accurate calibration constants for motion sensors
  - DualSense (PS5): ±2000°/s gyro, ±4g accelerometer at 16-bit resolution
  - DualShock 4 (PS4): ±2000°/s gyro, ±4g accelerometer at 16-bit resolution
  - DualShock 3 (PS3): ±500°/s gyro, ±2g accelerometer at 10-bit resolution
  - Nintendo Switch: ±2000°/s gyro, ±8g accelerometer at 16-bit resolution
- **Touchpad Resolution Constants**: Precise touchpad dimensions for coordinate normalization
  - DualShock 4: 1920×943 pixels
  - DualSense: 1920×1080 pixels
  - Steam Controller/Deck: 32767×32767 (16-bit range)
- **Hardware Constants Module** (`src/constants.rs`): Analog stick and trigger conventions
  - Standard 8-bit conventions (0-255 range, 128 center)
  - Normalization/denormalization helpers
  - Configurable deadzone application
- **Connection Type Detection**: `ConnectionType` enum and `connection_type_hint()` method
- **Controller Quirks System**: `ControllerQuirk` enum for handling device-specific behaviors
- **New Example**: `controller_database.rs` demonstrating enhanced detection capabilities
- **Documentation**: Technical analysis and integration guides in `docs/`

## [0.1.4] - 2026-01-22

### Added

- Amazon Luna controller detection and support
  - Automatically detected via controller name containing "luna" or "amazon"
  - Uses Xbox-style button layout (A/B/X/Y, LB/RB, LT/RT)
  - SDL GameControllerDB mappings available for all platforms

## [0.1.3] - 2026-01-16

### Added

- Hardware integration examples:
  - `ps5_dualsense_motion.rs` - DualSense gyro/touchpad via HID
  - `switch_pro_gyro.rs` - Switch Pro Controller gyro via SDL2
  - `steam_touchpad.rs` - Steam Deck/Controller touchpad via Steam Input API
- Motion backend abstraction system (`src/motion/`):
  - `MotionBackend` trait for platform-specific implementations
  - `DualSenseBackend` implementation using `dualsense-rs` crate
  - Stub backend for platforms without motion support
- New Cargo features:
  - `motion-backends` - Enable motion backend abstractions
  - `dualsense` - Enable DualSense HID support via `dualsense-rs`
- Documentation:
  - `docs/HARDWARE_INTEGRATION_GUIDE.md` - Comprehensive hardware integration guide
  - `examples/README.md` - Examples documentation with feature matrix
  - `CONTRIBUTING.md` - Contribution guidelines
  - `CONTRIBUTORS.md` - Maintainer attribution
  - `VERSIONING.md` - Branch strategy and versioning documentation
- CI/CD infrastructure:
  - GitHub Actions workflows (CI, security audit, release, CodeQL)
  - Dependabot configuration with auto-merge for patches
  - Multi-branch support for Bevy version maintenance (0.17, 0.18)

### Changed

- Dependabot now ignores Bevy minor/major updates (requires manual migration)
- CI workflows run on `main`, `bevy-0.17`, and `bevy-0.18` branches
- Repository metadata updated with correct URLs and documentation links

## [0.1.2] - 2026-01-16

### Fixed

- Fixed compilation errors in examples due to Bevy 0.17 API changes
- Updated `ChildBuilder` references to `ChildSpawnerCommands` in all examples
- Fixed `MessageWriter.send()` calls to use `.write()` method
- Fixed `ControllerConfig::load()` to use `load_from_file()` in config_persistence example
- Resolved Camera2d initialization in examples
- Added `StartRemapEvent` to prelude exports

### Added

- Dependency check configuration with `deny.toml` for CI
- Security audit configuration with `.cargo/audit.toml`
- License allowlist including MIT, Apache-2.0, BSD-*, ISC, Zlib, MIT-0, CC0-1.0, Unicode-3.0, MPL-2.0
- Advisory ignore for RUSTSEC-2024-0436 (unmaintained paste crate from Bevy dependencies)

## [0.1.1] - 2026-01-15

### Added

- Initial release of bevy-archie controller support library
- Core input device detection (mouse, keyboard, gamepad)
- Action mapping system with customizable bindings
- Controller icon system with automatic layout detection
- Support for Xbox, PlayStation, Nintendo, Steam, and Stadia controllers
- Controller remapping at runtime
- Virtual keyboard for controller text input
- Virtual cursor for gamepad-controlled mouse navigation
- Configuration persistence to JSON files
- Per-stick sensitivity and inversion settings
- Configurable deadzone support
- Haptic feedback system with rumble patterns (Constant, Pulse, Explosion, DamageTap, HeavyImpact, Engine, Heartbeat)
- Input buffering and combo detection system
- Action modifiers (Tap, Hold, DoubleTap, LongPress, Released)
- Multiplayer support with player-to-controller assignment
- Controller profiles with automatic detection
- Gyroscope and accelerometer support (placeholder - platform-specific implementation required)
- PlayStation touchpad support with multi-touch and gesture detection
- Input debugging tools with visualization
- Input recording and playback for automated testing
- Comprehensive documentation and examples

### Changed

- Migrated from Bevy 0.16 Event system to Bevy 0.17 Message system
- All event types now use `Message` trait
- `EventReader`/`EventWriter` replaced with `MessageReader`/`MessageWriter`

### Fixed

- 162 clippy pedantic warnings resolved
- Improved API design with `#[must_use]` attributes on 56+ methods
- Refactored `VirtualCursorState` to use enum instead of multiple booleans
- Optimized query iteration patterns
- Fixed documentation with proper backticks for product names

### Dependencies

- bevy = "0.17.3"
- serde = "1.0"
- serde_json = "1.0"
- dirs = "5.0"

## [0.1.0] - 2026-01-15

### Added

- Initial project structure
- Core plugin architecture
- Basic input detection
- Action system foundation

---

## Version History

### Version Numbering

This project follows [Semantic Versioning](https://semver.org/):

- **MAJOR** version for incompatible API changes
- **MINOR** version for backwards-compatible functionality additions
- **PATCH** version for backwards-compatible bug fixes

### Compatibility

- **Bevy 0.17.x**: Current version (0.1.x)
- Future versions will track Bevy releases

### Deprecated Features

None yet.

### Removed Features

None yet.

---

## Contributing

When contributing, please:

1. Add entries to the `[Unreleased]` section
2. Use the categories: Added, Changed, Deprecated, Removed, Fixed, Security
3. Keep entries concise and user-focused
4. Link to relevant issues/PRs where applicable
5. Update this changelog in the same PR as the changes

## Release Process

1. Update version in `Cargo.toml`
2. Move `[Unreleased]` changes to new version section
3. Update version comparison links
4. Create git tag
5. Publish to crates.io (when ready)

---

[0.1.4]: https://github.com/greysquirr3l/bevy-archie/releases/tag/v0.1.4
[0.1.3]: https://github.com/greysquirr3l/bevy-archie/releases/tag/v0.1.3
[0.1.2]: https://github.com/greysquirr3l/bevy-archie/releases/tag/v0.1.2
[0.1.1]: https://github.com/greysquirr3l/bevy-archie/releases/tag/v0.1.1
[0.1.0]: https://github.com/greysquirr3l/bevy-archie/releases/tag/v0.1.0
