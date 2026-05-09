# Data Model: Project Setup

| Entity | Fields | Relationships | Validation Rules |
| ------ | ------ | ------------- | ---------------- |
| Desktop Window Placement | `window_position`, `window_size`, `monitor_name`, `monitor_position`, `monitor_size`, `relative_position` | Saved on normal close; loaded on next desktop startup | Size must be positive; restored placement must be visible on an available screen |
| Screen Identity | `monitor_name`, `monitor_position`, `monitor_size` | Associated with the saved desktop window placement | Matching prefers same named screen and size, then same named screen, then same position |
| Local Runtime State | Placement JSON file under `data/local_storage/` | Owned by the local machine and not committed | Missing, unreadable, or invalid data is ignored |
| Shared Window Runtime | Window defaults, placement state, and restore/save systems under `bevy/crates/shared` | Composed by the game app and reusable by future app shells | Must not contain card-specific rendering, interaction, or gameplay state |
| Project Script | Dependency-check, headless compile, test, desktop run, desktop hot reload, web run, and stop entry points under root `scripts` | Used by VS Code tasks and manual terminal workflows | Must run from repository root |
| Compile Workflow | `action`, `package/workspace scope`, `target_dir`, `target_triple`, `features`, `release`, linker/cache options | Shared by desktop, web, tests, and dependency warmup scripts | Must print the Cargo command and fail on non-zero Cargo exit |
| Desktop Hot Reload Workflow | `dioxus_cli_version`, `target_dir`, `package`, `binary`, `env`, optional extra `dx` args | Runs the desktop package through Dioxus CLI hot patching | Must verify compatible `dx`, keep output in the calling terminal, and avoid Windows dynamic-linking conflicts by default |
| VS Code Task | Build, test, desktop run, and desktop hot reload task definitions | Calls repository scripts with integrated terminal presentation | Must reveal output in VS Code terminal |

## State Transitions

| Event | Previous State | New State | Side Effects |
| ----- | -------------- | --------- | ------------ |
| First launch without saved placement | No placement | 1024x768 centered primary window | No placement file required |
| Move or resize while running | In-memory placement unknown or stale | In-memory placement updated | No file write |
| Normal close | In-memory/current placement available | Placement persisted locally | Writes ignored JSON under `data/local_storage/` |
| Relaunch with valid placement | Placement file available | Exact screen, x/y, and size restored | Window opens where reviewer left it |
| Relaunch with invalid/off-screen placement | Placement file invalid or unavailable | 1024x768 centered primary fallback | Invalid placement is ignored |
| Start desktop hot reload | No hot reload process | Dioxus CLI hot-patch session running | Output remains in the terminal until Ctrl+C or stop workflow |
