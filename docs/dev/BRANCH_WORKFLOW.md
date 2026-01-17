# Branch Workflow & Synchronization

> **Note:** This document is untracked and for local development reference only.

## Branch Strategy

| Branch | Purpose | Bevy Version | Status |
| -------- | --------- | -------------- | -------- |
| `main` | Latest stable release | 0.17.x | Active |
| `bevy-0.17` | Maintenance for Bevy 0.17 | 0.17.x | Active |
| `bevy-0.18` | Migration to Bevy 0.18 | 0.18.x | Development |

## Automatic Branch Sync

The `sync-branches.yml` workflow automatically cherry-picks commits from `main` to `bevy-0.17`.

### How It Works

1. Push a commit to `main`
2. Workflow triggers automatically
3. Commit is cherry-picked to `bevy-0.17`
4. If conflicts occur, an issue is created

### Skipping Sync

Add `[skip-sync]` to your commit message to prevent auto-sync:

```bash
git commit -m "feat: main-only feature [skip-sync]"
```

### Manual Sync (Workflow Dispatch)

Go to Actions → "Sync Branches" → Run workflow:

- **source_branch**: Branch to cherry-pick from (default: main)
- **target_branch**: Branch to cherry-pick to (default: bevy-0.17)
- **commit_sha**: Specific commit (leave empty for latest)

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
| `sync-branches.yml` | Push to main | Sync commits to bevy-0.17 |

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
| main | 1.88 | Matches Bevy 0.17.3 |
| bevy-0.17 | 1.88 | Matches Bevy 0.17.3 |
| bevy-0.18 | 1.89 | Required for Bevy 0.18 |

## Bevy Migration Process

When migrating to a new Bevy version:

1. Create/checkout the `bevy-0.XX` branch
2. Update `Cargo.toml` with new Bevy version
3. Update MSRV in `Cargo.toml` and `ci.yml`
4. Fix breaking changes (see `docs/dev/BEVY_0.17_TO_0.18_MIGRATION.md`)
5. Run full test suite
6. Update `CHANGELOG.md`
7. When stable, merge to `main`

## Publishing to crates.io

```bash
# Ensure CARGO_REGISTRY_TOKEN is set in GitHub secrets
# Tag the release
git tag v0.1.4
git push origin v0.1.4

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
