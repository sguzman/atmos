# Agentic Systems

Focus: the reusable cognition stack behind Atmos agents.

## Milestone G1: Perception and world understanding

- [ ] Represent perceptions as structured observations rather than raw engine queries.
- [ ] Introduce affordances such as `Edible`, `Carryable`, `Talkable`, `Sleepable`, `Usable`.
- [ ] Separate sensed world state from inferred or remembered world state.
- [ ] Track uncertainty or confidence where useful.

## Milestone G2: Beliefs and memory

- [ ] Store remembered objects, locations, and social facts.
- [ ] Support stale or incorrect beliefs.
- [ ] Distinguish current perception from remembered knowledge.
- [ ] Add memory decay or refresh rules.

## Milestone G3: Needs, values, and utility

- [ ] Represent needs as normalized pressures.
- [ ] Represent action desirability as explicit utility or scoring terms.
- [ ] Make utility inspection visible in debugging.
- [ ] Allow personality or role to modify scoring.

## Milestone G4: Intentions and tasks

- [ ] Separate high-level intention from low-level execution.
- [ ] Add task decomposition such as `AcquireFood -> MoveTo -> Consume`.
- [ ] Support interruption, cancellation, and retry.
- [ ] Support failure reasons and fallback behavior.

## Milestone G5: Multi-agent systemic behavior

- [ ] Relationships can influence action choice.
- [ ] Ownership, permission, or social norms can influence action choice.
- [ ] Agent-to-agent observation can change plans.
- [ ] Socially legible agentic behavior emerges without hardcoded scenes.
