# Physical and Systemic Simulation Lab

Focus: reusable approximate simulations that create interesting game behavior and can later interact with agents, environments, vehicles, and each other.

The target is not engineering-grade fidelity. The target is a coherent enough model that causes, state, constraints, and consequences are visible and composable.

## Milestone S0: Shared systems conventions

- [ ] Establish units/conventions for physical quantities used across showcases.
- [ ] Keep configuration semantic rather than exposing raw engine internals unnecessarily.
- [ ] Separate deterministic/system logic from rendering where practical.
- [ ] Provide inspection/debug surfaces for important invisible state.
- [ ] Establish a consistent showcase-selection pattern for systems-lab scenes.

## Milestone S1: General physical systems

- [ ] Showcase forces, mass, friction, restitution, and constraints in a legible scene.
- [ ] Showcase springs/dampers or other useful mechanical constraints.
- [ ] Showcase joints/hinges as reusable mechanical primitives.
- [ ] Add parameterized tests for core deterministic calculations where appropriate.

## Milestone S2: Water and fluids

Start with useful approximations before pursuing expensive fluid solvers.

- [ ] `fluid_basic` demonstrates a bounded body of water or fluid quantity with inspectable state.
- [ ] Model flow/transfer between connected containers or regions.
- [ ] Add buoyancy or displacement behavior useful to gameplay.
- [ ] Add pressure/head/height effects where they create meaningful behavior.
- [ ] Demonstrate a leak, drain, pump, or valve.
- [ ] Keep higher-fidelity grid/particle fluid simulation as an optional research direction rather than a prerequisite.

## Milestone S3: Gears and mechanical transmission

- [ ] `gear_basic` demonstrates two meshed gears with correct directional/ratio behavior.
- [ ] Represent shafts and rotational state explicitly enough to inspect.
- [ ] Propagate torque/speed through a simple gear train approximately.
- [ ] Add clutch/disengagement or another controllable mechanical connection.
- [ ] Explore belts/chains/pulleys if they provide useful reusable primitives.
- [ ] Demonstrate a mechanical load changing system behavior.

## Milestone S4: Electricity

- [ ] `electric_basic` demonstrates source -> conductor -> switch -> load.
- [ ] Represent electrical nodes/connections semantically.
- [ ] Model voltage/current/power at an intentionally approximate game-useful level.
- [ ] Add resistive loss or capacity limits where useful.
- [ ] Add batteries or other stored-energy sources.
- [ ] Add motors/generators as bridges between electrical and mechanical systems.
- [ ] Provide a live circuit/state inspection surface.

## Milestone S5: Fire and heat

- [ ] `fire_basic` demonstrates fuel, ignition, burning, heat, and burnout.
- [ ] Separate material fuel/flammability from current burning state.
- [ ] Demonstrate fire spread based on understandable local conditions.
- [ ] Represent heat transfer approximately enough for cross-system interactions.
- [ ] Demonstrate extinguishing/cooling.
- [ ] Explore smoke/oxygen only when they provide clear systemic value.

## Milestone S6: Approximate whole-vehicle simulation

The vehicle goal is specifically **not** a monolithic `CarController` that fakes every outcome. Build a vehicle from subsystems that are each simulated to an approximate but meaningful degree.

- [ ] Define a declarative vehicle assembly made from parts/subsystems.
- [ ] Simulate wheel contact and traction approximately.
- [ ] Simulate suspension and steering.
- [ ] Simulate braking.
- [ ] Simulate an engine or electric motor as a power source.
- [ ] Simulate gearing/transmission between power source and wheels.
- [ ] Simulate stored energy: fuel, battery, or both.
- [ ] Connect an electrical system to vehicle loads/devices where appropriate.
- [ ] Expose enough subsystem state to understand why the vehicle behaves as it does.
- [ ] Produce at least one drivable showcase whose behavior emerges from the assembled subsystems.

## Milestone S7: Mixed systems

Mixed showcases are where the laboratory becomes especially valuable.

- [ ] Electricity can drive a mechanical motor/load.
- [ ] Gears can transmit power produced by another system.
- [ ] Water can cool or extinguish fire.
- [ ] Water/electricity interactions have explicit modeled consequences where useful.
- [ ] Heat can affect materials or another simulated system.
- [ ] A vehicle combines multiple independently demonstrated subsystems.
- [ ] At least one `mixed_*` showcase demonstrates a chain of consequences across three systems.

## Later integration

Successful systems may eventually become things agents perceive, operate, repair, fear, exploit, build, or depend upon. That integration is desirable, but it is not required before the independent system is worth exploring.
