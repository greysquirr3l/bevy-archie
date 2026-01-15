# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[0.1.0]: https://github.com/greysquirr3l/bevy-archie/releases/tag/v0.1.0
