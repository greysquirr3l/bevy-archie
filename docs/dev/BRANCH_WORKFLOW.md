# Branch Workflow & Synchronization

> **Note:** This document is untracked and for local development reference only.

## Branch Strategy

Branch policy (effective 2026-10-22): `main` is Bevy **0.20.x** — the migration has landed and `bevy-0.20` was merged into `main` via PR #51. The maintenance branches are `bevy-0.19` (Bevy 0.19.x, cut from `main` at v0.3.0 on 2026-09-21) and `bevy-0.18` (Bevy 0.18.x). Bevy 0.17 support is end-of-life and the `bevy-0.17` branch has been retired; final 0.1.x releases remain on crates.io.

| Branch      | Purpose                                   | Bevy Version   | Status      |
| ----------- | ----------------------------------------- | -------------- | ----------- |
| `main`      | Latest stable release                     | 0.20.x         | Active      |
| `bevy-0.19` | Maintenance for Bevy 0.19                 | 0.19.x         | Maintenance |
| `bevy-0.18` | Maintenance for Bevy 0.18                 | 0.18.x         | Maintenance |
| `release/*` | Release staging for a bevy_archie version | same as `main` | Transient   |

`bevy-0.20` remains as a merged, historical branch — it is no longer a migration target and will not receive further commits.

### Release branches

`release/<version>` is a **staging** branch, distinct from the `bevy-*` **maintenance** branches:

- `release/0.4` holds the release-prep commits for bevy_archie 0.4.x. It is cut from `main` and deleted once the release ships.
- `bevy-0.19` / `bevy-0.18` are long-lived lines that receive backports _after_ a release ships, because those releases track an older Bevy.

Two constraints make the release branch work, both easy to get wrong:

1. **`ci.yml` must trigger on `release/*`.** `release.yml`'s `verify-ci` job waits for a CI workflow result on the tagged commit. A release branch without CI would hang the release indefinitely.
2. **The release branch must be squash-merged into `main`** with the subject `chore: bump version to <version>`. The `auto-tag.yml` workflow gates on `main`'s HEAD subject matching `chore:*bump version*` — a plain merge commit ("Merge pull request #N …") does not match, so the tag would never be pushed.

### Release flow

1. Cut `release/<version>` from `main`
2. Land the release-prep commits: version in `Cargo.toml`, `[Unreleased]` moved into the new version section, version comparison links updated
3. Squash-merge into `main` with the subject `chore: bump version to <version>`
4. `auto-tag.yml` pushes `v<version>` once CI is green
5. `release.yml` verifies CI on the tagged commit, publishes to crates.io, and creates the GitHub Release using the matching CHANGELOG section
6. Delete the release branch

Steps 4–6 publish to crates.io. That is irreversible — verify the CHANGELOG section and `Cargo.toml` version before merging.

> The pre-cut state of `bevy-0.19` (at `7f4ee28`, before the 2026-09-21 cut from `main`) is preserved as the tag `pre-cut/bevy-0.19-2026-09-21` so the granular CI hardening history is recoverable. The same CI hardening changes also live on `main` under the squash PR #44 (`b25486f`).

## Backport Strategy

There is no automatic `sync-branches.yml` workflow in this repository. Backports from `main` to `bevy-0.19` and `bevy-0.18` are manual cherry-picks when needed. Most fixes are Bevy-version-agnostic and should go to all three branches; changes touching version-gated APIs (text, UI widgets, settings) generally apply only to the branches on that Bevy version.

### How It Works

1. Push a commit to `main`
2. Decide which maintenance branches need it, and whether the backport is Bevy-version-agnostic
3. Cherry-pick the commit onto each target branch
4. Resolve conflicts locally if they occur — a conflict in version-gated code usually means the backport does not apply to that branch

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
git checkout bevy-0.19

# Cherry-pick the commit
git cherry-pick <commit-sha>

# If conflicts:
# 1. Resolve conflicts in your editor
# 2. git add .
# 3. git cherry-pick --continue

# Push
git push origin bevy-0.19

# Return to main
git checkout main
```

## CI/CD Workflows

| Workflow                    | Trigger                                           | Purpose                                 |
| --------------------------- | ------------------------------------------------- | --------------------------------------- |
| `ci.yml`                    | Push/PR to main, bevy-0.19, bevy-0.18, release/\* | Check, Test, Clippy, Format, Docs, MSRV |
| `security.yml`              | Push/PR, weekly schedule                          | cargo-audit, cargo-deny                 |
| `codeql.yml`                | Push/PR, weekly schedule                          | Security analysis                       |
| `release.yml`               | Tag v*.*.\*                                       | Publish to crates.io                    |
| `dependabot-auto-merge.yml` | Dependabot PRs                                    | Auto-merge patch updates                |
| `auto-tag.yml`              | CI completion on `main`                           | Push `v<version>` after a version bump  |

## Branch Protection

`main` and `bevy-0.18` have OSSF branch protection:

- **Required status checks:** Check, Test, Clippy, Format
- **Strict status checks:** Branch must be up-to-date
- **Enforce admins:** `false` (admin can bypass)
- **Force pushes:** Blocked
- **Branch deletion:** Blocked

## Dependabot Configuration

- **Cargo updates:** Weekly on Monday 9 AM ET
- **GitHub Actions updates:** Weekly on Monday 9 AM ET
- **Bevy updates:** Ignored (manual migration required)
- **Target branches:** main, bevy-0.19, bevy-0.18

## MSRV (Minimum Supported Rust Version)

| Branch    | MSRV            | Notes                                            |
| --------- | --------------- | ------------------------------------------------ |
| main      | 1.97.1          | Matches `rust-version` in `Cargo.toml` on `main` |
| bevy-0.19 | Branch-specific | Check that branch's `Cargo.toml`                 |
| bevy-0.18 | Branch-specific | Check that branch's `Cargo.toml`                 |

## Bevy Migration Process

When migrating to a new Bevy version:

1. Create/checkout the `bevy-0.XX` branch
2. Update `Cargo.toml` with new Bevy version
3. Update MSRV in `Cargo.toml` and `ci.yml`
4. Fix breaking changes (see the Bevy release notes and upstream migration guide for that version)
5. Run full test suite
6. Update `CHANGELOG.md`
7. Keep `main` as the latest Bevy release line and demote the previous release line to a `bevy-0.XX` maintenance branch

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
