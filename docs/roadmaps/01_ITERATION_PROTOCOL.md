# Iteration Protocol

## Default implementation rhythm

- [x] Start from one visible vertical slice, not a framework in isolation.
- [x] Add a declarative representation when the behavior proves useful.
- [x] Add tests for parsing, semantics, and invariants.
- [x] Add a debugger or inspection surface showing why the system behaved as it did.
- [x] Connect the new system to one live agent.
- [x] Make the result physically visible in-world.
- [ ] Make at least one other system react to it.

## Things to avoid

- [ ] Do not spend months on AI abstractions with no moving creature.
- [ ] Do not spend months on animation abstractions with no emotions driving them.
- [ ] Do not spend months on procedural modeling with nobody inhabiting the output.
- [ ] Do not rewrite stable working systems without a vertical-slice payoff.

## Ongoing workflow expectations

- [x] Keep `cargo check --all-targets` green.
- [x] Keep `cargo test --all-targets` green.
- [x] Keep `cargo fmt --all -- --check` green.
- [x] Drive `cargo clippy --all-targets -- -D warnings` to green and keep it there.
- [ ] Commit only after the validation set is green for the intended scope.
- [x] Update roadmap checkboxes as work lands.

## Current development baseline

- [x] Bevy 0.19 migration is substantially in place.
- [x] A first autonomous-agent simulation slice exists.
- [x] The full "green after all checks" workflow still needs to be maintained continuously.
