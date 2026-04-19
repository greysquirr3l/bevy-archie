# Advanced Features

This guide collects advanced controller workflows that were previously embedded in the README.

## Feature Areas

- Haptic feedback patterns and `RumbleRequest`
- Input buffering and combo detection
- Multiplayer controller assignment and ownership
- Action modifiers (tap, hold, double-tap, long-press)
- Touchpad gestures (tap, swipe, pinch)
- Motion controls (gyro + accelerometer)
- Input debugging/recording/playback
- Virtual input composites (`VirtualAxis`, `VirtualDPad`, `VirtualButton`)
- Button chords and clash strategy
- Conditional bindings by game state
- Input-driven state machine transitions
- Input mocking for tests
- Mobile touch joystick
- Network input synchronization (`ActionDiff` / rollback-friendly diffing)

## Recommended Example Entry Points

- [examples/basic_input.rs](../examples/basic_input.rs)
- [examples/controller_icons.rs](../examples/controller_icons.rs)
- [examples/remapping.rs](../examples/remapping.rs)
- [examples/virtual_cursor.rs](../examples/virtual_cursor.rs)
- [examples/config_persistence.rs](../examples/config_persistence.rs)
- [examples/ps5_dualsense_motion.rs](../examples/ps5_dualsense_motion.rs)
- [examples/switch_pro_gyro.rs](../examples/switch_pro_gyro.rs)
- [examples/steam_touchpad.rs](../examples/steam_touchpad.rs)

## Hardware Notes

Gyroscope, touchpad, and adaptive trigger capabilities require controller-specific data pipelines. For implementation guidance, see [Hardware Integration Guide](HARDWARE_INTEGRATION_GUIDE.md).

## Migration Note

If you are migrating old Bevy 0.16-era event code, remember that Bevy 0.17+ uses messages (`MessageReader` / `MessageWriter`) rather than events.
