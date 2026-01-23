# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0-beta.2] - 2026-01-22

### Added

- Amazon Luna controller detection and support
  - Automatically detected via controller name containing "luna" or "amazon"
  - Uses Xbox-style button layout (A/B/X/Y, LB/RB, LT/RT)
  - SDL GameControllerDB mappings available for all platforms

## [0.2.0] - Unreleased

### Breaking Changes

- **Bevy 0.18 required** - This release targets Bevy 0.18 and is not compatible with earlier versions
- **Minimum Rust version bumped to 1.89** - Required by Bevy 0.18

### Changed

- Updated all dependencies to Bevy 0.18
- `BorderRadius` is now set via `Node.border_radius` field instead of as a separate component
- Integration tests now require `StatesPlugin` to be added before `ControllerPlugin`
- Icon system gracefully handles missing `AssetServer` (returns early instead of panicking)

### Fixed

- Fixed `Bundle` trait changes - tuples of 4+ components now use explicit Bundle structs
- Fixed virtual keyboard test assertions for symbol layout

### Migration Guide

See `docs/dev/BEVY_0.17_TO_0.18_MIGRATION.md` for detailed migration instructions.

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

[0.1.3]: https://github.com/greysquirr3l/bevy-archie/releases/tag/v0.1.3
[0.1.2]: https://github.com/greysquirr3l/bevy-archie/releases/tag/v0.1.2
[0.1.1]: https://github.com/greysquirr3l/bevy-archie/releases/tag/v0.1.1
[0.1.0]: https://github.com/greysquirr3l/bevy-archie/releases/tag/v0.1.0
