# Contributing to bevy-archie

Thank you for your interest in contributing to bevy-archie! This document provides guidelines and information for contributors.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:

   ```bash
   git clone https://github.com/YOUR_USERNAME/bevy-archie.git
   cd bevy-archie
   ```

3. **Add the upstream remote**:

   ```bash
   git remote add upstream https://github.com/greysquirr3l/bevy-archie.git
   ```

## Development Setup

### Prerequisites

- Rust 1.85 or later (see `rust-version` in Cargo.toml)
- System dependencies for Bevy:
  - **Linux**: `libudev-dev libasound2-dev`
  - **macOS**: No additional dependencies
  - **Windows**: No additional dependencies

### Building

```bash
# Build with default features
cargo build

# Build with all features
cargo build --all-features

# Build examples
cargo build --examples --features motion-backends
```

### Testing

```bash
# Run tests
cargo test --features motion-backends

# Run tests with all features
cargo test --all-features

# Run a specific test
cargo test test_name
```

### Linting

We use strict clippy lints. Before submitting:

```bash
# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-features -- -D warnings

# Run clippy on examples
cargo clippy --examples --features motion-backends -- -D warnings
```

## Making Changes

### Branch Naming

- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation changes
- `refactor/description` - Code refactoring

### Commit Messages

We use conventional commits:

```text
type(scope): description

[optional body]

[optional footer]
```

Types:

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Formatting, missing semicolons, etc.
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `test`: Adding missing tests
- `chore`: Maintenance tasks

Examples:

```text
feat(gyro): add shake gesture detection
fix(touchpad): correct finger tracking delta calculation
docs(readme): update controller support matrix
```

### Pull Request Process

1. **Create a feature branch** from `main`
2. **Make your changes** with clear, focused commits
3. **Add tests** for new functionality
4. **Update documentation** as needed
5. **Ensure CI passes** (clippy, tests, formatting)
6. **Submit a pull request** with a clear description

### PR Checklist

- [ ] Code compiles without warnings (`cargo check --all-features`)
- [ ] All tests pass (`cargo test --all-features`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Clippy passes (`cargo clippy --all-features -- -D warnings`)
- [ ] Documentation is updated if needed
- [ ] CHANGELOG.md is updated for notable changes

## Code Style

### Rust Guidelines

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` defaults
- Document all public items with doc comments
- Use meaningful variable and function names
- Prefer explicit types in public APIs

### Bevy Guidelines

- Use Bevy's naming conventions (e.g., `*Plugin`, `*Event`, `*Resource`)
- Systems should be focused and single-purpose
- Use `Commands` sparingly; prefer direct component access
- Document system ordering requirements

### Documentation

- All public items must have doc comments
- Include examples in doc comments where helpful
- Use `# Examples` sections for complex APIs
- Keep comments up to date with code changes

## Reporting Issues

### Bug Reports

Include:

- Bevy version and bevy-archie version
- Operating system and Rust version
- Controller model (if applicable)
- Minimal reproduction steps
- Expected vs actual behavior
- Relevant error messages or logs

### Feature Requests

Include:

- Clear description of the feature
- Use case and motivation
- Proposed API (if applicable)
- Any alternatives considered

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and grow
- Follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct)

## License

By contributing, you agree that your contributions will be licensed under the same dual MIT/Apache-2.0 license as the project.

## Questions?

- Open a [GitHub Discussion](https://github.com/greysquirr3l/bevy-archie/discussions)
- Check existing [Issues](https://github.com/greysquirr3l/bevy-archie/issues)

Thank you for contributing! 🎮
