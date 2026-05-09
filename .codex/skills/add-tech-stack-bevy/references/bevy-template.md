# Bevy Template Reference

Reference repo: `https://github.com/SamuelAsherRivello/bevy-project-template`

## Signals To Import

| Area | Reference Signal |
| ---- | ---------------- |
| Root purpose | Starting point for Bevy projects with hot reload and Rust coding standards. |
| Source layout | `bevy/crates/game` plus `bevy/crates/shared`. |
| Runtime folders | `Runtime/Components`, `Runtime/Resources`, `Runtime/Systems`, `Runtime/Plugins`, `Runtime/Shaders`. |
| Assets | `assets/audio`, `assets/shaders`, and game-facing asset folders under the game crate. |
| Scripts | `Scripts/Common/InstallDependencies.ps1`, `RunGameWithHotReload.ps1`, plus build/run/test/stop scripts under `Scripts/Other`. |
| Tooling | Workspace manifest, toolchain file, editor task settings, and hot reload support. |

## Adaptation Rules

- Keep the ECS split when adding Bevy code: components hold data, systems own behavior, plugins group features, resources hold shared state.
- Put reusable game assets in the Bevy asset tree once Bevy is selected.
- Keep Codex/Specify infrastructure at the root; Bevy is the implementation stack, not the whole repo identity unless requested.
- If adding scripts, use this repository's root `scripts` convention unless intentionally adopting the reference repo's `Scripts/Common` and `Scripts/Other` split.
