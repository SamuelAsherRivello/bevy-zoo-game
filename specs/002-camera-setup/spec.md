# Feature Specification: Camera Setup

**Feature Branch**: `002-camera-setup`  
**Created**: 2026-05-09  
**Status**: Draft  
**Input**: User description: "Add the camera setup as its own feature before DebugHUD and card inspection. The app needs a 3D camera with basic parameters so later features can render the card scene and still show the overlay HUD."

## Clarifications

### Session 2026-05-09

- Q: What default 3D camera placement and projection should this feature use? -> A: Use a fixed perspective camera with typical FOV, positioned at `(0, 0, 5)` by default, looking down toward the world origin or toward a user-specified target object when one exists.
- Q: What default scene background should camera setup use? -> A: Use a neutral dark gray background.
- Q: Should the camera respond to input in this feature? -> A: The camera never moves or responds to input in this feature.
- Q: Where should camera setup live? -> A: Camera marker/defaults/setup behavior is reusable system-level functionality and belongs in `bevy/crates/shared`; `bevy/crates/game` should compose it when building the zoo game scene.
- Q: How does camera setup relate to `AppScene`? -> A: The primary 3D camera is part of reloadable `AppScene` content, so scene reloads rebuild the camera along with other scene-owned content such as lights and models.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Show a 3D Scene View (Priority: P1)

A reviewer launches the prototype and the app has a usable 3D camera ready to frame future scene content.

**Why this priority**: Later card inspection work depends on a stable 3D view before card geometry and interaction are added.

**Independent Test**: Launch the prototype and verify that the scene has an active camera with a predictable viewpoint.

**Acceptance Scenarios**:

1. **Given** the prototype is running, **When** the scene starts, **Then** there is one active 3D camera for the primary window.
2. **Given** the 3D camera is active, **When** later scene content is added near the origin, **Then** the camera can frame that content without requiring a menu or extra setup.

---

### User Story 2 - Keep Camera Parameters Stable (Priority: P1)

A developer can rely on basic camera position, orientation, and projection defaults while implementing later features.

**Why this priority**: Stable camera defaults reduce rework in DebugHUD and card-inspection features.

**Independent Test**: Inspect the camera setup and verify that it uses documented basic position, look direction, field of view, near/far range, and neutral dark gray clear color expectations.

**Acceptance Scenarios**:

1. **Given** the app starts, **When** the camera is created, **Then** it uses a fixed perspective view with typical FOV and looks down toward the scene origin or toward a user-specified target object when one exists.
2. **Given** the camera is active, **When** the app runs without card content, **Then** it does not move, orbit, pan, zoom, or respond to any input.

---

### User Story 3 - Preserve Overlay Compatibility (Priority: P2)

A reviewer can still see later overlay diagnostics, including the DebugHUD, after the 3D camera setup is present.

**Why this priority**: The camera foundation must not block the reviewer-facing overlay used by later diagnostics.

**Independent Test**: Combine this feature with the DebugHUD feature and verify that the overlay remains visible while the 3D camera is active.

**Acceptance Scenarios**:

1. **Given** the 3D camera setup and DebugHUD are both present, **When** the reviewer launches the app, **Then** the DebugHUD remains visible above the scene.
2. **Given** the 3D camera setup is active, **When** the window is resized, **Then** the camera remains usable for scene content.

### Edge Cases

- If no card content exists yet, the app should still start successfully.
- If the window is resized, the active camera should continue rendering the primary scene.
- If DebugHUD is enabled later, camera setup should not hide or replace the overlay.
- If browser WebGPU is used later, the camera setup should not depend on desktop-only behavior.
- If `AppScene` reloads, the camera should be recreated or refreshed back to the documented default camera state.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The app MUST create one active 3D camera for the primary scene.
- **FR-002**: The 3D camera MUST have documented basic position, orientation, perspective projection, typical FOV, near/far range, and neutral dark gray clear color expectations.
- **FR-003**: The 3D camera MUST initially frame future origin-centered card content by looking down toward the world origin, unless a user-specified target object is provided.
- **FR-004**: The 3D camera MUST remain stationary and MUST NOT respond to keyboard, mouse, controller, or gameplay input in this feature.
- **FR-005**: The camera setup MUST allow later DebugHUD overlay content to remain visible.
- **FR-006**: The camera setup MUST support Windows desktop and browser WebGPU targets.
- **FR-007**: This feature MUST NOT include card geometry, card shaders, pointer-driven card rotation, DebugHUD controls, menus, or gameplay.
- **FR-008**: Camera marker, defaults, setup, and input-stability behavior MUST be implemented as reusable shared runtime functionality under `bevy/crates/shared`; card-specific scene code in `bevy/crates/game` may consume the shared camera but MUST NOT own this feature.
- **FR-009**: The primary 3D camera MUST be composed as part of reloadable `AppScene` content, and an `AppScene` reload MUST restore the documented camera defaults.

### Key Entities

- **Primary 3D Camera**: The scene camera used to frame future card content.
- **Camera Defaults**: The agreed starting position `(0, 0, 5)`, target rule, perspective projection, typical FOV, clipping range, and neutral dark gray clear color.
- **Overlay Compatibility**: The requirement that diagnostics can render visibly over the 3D scene in later features.
- **Shared Camera Runtime**: Reusable camera marker, defaults, setup system, and tests owned by `bevy/crates/shared` for use by the game crate and future app surfaces.
- **AppScene Camera Content**: The camera portion of reloadable `AppScene` content that returns to documented defaults when the app scene reloads.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In 100% of launch checks, exactly one primary 3D scene camera exists.
- **SC-002**: Camera defaults document a fixed perspective camera, typical FOV, default `(0, 0, 5)` position, and origin-or-target look direction that can be verified without running card-inspection behavior.
- **SC-003**: In combined checks with DebugHUD, the DebugHUD remains visible while the 3D camera is active.
- **SC-004**: During input checks, keyboard and mouse input do not move, orbit, pan, or zoom the camera.
- **SC-005**: Final acceptance verification passes for Windows desktop and browser WebGPU, or any blocked target is documented with the exact blocker.
- **SC-006**: After an `AppScene` reload, the primary 3D camera again exists exactly once and matches the documented default camera state.

## Assumptions

- The first visible 3D content will be origin-centered card content from `004-card-inspection-poc`.
- The camera setup is a foundation feature and should stay minimal.
- DebugHUD overlay rendering is owned by `003-debug-hud`, but this feature must not block it.
- `bevy/crates/shared` owns reusable camera setup; `bevy/crates/game` owns card-specific scene content that uses the shared camera.
- `001-project-setup` owns the `AppScene` lifecycle contract; this feature supplies the camera content composed into that lifecycle.
