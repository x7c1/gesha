# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.12](https://github.com/x7c1/gesha/compare/gesha-v0.0.11...gesha-v0.0.12) - 2025-04-12

### Added

- generate From and TryFrom impls for numeric enums ([#148](https://github.com/x7c1/gesha/pull/148))

## [0.0.11](https://github.com/x7c1/gesha/compare/gesha-v0.0.10...gesha-v0.0.11) - 2025-04-05

### Added

- generate doc comments for fields ([#144](https://github.com/x7c1/gesha/pull/144))
- create gesha-collections ([#141](https://github.com/x7c1/gesha/pull/141))

### Fixed

- allow "default" in the responses field ([#145](https://github.com/x7c1/gesha/pull/145))

## [0.0.10](https://github.com/x7c1/gesha/compare/gesha-v0.0.9...gesha-v0.0.10) - 2025-03-29

### Added

- support boolean enum values ([#138](https://github.com/x7c1/gesha/pull/138))

### Fixed

- skip unrecognized formats ([#140](https://github.com/x7c1/gesha/pull/140))

### Other

- update Cargo.toml dependencies
- *(deps)* update all non-major dependencies ([#137](https://github.com/x7c1/gesha/pull/137))

## [0.0.9](https://github.com/x7c1/gesha/compare/gesha-v0.0.8...gesha-v0.0.9) - 2025-03-22

### Added

- support numeric enum values ([#132](https://github.com/x7c1/gesha/pull/132))

### Other

- migrate to Rust 2024 edition ([#135](https://github.com/x7c1/gesha/pull/135))
- refactor error categories ([#136](https://github.com/x7c1/gesha/pull/136))

## [0.0.8](https://github.com/x7c1/gesha/compare/gesha-v0.0.7...gesha-v0.0.8) - 2025-03-16

### Added

- collapse single-item oneOf ([#129](https://github.com/x7c1/gesha/pull/129))

### Fixed

- show errors when gesha-verify fails during conversion ([#131](https://github.com/x7c1/gesha/pull/131))

### Other

- update Cargo.lock dependencies

## [0.0.7](https://github.com/x7c1/gesha/compare/gesha-v0.0.6...gesha-v0.0.7) - 2025-03-08

### Added

- collapse single-item allOf ([#124](https://github.com/x7c1/gesha/pull/124))

### Fixed

- allow ascii symbols as enum values ([#127](https://github.com/x7c1/gesha/pull/127))

### Other

- update Cargo.lock dependencies

## [0.0.6](https://github.com/x7c1/gesha/compare/gesha-v0.0.5...gesha-v0.0.6) - 2025-03-01

### Added

- enhance error messages (#117)
- support newtype conversion from $ref schemas (#119)

### Fixed

- allow non-PascalCase type names in $ref (#122)
- prevent crash when inline items are nested (#120)

### Other

- update Cargo.lock dependencies

## [0.0.5](https://github.com/x7c1/gesha/compare/gesha-v0.0.4...gesha-v0.0.5) - 2025-02-26

### Other

- updated the following local packages: gesha-core, gesha-rust-shapes

## [0.0.4](https://github.com/x7c1/gesha/compare/gesha-v0.0.3...gesha-v0.0.4) - 2025-02-25

### Added

- keep running if errors are detected (#113)

## [0.0.3](https://github.com/x7c1/gesha/compare/gesha-v0.0.2...gesha-v0.0.3) - 2025-02-23

### Other

- updated the following local packages: gesha-core

## [0.0.2](https://github.com/x7c1/gesha/compare/gesha-v0.0.1...gesha-v0.0.2) - 2025-02-22

### Other

- update Cargo.lock dependencies
