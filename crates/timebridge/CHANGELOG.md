# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0](https://github.com/sunsided/timebridge/compare/v0.1.0...v0.2.0) - 2026-11-19

### Added

- Add release-plz and tag-driven crates.io publishing
- Add per-crate READMEs and absolute README links
- Add REUSE licensing with SPDX license texts

### Changed

- Cover the core crate's error message and clock reference impls
- Link SystemTime in the timebridge docs by URL
- Unlink SystemTime in the timebridge crate docs
- Keep pre-1601 SystemTime round trips on non-Windows
- Tighten lints and forbid unsafe code
- Render mermaid diagrams on docs.rs

### Fixed

- Fix SystemTime round-trip test range on Windows
