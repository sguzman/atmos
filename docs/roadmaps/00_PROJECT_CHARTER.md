# Atmos Project Charter

## North star

Atmos builds declarative 3D worlds in which autonomous agents live under systemic pressures, form beliefs and emotions, choose actions, visibly embody those internal states through expressive motion, and inhabit bodies and environments increasingly compiled from semantic geometric descriptions.

## Core architectural doctrine

Atmos is not three separate efforts.

- It is not "AI over here, animation over there, procedural geometry somewhere else."
- It is one causal architecture.
- Every subsystem should eventually participate in visible simulated life.

## Non-negotiable causal chain

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

- Prefer vertical slices over isolated subsystem work.
- Keep TOML as the semantic authoring layer unless a stronger reason emerges.
- Make internal state debuggable.
- Separate "what the mind wants" from "how the body performs it."
- Treat expressive behavior as simulation data, not decoration.
- Treat declarative geometry as a source language, not an excuse to block gameplay progress.

## Long-term end states

- believable autonomous agents
- extensible agentic system architecture
- emotional and social simulation
- expressive embodied animation
- generative declarative character and world modeling
- a small but legible simulated village
