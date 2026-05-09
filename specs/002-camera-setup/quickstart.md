# Quickstart: Camera Setup

| Step | Command | Expected Result |
| ---- | ---- | ---- |
| Run tests | `scripts/other/RunTests.ps1` | Workspace tests pass, including camera startup/default/input stability checks |
| Launch desktop | `scripts/main/RunAppDesktop.ps1` | App opens with the existing DebugHUD overlay and a neutral dark gray scene background |
| Browser WebGPU check | Serve/build with the repository browser workflow when available | App starts without desktop-only camera dependencies; document any browser blocker exactly |

## Verification Notes

| Date | Check | Result |
| ---- | ---- | ---- |
| 2026-05-09 | `scripts/other/RunTests.ps1` | ✅ Passed: 9 tests, including primary camera count, defaults, and input stability |
| 2026-05-09 | Browser WebGPU | ❌ Blocked: this repository currently has no browser serve/build script to execute |

## Manual Acceptance Checks

| Scenario | Expected Result |
| ---- | ---- |
| Startup | Exactly one primary 3D camera is active for the primary scene |
| Future origin content | The default viewpoint is positioned to frame origin-centered content |
| Input | WASD, mouse movement, and DebugHUD toggles do not move the camera |
| Overlay | DebugHUD and inspector overlay behavior remains visible and independent of the 3D camera |
