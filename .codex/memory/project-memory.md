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
Note: DebugHUD key labels are grouped by toggle behavior. Non-toggle keys appear on the first key line exactly as `KEYS: WASD, R`; toggle keys appear on the second key line exactly as `KEYS: F, I, H`. `H` is the hot-reload auto-restart toggle. Update these grouped lines and related tests/specs whenever DebugHUD keys change.
Source: user
