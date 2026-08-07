# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

### Added

- `keyframes` support for `@keyframes` animations, hashed and referenced from
  `animation` / `animation-name` values.
- `static-extract` feature for zero-runtime CSS: styles are embedded in a wasm
  custom section and pulled into a static stylesheet by the `yew-sc-extract`
  xtask.
- `rust-version` (MSRV) pinned to 1.88 across all workspace crates — the
  codebase relies on stabilized let-chains.

### Changed

- Removed the unused `yew` dependency from `yew-sc-macros` (the crate only
  emits `::yew::...` paths as tokens; it never calls into the `yew` crate
  itself).

## [0.1.0]

- Initial release: `styled_component!` macro with compile-time CSS and
  element validation, hashed class names, dynamic values via `$name` /
  `${ expr }`, and `#[styled_props]` for bring-your-own props structs.
