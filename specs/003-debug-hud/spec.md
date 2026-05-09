# Feature Specification: DebugHUD

**Feature Branch**: `003-debug-hud`  
**Created**: 2026-05-09  
**Status**: Draft  
**Input**: User description: "Copy how the HUD works in https://github.com/SamuelAsherRivello/bevy-jam-1. Keep the main HUD panel, title/status text, and key legend. WASD should be visible but do nothing. Keep F and I only and bring in that functionality. Do not include toast, minimap, reticle, or other HUD-related systems. Add tests and a RunTests script."

## Clarifications

### Session 2026-05-09

- Q: Which bevy-jam-1 HUD content should be kept? -> A: Keep the top-left HUD panel, title/status text, key legend with non-functional `WASD`, and functional `F` and `I`; exclude toast, minimap, reticle, and other listed HUD systems.
- Q: How should `003-debug-hud` relate to the no-HUD rule in `004-card-inspection-poc` when both are implemented? -> A: `003-debug-hud` replaces the no-HUD rule in `004`; the final app should show the DebugHUD by default.
- Q: Should `004-card-inspection-poc` and `003-debug-hud` explicitly require both Windows desktop and browser WebGPU verification? -> A: Both specs require final Windows desktop and browser WebGPU verification; iterative builds may target desktop only.
- Q: How should debug overlay scripts, tasks, docs, and source-facing labels be named? -> A: Rename all debug overlay scripts, tasks, docs, and source-facing labels to use `DebugHUD` so generic `HUD` remains available for a future user-facing HUD.
- Q: Should `WASD` labels visually react to key presses? -> A: `WASD` labels may visually highlight while pressed through a DebugHUD/InputSystem key-state capture, but no gameplay, camera, card, or other non-DebugHUD system may consume those keys in this spec.
- Q: Which approved DebugHUD keys are toggles? -> A: DebugHUD key labels are classified as toggle or non-toggle: `F` and `I` are toggles; `W`, `A`, `S`, and `D` are non-toggle hold indicators.
- Q: Where should DebugHUD and its input capture live? -> A: DebugHUD UI, inspector toggling, diagnostic key classification, and the DebugHUD/InputSystem-style key-state capture are reusable system-level diagnostics and belong in `bevy/crates/shared`; `bevy/crates/game` should only compose them with card-specific features.
- Q: How should scene restart appear in the DebugHUD? -> A: Add `R` as a non-toggle DebugHUD key on the first key line; pressing it reloads the app scene content, including camera, lights, and models.
- Q: How should DebugHUD key labels be grouped? -> A: Non-toggle keys must appear on the first key line exactly as `KEYS: WASD, R`; toggle keys must appear on the second key line exactly as `KEYS: F, I, H`, where `H` toggles hot-reload auto-restart.
- Q: Are `H` and `R` part of the approved DebugHUD input set? -> A: Yes. `H` and `R` are included approved DebugHUD keys: `H` is a persisted toggle for hot-reload auto-restart, and `R` is a non-toggle operation that reloads `AppScene`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Show DebugHUD Panel (Priority: P1)

A reviewer sees a top-left DebugHUD panel adapted from the bevy-jam-1 HUD pattern while running the card-inspection prototype.

**Why this priority**: The HUD provides lightweight review diagnostics without adding gameplay UI.

**Independent Test**: Launch the prototype and verify that a translucent top-left HUD panel appears with title/status text and the expected key labels.

**Acceptance Scenarios**:

1. **Given** the prototype is running, **When** the reviewer observes the top-left corner, **Then** a translucent DebugHUD panel is visible by default.
2. **Given** the HUD is visible, **When** the reviewer reads it, **Then** it shows the prototype title, frame/status text, first key line `KEYS: WASD, R`, and second key line `KEYS: F, I, H`.

---

### User Story 2 - Toggle FPS With F (Priority: P1)

A reviewer presses `F` to show or hide FPS diagnostic text in the HUD.

**Why this priority**: FPS is the primary runtime diagnostic needed for this visual POC.

**Independent Test**: Press `F` while the prototype is running and verify that FPS text toggles visibility without affecting the card.

**Acceptance Scenarios**:

1. **Given** the HUD is visible and FPS is hidden, **When** the reviewer presses `F`, **Then** FPS text appears in the HUD.
2. **Given** FPS text is visible, **When** the reviewer presses `F` again, **Then** FPS text is hidden.
3. **Given** the reviewer toggles FPS, **When** the card is visible, **Then** card position, rotation target, and camera behavior are unchanged.

---

### User Story 3 - Toggle Inspector With I (Priority: P1)

A reviewer presses `I` to show or hide the inspector while preserving the POC scene behavior.

**Why this priority**: Inspector access helps review runtime entities and state without adding gameplay.

