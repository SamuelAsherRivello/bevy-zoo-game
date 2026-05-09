# Implementation Plan: Card Inspection POC

**Branch**: `004-card-inspection-poc` | **Date**: 2026-05-09 | **Spec**: `specs/004-card-inspection-poc/spec.md`
**Input**: Feature specification from `specs/004-card-inspection-poc/spec.md`

## Summary

Add the V0.1 card inspection prototype to the existing Bevy runtime: one centered poker-proportion white thin slab, a fixed 3D camera, the approved DebugHUD visible by default, and smooth pointer-driven card tilt capped at 20 degrees per axis. The implementation stays in the existing ECS modules with source-defined card defaults, Bevy mesh/material resources, and tests that verify startup, proportions, input mapping, smoothing, and camera stability.

## Technical Context

| Item | Decision |
| ---- | -------- |
| Language/Version | Rust 2024 workspace |
| Primary Dependencies | Bevy 0.18.1, bevy-inspector-egui 0.36.0 |
| Storage | N/A; card defaults and input state are runtime resources only |
| Testing | `scripts/other/RunTests.ps1` runs `cargo test --workspace` |
| Target Platform | Windows desktop and browser WebGPU |
| Project Type | Bevy ECS desktop/browser game prototype |
| Performance Goals | One card mesh and material; pointer target updated from cursor data; smoothing reaches target orientation within 100 ms |
| Constraints | Keep card-specific implementation under `bevy/crates/game/src/runtime/`; consume reusable window, camera, DebugHUD, inspector, and diagnostic input behavior from `bevy/crates/shared`; keep exactly one card; preserve fixed camera; no gameplay, textures, bevels, rich materials, menus, or HUD beyond DebugHUD |
| Scale/Scope | V0.1 only: one centered white thin slab with pointer-responsive rotation |

## Constitution Check

| Gate | Status | Notes |
| ---- | ------ | ----- |
| Active spec and repo guidance followed | ✅ | Implements `004-card-inspection-poc` and keeps `003-debug-hud` visible by default |
| Source, assets, scripts, docs, and tests stay in approved locations | ✅ | Card-specific runtime code stays under `bevy/crates/game/src/runtime/`; reusable system-level runtime behavior remains in `bevy/crates/shared`; no new runtime assets are needed |
| Visible feedback requirements respected | ✅ | DebugHUD remains the only HUD and is visible by default; no async loading flow is added |
| Local state remains explicit | ✅ | No persistence is added beyond existing window placement behavior |
| Data changes are explicit | ✅ | No database, migration, or seed data changes |
| Real behavior verification path | ✅ | Unit tests plus desktop and browser WebGPU quickstart checks are documented |
| Rust and Bevy ECS standards | ✅ | Card, pointer, and camera behavior use components, resources, systems, and plugin wiring |
| Windows desktop and browser WebGPU parity | ✅ | Uses Bevy mesh, material, input, transform, and camera APIs shared by both targets |

## Project Structure

### Documentation (this feature)

```text
specs/004-card-inspection-poc/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── card-inspection.md
└── tasks.md
```

### Source Code (repository root)

```text
bevy/crates/game/src/runtime/
├── components/mod.rs    # Card placeholder and camera marker components
├── plugins/mod.rs       # Startup/update system wiring and automated tests
├── resources/mod.rs     # Card dimensions, smoothing, and pointer target resources
└── systems/mod.rs       # Card spawn, pointer tracking, smoothing, and existing HUD/camera systems

scripts/
├── main/
│   ├── InstallDependencies.ps1
│   └── RunAppDesktop.ps1
└── other/
    ├── RunTests.ps1
    └── StopApp.ps1
```

**Structure Decision**: Extend the game crate only for card-specific runtime ECS behavior. Reuse shared window, camera, DebugHUD, inspector, and diagnostic input systems from `bevy/crates/shared`. Do not add a separate card plugin, rendering asset pipeline, shader file, gameplay module, or UI framework for V0.1.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
| --------- | ---------- | ------------------------------------ |
| N/A | N/A | N/A |

## Phase 0 Research

See `specs/004-card-inspection-poc/research.md`.

## Phase 1 Design

See `specs/004-card-inspection-poc/data-model.md`, `specs/004-card-inspection-poc/contracts/card-inspection.md`, and `specs/004-card-inspection-poc/quickstart.md`.

## Post-Design Constitution Check

| Gate | Status | Notes |
| ---- | ------ | ----- |
| No unresolved clarifications | ✅ | Tilt, smoothing, geometry, camera, HUD relationship, and target platforms are clarified |
| Target parity documented | ✅ | Desktop and browser WebGPU checks are included in quickstart |
| Scope remains V0.1 | ✅ | Rendering is limited to a plain white thin slab with no gameplay or content systems |
