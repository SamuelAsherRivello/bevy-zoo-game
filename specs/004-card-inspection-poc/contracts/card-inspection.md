# Contract: Card Inspection POC

| Requirement | Contract |
| ---- | ---- |
| Source ownership | Card placeholder geometry, card defaults, pointer mapping, and smoothing live under `bevy/crates/game`; reusable window, camera, DebugHUD, inspector, and diagnostic input systems remain under `bevy/crates/shared` |
| Card count | After startup, exactly one entity has `CardPlaceholder` |
| Card name | The card entity is named `Poker Card Placeholder` |
| Card dimensions | The card uses width `0.063`, height `0.088`, and slight positive thickness |
| Card appearance | The card material is plain white with no texture asset, text, bevel, or card art |
| Card placement | The card remains centered at world origin during rotation |
| Initial orientation | The card starts in a neutral orientation facing the fixed primary camera |
| Pointer mapping | Center cursor maps to neutral; upper-right maps to positive target tilt for the upper-right frustum direction; other edges and corners map correspondingly |
| Tilt limit | Target pitch and yaw are clamped to 20 degrees from neutral |
| Smoothing | Runtime rotation moves toward the target over time and is configured for a 100 ms response target |
| Camera stability | Pointer input never mutates the `PrimarySceneCamera` transform |
| HUD exception | The approved DebugHUD remains visible by default and is the only HUD exception |

## Verification

| Check | Expected Result |
| ---- | ---- |
| Startup query | One card, one primary camera, one DebugHUD, and one inspector state exist |
| Ratio test | `height / width` equals `88 / 63` within 2% |
| Pointer target test | Corner inputs produce clamped target rotations at or below 20 degrees |
| Smoothing test | Card rotation changes toward, but does not instantly snap to, the target |
| Camera stability test | Primary camera transform is unchanged after pointer input and an update |
