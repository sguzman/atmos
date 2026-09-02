# Showcase Doctrine

Showcases are first-class Atmos deliverables.

Their purpose is to make one capability visible, understandable, testable, and reusable before it is buried inside a larger game or simulation.

## Showcase rule

A good showcase answers one question clearly:

> What does this system actually do when I run it?

A showcase should normally have:

- a narrow concept or claim
- a small scene with obvious behavior
- semantic/declarative configuration where appropriate
- reusable implementation rather than scene-specific fake behavior
- instrumentation or debug output for invisible state
- automated tests for deterministic logic and invariants
- a documented command or selector for running it
- minimal dependencies on unrelated systems

A showcase does **not** need to be visually final. It does need to make the capability legible.

## Development ladder

When useful, grow a system through this sequence:

1. **seed** — smallest closed behavior
2. **basic showcase** — canonical understandable example
3. **choice/stress showcase** — parameters or pressure produce meaningfully different outcomes
4. **failure/recovery showcase** — system handles invalidation, interruption, or boundary cases
5. **mixed showcase** — system interacts with at least one independently developed capability
6. **world integration** — capability participates in a larger simulated world

Not every track needs every rung.

## Showcase families

Use clear semantic names. Expected families include:

- `agent_*` — autonomous/agentic behavior
- `emotion_*` — appraisal, emotion, social expression
- `animation_*` — locomotion, gaze, procedural or expressive motion
- `model3d_*` — declarative/generative 3D modeling
- `physics_*` — general physical/mechanical systems
- `fluid_*` — water and fluid behavior
- `gear_*` — gearing and mechanical transmission
- `electric_*` — electrical networks and devices
- `fire_*` — heat, ignition, fuel, burning, spread
- `vehicle_*` — vehicles composed from simulated subsystems
- `design2d_*` — declarative vector/SVG-like design
- `billboard_*` — 2D designs presented in 3D space
- `mixed_*` — explicit cross-system experiments

Names are conventions, not an excuse to build a second hardcoded scene architecture. Prefer one discoverable showcase-selection mechanism.

## Anti-patterns

Avoid:

- scripting the exact outcome a supposedly systemic showcase is meant to demonstrate
- copying the same implementation into multiple showcase scenes
- hiding important state so behavior looks arbitrary
- requiring the full village simulation to test an independent capability
- turning every experiment into a permanent generalized framework before it has proved useful
- pursuing physical fidelity beyond what creates interesting, understandable game behavior

## Success criterion

The showcase catalog should eventually feel like a workshop shelf: a developer can pick a system, run a small scene, understand it, change semantic parameters, and immediately see what that capability could contribute to a game.
