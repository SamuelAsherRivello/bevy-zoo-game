# Research: DebugHUD

| Decision | Rationale | Alternatives Considered |
| -------- | --------- | ----------------------- |
| Use Bevy UI `Text`, `TextSpan`, `Node`, and `BackgroundColor` for the HUD panel | These APIs are native to Bevy, work with ECS, and avoid adding a browser-only or desktop-only overlay dependency | Egui-only HUD was rejected because the feature requests a Bevy HUD pattern, while egui remains reserved for the inspector |
| Keep DebugHUD and diagnostic input in `bevy/crates/shared` | DebugHUD, inspector visibility, and diagnostic key capture are reusable system-level tools rather than card-specific gameplay | Keeping the DebugHUD in `bevy/crates/game` was rejected because future non-card prototypes should be able to reuse the same diagnostics |
| Store FPS visibility and sample data in `DebugHudState` | A resource makes diagnostic state explicit and easy to test without coupling it to entities | Component-only state was rejected because the FPS toggle is app-level diagnostic state |
| Store inspector visibility in an `InspectorState` component | The inspector window is an entity-like diagnostic surface with position and size settings | A global bool was rejected because it would mix inspector window metadata with unrelated HUD state |
| Treat `WASD` as non-toggle DebugHUD input indicators | The copied HUD pattern may show key hold state, but that state is diagnostic-only and must not drive card, camera, gameplay, selection, or toggles | Gameplay input handling was rejected because it would exceed the DebugHUD scope |
| Use `bevy-inspector-egui` for the inspector | The dependency is already in the workspace and provides entity inspection without building custom inspector UI | A custom inspector was rejected as unnecessary scope |
| Use `scripts/other/RunTests.ps1` for test execution | The repository requires repeatable scripts under `scripts/`; this script runs the full workspace test suite | Ad hoc `cargo test` only was rejected because the feature explicitly requires a RunTests script |
