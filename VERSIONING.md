# Versioning and Branch Strategy

bevy\_archie follows [Semantic Versioning](https://semver.org/) and maintains compatibility branches for different Bevy versions.

## Branch Strategy

| Branch | Bevy Version | Status | Crate Version |
| --- | --- | --- | --- |
| `main` | 0.18.x | Active Development | 0.2.x |
| `bevy-0.18` | 0.18.x | Maintenance mirror | 0.2.x |
| `bevy-0.17` | 0.17.x | Deprecated in 30 days (maintenance only) | 0.1.x |

## Deprecation Notice

Bevy 0.17 support (crate `0.1.x`, branch `bevy-0.17`) is scheduled for deprecation in 30 days.

- Deprecation date: 2026-05-18
- During this window, only critical fixes should target `bevy-0.17`
- New development should target Bevy 0.18 (`main`, crate `0.2.x`)

### Branch Lifecycle

1. **main** - Always tracks the latest stable Bevy version we fully support
2. **bevy-X.Y** - Created when a new Bevy version releases, for migration work
3. Once migration is complete and stable, the version branch merges to `main`
4. The previous version branch enters maintenance mode (security/critical fixes only)

## Version Mapping

| bevy\_archie | Bevy | Rust | Notes |
| --- | --- | --- | --- |
| 0.2.x | 0.18.x | 1.95+ | Current stable |
| 0.1.x | 0.17.x | 1.95+ | Deprecated in 30 days |

## Upgrade Path

### From bevy\_archie 0.1.x (Bevy 0.17) to 0.2.x (Bevy 0.18)

See [Migration Guide](docs/dev/BEVY_0.17_TO_0.18_MIGRATION.md) for detailed instructions.

Key changes in 0.2.x:

- Updated to Bevy 0.18 APIs
- Active feature development and fixes
- 0.17 support moving to end-of-support after deprecation window

## Dependency Management

Dependabot is configured to:

- **Allow** patch updates for Bevy (0.17.1 → 0.17.2)
- **Ignore** minor/major Bevy updates (requires manual migration)
- **Auto-merge** patch updates for non-Bevy dependencies

This ensures stability while keeping dependencies secure.

## Release Process

### Patch Releases (0.1.1 → 0.1.2)

- Bug fixes and security updates
- No breaking changes
- Released from maintenance branches as needed

### Minor Releases (0.1.x → 0.2.0)

- New Bevy version support
- May contain breaking changes
- Released when migration to new Bevy version is complete

### Creating a Release

```bash
# Ensure version in Cargo.toml matches
git tag v0.1.3
git push origin v0.1.3
```

The release workflow automatically:

1. Verifies version matches tag
2. Publishes to crates.io
3. Creates GitHub release with changelog

## Supporting Multiple Bevy Versions

Users can select their Bevy version in `Cargo.toml`:

```toml
# For Bevy 0.17
[dependencies]
bevy_archie = "0.1"

# For Bevy 0.18 (when available)
[dependencies]
bevy_archie = "0.2"
```

## Contributing

When contributing, target the appropriate branch:

- New features and standard fixes → `main`
- Bevy 0.18 maintenance-only patches → `bevy-0.18`
- Bevy 0.17 critical fixes only (until deprecation date) → `bevy-0.17`
