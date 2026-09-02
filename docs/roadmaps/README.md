# Atmos Roadmaps

This folder is the working milestone package for Atmos.

These files are meant to be edited continuously as implementation advances. When work lands, the relevant checkbox should be updated in the matching file.

## Atmos has two complementary modes

### Showcase laboratory

Build small executable scenes that prove interesting game systems independently: autonomous behavior, animation, declarative visual techniques, physical simulation, fluids, gears, electricity, fire, vehicles, and combinations of those systems.

See `05_SHOWCASE_DOCTRINE.md` for the rules that make showcases useful rather than disposable demos.

### Integrated living world

Compose mature capabilities into agents, households, and eventually a small simulated village. `80_INTEGRATION_MILESTONES.md` defines the recommended order for this track.

The showcase laboratory can advance in parallel; it does not have to wait for the village roadmap.

## File map

- `00_PROJECT_CHARTER.md` — project north star and scope.
- `01_ITERATION_PROTOCOL.md` — how implementation work should proceed.
- `05_SHOWCASE_DOCTRINE.md` — what a first-class Atmos showcase is.
- `10_AUTONOMOUS_AGENTS.md` — autonomous-agent behavior.
- `20_AGENTIC_SYSTEMS.md` — reusable cognition architecture.
- `30_EMOTIONAL_SYSTEMS.md` — emotion and social inference.
- `40_EXPRESSIVE_ANIMATION.md` — embodiment and expressive motion.
- `50_GENERATIVE_MODELING.md` — declarative/generative 3D modeling.
- `60_VILLAGE_LIFE.md` — households and village-scale simulation.
- `70_SYSTEMS_LAB.md` — physics, fluids, gears, electricity, fire, vehicles, and mixed systems.
- `75_DECLARATIVE_2D.md` — SVG-like 2D design and world billboards.
- `80_INTEGRATION_MILESTONES.md` — recommended living-world integration order.
- `90_RESEARCH_TRACKS.md` — experimental branches that should not derail production architecture.

## Status conventions

- `[ ]` not started
- `[~]` in progress
- `[x]` completed

## Living-world causal chain

`world systems -> perception -> beliefs/memory -> needs/emotions/relationships -> decision -> action -> motor intent -> animation/expression -> visible behavior -> social/systemic reaction`

## Broader laboratory goal

Atmos should become a workshop for testing systems that are interesting to see in games. A system can earn its place by being a compelling, understandable showcase even before it is integrated into the village. Successful systems should remain reusable and increasingly composable so later mixed showcases can produce deeper chains of cause and effect.
