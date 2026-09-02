# Declarative 2D and Billboard Design

Focus: make 2D visual construction fit Atmos's declaration-first authoring philosophy rather than requiring hand-authored raster assets for everything.

The conceptual target is an SVG-like semantic/vector language that can generate reusable 2D art and then place that art in UI, textures, signs, sprites, or 3D-world billboards.

## Milestone D0: Declarative 2D foundation

- [ ] Define a minimal semantic 2D document representation.
- [ ] Support transforms, grouping, layering, and reusable named elements.
- [ ] Keep source human-readable and diffable.
- [ ] Establish deterministic rendering/export tests where practical.

## Milestone D1: SVG-like vector primitives

- [ ] Rectangle, ellipse/circle, line/polyline, and polygon primitives.
- [ ] Bezier/path support.
- [ ] Fill and stroke controls.
- [ ] Transform composition.
- [ ] Groups and reusable symbols/components.
- [ ] Text support at a useful first level.
- [ ] Parameterized repetition, mirroring, and simple procedural layout.
- [ ] `design2d_basic` showcase demonstrates a complete design authored declaratively.

## Milestone D2: Semantic/procedural 2D design

- [ ] Allow higher-level declarations to compile into primitive vector geometry.
- [ ] Support style tokens/palettes without coupling them to specific designs.
- [ ] Support data-driven labels/icons/signage.
- [ ] Demonstrate several visually distinct results from one reusable declaration/schema.

## Milestone D3: 2D billboards in 3D

- [ ] Render a declarative 2D design onto/in a 3D-world surface.
- [ ] Support fixed-orientation signs/panels.
- [ ] Support camera-facing billboards where useful.
- [ ] Preserve useful transparency/material controls.
- [ ] Allow dynamic text/data to update a billboard without hand-authoring a new texture.
- [ ] `billboard_basic` demonstrates a declaratively authored design functioning as a world object.

## Milestone D4: Mixed 2D/3D visual language

- [ ] Use generated 2D art as signage on declarative 3D buildings or props.
- [ ] Support labels/diagrams/debug visualization generated from simulation state.
- [ ] Explore decals, posters, maps, gauges, dashboards, and interfaces as shared 2D constructs.
- [ ] Demonstrate a vehicle or machine whose gauges/signage are driven by live system state.

## Guiding rule

2D declarations should describe visual meaning and composition, not merely become verbose serialization of rendering-engine internals.
