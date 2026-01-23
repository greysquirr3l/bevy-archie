# Test Coverage Guide

This document covers how to run tests and generate coverage reports for bevy_archie.

## Running Tests

### Unit Tests

```bash
# Run all unit tests
cargo test --lib

# Run specific test module
cargo test --lib config::tests
cargo test --lib actions::tests
cargo test --lib icons::tests

# Run with verbose output
cargo test --lib -- --nocapture
```

### Integration Tests

```bash
# Run integration tests
cargo test --test integration_tests

# Run all tests (unit + integration)
cargo test
```

### Feature-Specific Tests

```bash
# Run with all features enabled
cargo test --all-features

# Run with specific features
cargo test --features "dualsense"
```

## Test Coverage Analysis

### Installing Tarpaulin

[cargo-tarpaulin](https://github.com/xd009642/tarpaulin) is a code coverage tool for Rust.

```bash
# Install tarpaulin
cargo install cargo-tarpaulin
```

### Generating Coverage Reports

#### HTML Report

```bash
# Generate HTML report in coverage/ directory
cargo tarpaulin --out Html --output-dir coverage

# Open the report (macOS)
open coverage/tarpaulin-report.html

# Open the report (Linux)
xdg-open coverage/tarpaulin-report.html
```

#### Console Output

```bash
# Console-only output
cargo tarpaulin --out Stdout

# Both HTML and console
cargo tarpaulin --out Html --out Stdout --output-dir coverage
```

#### With All Features

```bash
cargo tarpaulin --all-features --out Html --output-dir coverage
```

#### JSON Output (for CI)

```bash
cargo tarpaulin --out Json --output-dir coverage
```

### Coverage Configuration

You can create a `tarpaulin.toml` file for persistent configuration:

```toml
[default]
output-dir = "coverage"
out = ["Html", "Stdout"]
all-features = true
workspace = true
timeout = "120s"

# Exclude test code from coverage
exclude-files = ["tests/*", "examples/*"]
```

Then run with:

```bash
cargo tarpaulin
```

## Test Structure

### Unit Tests

Each module contains its own test submodule:

```text
src/
├── actions.rs      # src/actions/tests
├── config.rs       # src/config/tests  
├── detection.rs    # src/detection/tests
├── icons.rs        # src/icons/tests
├── gyro.rs         # src/gyro/tests
├── haptics.rs      # src/haptics/tests
├── touchpad.rs     # src/touchpad/tests
└── ...
```

### Integration Tests

Integration tests are in the `tests/` directory:

```text
tests/
└── integration_tests.rs
```

These test end-to-end functionality including:

- Plugin initialization
- Resource management
- System execution order
- Event/Message handling

## Coverage Goals

| Module | Target | Description |
| -------- | -------- | ------------- |
| `actions` | 90% | Core action mapping system |
| `config` | 90% | Configuration and persistence |
| `detection` | 85% | Device detection logic |
| `icons` | 90% | Icon filename generation |
| `haptics` | 85% | Rumble patterns |
| `profiles` | 85% | Controller profiles |
| `gyro` | 80% | Motion control data |
| `touchpad` | 80% | Touchpad data |
| **Overall** | **80%** | Minimum target |

## Continuous Integration

For CI pipelines, use the JSON output format:

```yaml
# GitHub Actions example
- name: Run tests with coverage
  run: |
    cargo install cargo-tarpaulin
    cargo tarpaulin --all-features --out Json --output-dir coverage
    
- name: Upload coverage
  uses: codecov/codecov-action@v3
  with:
    files: coverage/tarpaulin-report.json
```

## Troubleshooting

### Tarpaulin Hangs

If tarpaulin hangs, try increasing the timeout:

```bash
cargo tarpaulin --timeout 300
```

### Missing Coverage Data

Some code may not show coverage due to:

- Conditional compilation (`#[cfg(...)]`)
- Macros that expand to untested code
- Code that requires hardware (gyro, touchpad)

### Platform-Specific Issues

Tarpaulin works best on Linux. For macOS/Windows, consider:

- Running in a Linux container
- Using `cargo-llvm-cov` as an alternative

```bash
# Alternative: llvm-cov
cargo install cargo-llvm-cov
cargo llvm-cov --html
```
