---
name: add-tech-stack-bevy
description: "Add or plan a Bevy technology stack overlay for the Codex Project Template using add-tech-stack first and the Bevy reference repository https://github.com/SamuelAsherRivello/bevy-project-template. Use when Codex is asked to add Bevy, game architecture, ECS plugins/systems/components/resources, Bevy assets, hot reload, game scripts, or Bevy-specific README/AGENTS/spec guidance."
---

# Add Bevy Stack

## Overview

Use this after `add-tech-stack` when the selected technology is Bevy. The overlay should preserve the generic Codex/Specify foundation while adding Bevy's game architecture deliberately.

Read `references/bevy-template.md` before editing files for Bevy stack work.

## Workflow

1. Apply the shared `add-tech-stack` workflow first.
2. Use `https://github.com/SamuelAsherRivello/bevy-project-template` as the Bevy reference unless the user names another source.
3. Add Bevy conventions only when the project is intentionally a game, simulation, editor, or Bevy runtime.
4. Keep ECS concepts explicit: components, resources, systems, plugins, assets, tests, and hot reload.
5. Update README, AGENTS, specs, and memory with Bevy-specific commands and architecture.
6. Keep generated game assets under the chosen Bevy asset folder.
7. Verify with the Bevy build/run/test commands introduced by the implementation.

## Guardrails

- Do not use this skill for generic Rust projects; use `add-tech-stack-rust`.
- Do not import sample gameplay unless the user asks for it.
- Do not let Bevy folder naming override existing project conventions without documenting the decision.
