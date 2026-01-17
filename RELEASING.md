# Release Process

This document describes how to release new versions of bevy-archie.

## Version Strategy

bevy-archie follows a version-based release strategy aligned with Bevy versions:

| bevy-archie | Bevy   | Branch      | Status      |
|-------------|--------|-------------|-------------|
| 0.1.x       | 0.17   | `main`      | Current     |
| 0.2.x       | 0.18   | `bevy-0.18` | Beta        |
| 0.3.x       | 0.19   | (future)    | Planned     |

## Branch Structure

- **`main`** - Latest stable release (currently Bevy 0.17)
- **`bevy-0.17`** - Maintenance branch, auto-synced from `main`
- **`bevy-0.18`** - Next major version (Bevy 0.18 migration)

## Release Types

### Patch Release (0.1.x → 0.1.y)

Bug fixes and minor improvements for the current stable version.

1. Ensure all changes are merged to `main`
2. Update `CHANGELOG.md` with release date
3. Bump version in `Cargo.toml`
4. Commit: `git commit -am "chore: release v0.1.x"`
5. Tag: `git tag v0.1.x`
6. Push: `git push origin main --tags`

The release workflow will automatically:
- Publish to crates.io
- Create a GitHub release with changelog notes

### Major Release (0.1.x → 0.2.0)

New Bevy version support.

1. Complete migration on the `bevy-0.X` branch
2. Update `CHANGELOG.md`:
   - Change `[0.2.0] - Unreleased` to `[0.2.0] - YYYY-MM-DD`
3. Ensure version in `Cargo.toml` is correct (remove `-beta.N` suffix)
4. Merge to `main`: `git checkout main && git merge bevy-0.18`
5. Tag: `git tag v0.2.0`
6. Push: `git push origin main --tags`
7. Create new maintenance branch: `git checkout -b bevy-0.18 && git push origin bevy-0.18`
8. Update `sync-branches.yml` to sync `main` → new maintenance branch

### Pre-release (alpha/beta/rc)

For testing before a major release.

1. Set version in `Cargo.toml` to `0.2.0-beta.1` (or `-alpha.1`, `-rc.1`)
2. Tag from the feature branch: `git tag v0.2.0-beta.1`
3. Push: `git push origin bevy-0.18 --tags`

Pre-releases are automatically marked as such on GitHub.

### Backport Release

Critical fixes for older Bevy versions.

1. Cherry-pick fix to maintenance branch: `git checkout bevy-0.17 && git cherry-pick <commit>`
2. Bump patch version in `Cargo.toml`
3. Update `CHANGELOG.md`
4. Tag from maintenance branch: `git tag v0.1.x`
5. Push: `git push origin bevy-0.17 --tags`

## Pre-release Checklist

Before any release:

- [ ] All CI checks pass
- [ ] `cargo test --all-features` passes locally
- [ ] `cargo clippy --all-features` has no warnings
- [ ] `cargo doc --all-features` builds without errors
- [ ] `CHANGELOG.md` is updated with release date
- [ ] Version in `Cargo.toml` matches intended release

## Secrets Required

The release workflow requires:

- `CARGO_REGISTRY_TOKEN` - API token for crates.io publishing
- `GITHUB_TOKEN` - Automatically provided for GitHub releases

## Troubleshooting

### Version mismatch error

The release workflow validates that `Cargo.toml` version matches the tag.
If you see this error, ensure:
```bash
# Tag version (without 'v' prefix)
git describe --tags  # e.g., v0.1.3 → version should be 0.1.3

# Cargo.toml version
grep '^version' Cargo.toml
```

### crates.io publish failed

- Check that `CARGO_REGISTRY_TOKEN` secret is set and valid
- Ensure you have publish rights on crates.io
- Verify the version doesn't already exist on crates.io

### GitHub release missing changelog

The workflow extracts notes from `CHANGELOG.md` matching the version.
Ensure your changelog follows the format:
```markdown
## [0.1.3] - 2026-01-16

### Added
- Feature description
```
