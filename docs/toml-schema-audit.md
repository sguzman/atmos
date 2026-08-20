# TOML and schema audit

Atmos already has the right high-level separation: Rust implements engine capabilities while TOML describes scenes, entities, inputs, actions, dialogue, rendering parameters, and reusable content. That semantic authoring boundary should be preserved while Bevy internals continue to evolve.

## What is already strong

- Scene-level configuration is split from entity templates, actions, input, dialogue, and overlays.
- Reusable entity templates and overrides avoid forcing every scene to hard-code Bevy ECS construction.
- JSON schemas exist for the major authoring families: 2D/3D entities, entity collections, combo entities, actions, dialogue, lights, and worlds.
- Runtime TOML loading and the mesh cache already prove that the authoring layer is more than static serialization.
- The TOML vocabulary describes game concepts rather than exposing raw Bevy component dumps.

## Gaps found in the resurrection pass

1. The repository had no automated test proving that every checked-in TOML file still parses.
2. JSON schemas existed, but CI did not enforce that runtime Rust types and schemas remained synchronized.
3. Startup world selection still performs a direct filesystem read in `main.rs`; this is acceptable for now but bypasses the custom Bevy asset path used elsewhere.
4. Schema evolution has no explicit version field or migration policy yet.
5. The existing action configuration is broad and powerful enough that semantic validation will eventually be more valuable than syntax-only validation.
6. Generated mesh cache files live beneath `assets/`; cache ownership and source-control policy should remain explicit as the project grows.

## Decisions

- TOML remains Atmos's public authoring format.
- Bevy scene/BSN infrastructure may be used underneath Atmos later, but `.bsn` is not the user-facing replacement for the semantic TOML layer.
- New simulation systems should expose domain concepts in TOML (`hunger`, `nutrition`, `perception_radius`) rather than serialized engine implementation details.
- CI now treats checked-in TOML as executable API and parses every asset TOML.
- New complex configuration families should additionally receive typed Rust validation tests, as the first autonomous-agent demo does.

## Follow-up work

- Add automated JSON Schema validation for TOML families whose file-to-schema mapping is unambiguous.
- Introduce schema/config versioning before incompatible authoring changes accumulate.
- Move direct startup filesystem reads behind the same asset/config abstraction where doing so improves hot reload and error reporting.
- Add semantic cross-reference validation (missing templates, invalid action IDs, broken paths) rather than limiting validation to syntax.
