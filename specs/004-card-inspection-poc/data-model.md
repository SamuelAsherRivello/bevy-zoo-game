# Data Model: Card Inspection POC

| Entity | Fields | Validation Rules | Lifecycle |
| ---- | ---- | ---- | ---- |
| Game Card Runtime | `bevy/crates/game` card components, resources, systems, plugin wiring, and tests | Must contain only card-specific POC behavior and may consume shared window, camera, DebugHUD, inspector, and diagnostic input systems | Composed with shared runtime systems during app startup |
| Card Placeholder | `width = 0.063`, `height = 0.088`, `thickness`, `max_tilt_radians`, centered transform, white material | Height-to-width ratio must match `88:63` within 2%; maximum tilt must not exceed 20 degrees per axis; exactly one card exists after startup | Spawned once during startup; remains centered; rotation updates during runtime |
| Pointer Position | Last valid normalized pointer coordinates in `[-1, 1]` for X and Y | Values are clamped; missing cursor data keeps the last valid target | Starts at neutral center; updates while cursor is inside the primary window |
| Card Inspection State | Target rotation and smoothing response seconds | Target rotation is derived only from pointer position; smoothing target is 100 ms | Initialized neutral; updated before card transform smoothing each frame |
| Fixed Camera | Existing `PrimarySceneCamera`, transform, perspective projection, clear color | Camera transform must remain equal to `PrimaryCameraDefaults::transform()` during pointer input | Spawned by existing camera setup; not modified by card inspection systems |
| POC Version Scope | V0.1 limits for rendering and interaction | No gameplay, multiple cards, textures, text, bevels, card backs, scoring, dragging, or deck behavior | Used as implementation boundary for this feature |
