---
name: refresh-readme
description: "Update README facts, tables, links, and structure from current repository truth while preserving the established format."
---

# Refresh README

## Overview

Keep image references aligned but do not regenerate images unless requested.

## Workflow

1. Inspect the relevant repository files first.
2. Keep changes scoped to the active request or spec.
3. Prefer project-local conventions over generic assumptions.
4. Record verification steps and unresolved risks before finishing.

## Guardrails

- Keep this template project-neutral unless a generated project intentionally specializes it.
- Do not store or print secrets.
- Do not run destructive Git or database operations.