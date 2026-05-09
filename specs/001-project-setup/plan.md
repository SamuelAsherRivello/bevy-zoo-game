# Implementation Plan: Project Setup

**Branch**: `001-project-setup` | **Date**: 2026-05-09 | **Spec**: `specs/001-project-setup/spec.md`
**Input**: Feature specification from `specs/001-project-setup/spec.md`

## Summary

Provide repeatable dependency setup, test, desktop run, desktop hot reload, web run, and stop entry points; support VS Code task execution in the integrated terminal; default the desktop app to a 1024x768 window; and persist desktop window size, x/y position, and screen identity as ignored local runtime state that restores on the next desktop launch.

## Technical Context

| Item | Decision |
| ---- | -------- |
| Language/Version | Rust 2024 workspace |
| Primary Dependencies | Bevy 0.18.1; serde and serde_json for local placement state |
| Hot Reload Tooling | Dioxus CLI 0.7.x hot patching, inspired by `bevy-jam-1` |
| Storage | Ignored local JSON under `data/local_storage/window-placement.json` |
| Testing | `scripts/other/RunTests.ps1` runs `cargo test --workspace` with the shared desktop target cache |
| Target Platform | Windows desktop primary for placement; browser WebGPU runs through `scripts/main/RunAppWeb.ps1` without desktop placement dependency |
| Project Type | Bevy desktop/browser game prototype |
| Performance Goals | Placement load/save is tiny local file IO only at startup and normal close |
| Constraints | Keep reusable window setup and placement behavior under `bevy/crates/shared`; keep scripts under `scripts`; keep generated build/web/hot-reload output under ignored target/generated paths; keep local runtime persistence under ignored `data/local_storage/`; do not add card, DebugHUD, or gameplay behavior for this feature |
| Scale/Scope | One primary desktop window, two-screen restore support, repository scripts for desktop, desktop hot reload, and web workflows, VS Code tasks |

## Constitution Check

| Gate | Status | Notes |
| ---- | ------ | ----- |
| Active spec and repo guidance followed | ✅ | Implements `001-project-setup` behavior only |
| Source, scripts, and state stay in approved locations | ✅ | Reusable window/runtime code under `bevy/crates/shared`; game app wiring under `bevy/crates/game`; scripts under `scripts`; local state under ignored `data/local_storage/` |
| Visible feedback requirements respected | ✅ | Script output remains visible in terminal workflows, including hot reload |
| Browser/local storage constraints | ✅ | Desktop placement state is file-based and does not introduce browser persistence |
| Real behavior verification path | ✅ | Dependency, test, desktop, web, and stop scripts are documented with check-only paths where available |
| Rust and Bevy ECS standards | ✅ | Placement tracking is implemented as resources and systems |
| Target parity risk documented | ✅ | Desktop placement applies only to desktop; browser startup is verified through the web runner |

## Project Structure

```text
bevy/crates/shared/src/
└── runtime/
    ├── window.rs                    # Window default and placement contracts
    ├── resources.rs                 # Window placement state and file IO
    └── systems.rs                   # Window restore, tracking, and close-save systems

bevy/crates/game/src/
└── main.rs                          # Composes shared window/runtime setup into the game app

scripts/
├── main/
│   ├── InstallDependencies.ps1
│   ├── RunAppDesktop.ps1
│   ├── RunAppDesktopHotReload.ps1
│   └── RunAppWeb.ps1
└── other/
    ├── CompileApp.ps1
    ├── RunTests.ps1
    └── StopApp.ps1

.vscode/
└── tasks.json                      # Local VS Code task entries
```

**Structure Decision**: Keep placement behavior in `bevy/crates/shared` because window management is system-level reusable runtime behavior. `bevy/crates/game` should only compose the shared setup into the game app. Add hot reload as a repository script/tooling workflow rather than replacing the normal desktop run path. Do not introduce app-wide settings screens, card rendering, DebugHUD behavior, gameplay behavior, or a broader configuration framework.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
| --------- | ---------- | ------------------------------------ |
| N/A | N/A | N/A |

## Phase 0 Research

See `specs/001-project-setup/research.md`.

## Hot Reload Design

| Decision | Detail |
| -------- | ------ |
| Entry point | `scripts/main/RunAppDesktopHotReload.ps1` |
| Tool | Dioxus CLI 0.7.x with hot patch support |
| Package/bin | `bevy-zoo-game` package and default desktop binary |
| Target cache | `target/run-app-desktop-hot-reload` |
| Environment | `CARGO_INCREMENTAL=1`, `WGPU_BACKEND=dx12`, `BEVY_ASSET_ROOT=<repo root>` |
| Windows compatibility | Do not enable the `fast-dev` Bevy dynamic-linking feature by default during hot patching |
| Stop behavior | `scripts/other/StopApp.ps1` includes project-local `dx.exe` processes |

## Phase 1 Design

See `specs/001-project-setup/data-model.md`, `specs/001-project-setup/contracts/window-placement.md`, and `specs/001-project-setup/quickstart.md`.

## Post-Design Constitution Check

| Gate | Status | Notes |
| ---- | ------ | ----- |
| No unresolved clarifications | ✅ | Restore, fallback, and save timing are clarified |
| Local state remains explicit | ✅ | Placement state path and ignored status are documented |
| Verification documented | ✅ | Dependency checks, unit tests, desktop checks, desktop hot reload checks, web checks, localhost serving, and cleanup checks are listed |
