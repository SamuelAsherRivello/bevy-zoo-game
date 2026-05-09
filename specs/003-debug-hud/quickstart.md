# Quickstart: DebugHUD

| Goal | Command |
| ---- | ------- |
| Install dependencies once | `scripts/main/InstallDependencies.ps1` |
| Run automated tests | `scripts/other/RunTests.ps1` |
| Run Windows desktop app | `scripts/main/RunAppDesktop.ps1` |
| Check Windows desktop compile without launch | `scripts/main/RunAppDesktop.ps1 -CheckOnly` |
| Run Windows desktop app in release mode | `scripts/main/RunAppDesktop.ps1 -Release` |
| Run Windows desktop app with explicit Windows target cache | `scripts/main/RunAppDesktop.ps1 -TargetTriple x86_64-pc-windows-msvc` |
| Stop desktop app and project build processes | `scripts/other/StopApp.ps1` |

## Manual Acceptance

| Check | Expected Result |
| ----- | --------------- |
| Launch desktop app | A translucent DebugHUD panel appears near the top-left |
| Read HUD contents | The panel shows title/status text, first key line `KEYS: WASD, R`, and second key line `KEYS: F, I, H` |
| Press `F` repeatedly | FPS text toggles on each press; inspector visibility does not change |
| Press `I` repeatedly | Inspector visibility toggles on each press; FPS visibility does not change |
| Press `H` repeatedly | Hot-reload auto-restart toggles on each press |
| Press `W`, `A`, `S`, `D` | DebugHUD-only hold feedback may update; no gameplay, camera, card, or diagnostic toggle changes |
| Press `R` | App scene content reloads; `R` remains a non-toggle key |
| Resize the window | HUD remains anchored and scales consistently |

## Browser WebGPU Verification

| Status | Notes |
| ------ | ----- |
| Required before final cross-target acceptance | Use the project browser/WebGPU workflow once available in scripts or CI; if unavailable, record the exact missing toolchain or script blocker in the feature verification notes |
