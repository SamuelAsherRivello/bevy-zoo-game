# Shared Tech Stack Philosophy

## Purpose

`codex-project-template` starts as an agent-ready, project-neutral foundation. Technology stacks are overlays that add conventions, commands, and source layout only when a project chooses them.

## Shared Conventions

| Area | Convention |
| ---- | ---------- |
| Specs | Keep active feature specs in `specs/`. |
| Codex | Keep repo-local skills, memory, and rules in `.codex/`. |
| Specify | Keep workflow templates, scripts, and constitution in `.specify/`. |
| Assets | Use the selected stack's runtime asset folder; Bevy assets live under `bevy/crates/game/assets/`. |
| Scripts | Start repeatable commands in root `scripts/`; stack overlays may introduce a more idiomatic scripts folder if the reference repo uses one consistently. |
| README | Keep the current README shape: TOC, Pics, Getting Started, Details, Structure, Features, Credits. |
| Images | Keep README images in `documentation/images/` unless a generated stack deliberately changes documentation conventions. |

## Overlay Policy

| Overlay | Reference Repo | Import Style |
| ------- | -------------- | ------------ |
| Rust | `https://github.com/SamuelAsherRivello/rust-project-template` | Import workspace, hot reload, and script philosophy; avoid game/demo assumptions unless requested. |
| Bevy | `https://github.com/SamuelAsherRivello/bevy-project-template` | Import ECS/game structure, asset folders, and hot reload workflow; keep it clearly Bevy-specific. |
| Dioxus | `https://github.com/SamuelAsherRivello/dioxus-project-template` | Import Dioxus 0.7, web/desktop split, Spec Kit alignment, and README feature-matrix discipline. |

## Decision Rules

- If a stack has not been selected, keep generated files generic.
- If a stack is selected, update root docs and memory in the same change.
- If a stack introduces generated output, add ignore rules before running commands that create it.
- If reference repos conflict with this template, keep Codex/Specify conventions and adapt the stack around them.
