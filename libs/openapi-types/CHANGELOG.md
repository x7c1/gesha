# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.9](https://github.com/x7c1/gesha/compare/openapi-types-v0.0.8...openapi-types-v0.0.9) - 2025-04-12

### Added

- generate From and TryFrom impls for numeric enums ([#148](https://github.com/x7c1/gesha/pull/148))

## [0.0.8](https://github.com/x7c1/gesha/compare/openapi-types-v0.0.7...openapi-types-v0.0.8) - 2025-04-05

### Added

- generate doc comments for fields ([#144](https://github.com/x7c1/gesha/pull/144))
- create gesha-collections ([#141](https://github.com/x7c1/gesha/pull/141))

### Fixed

- allow "default" in the responses field ([#145](https://github.com/x7c1/gesha/pull/145))

## [0.0.7](https://github.com/x7c1/gesha/compare/openapi-types-v0.0.6...openapi-types-v0.0.7) - 2025-03-29

### Added

- support boolean enum values ([#138](https://github.com/x7c1/gesha/pull/138))

### Fixed

- skip unrecognized formats ([#140](https://github.com/x7c1/gesha/pull/140))

## [0.0.6](https://github.com/x7c1/gesha/compare/openapi-types-v0.0.5...openapi-types-v0.0.6) - 2025-03-22

### Added

- support numeric enum values ([#132](https://github.com/x7c1/gesha/pull/132))

### Other

- refactor error categories ([#136](https://github.com/x7c1/gesha/pull/136))
- migrate to Rust 2024 edition ([#135](https://github.com/x7c1/gesha/pull/135))

## [0.0.5](https://github.com/x7c1/gesha/compare/openapi-types-v0.0.4...openapi-types-v0.0.5) - 2025-03-01

### Added

- enhance error messages (#117)

## [0.0.4](https://github.com/x7c1/gesha/compare/openapi-types-v0.0.3...openapi-types-v0.0.4) - 2025-02-26

### Fixed

- prevent crash when unspecified type found (#114)

## [0.0.3](https://github.com/x7c1/gesha/compare/openapi-types-v0.0.2...openapi-types-v0.0.3) - 2025-02-25

### Added

- keep running if errors are detected (#113)
- enable tracking of keys leading to an error (#112)

### Fixed

- prevent crash when using inline allOf with array newtype (#110)
