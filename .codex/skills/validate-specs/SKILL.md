---
name: validate-specs
description: "Compare specs, plans, tasks, and implementation truth for drift."
---

# Validate Specs

## Overview

Report contradictions, missing acceptance criteria, and checked-off work that is not implemented.

## Workflow

1. Inspect the relevant repository files first.
2. Keep changes scoped to the active request or spec.
3. Prefer project-local conventions over generic assumptions.
4. Record verification steps and unresolved risks before finishing.

## Guardrails

- Keep this template project-neutral unless a generated project intentionally specializes it.
- Do not store or print secrets.
- Do not run destructive Git or database operations.