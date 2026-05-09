# Project Memory

## Repository Conventions

| Topic | Decision |
| ----- | -------- |
| Purpose | Bevy ECS card game built from the Codex Project Template. |
| Root docs | Keep Codex and Specify guidance visible while documenting Bevy game conventions. |
| Scripts | Put repeatable project commands in root `scripts`. |
| Assets | Put runtime assets under `bevy/crates/game/assets`. |
| Specs | Put active feature specs in `specs`. |
| Images | Keep README images in `documentation/images`. |

## Bevy Stack

| Topic | Decision |
| ----- | -------- |
| Workspace | Rust workspace at the repository root. |
| Game crate | `bevy/crates/game` with package name `bevy-card-game`. |
| Shared crate | `bevy/crates/shared` for reusable non-Bevy game constants and logic. |
| ECS layout | Keep components, resources, systems, and plugins under `bevy/crates/game/src/runtime`. |
| Verification | Use `scripts/main/InstallDependencies.ps1` once per machine, then `scripts/main/RunTests.ps1`, `scripts/main/RunAppDesktop.ps1`, and `scripts/other/StopApp.ps1`. |
| Desktop warm builds | `RunAppDesktop.ps1` uses a dedicated `target/run-app-desktop` cache and enables the `fast-dev` feature for Bevy dynamic linking on non-release runs. |
| Dependency install | `InstallDependencies.ps1` warms the `target/run-app-desktop` cache with `cargo build -p bevy-card-game --features fast-dev`. |

## Notes

- Do not store secrets, credentials, private keys, tokens, or personal data in memory files.
- Add durable project decisions here only when they help future agents avoid rediscovery.

## 2026-05-09 - DebugHUD Key Grouping
Type: Convention
Scope: repo
Note: DebugHUD key labels are grouped by toggle behavior using the existing format. Non-toggle keys appear at the end of the first key line, e.g. add non-toggle `T` after `WASD, R` without extra descriptive words. Toggle keys appear on the second key line. Do not add labels such as `THEME` or other explanatory words to HUD key lines unless explicitly requested.
Source: user

## 2026-05-09 - GLB Model Loading And Texturing
Type: Lesson
Scope: Bevy runtime
Note: For models that should render with their authored materials and textures, prefer the model-browser pattern in `Bevy/Crates/Game/src/runtime/systems/mod.rs`: load `GltfAssetLabel::Scene(0).from_asset(model_path)` through `AssetServer` and spawn it as `SceneRoot(scene_handle)`. This lets Bevy's GLTF loader preserve the scene hierarchy, material assignments, UVs, and embedded GLB texture/material data. Use asset paths relative to `Bevy/Crates/Game/assets`, such as `Models/kenney_cube-pets_1.0/Models/GLB format/animal-polar.glb`.
Detail: Avoid the custom `glb_mesh::mesh_from_glb` path when visual fidelity is the goal. That path extracts only the first mesh primitive into a single `Mesh` and requires a manually supplied `StandardMaterial` and optional PNG colormap, so it can lose authored multi-part scene data and material setup. It is acceptable for tightly controlled procedural-style scene pieces or tests, but not as the default for properly textured display models.
Source: user/modelbrowser implementation

## 2026-05-09 - Lazy Runtime Asset Loading
Type: Convention
Scope: Bevy runtime
Note: Runtime assets and models load lazily by default. Do not wait for an asset/model or show a preloader unless the behavior technically requires the asset before continuing. GameScene model entities should spawn `SceneRoot` handles immediately; Model Browser should request loads in grid order starting from the upper-left cell and allow async completion to pop models in as they finish.
Source: user
