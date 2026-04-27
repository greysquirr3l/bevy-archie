# Branch Workflow & Synchronization

> **Note:** This document is untracked and for local development reference only.

## Branch Strategy

Branch policy (effective 2026-04-26): `main` is Bevy 0.18.x. The only other branch line is `bevy-0.17` for Bevy 0.17.x, and that branch is scheduled for retirement on 2026-05-18 (less than 30 days).

| Branch | Purpose | Bevy Version | Status |
| -------- | --------- | -------------- | -------- |
| `main` | Latest stable release | 0.18.x | Active |
| `bevy-0.17` | Final maintenance for Bevy 0.17 | 0.17.x | Retirement scheduled for 2026-05-18 |

## Backport Strategy

There is no automatic `sync-branches.yml` workflow in this repository. Backports from `main` to `bevy-0.17` are manual cherry-picks when needed.

### How It Works

1. Push a commit to `main`
2. Decide whether the change should also ship on `bevy-0.17`
3. Cherry-pick the commit onto `bevy-0.17`
4. Resolve conflicts locally if they occur

### Skipping Sync

Add `[skip-sync]` to your commit message to prevent auto-sync:

```bash
git commit -m "feat: main-only feature [skip-sync]"
```

`[skip-sync]` is a human reminder only. It documents that a change is intentionally `main`-only; no workflow consumes it.

## Manual Cherry-Pick Process

If you need to manually sync (e.g., conflict resolution):

```bash
# From main, after committing
git push origin main

# Switch to target branch
git checkout bevy-0.17

# Cherry-pick the commit
git cherry-pick <commit-sha>

# If conflicts:
# 1. Resolve conflicts in your editor
# 2. git add .
# 3. git cherry-pick --continue

# Push
git push origin bevy-0.17

# Return to main
git checkout main
```

## CI/CD Workflows

| Workflow | Trigger | Purpose |
| ---------- | --------- | --------- |
| `ci.yml` | Push/PR to main, bevy-0.17 | Check, Test, Clippy, Format, Docs, MSRV |
| `security.yml` | Push/PR, weekly schedule | cargo-audit, cargo-deny |
| `codeql.yml` | Push/PR, weekly schedule | Security analysis |
| `release.yml` | Tag v*.*.* | Publish to crates.io |
| `dependabot-auto-merge.yml` | Dependabot PRs | Auto-merge patch updates |

## Branch Protection

Both `main` and `bevy-0.17` have OSSF branch protection:

- **Required status checks:** Check, Test, Clippy, Format
- **Strict status checks:** Branch must be up-to-date
- **Enforce admins:** `false` (admin can bypass)
- **Force pushes:** Blocked
- **Branch deletion:** Blocked

## Dependabot Configuration

- **Cargo updates:** Weekly on Monday 9 AM ET
- **GitHub Actions updates:** Weekly on Monday 9 AM ET
- **Bevy updates:** Ignored (manual migration required)
- **Target branches:** main, bevy-0.17

## MSRV (Minimum Supported Rust Version)

| Branch | MSRV | Notes |
| -------- | ------ | ------- |
| main | 1.94 | Matches `rust-version` in `Cargo.toml` on `main` |
| bevy-0.17 | Branch-specific | Check that branch's `Cargo.toml` |

## Bevy Migration Process

When migrating to a new Bevy version:

1. Create/checkout the `bevy-0.XX` branch
2. Update `Cargo.toml` with new Bevy version
3. Update MSRV in `Cargo.toml` and `ci.yml`
4. Fix breaking changes (see `docs/dev/BEVY_0.17_TO_0.18_MIGRATION.md`)
5. Run full test suite
6. Update `CHANGELOG.md`
7. Keep `main` as the Bevy 0.18.x line and retire obsolete support branches on schedule

## Publishing to crates.io

```bash
# Ensure CARGO_REGISTRY_TOKEN is set in GitHub secrets
# Tag the release
git tag vX.Y.Z
git push origin vX.Y.Z

# release.yml workflow will automatically publish
```

## Troubleshooting

### CI Fails with Missing System Dependencies

Linux CI needs these packages:

```text
libudev-dev libasound2-dev libwayland-dev libxkbcommon-dev
libx11-dev libxi-dev libxcursor-dev libxrandr-dev libxinerama-dev
```

### Clippy Fails with New Lints

New Rust versions add lints. Fix them or add targeted `#[allow()]`:

```rust
#[allow(clippy::new_lint_name)]
```

### Sync Workflow Creates Conflict Issue

1. Check the issue for instructions
2. Manually resolve the conflict
3. Close the issue