**Independent Test**: Press `I` while the prototype is running and verify that inspector visibility toggles without affecting the card.

**Acceptance Scenarios**:

1. **Given** the inspector is hidden, **When** the reviewer presses `I`, **Then** the inspector becomes visible.
2. **Given** the inspector is visible, **When** the reviewer presses `I` again, **Then** the inspector becomes hidden.
3. **Given** the reviewer toggles the inspector, **When** the card is visible, **Then** card position, rotation target, and camera behavior are unchanged.

---

### User Story 4 - Keep WASD Non-Functional (Priority: P2)

A reviewer sees `W`, `A`, `S`, and `D` in the HUD key legend, and those labels may visually react while pressed, but those keys do not trigger gameplay, camera, card movement, or selection behavior.

**Why this priority**: The visual pattern is copied from bevy-jam-1, but the card POC must not inherit aircraft movement or gameplay controls.

**Independent Test**: Press `W`, `A`, `S`, and `D` while the prototype is running and verify that no gameplay or card behavior changes.

**Acceptance Scenarios**:

1. **Given** the prototype is running, **When** the reviewer presses `W`, `A`, `S`, or `D`, **Then** the DebugHUD key legend may show the pressed key state, but the card does not move, rotate because of those keys, select, score, shoot, or trigger gameplay.
2. **Given** `WASD` key labels are visible, **When** the reviewer presses those keys, **Then** FPS and inspector visibility remain unchanged.
3. **Given** `WASD` key labels are visible, **When** the reviewer releases those keys, **Then** their pressed-state feedback clears because they are non-toggle hold indicators.

### Edge Cases

- If the application window size changes, the HUD should scale consistently with the window while staying anchored near the top-left.
- If FPS is hidden, the HUD should not reserve visible FPS text content.
- If the inspector is hidden, no inspector panel should be visible.
- If `F` or `I` is pressed repeatedly, each key press should toggle only its corresponding diagnostic state.
- If `H` is pressed repeatedly, each key press should toggle only the persisted hot-reload auto-restart state.
- If `R` is pressed repeatedly, each key press should invoke an `AppScene` reload and should not change any toggle state.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The prototype MUST include one top-left DebugHUD panel adapted from the bevy-jam-1 HUD pattern.
- **FR-001A**: This DebugHUD feature replaces the no-HUD rule from `004-card-inspection-poc` for the final combined app; the DebugHUD MUST be visible by default.
- **FR-002**: The DebugHUD MUST show the prototype title and frame/status text.
- **FR-003**: The DebugHUD MUST show non-toggle key labels on the first key line exactly as `KEYS: WASD, R`.
- **FR-003A**: The DebugHUD MUST show toggle key labels on the second key line exactly as `KEYS: F, I, H`.
- **FR-003B**: The `R` key MUST reload the app scene content, including camera, lights, and models, and MUST remain non-toggle.
- **FR-003C**: The `R` key MUST invoke the `AppScene` reload method defined by `001-project-setup`.
- **FR-004**: The `F` key MUST toggle FPS visibility in the DebugHUD.
- **FR-005**: The `I` key MUST toggle inspector visibility.
- **FR-005A**: The `H` key MUST toggle hot-reload auto-restart.
- **FR-005B**: The `F`, `I`, and `H` toggle states MUST default to `false` on first run, persist to local storage when changed, and restore on later desktop launches.
- **FR-005C**: When hot-reload auto-restart is enabled, a desktop hot-reload patch MUST reload `AppScene` using the same behavior as the `R` key; when disabled, hot-reload patches MUST NOT reload the scene.
- **FR-006**: The `W`, `A`, `S`, and `D` keys MAY be captured by the DebugHUD/InputSystem for visible key-state feedback, but MUST NOT trigger movement, gameplay, camera, card, selection, scoring, deck behavior, or any other non-DebugHUD behavior in this spec.
- **FR-006A**: Approved DebugHUD keys MUST be classified as toggle or non-toggle: `F`, `I`, and `H` are toggles, while `W`, `A`, `S`, `D`, and `R` are non-toggle hold indicators.
- **FR-007**: The HUD MUST use a translucent top-left panel style comparable to the bevy-jam-1 HUD.
- **FR-008**: The HUD MUST scale responsively when the application window size changes.
- **FR-009**: The implementation MUST include automated tests for HUD creation, `F` toggle behavior, `I` toggle behavior, and non-functional `WASD` behavior.
- **FR-010**: The repository MUST include a `RunTests` script that runs the automated test suite.
- **FR-010A**: Debug overlay scripts, tasks, docs, and source-facing labels MUST use `DebugHUD` naming rather than generic `HUD` naming so generic `HUD` remains available for a future user-facing HUD.
- **FR-010B**: The feature MUST include an InputSystem-style key-state capture for approved DebugHUD keys: `W`, `A`, `S`, `D`, `R`, `F`, `I`, and `H`.
- **FR-011**: This feature MUST NOT include bevy-jam-1 toast, minimap, reticle, autopilot, shooting, health, score, or gameplay HUD behavior.
- **FR-012**: The DebugHUD MUST support both Windows desktop and browser WebGPU before completion; during implementation iterations, desktop-only builds are acceptable for fast feedback.
- **FR-013**: DebugHUD UI, inspector visibility, approved diagnostic input capture, and key classification MUST be implemented as reusable shared runtime functionality under `bevy/crates/shared`; game-specific code in `bevy/crates/game` may consume these diagnostics but MUST NOT own them.

