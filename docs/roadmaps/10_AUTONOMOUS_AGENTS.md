# Autonomous Agents

Focus: believable single-agent and small-group behavior under concrete world pressures.

## Milestone A0: Buildable seed

- [x] Create an isolated simulation plugin boundary.
- [x] Add a first hungry-agent demo configuration.
- [x] Add typed config validation for the simulation seed.
- [ ] Keep the seed passing full workflow validation as the simulation grows.

## Milestone A1: The Hungry Walker

- [x] One agent has hunger that rises over time.
- [x] Food is represented as a world entity with an edible affordance.
- [x] The agent can perceive food inside a radius.
- [x] The agent can choose the nearest viable food target.
- [x] The agent can move into interaction range.
- [x] The agent can consume food and reduce hunger.
- [x] The demo can run as one closed autonomous loop without scripting each step.

## Milestone A2: Explainable autonomy

- [x] Debug output shows perceived objects.
- [x] Debug output shows candidate intentions with scores.
- [x] Debug output shows the selected intention.
- [x] Debug output shows the current subtask or action stage.
- [x] Debug output is available in a persistent in-engine or log-based inspection surface.

## Milestone A2.5: Hungry Walker showcase suite

- [x] `hungry_basic` demonstrates the full hunger -> perceive -> choose -> walk -> eat -> idle loop.
- [x] `hungry_choice` demonstrates explainable scored choice between several food candidates.
- [x] `hungry_replan` demonstrates target invalidation and target reselection while pursuing food.
- [x] `hungry_perception` demonstrates that out-of-range food is ignored and never scored.

## Milestone A3: Broader needs

- [ ] Add thirst.
- [ ] Add fatigue.
- [ ] Add simple scheduling pressure or time-of-day effects.
- [ ] Add inventory or carry-state support for actions.
- [ ] Make need tradeoffs visible in decision making.

## Milestone A4: Small-group agents

- [ ] Multiple agents can run concurrently.
- [ ] Agents can contend over shared resources.
- [ ] Delays and scarcity can alter action selection.
- [ ] Agent state remains legible under concurrency.
