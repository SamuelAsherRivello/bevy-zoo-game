---
name: plan-before-coding
description: "Inspect the repo and define scope, risks, approach, and verification before changing implementation files."
---

# Plan Before Coding

## Overview

Use this when behavior is broad, ambiguous, or tied to specs.

## Workflow

1. Inspect the relevant repository files first.
2. Keep changes scoped to the active request or spec.
3. Prefer project-local conventions over generic assumptions.
4. Record verification steps and unresolved risks before finishing.

## Guardrails

- Keep this template project-neutral unless a generated project intentionally specializes it.
- Do not store or print secrets.
- Do not run destructive Git or database operations.