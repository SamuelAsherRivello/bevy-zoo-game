# Data Model: DebugHUD

| Entity | Fields | Relationships | Validation Rules |
| ------ | ------ | ------------- | ---------------- |
| Shared Debug Runtime | `bevy/crates/shared` DebugHUD, inspector, diagnostic input capture, and tests | Composed by `bevy/crates/game` and reusable by future app surfaces | Must not contain card-specific interaction, rendering, selection, scoring, or gameplay state |
| DebugHUD Panel | `DebugHudText`, Bevy `Text`, `Node`, `BackgroundColor` | Owns child `TextSpan` key labels and optional FPS text span | One panel is spawned by default; it remains anchored near the top-left |
| Key Legend Label | `DebugHudKeyText.key_code`, `DebugHudKeyText.is_toggle`, `UnderlineColor` | Child text span of the HUD panel | Labels exist for `W`, `A`, `S`, `D`, `F`, and `I`; `WASD` labels are non-toggle hold indicators and may reflect pressed state only inside the DebugHUD |
| FPS Toggle State | `DebugHudState.is_fps_visible`, `fps_accumulated_seconds`, `fps_accumulated_frames`, `fps_display_value` | Read by `update_debug_hud`; displayed through `DebugHudFpsText` | `F` toggles visibility only; hidden FPS uses an empty text span |
| Inspector State | `InspectorState.is_visible`, `x`, `y`, `width`, `height` | Read by `inspector_ui`; toggled by `toggle_inspector`; reflected by the `I` key label | `I` toggles inspector visibility only; hidden inspector draws no egui window |
| Game Tick State | `GameTicks.0` | Read by HUD status text | Increments during updates and is displayed as frame/status text |

## State Transitions

| Input | Previous State | New State | Side Effects |
| ----- | -------------- | --------- | ------------ |
| `F` just pressed | FPS hidden | FPS visible | `DebugHudFpsText` shows sampled FPS text |
| `F` just pressed | FPS visible | FPS hidden | `DebugHudFpsText` becomes empty |
| `I` just pressed | Inspector hidden | Inspector visible | Inspector egui window can render |
| `I` just pressed | Inspector visible | Inspector hidden | Inspector egui window does not render |
| `W`, `A`, `S`, or `D` pressed | Any diagnostic state | No toggle state changes | May update DebugHUD-only hold feedback; no gameplay, movement, camera, card, selection, score, or deck behavior |
