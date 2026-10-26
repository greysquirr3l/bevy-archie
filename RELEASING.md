# Release Process

This document describes how to release new versions of bevy-archie.

## Version Strategy

bevy-archie follows a version-based release strategy aligned with Bevy versions:

| bevy-archie | Bevy | Branch      | Status      |
| ----------- | ---- | ----------- | ----------- |
| 0.5.x       | 0.20 | `main`      | Current     |
| 0.4.x       | 0.20 | `main`      | Maintenance |
| 0.3.x       | 0.19 | `bevy-0.19` | Maintenance |
| 0.2.x       | 0.18 | `bevy-0.18` | Maintenance |

## Branch Structure

- **`main`** - Latest stable release (currently Bevy 0.20, crate `0.5.x`)
- **`bevy-0.19`** - Maintenance branch for Bevy 0.19.x (crate `0.3.x`)
- **`bevy-0.18`** - Maintenance branch for Bevy 0.18.x (crate `0.2.x`)

Bevy 0.17 support (crate `0.1.x`) is end-of-life and the `bevy-0.17` branch has been retired.

## Release Types

### Patch Release (0.5.x → 0.5.y)

Bug fixes and minor improvements for the current stable version.

1. Ensure all changes are merged to `main`
2. Update `CHANGELOG.md` with release date
3. Bump version in `Cargo.toml`
4. Commit: `git commit -am "chore: release v0.5.x"`
5. Tag: `git tag v0.5.x`
6. Push: `git push origin main --tags`

The release workflow will automatically:

- Publish to crates.io
- Create a GitHub release with changelog notes

### Minor Release (0.4.x → 0.5.0)

New crate-level features, or a new Bevy version.

1. Complete migration on the `bevy-0.X` branch (for a Bevy bump) or land the feature PRs on
   `main` (for a crate-level release)
2. Update `CHANGELOG.md`:
   - Change `[0.5.0] - Unreleased` to `[0.5.0] - YYYY-MM-DD`
3. Ensure version in `Cargo.toml` is correct (remove any `-beta.N` suffix)
4. Update `VERSIONING.md`, `SECURITY.md`, and the README version table
5. Merge to `main`: `git checkout main && git merge bevy-0.20`
6. Tag: `git tag v0.5.0`
7. Push: `git push origin main --tags`
8. For a Bevy bump, create the new maintenance branch:
   `git checkout -b bevy-0.20 && git push origin bevy-0.20`

### Major Release (0.5.x → 0.6.0)

Breaking API changes.

1. Land the breaking changes on a feature branch
2. Update `CHANGELOG.md` with a `### Breaking Changes` section and a migration guide under
   `docs/dev/`
3. Ensure version in `Cargo.toml` is correct
4. Merge to `main`, tag, and push as above

### Pre-release (alpha/beta/rc)

For testing before a major release.

1. Set version in `Cargo.toml` to `0.6.0-beta.1` (or `-alpha.1`, `-rc.1`)
2. Tag from the feature branch: `git tag v0.6.0-beta.1`
3. Push: `git push origin main --tags`

Pre-releases are automatically marked as such on GitHub.

### Backport Release

Critical fixes for older Bevy versions.

1. Cherry-pick fix to maintenance branch: `git checkout bevy-0.19 && git cherry-pick <commit>`
2. Bump patch version in `Cargo.toml`
3. Update `CHANGELOG.md`
4. Tag from maintenance branch: `git tag v0.3.x`
5. Push: `git push origin bevy-0.19 --tags`

## Pre-release Checklist

Before any release:

- [ ] All CI checks pass
- [ ] `cargo test --all-features` passes locally
- [ ] `cargo clippy --all-features --all-targets -- -D warnings` is clean
- [ ] `cargo fmt --all -- --check` is clean
- [ ] `cargo doc --all-features --no-deps` builds without warnings
- [ ] `cargo deny check` passes
- [ ] `markdownlint-cli2 "**/*.md" "#target/**"` is clean
- [ ] `CHANGELOG.md` is updated with release date
- [ ] Version in `Cargo.toml` matches intended release
- [ ] `VERSIONING.md`, `SECURITY.md`, and the README version table are current

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
git describe --tags  # e.g., v0.5.0 → version should be 0.5.0

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
## [0.5.0] - 2026-10-26

### Added
- Feature description
```
