# Migration Guide: bevy_archie 0.3.x (Bevy 0.19) → 0.4.x (Bevy 0.20)

This release upgrades `bevy_archie` to **Bevy 0.20**. As with the 0.18 → 0.19 bump, no breaking changes were introduced on the `bevy_archie` side — the library API is unchanged. Only the Bevy dependency version bumped and the example code was updated for Bevy 0.20's new APIs.

> **Status:** Bevy 0.20.0 final shipped on crates.io on **2026-10-08**. This branch tracks stable Bevy **`0.20`** (resolves to `0.20.0`). The `bevy` version pin in `Cargo.toml` was relaxed from the `0.20.0-rc.1` pre-release pin it carried during the RC cycle.
>
> **MSRV note:** Bevy 0.20.0 final raised its MSRV to **Rust 1.97.1** (the `0.20.0-rc.1` pre-release still allowed 1.96). `bevy_archie`'s `rust-version` is therefore raised from `1.96` to `1.97.1`, and the MSRV CI job pin moves with it.

For the upstream Bevy 0.19 → 0.20 migration notes, see the [official Bevy migration guide](https://bevy.org/learn/migration-guides/0-19-to-0-20/).

## Dependency bump

```toml
# before
bevy = { version = "0.19", ... }

# after
bevy = { version = "0.20", ... }

# MSRV, raised by Bevy 0.20.0 final
rust-version = "1.97.1"  # was "1.96"
```

`bevy_archie` `0.4.x` is the new main line on Bevy 0.20. The previous `0.3.x` line (Bevy 0.19) continues to receive maintenance patches on the [`bevy-0.19`](https://github.com/greysquirr3l/bevy-archie/tree/bevy-0.19) branch, which was cut from `main` at v0.3.0 on 2026-09-21.

## Example updates

The only consumer-facing changes in this repo were in the example code, where two Bevy 0.20 API shifts applied.

### `FontSource::SansSerif` / `FontSource::Monospace` → constructor methods

Bevy 0.20 removed the bare `FontSource::SansSerif` / `FontSource::Monospace` enum variants and replaced them with `const fn` constructors:

```rust
// before (Bevy 0.19)
TextFont {
    font: FontSource::SansSerif,
    ..default()
}

// after (Bevy 0.20)
TextFont {
    font: FontSource::sans_serif(),
    ..default()
}
```

`sans_serif()` / `monospace()` resolve at runtime through the platform's system font discovery — the same semantic families as before, just behind methods. Fonts remain **system-discoverable**; nothing is vendored into the repo.

### UI button clicks: `Interaction` query → `On<Activate>` observer

Bevy 0.20 deprecated the `Button` marker component and the `Interaction` enum in favour of the new `bevy_ui_widgets::Button` widget, which uses observers for activation events instead of polled interaction state. Only the `remapping.rs` example is affected.

The old pattern:

```rust
// before (Bevy 0.19)
.parent.spawn((
    Node { ... },
    Button,                              // bevy_ui marker
    RemapActionButton(action),
))
// ...
fn handle_remap_ui(
    interaction_query: Query<(&Interaction, &RemapActionButton), Changed<Interaction>>,
    mut remap_events: MessageWriter<StartRemapEvent>,
) {
    for (interaction, remap_button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            remap_events.write(StartRemapEvent::new(remap_button.0));
        }
    }
}
```

The new pattern (Bevy 0.20):

```rust
// after (Bevy 0.20)
use bevy::ui_widgets::{Activate, Button};

.parent.spawn((
    Node { ... },
    Button,                              // bevy::ui_widgets::Button
    RemapActionButton(action),
))
.observe(
    |trigger: On<Activate>,
     actions: Query<&RemapActionButton>,
     mut remap_events: MessageWriter<StartRemapEvent>| {
        if let Ok(action) = actions.get(trigger.entity) {
            remap_events.write(StartRemapEvent::new(action.0));
        }
    },
);
```

The `handle_remap_ui` system no longer polls `Interaction` — it just refreshes the binding-text displays. Button activation flows through the per-entity observer.

### Controller config persistence → Bevy `SettingsPlugin`

> **Moved.** Controller config persistence moved to Bevy's `SettingsPlugin` in
> `bevy_archie` **0.5.0**, not in 0.4.0. The full recipe lives in the
> [0.4 → 0.5 migration guide](0.4_TO_0.5_MIGRATION.md#controller-config-persistence--bevy-settingsplugin).

## Notes for downstream consumers

- **Library code (`src/`) compiles without modification.** No `bevy_archie` types changed.
- **All 320 unit tests and 17 integration tests pass on Bevy 0.20 unchanged.**
- **No new feature flags were added or removed.** Existing `icons`, `virtual_keyboard`, `remapping`, `motion-backends`, and `dualsense` features continue to work identically.
- **MSRV raised to Rust 1.97.1.** Bevy 0.20.0 final requires 1.97.1 (the `0.20.0-rc.1` pre-release still allowed 1.96). Consumers on Rust 1.96 must upgrade.
- **Incidental lint fix.** `examples/steam_touchpad.rs` had two `(x + 1.0) * 0.5` → `f32::midpoint(x, 1.0)` rewrites for the new Rust 1.98 `clippy::manual_midpoint` lint. No behaviour change.

## Branching model

| Bevy | bevy_archie | Branch                       |
| ---- | ----------- | ---------------------------- |
| 0.20 | 0.5.x       | `main`                       |
| 0.20 | 0.4.x       | `main` (superseded by 0.5.x) |
| 0.19 | 0.3.x       | `bevy-0.19` (maintenance)    |
| 0.18 | 0.2.x       | `bevy-0.18` (maintenance)    |
| 0.17 | 0.1.x       | `bevy-0.17` (retired)        |
