# Joypad OS Attribution Guide

## License Compatibility

**Good news**: Both projects use Apache 2.0 licensing!

- **Joypad OS**: Apache 2.0
- **Bevy Archie**: Dual-licensed MIT OR Apache 2.0

✅ **Fully compatible** - you can reference Apache 2.0 code in an Apache 2.0 project.

---

## What Requires Attribution?

Apache 2.0 requirements apply **only if you directly copy or translate code**. Here's the breakdown:

### ✅ **No Attribution Required** (Factual Data / Ideas)

These are **not copyrightable**, so no legal obligation:

1. **VID/PID Values** (factual measurements)
   - `Sony DS5: VID 0x054C, PID 0x0CE6`
   - These are industry-standard hardware identifiers

2. **Calibration Constants** (factual specifications)
   - `DUALSENSE_ACCEL_RANGE: 16384.0` (hardware specification)
   - `DS4_TOUCHPAD_WIDTH: 1920` (hardware specification)

3. **Data Structure Concepts** (industry standards)
   - Using 3-element arrays for accelerometer data `[x, y, z]`
   - Touchpad finger tracking patterns (standard HID format)

4. **API Design Patterns** (ideas/concepts)
   - Organizing controllers by VID/PID in a HashMap
   - Separating motion data into gyro/accel components

### ⚠️ **Attribution Recommended** (Best Practice)

Even when not legally required, it's good practice to acknowledge:

1. **Inspired Implementations**
   - "Controller database structure inspired by Joypad OS"
   - Add to comments or documentation

2. **Reference Data Collections**
   - If you use their comprehensive VID/PID list as a reference

### ⛔ **Attribution REQUIRED** (Derivative Works)

Apache 2.0 compliance needed if you:

1. **Translate C code to Rust**
   - Line-by-line translation of functions
   - Example: Translating `hid_parser.c` logic to Rust

2. **Copy Substantial Algorithms**
   - HID descriptor parsing logic
   - Complex controller detection algorithms

3. **Adapt Code Structures**
   - Porting their state machine implementations
   - Reusing substantial code logic

---

## How to Provide Attribution

### Scenario 1: **Reference-Based Implementation** (Recommended Approach)

**What you're doing:**

- Using VID/PID values from public databases
- Implementing similar data structures based on HID standards
- Following similar API patterns

**Attribution approach:**

#### Option A: Add to README.md (Minimal)

```markdown
## Acknowledgments

Controller hardware specifications and detection patterns informed by research
into [Joypad OS](https://github.com/joypad-ai/joypad-os) and industry-standard
HID descriptors.
```

#### Option B: Add to Source Files (Recommended)

Add comments where you use specific values:

```rust
// src/detection_database.rs

/// Controller detection database
/// 
/// VID/PID values sourced from:
/// - USB-IF Vendor ID Database
/// - Joypad OS controller registry (https://github.com/joypad-ai/joypad-os)
/// - Community controller databases
pub struct ControllerDatabase {
    // ...
}

// Later in the code:
db.register(0x054C, 0x0CE6, ControllerEntry {
    name: "Sony DualSense",
    // VID/PID from USB-IF database and verified against Joypad OS
    // ...
});
```

```rust
// src/motion/dualsense.rs

/// DualSense motion sensor calibration
/// 
/// Calibration values based on DualSense hardware specifications
/// and verified against Joypad OS implementation.
/// Reference: https://github.com/joypad-ai/joypad-os/blob/main/src/core/input_event.h
pub const DUALSENSE_ACCEL_RANGE: f32 = 16384.0;  // ±8g at 16-bit resolution
pub const DUALSENSE_GYRO_RANGE: f32 = 1024.0;    // ±2000 dps at 16-bit
```

---

### Scenario 2: **Code Translation/Adaptation** (If You Port C Code)

**Apache 2.0 Requirements:**

1. **Include Apache 2.0 License**
   - Already satisfied (bevy-archie is Apache 2.0 compatible)

2. **Preserve Copyright Notices**

Add to affected source files:

