# API Reference Guide

This page is a concise map of core public concepts. For full API details, use docs.rs:

- [docs.rs/bevy_archie](https://docs.rs/bevy_archie)

## System Sets

`ControllerSystemSet` execution order:

1. `Detection`
2. `Actions`
3. `UI`

## Core Resources and Types

- `InputDeviceState` - active device and gamepad context
- `ActionState` - action pressed/just_pressed/just_released/value queries
- `ActionMap` - binding map for gamepad/keyboard/mouse
- `ControllerConfig` - deadzone, sensitivity, inversion, layout settings

## Action Enum

`GameAction` contains built-in actions for:

- Navigation (`Confirm`, `Cancel`, `Pause`, `Select`)
- Movement and look
- Shoulders/triggers
- UI paging
- Custom slots

## Messages

bevy_archie uses Bevy message APIs (`MessageReader`, `MessageWriter`) for runtime signals such as:

- device changes
- controller connect/disconnect
- remap flow
- rumble requests
- combo/modifier/chord detections
- motion/touchpad gestures
- debug recording controls

## Related Docs

- [Advanced Features Guide](ADVANCED_FEATURES.md)
- [Hardware Integration Guide](HARDWARE_INTEGRATION_GUIDE.md)
- [Test Coverage Guide](TEST_COVERAGE.md)
