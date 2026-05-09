# Feature Specification: Project Setup

**Feature Branch**: `001-project-setup`  
**Created**: 2026-05-09  
**Status**: Draft  
**Input**: User description: "Basic setup includes a strong repeatable scripts workflow for dependency setup, tests, desktop runs, desktop hot reload runs, web runs, cleanup, VS Code task support, a 1024x768 desktop window, and remembering the window's last size, x/y position, and screen so the app reopens where the reviewer left it."

## Clarifications

### Session 2026-05-09

- Q: How should saved desktop window placement restore across launches? -> A: Restore the exact x/y and size from wherever the app was last closed on any connected monitor, including two-screen setups; fall back only if that saved monitor is unavailable or the saved position would open fully off-screen.
- Q: What fallback should the app use when saved placement is invalid or off-screen? -> A: Open centered on the primary screen at 1024x768.
- Q: When should desktop window placement be saved? -> A: Save placement only when the window closes normally.
- Q: Where should reusable window setup and placement behavior live? -> A: Window setup, placement state, and related system-level runtime behavior belong in `bevy/crates/shared`; `bevy/crates/game` should only wire that shared behavior into the game app.
- Q: Where should agents update the approved desktop default size? -> A: Update `DEFAULT_WINDOW_WIDTH` and `DEFAULT_WINDOW_HEIGHT` in `bevy/crates/shared/src/window.rs`, then update this spec when the approved size changes.
- Q: Should desktop placement persistence remember both window position and size? -> A: Yes. The app must save both the closed x/y position and closed window size, then restore both on the next desktop run when the saved monitor is available.
- Q: What persistence library should store desktop window placement? -> A: Use `bevy-persistent` to store the placement as a persistent Bevy resource; do not hand-roll direct file read/write for this config-style state.
- Q: What units should be used for saved window size? -> A: Save and restore window size in Bevy logical window units so high-DPI monitors and Windows snap layouts reopen at the same apparent size; monitor positions remain physical desktop coordinates for multi-monitor placement.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Run Repeatable Project Scripts (Priority: P1)

A developer or reviewer can install/check dependencies, test, run desktop, run desktop hot reload, run browser WebGPU, and stop project-local processes through documented repository scripts instead of ad hoc commands.

**Why this priority**: Reliable scripts are the foundation for every later feature and keep local verification repeatable.

**Independent Test**: From the repository root, run the documented dependency, test, desktop, desktop hot reload, web, and stop entry points and verify each one starts from the expected working directory and reports the command it is executing.

**Acceptance Scenarios**:

1. **Given** the repository is checked out, **When** the reviewer runs the dependency script, **Then** Rust, Cargo, target availability, metadata, and warm desktop cache readiness are checked or prepared with visible terminal output.
2. **Given** the repository is checked out, **When** the reviewer runs the desktop script, **Then** changed desktop code builds in the dedicated desktop target cache and the desktop app opens.
3. **Given** the repository is checked out and hot reload tools are installed, **When** the reviewer runs the desktop hot reload script, **Then** the script starts a Dioxus CLI hot-patch workflow for the Bevy desktop package with visible terminal output.
4. **Given** the repository is checked out, **When** the reviewer runs the web script, **Then** the app builds for `wasm32-unknown-unknown`, is packaged for the browser, is served from localhost, and opens in the browser.
5. **Given** the repository is checked out, **When** the reviewer runs the test script, **Then** the automated test suite runs from the repository root using the shared fast desktop cache.
6. **Given** the repository is checked out in VS Code, **When** the reviewer starts the desktop run or desktop hot reload task, **Then** command output appears in the VS Code integrated terminal.

---

### User Story 2 - Open a Predictable Desktop Window (Priority: P1)

A reviewer launches the desktop app and sees a normal resizable window at the project-approved default size.

**Why this priority**: A predictable desktop surface is required before camera, DebugHUD, or card inspection behavior can be reviewed consistently.

**Independent Test**: Launch the app without a saved placement and verify the initial window is 1024x768.

**Acceptance Scenarios**:

