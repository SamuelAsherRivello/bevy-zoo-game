---
name: security-review
description: "Inspect code, configuration, workflows, dependencies, file handling, network exposure, and secret handling."
---

# Security Review

## Overview

Do not print secrets; describe where users should enter them directly.

## Workflow

1. Inspect the relevant repository files first.
2. Keep changes scoped to the active request or spec.
3. Prefer project-local conventions over generic assumptions.
4. Record verification steps and unresolved risks before finishing.

## Guardrails

- Keep this template project-neutral unless a generated project intentionally specializes it.
- Do not store or print secrets.
- Do not run destructive Git or database operations.