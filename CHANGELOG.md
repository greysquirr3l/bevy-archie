# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.0] - 2026-10-26

### Breaking Changes

- **`VirtualKeyboard::cursor` is now a grapheme-cluster index** instead of a byte offset into
  `VirtualKeyboard::buffer`. This fixes a panic on non-ASCII input, but code that reads or writes
  `cursor` directly must be updated — see the
  [migration recipe](docs/dev/0.4_TO_0.5_MIGRATION.md#virtual-keyboard-the-cursor-is-now-a-grapheme-index).
  Code that only uses `add_char`, `backspace`, `cursor_left`, `cursor_right`, and `clear` is
  unaffected
- **`VirtualKeyboard::max_length` is now counted in grapheme clusters** rather than bytes, to match
  `EditableText::max_characters`. A buffer of multi-byte characters can now reach the configured
  length sooner than it did before

### Added

- **`ControllerConfig` is now a Bevy `SettingsGroup`**: `ControllerConfig` derives
  `bevy::settings::SettingsGroup` and is registered with `ReflectSettingsGroup`, so it can be
  persisted to disk with Bevy's `SettingsPlugin` (TOML at `{preferences_dir}/{app_id}/settings.toml`,
  with atomic writes and a browser local-storage fallback). `ControllerPlugin::with_settings(app_id)`
  adds the `SettingsPlugin` for you, and `ControllerPlugin::with_settings_app_name(app_id)` sets the
  app id on an existing plugin value
- **`ControllerPlugin::settings_app_name`**: new public field on the plugin, `Option<String>`
- **`VirtualKeyboard::grapheme_count`**: new public method returning the length of the input buffer
  in grapheme clusters

### Changed