1. **Given** no saved window placement exists, **When** the reviewer launches the desktop app, **Then** the window opens at 1024x768.
2. **Given** the window is visible, **When** the reviewer resizes it, **Then** the app remains usable and does not lose the ability to close normally.

---

### User Story 3 - Restore Last Window Position And Size (Priority: P2)

A reviewer moves or resizes the desktop window to any connected monitor, closes the app there, and sees the next run reopen at that last closed position and size.

**Why this priority**: Reopening in the previous location reduces friction during repeated local review sessions.

**Independent Test**: Move the window to a known position on each connected monitor, resize it, close the app, reopen it, and verify the window returns to the monitor, x/y position, and size from the last close.

**Acceptance Scenarios**:

1. **Given** the reviewer has moved or resized the app window to any connected monitor and position, **When** the reviewer closes the app normally, **Then** the app records the closed window size, x/y position, and monitor identity as local runtime state.
2. **Given** saved placement exists and the same monitor is available, **When** the reviewer launches the app again, **Then** the app opens on that monitor at the saved x/y position and size, including when the saved monitor is one of multiple connected monitors.
3. **Given** saved placement exists but the previous screen is unavailable, invalid, or off-screen, **When** the reviewer launches the app, **Then** the app opens centered on the primary screen at 1024x768.

### Edge Cases

- If saved placement data is missing, the app should use the default 1024x768 desktop window.
- If saved placement data is invalid or unreadable, the app should ignore it and continue launching centered on the primary screen at 1024x768.
- If the saved screen is disconnected, the app should fall back to the primary screen centered at 1024x768.
- If the window is closed on any connected monitor without being moved or resized during that run, the app should still preserve the best known position and size on normal close.
- If the app runs in a browser target, desktop window placement persistence should not block browser startup, wasm packaging, localhost serving, or browser launch.
- If a prior desktop app, build process, or web server is still running, the stop workflow should clean up project-local processes without requiring a machine restart.
- If desktop hot reload tooling is missing or incompatible, the hot reload script should fail with an actionable Dioxus CLI install or version message instead of silently falling back to a normal desktop run.
- If the app runs through desktop hot reload on Windows, the workflow should avoid Bevy dynamic linking when it would conflict with Dioxus hot patching.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The repository MUST provide repeatable scripts for dependency setup/checks, testing, desktop app runs, desktop hot reload app runs, browser WebGPU app runs, and stopping project-local processes.
- **FR-001A**: The repository MUST keep repeatable scripts under root `scripts`.
- **FR-001B**: The repository SHOULD keep user-facing scripts under `scripts/main` and support/helper scripts under `scripts/other`.
- **FR-001C**: The desktop run workflow MUST use a dedicated target cache and support a check-only mode that compiles without launching the app.
- **FR-001D**: The desktop hot reload workflow MUST use Dioxus CLI 0.7.x hot patching against the Bevy desktop package and MUST keep output attached to the calling terminal.
- **FR-001E**: The desktop hot reload workflow MUST use a dedicated hot reload target cache, Cargo incremental compilation, `WGPU_BACKEND=dx12`, and `BEVY_ASSET_ROOT` rooted at the repository.
- **FR-001F**: The dependency workflow MUST verify or install the Dioxus CLI version needed by the desktop hot reload workflow, with an opt-out for hot reload tool checks.
- **FR-001G**: The web run workflow MUST target `wasm32-unknown-unknown`, package the generated Wasm for browser use, serve it locally, and open it in a browser.
- **FR-001H**: The stop workflow MUST stop project-local desktop app, hot reload, build, and web server processes started by repository scripts.
- **FR-002**: The repository MUST provide VS Code task entries for test, desktop run, and desktop hot reload workflows.
- **FR-003**: The desktop run and desktop hot reload workflows MUST show command output in the VS Code integrated terminal when started through VS Code tasks.
- **FR-004**: The desktop app MUST default to a 1024x768 window when no valid saved placement exists.
- **FR-004A**: The approved desktop default width and height MUST be defined as `DEFAULT_WINDOW_WIDTH` and `DEFAULT_WINDOW_HEIGHT` in `bevy/crates/shared/src/window.rs`.
- **FR-005**: The desktop app MUST remember both the window x/y position and the window size, along with monitor identity, from wherever the window is closed on any connected monitor.
- **FR-005A**: The desktop app MUST save placement only on normal window close, not continuously during every move or resize.
- **FR-005B**: The desktop app MUST use `bevy-persistent` for window placement storage as a persistent resource under `data/local_storage/window-placement.json`; raw `std::fs` or ad hoc JSON read/write code MUST NOT be used for the normal persistence path.
- **FR-005C**: The desktop app MUST store window size in logical window units and MUST NOT persist physical pixel size as the restored window size.
- **FR-006**: The desktop app MUST restore both the remembered x/y position and remembered window size on the next desktop launch when that monitor is available, including multi-monitor desktop setups.
- **FR-007**: The desktop app MUST open centered on the primary screen at 1024x768 when the remembered screen or position is unavailable, invalid, or off-screen.
- **FR-008**: Window placement state MUST be local runtime state and MUST NOT be committed as source content.
- **FR-009**: Dependency, test, desktop run, desktop hot reload, web run, and stop scripts MUST work from the repository root.
- **FR-010**: Window setup and placement behavior MUST be treated as reusable system-level functionality under `bevy/crates/shared`; game-specific code may only compose it into the app.
- **FR-011**: This feature MUST NOT include card rendering, DebugHUD controls, gameplay, or card-inspection interaction behavior.

