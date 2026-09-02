# Atmos Project Charter

## North star

Atmos is a declarative game-systems laboratory and eventual systemic-world engine.

It exists for two complementary purposes:

1. **Showcase laboratory** — build small, legible scenes that prove interesting game systems in isolation.
2. **Integrated worlds** — compose successful systems into richer simulations, with autonomous village life as the central long-term integration target.

Atmos should make it cheap to ask, "what would this system look like in a game?", implement a bounded version, inspect it, and then decide whether to deepen or combine it.

## Core architectural doctrine

Atmos is not a collection of unrelated demos, but neither must every experiment immediately serve the village.

- Individual systems may begin as isolated showcases.
- Successful systems should expose reusable capabilities rather than scene-specific tricks.
- Systems should become composable where useful.
- TOML and other semantic declarations describe domain intent; Rust implements capabilities.
- A showcase is a first-class deliverable, not disposable prototype code.

For the living-world track, AI, animation, procedural geometry, and world systems form one causal architecture rather than separate projects.

## Living-world causal chain

`WORLD SYSTEMS`

`-> AGENT PERCEPTION`

`-> BELIEFS + MEMORY`

`-> NEEDS + VALUES + EMOTIONS + RELATIONSHIPS`

`-> DECISION / INTENTION`

`-> ACTION EXECUTION`

`-> MOTOR INTENT`

`-> ANIMATION + EXPRESSION`

`-> VISIBLE BEHAVIOR`

`-> OTHER AGENTS PERCEIVE IT`

`-> WORLD SYSTEMS CHANGE`

## Design principles

- Prefer visible vertical slices over speculative frameworks.
- Treat showcase scenes as executable documentation for a capability.
- Keep TOML as the semantic authoring layer unless a stronger reason emerges.
- Prefer declarative/generated representations for visual assets where practical.
- Make internal state and system behavior debuggable.
- Separate "what the mind wants" from "how the body performs it."
- Treat expressive behavior as simulation data, not decoration.
- Treat declarative geometry as a source language, not an excuse to block gameplay progress.
- Prefer understandable approximations over unnecessary physical fidelity.
- Design independent systems so they can later participate in mixed-system showcases.

## Long-term end states

- believable autonomous agents
- extensible agentic system architecture
- emotional and social simulation
- expressive embodied animation
- generative declarative 3D modeling
- declarative 2D/vector and billboard design
- reusable physical/systemic simulations such as fluids, mechanics, electricity, and fire
- vehicles assembled from approximately simulated subsystems rather than monolithic vehicle behavior
- mixed-system scenes where independent capabilities interact
- a small but legible simulated village
- a growing catalog of polished showcases for interesting systems that could exist in games
