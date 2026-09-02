# Integration Milestones

This file defines the recommended order for the **living-world / village integration track**.

It is not the only valid order for Atmos experimentation. Independent showcase work in physical systems, fluids, mechanics, electricity, fire, vehicles, 2D design, or modeling may proceed in parallel when interesting. Those systems only enter this sequence when they are being composed into the living world.

## M0: Green baseline

- [x] `cargo check --all-targets`
- [x] `cargo test --all-targets`
- [x] `cargo fmt --all -- --check`
- [x] `cargo clippy --all-targets -- -D warnings`
- [x] Freeze the migration baseline once green.

## M1: The Hungry Walker

- [x] One autonomous creature completes a hunger loop.
- [x] The loop is debuggable and explainable.
- [x] The loop is represented through declarative config where appropriate.

## M2: The Embodied Hungry Walker

- [ ] Cognition outputs motor intent instead of clip names.
- [ ] The body visibly approaches, turns, looks, and consumes.
- [ ] The same architecture can drive future agents.

## M3: The Declarative Person

- [ ] A first semantic character description compiles into a puppet-like body.
- [ ] That body can receive motor intent and express simple behavior.
- [ ] The style is acceptable even if still geometric and rough.

## M4: Emotion becomes physical

- [ ] Appraisal updates emotional state.
- [ ] Emotional state changes action preference.
- [ ] Emotional state changes visible expression.

## M5: Two social creatures

- [ ] Agents infer each other from visible behavior rather than direct state access.
- [ ] Social misunderstanding is possible.
- [ ] Relationship state can shift through interaction.

## M6: One household

- [ ] Multiple agents share resources and routines.
- [ ] At least one production/consumption chain exists.
- [ ] Several in-game days can run with legible emergent behavior.

## M7: The expressive cast

- [ ] Personality and emotion produce visibly distinct motion styles.
- [ ] Similar actions can look different across individuals.
- [ ] Expression meaningfully informs simulation and social inference.

## M8: Tiny village

- [ ] The project scales to a small but understandable village.
- [ ] Roles, homes, work, trade, conflict, and cooperation exist.
- [ ] The simulation remains inspectable.

## M9: Generative village

- [ ] Characters, props, and buildings increasingly come from semantic geometry descriptions.
- [ ] Visual variety scales without abandoning declarative authorship.

## Parallel showcase tracks

These do not gate M2-M9 and may be explored opportunistically:

- `70_SYSTEMS_LAB.md` — physical systems, fluids, gears, electricity, fire, vehicles, and mixed simulations
- `75_DECLARATIVE_2D.md` — SVG-like declarative design and 2D/3D billboards
- `50_GENERATIVE_MODELING.md` — declarative/generative 3D modeling
- `90_RESEARCH_TRACKS.md` — experiments that should not silently redefine production architecture
