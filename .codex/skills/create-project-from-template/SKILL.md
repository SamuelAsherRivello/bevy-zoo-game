---
name: create-project-from-template
description: "Create a new project repository from this generic Codex and Specify template."
---

# Create Project From Template

## Overview

Confirm the destination path is safe, copy only template files, update project identity, verify no excluded generated artifacts are present, and commit once when requested.

## Workflow

1. Inspect the relevant repository files first.
2. Keep changes scoped to the active request or spec.
3. Prefer project-local conventions over generic assumptions.
4. Record verification steps and unresolved risks before finishing.

## Guardrails

- Keep this template project-neutral unless a generated project intentionally specializes it.
- Do not store or print secrets.
- Do not run destructive Git or database operations.