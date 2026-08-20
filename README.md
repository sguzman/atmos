# Atmos Engine

Atmos is a Bevy-powered playground where every scene and interaction is driven by TOML. It pairs lightweight configuration with mesh caching, physics, and a handful of action systems so you can prototype novel ideas (like the slicing workflow outlined in `tmp/cut.md`) without rebuilding the engine every time.

## Repository layout

- `Cargo.toml`, `Cargo.lock` – the Bevy/Rapier/serde manifest that bundles the runtime and feature flags.
- `assets/` – runtime content:
  - `config.toml` – global settings for the window, camera, debug flags, and mesh cache paths.
  - `scenes/` – one folder per scene. Each scene houses:
    * `world.toml` – entry point that wires up camera, bounds, gravity, render tweaks, lights, and scene transitions.
    * `actions.toml`, `input.toml`, `entities.toml`, and the `entities/` directory – templates and overrides for triggers, controls, and spawnable primitives.
    * `tmp/` under a scene can be used for experimentation (e.g., `tmp/cut.md` captures design notes for the slicing pipeline).
  - `simulation/` contains small semantic simulation testbeds, beginning with `agent_demo.toml`.
  - `textures/`, `dialogue/`, and `overlay/` contain auxiliary assets referenced from the TOML files.
  - `.cache/meshes/` – precomputed mesh exports (including sliced halves) written by the mesh cache system during `cargo run -- bake`.
- `schemas/` – JSON schemas intended to validate the TOML files for scenes, entities, and actions so you know what fields are available.
- `src/` – everything that runs at runtime:
  - `main.rs` and `app_config.rs` parse the CLI, load `assets/config.toml`, register plugins, and bootstrap the `Menu` ↔ `Main` app states.
  - `scenes/` contains the domain logic: TOML loaders, mesh cache helpers, bounds/despawn systems, input/action wiring, and the `spawn` tree that instantiates entities, overlays, lights, and the cutting workflow.
  - `simulation/` is a deliberately separate plugin boundary for autonomous agents and other future world-simulation systems.
- `scenes/spawn/cut.rs` implements the modal cutting system: it looks for `ShapeConfig` entries marked `cuttable`, selects them while you hold the cut key via the preview plane, and uses the plane-slicing utilities described in `tmp/cut.md` to split cubes into two cached halves.

## TOML-driven behavior

- **Entities** – Each template under `assets/scenes/<scene>/entities/` defines a `name`, `transform`, `shape`, `material`, and `physics`. Set `shape.cuttable = true` (only box shapes currently) to let the slicer target that object. Templates blend overrides from `entities.toml` and spawn tables that reuse material assets while obeying physics overrides.
- **Actions & Triggers** – Actions (see `schemas/actions.schema.json`) describe rates, ranges, and colors for gestures like `grab`, `shoot`, and `cut`. Triggers map keys/mouse/volumes to those action IDs. The cut system uses the `cut` action, whose `angle_step_degrees`, `rotation_sensitivity`, `preview_size`, `preview_thickness`, `preview_color`, `preview_opacity`, and `preview_emissive` come straight from TOML. You can also switch the action `mode` between `hold` (default) and `toggle`, control how scroll-wheel ticks advance the preview via `wheel_rotation_sensitivity`, and set the confirmation button through `confirm_button` (defaults to `right` click) so the slicer never shares a binding with shooting.
- **Input** – The camera and system overlays are configured in `input.toml`; overlays toggle on demand while the cut system relies on the `c` key (or whatever trigger points at the cut action) as a hold-mode entry point.
- **Simulation** – New autonomous systems expose domain concepts such as hunger, perception, movement speed, and nutrition rather than serializing Bevy implementation details into content files.

## Cutting workflow

1. **Selection** – Hold the cut action key (or flip `mode = "toggle"` to keep it latched). While in that mode, every `CuttableShape` under a dynamic rigid body is raycasted so the preview plane appears on the cube you're pointing at (the plane uses the color configured via `actions.params.preview_color`).
2. **Preview** – Once an entity is selected, a transparent 2D preview (plane) spawns at the center and rotates about its vertical axis according to your mouse motion (and scroll-wheel rolls, which also advance via `wheel_rotation_sensitivity`). The angle snaps to the configuration’s resolution (`angle_step_degrees`), so you get crisp cuts without fighting continuous rotation.
3. **Slice** – Press the configured confirmation button (default `right` click) once the plane is aligned to invoke the slicing routine. The cutter:
   - Transforms the plane into the object’s local space.
   - Clips the cube’s triangles into two halves.
   - Caps the exposed boundary to keep the volumes manifold.
   - Builds mesh/collider pairs, caches the result under `assets/.cache/meshes/`, and spawns two new entities with the original material and sane physical properties (splitting mass/restitution/friction accordingly).
4. **Cache reuse** – Subsequent cuts with identical dimensions/angle reuse the cached mesh handles, so even repeated slicing stays responsive.
5. **References** – For the geometry/physics reasoning behind these steps, consult `tmp/cut.md`; it is a living notes file that explains the plane-slicing approach, collider heuristics, and physics trade-offs that inspired the implementation.

## Running the project

Atmos currently targets Windows as the active development baseline.

- `cargo run` boots the existing menu/main states and normal Atmos scene stack.
- `cargo run -- --agent-demo` runs the intentionally tiny autonomous-agent testbed instead. The agent becomes hungry, perceives nearby food, chooses a target, walks to it, consumes it, and reduces its hunger.
- `cargo run -- bake` precomputes mesh caches in `assets/.cache/meshes/`.
- Modify TOML under `assets/scenes/...` and the custom TOML asset loader can re-import it when the scene is reloaded.

## Development workflow

Before committing engine changes, run:

```powershell
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```

Windows CI runs the same checks. The asset test recursively parses every checked-in `assets/**/*.toml`, and the autonomous-agent configuration has additional typed validation tests.

See `docs/toml-schema-audit.md` for the current authoring/schema audit and the decision to keep TOML as Atmos's semantic public format while treating Bevy scene internals as an implementation detail.

## Autonomous-agent seed

The first simulation is intentionally primitive. It establishes a clean `src/simulation/` plugin boundary and one closed behavior loop:

`hunger rises → perceive food → choose nearest visible-in-radius food → move → consume → hunger falls`

There is no claim that this is the final AI architecture. Future iterations can replace target choice, perception, navigation, memory, planning, animation, and needs independently without coupling those systems back into the existing scene loader.

## Next steps

- Want new primitives? Add a template under a scene’s `entities/`, mark it `cuttable` if you want the slicer to target it, and extend `schemas/3d.entity.schema.json` if you add fields.
- Need advanced slicing? Follow the ideas in `tmp/cut.md` to expand beyond cubes, improve cap triangulation, or adjust the physics colliders/RigidBody mass split logic.
- Want richer agents? Extend the simulation plugin one capability at a time rather than turning the demo target selector into a monolithic AI system.
