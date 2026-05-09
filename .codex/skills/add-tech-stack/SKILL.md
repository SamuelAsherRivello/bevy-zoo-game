---
name: add-tech-stack
description: "Add or plan a project-specific technology stack in the Codex Project Template while preserving the template's shared setup philosophy. Use when Codex is asked to choose, introduce, document, or prepare a stack-neutral repo for a concrete language, framework, runtime, scripts, assets, specs, README updates, or follow-on stack overlay skills such as add-tech-stack-rust, add-tech-stack-bevy, or add-tech-stack-dioxus."
---

# Add Tech Stack

## Overview

Use this as the shared base workflow before adding any specific stack. It protects the original repo's purpose: Codex and Specify stay the foundation, while technology choices are explicit overlays.

Read `references/shared-philosophy.md` when the task asks for stack policy, root layout, README/AGENTS updates, or how stack overlays should compose.

## Workflow

1. Inspect the current repo tree, `README.md`, `AGENTS.md`, `.codex/project-identity.md`, `.specify/memory/constitution.md`, and active `specs/`.
2. Identify whether the user wants a general stack decision or a named overlay skill.
3. Preserve the project-neutral contract unless the user explicitly asks to specialize this repo.
4. Put reusable scripts in root `scripts` unless the selected stack's reference explicitly chooses another path.
5. Put runtime assets in the selected stack's asset folder, with Bevy assets under `bevy/crates/game/assets`.
6. Update README, AGENTS, specs, and `.codex/memory/project-memory.md` when the stack changes repository conventions.
7. Add only the minimum dependencies, files, and commands needed for the selected stack.
8. Verify with stack-appropriate commands and report any checks that cannot run.

## Overlay Selection

| User Intent | Use |
| ----------- | --- |
| Shared setup philosophy, generic scripts/assets/specs, or stack selection | `add-tech-stack` |
| General Rust workspace or CLI/library/application foundation | `add-tech-stack-rust` |
| Bevy ECS game project foundation | `add-tech-stack-bevy` |
| Dioxus full-stack/web/desktop app foundation | `add-tech-stack-dioxus` |

## Guardrails

- Do not silently turn the generic template into a stack-specific template.
- Do not add multiple stacks unless the user explicitly requests a combined project.
- Do not copy entire reference repositories by default; import the convention, not unrelated sample app content.
- Keep Codex/Specify files as first-class project infrastructure.