### Key Entities

- **Project Script**: A repository entry point for dependency setup, test, desktop run, desktop hot reload, web run, or stop workflows.
- **Desktop Hot Reload Script**: A repository entry point that runs the Bevy desktop package through Dioxus CLI hot patching for faster Rust gameplay/system iteration.
- **Desktop Window Placement**: The remembered window x/y position, size, and monitor identity from the last normal desktop close.
- **Window Placement Store**: A `bevy-persistent` persistent resource containing the optional latest desktop window placement.
- **Screen Identity**: The monitor information used to reopen the app on the same display when possible.
- **Local Runtime State**: Machine-local data used by the app under `data/local_storage/` and excluded from version control.
- **Shared Runtime System**: Reusable non-card functionality that belongs in `bevy/crates/shared`, including desktop window defaults and placement restore/save behavior. Desktop default size constants live in `bevy/crates/shared/src/window.rs`.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: The desktop run script completes the desktop build workflow from the repository root and opens the desktop app when not in check-only mode.
- **SC-002**: The test script completes the automated test suite from the repository root.
- **SC-002A**: The desktop hot reload script starts the Dioxus CLI hot-patch workflow from the repository root, reports the Dioxus CLI version, and keeps output in the invoking terminal.
- **SC-003**: In 100% of first-launch checks without saved placement, the desktop window opens at 1024x768.
- **SC-004**: In placement restore checks on each connected monitor, the app reopens within 20 physical pixels of the x/y position from the last close and restores the saved window size.
- **SC-005**: In disconnected-screen, invalid-data, and off-screen placement checks, the app opens centered on the primary screen at 1024x768 instead of restoring off-screen.
- **SC-006**: VS Code desktop run and desktop hot reload task output appears in the integrated terminal in 100% of task-launch checks.
- **SC-007**: The web run script builds the Wasm target, packages the browser bundle, serves it from localhost, and returns a successful HTTP response from the generated page.
- **SC-008**: The stop script terminates project-local desktop app, hot reload, build, and web server processes started by repository scripts.

## Assumptions

- The primary development environment is Windows desktop with VS Code.
- Window placement persistence is reviewer convenience state, not gameplay state.
- Local runtime state may live under an ignored generated-output location in the repository.
- Browser WebGPU is supported through the repository web runner, but desktop placement restore only applies to desktop windows.
- Desktop hot reload is a native development workflow only; browser WebGPU remains a build/serve workflow without hot reload.
- Hot reload is intended for explicitly hot-reload-enabled Rust systems and assets. Normal desktop run remains the fallback for release-like local review.
- `bevy/crates/shared` owns reusable runtime setup behavior; `bevy/crates/game` remains reserved for game-specific card and gameplay features.
