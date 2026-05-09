# Data Model: Camera Setup

| Entity | Fields | Validation Rules | Relationships |
| ---- | ---- | ---- | ---- |
| Shared Camera Runtime | `bevy/crates/shared` camera marker, defaults, setup system, and tests | Must not contain card geometry, pointer-driven card rotation, or gameplay state | Composed by `bevy/crates/game` and reusable by future app surfaces |
| Primary 3D Camera | `Name`, `PrimarySceneCamera`, `Camera3d`, `Projection`, `Transform` | Exactly one primary scene camera exists after startup; transform uses the defaults resource; projection is perspective | Spawned by shared runtime startup systems and used by future scene content |
| Camera Defaults | `position`, `target`, `fov_radians`, `near`, `far`, `clear_color` | Position defaults to `(0, 0, 5)`; target defaults to origin; `near > 0`; `far > near`; FOV is typical perspective range | Stored as `PrimaryCameraDefaults`; applied by camera setup and tests |
| Overlay Compatibility | DebugHUD UI entities and egui inspector remain separate from the 3D camera | Camera setup does not despawn or replace overlay UI entities and does not consume overlay inputs | Verified by existing DebugHUD tests and combined camera/HUD startup checks |

## State Transitions

| State | Trigger | Result |
| ---- | ---- | ---- |
| App startup without camera | `setup_primary_camera` runs | One primary scene camera is spawned with documented defaults |
| App receives input | Keyboard, mouse, or controller input | Primary scene camera transform remains unchanged in this feature |
| Window resizes | Bevy window/render pipeline updates projection internally | Primary scene camera remains present and usable |
