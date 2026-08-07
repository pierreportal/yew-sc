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
- `ssr` and `hydration` features, mirroring Yew's own. `styled_component!`
  now registers CSS directly in each component's render body instead of
  from a `use_effect`, since Yew never runs effects during server-side
  rendering. `yew_sc::stylesheet()` exposes the accumulated CSS for a server
  to embed in its response.

### Changed

- Removed the unused `yew` dependency from `yew-sc-macros` (the crate only
  emits `::yew::...` paths as tokens; it never calls into the `yew` crate
  itself).
- Removed the unused `syn` dependency from `yew-sc-core`.
- `yew-sc`'s `csr` feature is now opt-in via the `csr` feature flag (still
  the default) rather than hardcoded on the `yew` dependency, so a
  server-only build can select `ssr`/`hydration` instead.
- Split the runtime registry into `register_style` (pure bookkeeping, safe
  under SSR) and `sync_dom` (DOM-only `<style>` tag write, gated to
  `wasm32` — `web_sys` panics unconditionally on other targets rather than
  returning `None`, which made the previous `.unwrap()`-based code
  unusable off the browser).

## [0.1.0]

- Initial release: `styled_component!` macro with compile-time CSS and
  element validation, hashed class names, dynamic values via `$name` /
  `${ expr }`, and `#[styled_props]` for bring-your-own props structs.
