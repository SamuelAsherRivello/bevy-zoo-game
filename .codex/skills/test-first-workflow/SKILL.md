---
name: test-first-workflow
description: "Drive changes with tests or checks before production edits where practical."
---

# Test First Workflow

## Overview

Keep tests focused on behavior and regression risk.

## Workflow

1. Inspect the relevant repository files first.
2. Keep changes scoped to the active request or spec.
3. Prefer project-local conventions over generic assumptions.
4. Record verification steps and unresolved risks before finishing.

## Guardrails

- Keep this template project-neutral unless a generated project intentionally specializes it.
- Do not store or print secrets.
- Do not run destructive Git or database operations.