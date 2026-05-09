# Data Model: DebugHUD

| Entity | Fields | Relationships | Validation Rules |
| ------ | ------ | ------------- | ---------------- |
| Shared Debug Runtime | `bevy/crates/shared` DebugHUD, inspector, diagnostic input capture, and tests | Composed by `bevy/crates/game` and reusable by future app surfaces | Must not contain card-specific interaction, rendering, selection, scoring, or gameplay state |
| DebugHUD Panel | `DebugHudText`, Bevy `Text`, `Node`, `BackgroundColor` | Owns child `TextSpan` key labels and optional FPS text span | One panel is spawned by default; it remains anchored near the top-left |
| Key Legend Label | `DebugHudKeyText.key_code`, `DebugHudKeyText.is_toggle`, `UnderlineColor` | Child text span of the HUD panel | Non-toggle labels are shown on the first key line exactly as `KEYS: WASD, R`; toggle labels are shown on the second key line exactly as `KEYS: F, I, H` |
| FPS Toggle State | `DebugHudState.is_fps_visible`, `fps_accumulated_seconds`, `fps_accumulated_frames`, `fps_display_value` | Read by `update_debug_hud`; displayed through `DebugHudFpsText` | `F` toggles visibility only; hidden FPS uses an empty text span |
| Inspector State | `InspectorState.is_visible`, `x`, `y`, `width`, `height` | Read by `inspector_ui`; toggled by `toggle_inspector`; reflected by the `I` key label | `I` toggles inspector visibility only; hidden inspector draws no egui window |
| Hot Reload Auto-Restart State | `DebugHudState.is_hot_reload_autorestart_enabled` | Read by `update_debug_hud` and hot-reload restart systems; reflected by the `H` key label | `H` toggles hot-reload auto-restart |
| DebugHUD Input Persistence | `DebugHudInputStore.is_fps_visible`, `is_inspector_visible`, `is_hot_reload_autorestart_enabled` | Loaded into DebugHUD state at startup and written with `bevy-persistent` when `F`, `I`, or `H` toggles | First-run defaults for all persisted toggles are `false`; state is stored under ignored local storage |
| Game Tick State | `GameTicks.0` | Read by HUD status text | Increments during updates and is displayed as frame/status text |

## State Transitions

| Input | Previous State | New State | Side Effects |
| ----- | -------------- | --------- | ------------ |
| `F` just pressed | FPS hidden | FPS visible | `DebugHudFpsText` shows sampled FPS text |
| `F` just pressed | FPS visible | FPS hidden | `DebugHudFpsText` becomes empty |
| `I` just pressed | Inspector hidden | Inspector visible | Inspector egui window can render |
| `I` just pressed | Inspector visible | Inspector hidden | Inspector egui window does not render |
| `H` just pressed | Hot-reload auto-restart disabled | Hot-reload auto-restart enabled | Future hot reload patches may reload the app scene content |
| `H` just pressed | Hot-reload auto-restart enabled | Hot-reload auto-restart disabled | Future hot reload patches do not auto-reload the app scene content |
| `F`, `I`, or `H` just pressed | Any persisted toggle state | Current toggle state | Writes the updated toggle state to local persistent storage |
| `W`, `A`, `S`, or `D` pressed | Any diagnostic state | No toggle state changes | May update DebugHUD-only hold feedback; no gameplay, movement, camera, card, selection, score, or deck behavior |
| `R` just pressed | Current zoo scene loaded | Fresh zoo scene loaded | Restarts app scene content by reloading the camera, lights, and models; `R` remains a non-toggle hold indicator |
