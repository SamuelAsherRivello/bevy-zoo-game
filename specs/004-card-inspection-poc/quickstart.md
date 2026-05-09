# Quickstart: Card Inspection POC

| Scenario | Command / Action | Expected Result |
| ---- | ---- | ---- |
| Automated tests | `scripts/other/RunTests.ps1` | Workspace tests pass, including card count, proportions, pointer target, smoothing, and camera stability checks |
| Windows desktop run | `scripts/main/RunAppDesktop.ps1` | App opens directly to one centered white poker-proportion card with the DebugHUD visible by default |
| Lazy model loading | Toggle from GameScene to Model Browser with `B`, then back with `B` | Neither scene shows a preloader; GameScene model handles are spawned without waiting, and Model Browser requests loads from the upper-left grid cell onward while models may appear as each async load completes |
| Pointer center | Move cursor near the window center | Card eases toward neutral front-facing orientation |
| Pointer corners | Move cursor to each window corner | Card smoothly tilts toward the matching visible area and remains centered and visible |
| HUD scope | Observe the screen while interacting | Only the approved DebugHUD is visible; no menu, gameplay prompt, score, card text, art, or extra cards appear |
| Browser WebGPU | Build/run the Bevy web target using the project web workflow when available | Same one-card POC behavior works in a WebGPU-capable browser; document exact blocker if the local toolchain cannot run this target |
