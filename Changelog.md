# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [Unreleased]

## [0.5.0] - 2023-09-09

### Added
- reexport prime abbreviations to `::all`.
- add `N` abbreviation for `Numbers`.

### Remove
- remove reexported items at the root.

### Changed
- update `devela` to `0.9.0`.
- remove `std` from default features.
- rename enums prefixed by `Any` to `All`.
- rename enums in singular form to plural form.
- rename traits in plural form to singular form.
- deprecate features: `all_nostd`, `all_nostd_fast`.
- deprecate and rename feature `all_fast` to `full_fast`.
- deprecate and rename feature `all` to `full`.

### Fixed
- fix prime doc examples to avoid long exec times.
- fix including licenses in crate.
- fix enabling `devela/alloc`.
- misc. internal refactoring.
- avoid destructuring enums.
- fix several tests.

## [0.4.0] - 2023-08-29

Too many changes!

## [0.3.0] - 2023-04-27

## [0.2.0] - 2023-04-23

## [0.1.0] - 2022-12-04

## [0.0.7] - 2022-08-04

## [0.0.6] - 2022-07-12

## [0.0.5] - 2022-07-11

## [0.0.4] - 2022-07-08

## [0.0.3] - 2022-07-07

## [0.0.2] - 2022-07-06

## [0.0.1] - 2021-12-30

[unreleased]: https://github.com/andamira/numera/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/andamira/numera/releases/tag/v0.5.0
[0.4.0]: https://github.com/andamira/numera/releases/tag/v0.4.0
[0.3.0]: https://github.com/andamira/numera/releases/tag/v0.3.0
[0.2.0]: https://github.com/andamira/numera/releases/tag/v0.2.0
[0.1.0]: https://github.com/andamira/numera/releases/tag/v0.1.0
[0.0.7]: https://github.com/andamira/numera/releases/tag/v0.0.7
[0.0.6]: https://github.com/andamira/numera/releases/tag/v0.0.6
[0.0.5]: https://github.com/andamira/numera/releases/tag/v0.0.5
[0.0.4]: https://github.com/andamira/numera/releases/tag/v0.0.4
[0.0.3]: https://github.com/andamira/numera/releases/tag/v0.0.3
[0.0.2]: https://github.com/andamira/numera/releases/tag/v0.0.2
[0.0.1]: https://github.com/andamira/numera/releases/tag/v0.0.1

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
