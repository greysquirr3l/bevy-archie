# Security Policy

## Supported Versions

We release patches for security vulnerabilities. Currently supported versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take the security of bevy-archie seriously. If you believe you have found a security vulnerability, please report it to us as described below.

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to the repository owner at the email associated with the GitHub account [@greysquirr3l](https://github.com/greysquirr3l).

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

Please include the requested information listed below (as much as you can provide) to help us better understand the nature and scope of the possible issue:

* Type of issue (e.g. buffer overflow, injection, cross-site scripting, etc.)
* Full paths of source file(s) related to the manifestation of the issue
* The location of the affected source code (tag/branch/commit or direct URL)
* Any special configuration required to reproduce the issue
* Step-by-step instructions to reproduce the issue
* Proof-of-concept or exploit code (if possible)
* Impact of the issue, including how an attacker might exploit the issue

This information will help us triage your report more quickly.

## Preferred Languages

We prefer all communications to be in English.

## Policy

We follow the principle of [Responsible Disclosure](https://en.wikipedia.org/wiki/Responsible_disclosure).

## Security Considerations

### Input Validation

This library processes controller input from hardware devices. While the library itself does not execute arbitrary code from input data, users should be aware that:

* Controller input is read through Bevy's gamepad abstraction layer
* Configuration files are loaded from JSON and should be validated before use
* The library does not process untrusted network input by default

### Data Privacy

* Controller configurations may be saved to disk in platform-specific directories
* No telemetry or analytics data is collected by this library
* Users should be aware that saved configurations may contain gameplay preferences

### Dependencies

This project depends on:

* Bevy Engine (0.17.x) - A data-driven game engine
* serde/serde_json - For configuration serialization
* dirs - For platform-specific directory paths

We monitor our dependencies for security advisories through:

* GitHub Dependabot
* cargo-audit
* Regular dependency updates

### Best Practices

When using this library:

1. Validate user-provided configuration files before loading
2. Implement rate limiting if allowing users to save configurations frequently
3. Sanitize file paths if allowing custom configuration locations
4. Use the library's safe APIs and avoid `unsafe` code blocks where possible

## Acknowledgments

We appreciate the security research community and will acknowledge researchers who responsibly disclose vulnerabilities (with permission).
