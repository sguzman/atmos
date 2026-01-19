# Atmos Engine

This Bevy-based project (`atmos`) is structured around TOML-driven scene configuration, tooling for caching generated meshes, and a small state machine that swaps between a `Menu` and `Main` scene.

## Core pieces

- `Cargo.toml` – standard Rust workspace manifest (Bevy 0.17, Rapier, inspector, TOML/serde/etc.).
- `assets/` – runtime data:
  - `config.toml` – top-level app settings (mode, FPS limit, window, mouse, debug flags).
  - `scenes/` – per-scene folders (main world at `scenes/main/world.toml`) plus subfolders for entities, overlays, dialogue, etc.
  - `textures/`, `dialogue/`, `overlay/` – asset groups referenced by TOML configs.
- `schemas/` – JSON schemas describing entity/world/action/dialogue config shapes for tooling or validation.

## Runtime flow (`src/`)

- `main.rs`
  - Parses CLI (`allow_runtime_mesh` flag, `bake` subcommand).
  - Loads `AppConfig` from `assets/config.toml`, sets up log/window plugins, mesh cache settings, and registers custom TOML/mesh loaders.
  - Determines the starting `AppState` by parsing `assets/scenes/main/world.toml`.
  - Adds `MenuPlugin` and a `ScenePlugin` to manage the two-state lifecycle.
- `app_config.rs`
  - Deserializes `AppConfig` and exposes Bevy plugin/window/winit helpers plus debug toggles.
  - Falls back to defaults when `assets/config.toml` is missing or malformed.

- `scenes/`
  - `config/` – shared configuration structures (camera, bounds, lights, render, physics, etc.) used by TOML files.
  - `world.rs` – `WorldConfig` that mirrors `world.toml`.
  - `loaders.rs` – `TomlCache` and helper functions that load/parse TOML configs via `TomlAsset`.
  - `toml_asset.rs` – custom asset/loader that reads `.toml` as UTF-8 strings.
  - `mesh_cache.rs` – generates and caches mesh data (`.meshcache`) for shapes defined in entities/world TOML, plus bake CLI logic.
  - `scene/` submodules:
    - `menu.rs` – UI/input setup for `AppState::Menu`, including action bindings, overlay spawns, and quit/switch logic.
    - `spawn/plugin/mod.rs` (plus its nested modules) – `ScenePlugin` lifecycle for `AppState::Main`: handles setup, input, overlays, cursors, reloads, cleanup, and logging.

## Workflow

1. Run `cargo run` (optionally with `--allow-runtime-mesh` or `bake`) to start:
   - Loads TOML scene config via custom asset loaders.
   - Boots Bevy with configured window/log/debug settings.
   - Switches between `Menu` and `Main` states per the world `.toml`.
2. Add or tweak `assets/scenes/.../*.toml` (camera, overlays, entities, actions) to change behavior.
3. Use `cargo run -- bake` to precompute mesh caches for static shapes, stored under `assets/.cache/meshes/`.

Refer to the schemas in `schemas/` if you need guidance when authoring new TOML configs.
