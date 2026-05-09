# Research: Camera Setup

| Decision | Rationale | Alternatives Considered |
| ---- | ---- | ---- |
| Use Bevy's built-in `Camera3d` component for the primary scene camera | It is the standard Bevy 0.18 path for rendering 3D scene content and works for desktop and browser WebGPU targets | A custom render camera abstraction was unnecessary before there is camera movement, multiple scenes, or gameplay-specific camera behavior |
| Keep camera setup in `bevy/crates/shared` | Camera defaults, the marker component, fixed input behavior, and startup setup are reusable system-level rendering foundations | Keeping camera setup in `bevy/crates/game` was rejected because card-specific scene code should consume the shared camera rather than own it |
| Store camera defaults in a `PrimaryCameraDefaults` resource | Tests and later features can inspect one source of truth for position, target, FOV, clipping range, and clear color | Hard-coding values inside the spawn system would satisfy launch behavior but make defaults harder to document and verify |
| Spawn the camera once through shared runtime startup wiring | Shared startup wiring lets the game crate compose the camera without making the camera card-specific | A game-only camera setup was rejected because camera setup is useful beyond the card POC |
| Keep the camera fixed and input-free | The feature explicitly excludes orbit, pan, zoom, and gameplay input, and later card inspection should own interaction behavior | Adding camera controls now would make DebugHUD and card inspection tests less predictable |
| Use neutral dark gray `ClearColor` as an app resource | Clear color is a global Bevy render setting and satisfies the background requirement without scene geometry or assets | Adding a background mesh or skybox would exceed the feature scope |
