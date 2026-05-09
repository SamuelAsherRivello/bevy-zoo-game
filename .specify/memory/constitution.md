<!--
Sync Impact Report
Version change: 1.2.0 -> 1.3.0
Modified principles:
- II. Clear Project Boundaries: updated Bevy paths to lowercase Rust-style layout.
- VII. Implementation Standards: expanded to require typical Rust file and folder naming.
Added sections: None
Removed sections: None
Templates requiring updates:
- ✅ .specify/templates/plan-template.md
- ✅ .specify/templates/tasks-template.md
- ✅ .specify/templates/spec-template.md
Follow-up TODOs: None
-->

# Bevy Card Game Constitution

## Core Principles

### I. Project Intent Is The Contract
All implementation work MUST follow the active specification, the project constitution, and the repo-local agent guidance. Bevy-specific rules belong in project specs, README guidance, and repo-local Codex memory.

### II. Clear Project Boundaries
Project behavior belongs in the Bevy source tree under `bevy/crates`. Runtime assets belong under `bevy/crates/game/assets`; repeatable scripts belong under `scripts`.

### III. Visible User Feedback
User-facing workflows SHOULD provide visible loading, progress, success, and failure feedback when work is asynchronous or can fail.

### IV. Local State Stays Explicit
Local files, caches, generated outputs, and user preferences MUST be documented and kept out of version control unless a spec explicitly makes them source artifacts.

### V. Data Changes Are Explicit
Schema changes, migrations, seed data, and destructive data operations MUST be explicit in specs and plans. Destructive database operations are not performed by agents.

### VI. Verify Real Behavior
User-visible changes SHOULD be verified through the real target workflow when practical. Prefer repository scripts in `scripts` after a generated project defines them.

### VII. Implementation Standards
All implementation code MUST follow Rust and Bevy conventions, including explicit ECS ownership through components, resources, systems, and plugins.

All folders and files under Rust implementation trees MUST use typical Rust project conventions: lowercase `snake_case` module, crate, source, test, and asset directory names, with `Cargo.toml` and Rust-required filenames kept in their standard forms.

Specs and plans SHOULD record any framework-specific constraints before implementation begins.

Generated projects SHOULD keep implementation, tests, documentation, scripts, and assets in clearly named locations.

### VIII. Windows Desktop And Browser WebGPU Parity
The project and all user-visible capabilities MUST work on Windows desktop and in a browser through WebGPU unless a spec explicitly narrows the target for a temporary experiment. Rendering features, shaders, assets, UI overlays, input behavior, diagnostics, and tests SHOULD be designed with both targets in mind before implementation starts.

Plans and tasks MUST call out any target-specific risk, fallback, or verification gap for Windows desktop or browser WebGPU. A feature is not complete until the relevant desktop and browser workflows are both verified, or the unverified target is explicitly documented as blocked with the reason.

## Project Constraints

- Keep project-specific source layout documented in `README.md` and active specs.
- Keep runtime assets under `bevy/crates/game/assets`.
- Keep Rust workspace folders and files in typical Rust naming conventions, including lowercase crate and module directories.
- Preserve the Codex and Specify workflow files unless a generated project intentionally replaces them.
- Keep `documentation/` updated when README images or supporting docs change.
- Keep `documentation/images/Overview01.png` and `documentation/images/Workflow01.png` as replaceable README image slots.
- Do not introduce unrelated refactors while implementing a feature spec.
- Do not introduce rendering, shader, asset, UI, or input capabilities that only work on one target unless the active spec documents the temporary limitation and follow-up path.

## Development Workflow

1. Inspect current files before editing and keep changes scoped to the requested behavior.
2. Prefer repository scripts under `scripts` over ad hoc commands for setup, web serving, desktop serving, and tests.
3. For framework or dependency guidance, use official documentation and the project-local Codex rules before changing code.
4. If a port or build artifact is stale or locked, diagnose the actual process or path instead of assuming a clean environment.
5. Treat build output, dependency caches, runtime data, and test output as generated artifacts unless a spec explicitly says otherwise.
6. Verify Windows desktop and browser WebGPU workflows for user-visible features when practical; if one target cannot be verified, record the exact blocker.

## Governance

This constitution applies to all future Spec Kit specifications, plans, and task lists for this repository. Specs may add narrower acceptance criteria, but they must not contradict these principles without explicitly updating this constitution and documenting the reason.

**Version**: 1.3.0 | **Ratified**: 2026-04-30 | **Last Amended**: 2026-05-09