```rust
// src/hid_parser.rs (example if you translated hid_parser.c)

//! HID Report Descriptor Parser
//! 
//! Portions of this implementation adapted from Joypad OS
//! Copyright (c) 2024 Joypad AI
//! Licensed under the Apache License, Version 2.0
//! 
//! Original source: https://github.com/joypad-ai/joypad-os/blob/main/src/usb/usbh/hid/devices/generic/hid_parser.c
//! 
//! Modifications:
//! - Translated from C to Rust
//! - Adapted for Bevy ECS architecture
//! - Added additional HID usage page support

// ... your Rust code ...
```

1. **Create NOTICE File** (if significant code adapted)

Create `NOTICE` at repository root:

```
Bevy Archie
Copyright (c) 2024 [Your Name/Organization]

This product includes software developed by Joypad AI (https://joypad.ai/).

Portions of the HID parsing and controller detection systems are adapted from
Joypad OS (https://github.com/joypad-ai/joypad-os), which is licensed under
the Apache License, Version 2.0.

The original Joypad OS copyright notice:
    Copyright (c) 2024 Joypad AI
    Licensed under the Apache License, Version 2.0
```

1. **Document in CHANGELOG**

```markdown
## [0.2.0] - 2025-XX-XX

### Added
- Enhanced controller detection using VID/PID database inspired by Joypad OS
- HID descriptor parsing adapted from Joypad OS hid_parser implementation
  (See NOTICE file for attribution)
```

---

## Recommended Approach for Bevy Archie

Based on your integration analysis, here's what I recommend:

### ✅ **Use Reference-Based Approach** (No Apache 2.0 Compliance Needed)

**Implement:**

1. VID/PID database with values researched from multiple sources
2. Calibration constants based on hardware specifications
3. Your own Rust implementations inspired by patterns

**Provide:**

- Acknowledgment in README (minimal)
- Source code comments citing references (best practice)
- Keep the `JOYPAD_OS_INTEGRATION_ANALYSIS.md` doc

**Example additions:**

```rust
// src/detection.rs

/// Detect controller model from gamepad
/// 
/// Uses VID/PID matching against known controller database.
/// Database compiled from USB-IF registry, Joypad OS, and community sources.
pub fn detect_controller_model(gamepad: &Gamepad) -> ControllerModel {
    // Implementation inspired by Joypad OS's hid_registry.c approach
    // but implemented independently for Bevy/gilrs architecture
    // ...
}
```

### ⛔ **Avoid Direct Code Translation** (Unless Necessary)

**Why:**

- Different languages (C vs Rust)
- Different architectures (firmware vs application)
- Different abstractions (TinyUSB vs gilrs)

**Instead:**

- Understand the algorithms
- Implement in idiomatic Rust
- Cite as inspiration rather than derivative work

---

## Summary: What You Need To Do

### Current Plan (Reference-Based)

✅ **No Apache 2.0 compliance required**

**Recommended actions:**

1. Add acknowledgment section to README:

   ```markdown
   ## Acknowledgments
   
   Controller specifications and detection patterns informed by research into
   [Joypad OS](https://github.com/joypad-ai/joypad-os) and USB HID standards.
   ```

2. Add source comments where referencing specific values:

   ```rust
   // Calibration values verified against Joypad OS and hardware specs
   pub const DUALSENSE_ACCEL_RANGE: f32 = 16384.0;
   ```

3. Keep the integration analysis doc as reference material

### If You Later Translate Code

⚠️ **Apache 2.0 compliance required**

**Required actions:**

1. ✅ Already have Apache 2.0 license compatibility
2. Add copyright notices to translated files
3. Create NOTICE file with attribution
4. Document adaptations in comments
5. Update CHANGELOG with attribution

---

## Legal Disclaimer

This guide provides general information about Apache 2.0 licensing. For specific
legal advice, consult with a qualified attorney. When in doubt, be more generous
with attribution rather than less.

## Resources

- [Apache License 2.0 Full Text](https://www.apache.org/licenses/LICENSE-2.0)
- [Apache License FAQ](https://www.apache.org/foundation/license-faq.html)
- [Joypad OS Repository](https://github.com/joypad-ai/joypad-os)
- [USB-IF Vendor ID Database](https://www.usb.org/developers)
