# Expressive Animation

Focus: embodiment, expression, and fluid visible behavior.

## Milestone AN1: Motor intent boundary

- [ ] Define a `MotorIntent`-style layer between cognition and animation.
- [ ] Include desired velocity, facing, look target, interaction target, and stance.
- [ ] Keep high-level AI from directly choosing raw animation clips.

## Milestone AN2: Embodied locomotion

- [ ] Walking responds to desired velocity rather than discrete clip requests.
- [ ] Turning can begin with gaze/head orientation before torso alignment.
- [ ] Arrival behavior slows and settles near targets.
- [ ] Facing behavior aligns naturally for interaction.

## Milestone AN3: Expressive layering

- [ ] Base locomotion layer.
- [ ] Task or interaction layer.
- [ ] Gaze layer.
- [ ] Emotional/postural layer.
- [ ] Personality layer.
- [ ] Facial or micro-motion layer when available.

## Milestone AN4: Social readability

- [ ] Expression changes are visible enough for other agents or the player to interpret.
- [ ] Gaze, posture, and movement style carry simulation meaning.
- [ ] Similar actions can look different across personalities and emotional states.

## Milestone AN5: Research-facing extensions

- [ ] Support import of captured or reconstructed human animation as source material.
- [ ] Preserve semantic runtime control above imported clip data.
- [ ] Add contact cleanup, IK, and constraint layering where needed.
