---
name: add-tech-stack-rust
description: "Add or plan a Rust technology stack overlay for the Codex Project Template using add-tech-stack first and the single reference repository https://github.com/SamuelAsherRivello/rust-project-template. Use when Codex is asked to add Rust, create Rust workspace conventions, Rust scripts, Cargo setup, hot reload, Rust AGENTS/README guidance, or Rust-specific specs while preserving the generic Codex/Specify foundation."
---

# Add Rust Stack

## Overview

Use this after `add-tech-stack` when the selected technology is Rust. The Rust overlay should be inspired by `rust-project-template`, not a blind copy of every sample file.

Read `references/rust-template.md` before editing files for Rust stack work.

## Workflow

1. Apply the shared `add-tech-stack` workflow first.
2. Inspect the current repo and decide whether the task is planning only or implementation.
3. Use `https://github.com/SamuelAsherRivello/rust-project-template` as the only Rust template reference unless the user names another source.
4. Add Rust workspace structure, scripts, README rows, AGENTS rules, specs, and ignore patterns only as needed.
5. Preserve `.codex`, `.agents`, `.specify`, `.specs`, `specs`, `documentation`, and `project` as template infrastructure.
6. Prefer clear workspace boundaries, repeatable install/build/run/test scripts, and hot reload when it fits the project.
7. Verify with the Rust commands introduced by the implementation.

## Guardrails

- Do not add Bevy or Dioxus conventions from this skill.
- Do not assume the Rust project is a game unless the user asks for one.
- Do not remove Codex/Specify workflows while specializing the repo.
