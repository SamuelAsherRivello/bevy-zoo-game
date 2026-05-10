# Bevy Runtime Structure

Use this rule when planning, implementing, reviewing, or documenting Bevy runtime structure in this repository.

This rule imports the useful architecture guidance from `bevy-jam-1` without adopting its TitleCase folder and file convention. The current constitution remains authoritative for path casing: Rust implementation trees use typical lowercase Rust-style names unless a separate migration explicitly changes that decision.

## Source Basis

| Source | Imported Guidance |
| ---- | ---- |
| `bevy-jam-1` README | Two-crate Bevy workspace, game/shared split, runtime assets in the game asset tree, tests near the game crate, and hot-reload workflow expectations. |
| `bevy-jam-1` AGENTS.md | ECS runtime folders, feature plugins, system/component/resource naming intent, hot-reloadable systems, and keeping Spec Kit/constitution guidance authoritative. |
| This repository constitution | Lowercase Rust-style implementation paths, current `bevy/crates/...` ownership, and spec-driven changes. |

## Canonical Repo Rule Set

| Area | Rule |
| ---- | ---- |
| Crate split | Keep two active Bevy crates: `bevy/crates/game` owns the Bevy app, gameplay composition, scene-specific behavior, and game runtime assets; `bevy/crates/shared` owns reusable runtime support that is not card/gameplay-specific. |
| Runtime folders | Keep Bevy runtime source organized by ECS role under `bevy/crates/game/src/runtime/` and, when shared runtime grows, under matching lowercase shared runtime modules. |
| Components | Components hold entity-attached data. Keep component definitions in `components` modules and avoid embedding behavior that belongs in systems. |
| Resources | Resources hold shared state, configuration, handles, and app-wide runtime data. Keep resource definitions in `resources` modules. |
| Systems | Systems own runtime behavior, queries, startup work, input handling, spawning, update logic, and focused helpers. Keep scheduled systems in `systems` modules. |
| Plugins | Prefer feature plugins over direct app wiring. `main.rs` should compose runtime plugins clearly; plugin modules should own startup/update system registration for their feature area. |
| Hot reload | Hot-reloadable update systems use the repo-approved hot reload attribute, currently `#[hot]`, when the relevant feature is built for desktop hot reload. Do not require hot reload support for web-only or non-hot-reload paths. |
| Assets | Keep Bevy runtime assets under `bevy/crates/game/assets/`. Preserve existing asset conventions such as `bevy/crates/game/assets/models/...` and shader assets under `bevy/crates/game/assets/shaders/` unless a spec deliberately replaces them. |
| Tests | Keep tests aligned with feature/plugin behavior. Prefer tests near the crate or module that owns the behavior, and cover plugin composition when system wiring is part of the contract. |
| Path casing | Keep this repository's current lowercase Rust-style paths as the documented convention. Do not switch to the reference repo's `Bevy/Crates/Game/Runtime` TitleCase convention without a separate migration plan and constitution update. |

## Feature Structure Guidance

| Feature Element | Preferred Owner |
| ---- | ---- |
| Game-specific scene entities, model behavior, gameplay rules, and visual interaction | `bevy/crates/game` |
| Reusable window, camera, diagnostics, inspector, input classification, and runtime support | `bevy/crates/shared` |
| Feature startup/update wiring | Feature plugin in the owning crate |
| Entity data | Component module in the owning crate |
| Shared feature state and handles | Resource module in the owning crate |
| Behavior and scheduled functions | System module in the owning crate |

## Migration Guardrail

Do not move files or change module paths just to match the reference repository. If the project later adopts the reference repo's TitleCase convention, plan it as a separate migration covering source paths, Rust module declarations, Cargo workspace paths, scripts, README paths, active specs, and the constitution.

## Verification

| Check | Requirement |
| ---- | ---- |
| Documentation consistency | `AGENTS.md`, `.codex/rules/bevy-runtime-structure.md`, `.codex/README.md`, README, active specs, and `.specify/memory/constitution.md` must not contradict the chosen path casing or crate ownership. |
| Code migration, if any | Run `cargo test`, `scripts/other/RunTests.ps1`, and relevant desktop/web workflow checks after any future path or module migration. |
| Asset convention changes | Confirm README and active specs reference the final asset paths consistently before changing asset layout. |
