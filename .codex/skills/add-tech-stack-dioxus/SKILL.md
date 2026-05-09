---
name: add-tech-stack-dioxus
description: "Add or plan a Dioxus technology stack overlay for the Codex Project Template using add-tech-stack first and the Dioxus reference repository https://github.com/SamuelAsherRivello/dioxus-project-template. Use when Codex is asked to add Dioxus 0.7, web/desktop app structure, Dioxus routing/state/assets, fullstack setup, Dioxus scripts, README feature matrices, or Dioxus-specific AGENTS/spec guidance."
---

# Add Dioxus Stack

## Overview

Use this after `add-tech-stack` when the selected technology is Dioxus. The overlay should add Dioxus 0.7 conventions while preserving Codex/Specify as the repo's planning and agent infrastructure.

Read `references/dioxus-template.md` before editing files for Dioxus stack work.

## Workflow

1. Apply the shared `add-tech-stack` workflow first.
2. Use `https://github.com/SamuelAsherRivello/dioxus-project-template` as the Dioxus reference unless the user names another source.
3. Add Dioxus 0.7 guidance to AGENTS and specs before writing app code.
4. Preserve the web/desktop split when the generated project needs both targets.
5. Keep visible loading/status feedback for async app work.
6. Update README feature matrices when Dioxus features, components, routes, cache behavior, platform support, or suggested future work changes.
7. Verify with the Dioxus build/run/test commands introduced by the implementation.

## Guardrails

- Do not use removed Dioxus APIs such as `cx`, `Scope`, or `use_state`.
- Do not introduce browser SQLite or OPFS worker startup unless the user explicitly chooses that architecture.
- Do not copy template app routes or demo content unless the user asks for the Dioxus starter app itself.
