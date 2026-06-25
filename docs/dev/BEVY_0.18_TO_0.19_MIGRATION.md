# Migration Guide: bevy\_archie 0.2.x (Bevy 0.18) → 0.3.x (Bevy 0.19)

This release upgrades `bevy_archie` to **Bevy 0.19**. The library API is unchanged — no breaking changes were introduced on the bevy\_archie side. Only the Bevy dependency version bumped and the example code was updated for Bevy 0.19's new APIs.

For the upstream Bevy 0.18 → 0.19 migration notes, see the [official Bevy migration guide](https://bevy.org/learn/migration-guides/0-18-to-0-19/).

## Dependency bump

```toml
# before
bevy = { version = "0.18", ... }

# after
bevy = { version = "0.19", ... }
```

`bevy_archie` `0.3.x` is the new main line on Bevy 0.19. The previous `0.2.x` line (Bevy 0.18) continues to receive maintenance patches on the [`bevy-0.18`](https://github.com/greysquirr3l/bevy-archie/tree/bevy-0.18) branch.

## Example updates

The only consumer-facing changes in this repo were in the example code, where Bevy 0.19's text overhaul changed the `TextFont` struct:

### `TextFont.font_size` is now a `FontSize` enum

```rust
// before (Bevy 0.18)
TextFont {
    font_size: 24.0,
    ..default()
}

// after (Bevy 0.19)
TextFont {
    font_size: FontSize::Px(24.0),
    ..default()
}
```

The `FontSize` enum supports `Px`, `Vw`, `Vh`, `VMin`, `VMax`, and `Rem`. The `..default()` form continues to work and resolves to `FontSize::Px(20.0)` (Bevy's previous default).

### `TextFont.font` is now a `FontSource` enum

`Handle<Font>` converts via `From<Handle<Font>>` to `FontSource::Handle`, so existing `..default()` usage is unaffected. New options include `FontSource::Family("name")` for system font discovery and `FontSource::Monospace` / `FontSource::SansSerif` / etc. for semantic font categories.

## Notes for downstream consumers

- **Library code (`src/`) compiles without modification.** No `bevy_archie` types changed.
- **All 320 unit tests and 17 integration tests pass on Bevy 0.19** unchanged.
- **No new feature flags were added or removed.** Existing `icons`, `virtual_keyboard`, `remapping`, `motion-backends`, and `dualsense` features continue to work identically.
- **No `NonSend` resources are used internally.** All internal state moved seamlessly to Bevy 0.19's resources-as-components model.

## Branching model

| Bevy | bevy_archie | Branch |
|------|-------------|--------|
| 0.19 | 0.3.x | `main` |
| 0.18 | 0.2.x | `bevy-0.18` |
| 0.17 | 0.1.x | `bevy-0.17` (deprecated) |
