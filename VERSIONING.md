# Versioning and Branch Strategy

bevy\_archie follows [Semantic Versioning](https://semver.org/) and maintains compatibility branches for different Bevy versions.

## Branch Strategy

| Branch | Bevy Version | Status | Crate Version |
| --- | --- | --- | --- |
| `main` | 0.17.x | Active Development | 0.1.x |
| `bevy-0.17` | 0.17.x | Maintenance (patches only) | 0.1.x |
| `bevy-0.18` | 0.18.x | Migration in Progress | 0.2.x |

### Branch Lifecycle

1. **main** - Always tracks the latest stable Bevy version we fully support
2. **bevy-X.Y** - Created when a new Bevy version releases, for migration work
3. Once migration is complete and stable, the version branch merges to `main`
4. The previous version branch enters maintenance mode (security/critical fixes only)

## Version Mapping

| bevy\_archie | Bevy | Rust | Notes |
| --- | --- | --- | --- |
| 0.1.x | 0.17.x | 1.85+ | Current stable |
| 0.2.x | 0.18.x | 1.85+ | In development |

## Upgrade Path

### From bevy\_archie 0.1.x (Bevy 0.17) to 0.2.x (Bevy 0.18)

See [Migration Guide](docs/dev/BEVY_0.17_TO_0.18_MIGRATION.md) for detailed instructions.

Key changes in 0.2.x:

- Updated to Bevy 0.18 APIs
- Breaking changes in input handling (TBD)
- New features (TBD)

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

- Bug fixes for current stable → `main`
- Bug fixes for older Bevy → `bevy-X.Y` branch
- New features → `main` (will be forward-ported)
- Bevy migration work → `bevy-X.Y` branch