### Key Entities

- **DebugHUD Panel**: The top-left diagnostic UI surface showing prototype status and key labels.
- **FPS Toggle**: The `F` key behavior that shows or hides FPS text in the HUD.
- **Inspector Toggle**: The `I` key behavior that shows or hides inspector visibility.
- **Hot Reload Auto-Restart Toggle**: The `H` key behavior that enables or disables scene reload on hot-reload patches.
- **AppScene Reload Integration**: The connection between DebugHUD input and the `AppScene` reload method defined by `001-project-setup`.
- **DebugHUD InputSystem**: The debug-only key-state capture for approved DebugHUD keys: `W`, `A`, `S`, `D`, `R`, `F`, `I`, and `H`, including each key's toggle or non-toggle classification.
- **Restart App Key**: The non-toggle `R` key that reloads app scene content, including camera, lights, and models.
- **Non-Gameplay WASD Labels**: Visible `W`, `A`, `S`, and `D` key labels that preserve the copied HUD pattern and may show pressed state without adding gameplay controls.
- **RunTests Script**: A repeatable project script for running the automated test suite.
- **DebugHUD Naming**: The canonical naming convention for debug overlay scripts, tasks, docs, and source-facing labels.
- **Shared Debug Runtime**: Reusable DebugHUD, inspector, and diagnostic input state owned by `bevy/crates/shared`.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In 100% of launch checks, one top-left DebugHUD panel is visible.
- **SC-002**: In HUD content checks, the HUD includes title/status text, first key line `KEYS: WASD, R`, and second key line `KEYS: F, I, H`.
- **SC-003**: In toggle tests, pressing `F` changes FPS visibility state on each press and does not change inspector visibility.
- **SC-004**: In toggle tests, pressing `I` changes inspector visibility state on each press and does not change FPS visibility.
- **SC-005**: In keyboard behavior tests, pressing `W`, `A`, `S`, and `D` may update DebugHUD key-state feedback but produces no card movement, gameplay action, FPS toggle, inspector toggle, camera behavior, or non-DebugHUD behavior.
- **SC-005A**: In key classification tests, `F`, `I`, and `H` behave as toggles while `W`, `A`, `S`, `D`, and `R` behave as non-toggle hold indicators.
- **SC-005B**: In persistence checks, `F`, `I`, and `H` restore their last saved toggle values after relaunch and first-run values are all disabled.
- **SC-005C**: In AppScene reload checks, pressing `R` reloads `AppScene` without changing `F`, `I`, or `H` toggle values, and the persisted `H` value determines whether hot-reload patches also reload `AppScene`.
- **SC-006**: The `RunTests` script completes the automated test suite from the repository root.
- **SC-006A**: Review of scripts, tasks, docs, and source-facing labels finds `DebugHUD` naming for debug overlay-specific items and no generic debug overlay item named only `HUD`.
- **SC-007**: Reviewers identify no toast, minimap, reticle, autopilot, shooting, health, score, or gameplay HUD behavior in this feature.
- **SC-008**: Final acceptance verification passes for the DebugHUD on Windows desktop and browser WebGPU, or any blocked target is documented with the exact blocker.

## Assumptions

- The HUD is reviewer-facing diagnostic UI, not player-facing gameplay UI.
- The bevy-jam-1 HUD is the visual and behavioral reference for the panel, title/status text, key labels, and `F`/`I` toggle pattern.
- `WASD` remains visible to preserve the copied HUD pattern and may provide DebugHUD-only pressed feedback; it is intentionally non-functional for gameplay, card, camera, and other non-DebugHUD systems in this card POC.
- Toast, minimap, reticle, and gameplay HUD systems are intentionally excluded from this feature.
- Desktop-only builds are acceptable while iterating, but final completion requires Windows desktop and browser WebGPU verification.
- `bevy/crates/shared` owns reusable DebugHUD and diagnostic input behavior; `bevy/crates/game` owns card-specific behavior that may be inspected by those diagnostics.
- `001-project-setup` owns the `AppScene` lifecycle and hot-reload auto-restart persistence contract; this feature exposes the included `H` and `R` keys in the DebugHUD and wires them to that contract.