- **Controller config persistence**: the crate's hand-rolled JSON persistence is deprecated in favour
  of Bevy's `SettingsPlugin`. `ControllerConfig::load_or_default`, `load_from_file`, `save_default`,
  `save_to_file`, and `default_config_path` are now `#[deprecated]` and will be removed in 0.6.0 —
  see the
  [migration recipe](docs/dev/0.4_TO_0.5_MIGRATION.md#controller-config-persistence--bevy-settingsplugin).
  The JSON file written by the old helpers is not migrated automatically
- **`ControllerConfig::current_layout` is not persisted**: it is runtime state rather than a user
  preference, so it is excluded from the settings file (`#[reflect(ignore)]`). `forced_layout` *is*
  persisted
- **Examples now use Bevy's `bsn!` scene notation**: `basic_input.rs`, `config_persistence.rs`,
  `controller_database.rs`, `controller_icons.rs`, `remapping.rs`, and `virtual_cursor.rs` build
  their UI with `commands.spawn_scene(bsn! { ... })` instead of `commands.spawn((...))` and
  `.with_children(...)`. Visual output is unchanged. `remapping.rs` attaches its button observer with
  `on(...)` instead of `.observe(...)`, and `controller_icons.rs` / `remapping.rs` express their
  repeated sub-trees as scene functions
- **`ButtonIcon` and `GameAction` implement `Default`**: required for the types to be used in `bsn!`
  scenes, where components must be `Clone + Default`. `ButtonIcon::default()` is `FaceDown` and
  `GameAction::default()` is `Confirm`
- **`VirtualKeyboard::cursor` is now a grapheme-cluster index** instead of a byte offset, and
  `VirtualKeyboard::max_length` is counted in grapheme clusters to match
  `EditableText::max_characters`. The buffer is now edited with `unicode-segmentation`, which matches
  the grapheme model Bevy's `EditableText` uses
- **README**: Refreshed for Bevy 0.20 / bevy_archie 0.4.x. The Rust badge now reads 1.97.1+, the Bevy
  badge reads 0.20, and the branch-policy note and version-compatibility table list `main` as Bevy
  0.20.x with `bevy-0.19` and `bevy-0.18` as the maintenance branches
- **README**: Corrected code samples that no longer compiled against the current API —
  `ActionMap::bind` is now `bind_gamepad` (with `GamepadButton`, not `GamepadButtonType`),
  `ControllerConfig::force_layout` is `forced_layout`, `ControllerConfig::load_or_default` returns
  `Self` rather than a `Result`, `ControllerIconAssets::get_icon` takes
  `(ButtonIcon, ControllerLayout, IconSize, &AssetServer)`, and Bevy 0.20's `MessageReader` replaces
  `EventReader`
- **README**: Removed the `Actionlike` trait bullet — no such trait exists in the crate — and added a
  Cargo feature table, the `ControllerIconDisplay` auto-update pattern, the undocumented
  `controller_database.rs` example, and links to the Joypad OS documentation
- **README**: Fixed the controller support matrix to match `ControllerModel::default_layout` and the
  `supports_gyro` / `supports_touchpad` / `supports_adaptive_triggers` data (added Switch 2 GameCube;
  corrected the 8BitDo M30 layout), and removed broken links to `docs/TEST_COVERAGE.md` and
  `LICENSE-APACHE`

### Fixed

- **`VirtualKeyboard` panicked on non-ASCII input**: the input buffer was byte-indexed, so typing a
  multi-byte character (an accented letter, an emoji) and then another character tripped
  `String`'s char-boundary assertion and crashed the app. Backspace also deleted a single byte
  rather than a character. The buffer and cursor are now grapheme-cluster aware, covered by six new
  regression tests including ZWJ emoji sequences

### Internal

- **`unicode-segmentation` 1.13**: added as a dependency for grapheme-cluster aware text editing in
  the virtual keyboard
- **Virtual keyboard `EditableText` investigation**: the decision to keep `VirtualKeyboard` as the
  editing source of truth rather than migrating onto Bevy's `EditableText` is recorded in
  [`docs/dev/VIRTUAL_KEYBOARD_EDITABLE_TEXT.md`](docs/dev/VIRTUAL_KEYBOARD_EDITABLE_TEXT.md)

## [0.4.1] - 2026-10-25

### Changed

- **`dirs` 7.0**: Bump `dirs` 6.0 → 7.0. This is a semver-major bump on a direct dependency, but it is
  behaviourally inert for this crate. The only breaking change in `dirs` 7 is one line in
  `src/win.rs`, where `preference_dir()` now resolves to `known_folder_roaming_app_data()`
  instead of `known_folder_local_app_data()`. This crate calls only `dirs::config_dir()`, whose
  implementation and signature (`-> Option<PathBuf>`) are byte-identical between the two
  versions, and the dependency list is unchanged. The upstream project also moved from GitHub to
  Codeberg, which is the substance of the major bump

### Internal

- **CI action updates**: `actions/cache` v5.0.5 → v6.1.0, `actions/checkout` v7.0.0 → v7.0.1,
  `github/codeql-action` v4.36.2 → v4.38.1, `gitleaks/gitleaks-action` v2.0.8 → v3.0.0,
  `ossf/scorecard-action` v2.4.3 → v2.4.4, `softprops/action-gh-release` v3.0.1 → v3.0.3. No
  change to the library or to the public API

## [0.4.0] - 2026-10-25

### Changed

- **Bevy 0.20**: Upgrade `bevy` dependency 0.19 → `0.20` (stable, `0.20.0`, released 2026-10-08). The library API is unchanged — only examples were updated for Bevy 0.20's new APIs
- **MSRV**: Raised to Rust 1.97.1 from 1.96, to match Bevy 0.20.0 final (the `0.20.0-rc.1` pre-release still allowed 1.96). The MSRV CI job pin moves with it
- **Example typography**: `FontSource::SansSerif` / `FontSource::Monospace` enum variants are gone in Bevy 0.20 — examples now use the `FontSource::sans_serif()` / `FontSource::monospace()` constructors (system-discoverable, no fonts vendored)
- **Example UI**: `remapping.rs` migrated from the deprecated `Button` + `Interaction` query pattern to `bevy::ui_widgets::Button` + a per-entity `On<Activate>` observer
- **Incidental lint fix**: `examples/steam_touchpad.rs` — four `(x + 1.0) * 0.5` patterns replaced with `f32::midpoint(x, 1.0)` for the Rust 1.98 `clippy::manual_midpoint` lint. `f32::midpoint` is also the form that cannot overflow

### Fixed

- **`cargo deny` CI**: `cargo deny check --config deny.toml <check>` passed `--config` as a
  subcommand argument, which cargo-deny rejects (`unexpected argument '--config' found`) — the
  `Cargo Deny` job had been failing on every push. `deny.toml` is auto-discovered from the
  project root, so the flag was redundant; the invocations are now `cargo deny check <check>`
- **`cargo audit` CI**: the `Security Audit` job keeps its own `--ignore` list in
  `security.yml` rather than reading `deny.toml`, and had drifted — it still ignored
  `RUSTSEC-2024-0436` (`paste`, no longer in the dependency tree) and lacked the two
  advisories that do apply. Both tools now ignore the same two advisories, which are
  documented in `deny.toml` with the dependency chain that causes them and the condition
  that clears each one
- **CI clippy coverage**: the clippy step ran without `--all-targets`, so it checked only the
  library and silently skipped every `#[cfg(test)]` module and the `tests/` integration crate.
  Lints there — including stale `#[expect(clippy::float_cmp)]` attributes that no longer match
  anything — passed CI and then surfaced in the editor. Both the gating clippy step and the
  SARIF step now pass `--all-targets`

### Branching

- `main` is now the Bevy 0.20.x line (the migration landed via PR #51). `bevy-0.19` (Bevy
  0.19.x maintenance, cut from `main` at v0.3.0) and `bevy-0.18` (Bevy 0.18.x maintenance) are
  the maintenance branches. `bevy-0.20` remains as a merged, historical branch
- A `release/<version>` branch is used to stage each release: release-prep commits land
  there, and the branch is squash-merged into `main` with a `chore: bump version to <version>`
  subject, which is what the auto-tag workflow gates on

## [0.3.0] - 2026-06-25

### Changed

- **Bevy 0.19**: Upgrade `bevy` dependency 0.18 → 0.19 with all required API updates; `TextFont { font_size: N }` literals updated to the new `FontSize::Px(N)` form
- **`dirs` 6.0**: Bump `dirs` 5.0 → 6.0
- **Example typography**: Replace default font handle with semantic `FontSource` variants (`SansSerif`, `Monospace`, `BOLD`/`SEMIBOLD` weights) and `LetterSpacing::Px` on titles
- **MSRV**: Raised to Rust 1.96 to match Bevy 0.19 requirements
- **Branch policy**: `main` now tracks Bevy 0.19.x; `bevy-0.18` is the maintenance branch; `bevy-0.17` retired

### CI

- Pin all GitHub Actions to immutable commit SHAs (supply-chain hardening)
- Add `gitleaks` secret-scanning workflow (all branches, full history)
- Add OSSF Scorecard supply-chain scoring workflow
- Add `verify-ci` gate to `release.yml` — polls CI status before publishing
- Overhaul `codeql.yml`: `build-mode: none`, `security-extended` queries, `timeout-minutes: 30`
- Add cargo cache to cross-platform CI job
- Simplify `dependabot-auto-merge.yml` with `concurrency` group and `permissions: read-all`

## [0.2.4] - 2026-04-20

### Fixed

- **Windows asset paths**: Normalize backslash separators in controller icon paths on Windows
- **DualSense clippy**: Fix `map_unwrap_or` lint in DualSense motion state check

### Changed

- **Rust toolchain**: Switch to `stable` channel (from pinned `1.95.0`) for local dev and CI
- **CI toolchain alignment**: Ensure `rustfmt` and `clippy` components install on the active toolchain

### Removed

- Stray backup file (`src/motion/dualsense.rs.bak2`) excluded from crate distribution

## [0.2.3] - 2026-04-03

> **Note:** `v0.2.3` was tagged on `main` but never published to crates.io. Changes are included in `0.2.4`.

## [0.2.2] - 2026-03-12

### Added

- **Clippy SARIF scanning**: Clippy results now upload to GitHub Code Scanning via SARIF format
- **CodeQL on both branches**: Extended CodeQL analysis to scan `main` and `bevy-0.17`

### Removed

- **`sync-branches` workflow**: No longer needed after branch restructure — `main` (Bevy 0.18) and `bevy-0.17` are independent codebases

## [0.2.1] - 2026-03-12

### Changed

- **Branch restructure**: `main` now targets Bevy 0.18; Bevy 0.17 support continues on the [`bevy-0.17`](https://github.com/greysquirr3l/bevy-archie/tree/bevy-0.17) branch

### Fixed

- Fixed CI MSRV toolchain from 1.93 to 1.94 to match `rust-version` in Cargo.toml
- Updated README to reflect branch restructure and version compatibility table

## [0.2.0] - 2026-03-07

### Changed

- **MSRV Upgraded to Rust 1.94.0** (from 1.93)
  - Leverages `LazyLock::force_mut` for safer lazy state initialization in controller detection
  - Uses stabilized `Peekable::next_if_map` for more ergonomic input buffer peeking in state machines
  - Stable `impl TryFrom<char> for usize` for safer key mapping conversions
  - TOML v1.1 support enables future multiline string configurations in Cargo.toml
  - Enhanced closure lifetime handling improves correctness for input callback internals
  - Unicode 17.0 support provides updated character categorization for text input
  - Stable Cargo config include key allows better organization of build-time icon configurations

## [0.2.0-beta.4] - 2026-01-23

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

## [0.2.0-beta.3] - 2026-01-23

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

## [0.2.0-beta.2] - 2026-01-22

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
- License allowlist including MIT, Apache-2.0, BSD-\*, ISC, Zlib, MIT-0, CC0-1.0, Unicode-3.0, MPL-2.0
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

- **Bevy 0.20.x**: Current version (0.5.x)
- Older Bevy versions are maintained on the `bevy-0.19` and `bevy-0.18` branches

### Deprecated Features

- `ControllerConfig::load_or_default`, `load_from_file`, `save_default`, `save_to_file`, and
  `default_config_path` — use Bevy's `SettingsPlugin` instead. Removed in `0.6.0`.

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

1. Create a `release/<version>` branch from `main`
2. Update version in `Cargo.toml`
3. Move `[Unreleased]` changes to new version section
4. Update version comparison links
5. Squash-merge the release branch into `main` with the subject
   `chore: bump version to <version>` — the auto-tag workflow gates on `main`'s HEAD
   subject, so a plain merge commit will not trigger the tag
6. The auto-tag workflow pushes `v<version>`, and the Release workflow publishes to
   crates.io and creates the GitHub Release
7. Publish to crates.io (when ready)

---

[0.5.0]: https://github.com/greysquirr3l/bevy-archie/compare/v0.4.1..v0.5.0
[0.4.1]: https://github.com/greysquirr3l/bevy-archie/compare/v0.4.0..v0.4.1
[0.4.0]: https://github.com/greysquirr3l/bevy-archie/compare/v0.3.0..v0.4.0
[0.3.0]: https://github.com/greysquirr3l/bevy-archie/compare/v0.2.4..v0.3.0
[0.2.4]: https://github.com/greysquirr3l/bevy-archie/compare/v0.2.3..v0.2.4
[0.2.3]: https://github.com/greysquirr3l/bevy-archie/compare/v0.2.2..v0.2.3
[0.2.2]: https://github.com/greysquirr3l/bevy-archie/compare/v0.2.1..v0.2.2
[0.2.1]: https://github.com/greysquirr3l/bevy-archie/compare/v0.2.0..v0.2.1
[0.2.0]: https://github.com/greysquirr3l/bevy-archie/compare/v0.2.0-beta.4..v0.2.0
[0.1.3]: https://github.com/greysquirr3l/bevy-archie/releases/tag/v0.1.3
[0.1.2]: https://github.com/greysquirr3l/bevy-archie/releases/tag/v0.1.2
[0.1.1]: https://github.com/greysquirr3l/bevy-archie/releases/tag/v0.1.1
[0.1.0]: https://github.com/greysquirr3l/bevy-archie/releases/tag/v0.1.0
