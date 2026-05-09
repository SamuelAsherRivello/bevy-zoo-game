# Feature Specification: Card Inspection POC

**Feature Branch**: `004-card-inspection-poc`  
**Created**: 2026-05-09  
**Status**: Draft  
**Input**: User description: "This is a new POC for a game that will have a digital version of paper trading cards. There is no menu; the app jumps right to the POC. The POC has no HUD. It shows one card at the center of the screen. A 3D camera points at the card and the card normal faces the camera. Mouse movement updates the rotation of the card. The mouse at the upper right of the screen points the normal of the card to the upper right of the frustum, with the same behavior for all other screen positions. Rotation movement is smooth. The card is a simple 3D shape with a custom white shader, so it looks like a white rectangle. The size is based on real-world poker card dimensions from the reference image: 63 mm by 88 mm. V0.1 is only this functionality with a white rectangle. V0.2 will expand rendering, shaders, textures, and setup for one card. V0.3 may explore multiple cards and gameplay. For now, no gameplay."

## Clarifications

### Session 2026-05-09

- Q: What maximum rotation range should V0.1 support for pointer-driven card tilt? -> A: Moderate tilt: maximum 20 degrees from neutral on each axis.
- Q: What smoothing response target should V0.1 use when the pointer moves to a new position? -> A: Very responsive: reaches the target orientation within 100 ms.
- Q: What 3D card geometry should V0.1 require? -> A: Thin slab: front face plus slight thickness, no bevel.
- Q: What camera behavior should V0.1 require during pointer-driven inspection? -> A: Fixed camera: camera remains stationary and only the card rotates.
- Q: How should `003-debug-hud` relate to the no-HUD rule in `004-card-inspection-poc` when both are implemented? -> A: `003-debug-hud` replaces the no-HUD rule in `004`; the final app should show the DebugHUD by default.
- Q: Should `004-card-inspection-poc` and `003-debug-hud` explicitly require both Windows desktop and browser WebGPU verification? -> A: Both specs require final Windows desktop and browser WebGPU verification; iterative builds may target desktop only.
- Q: What should pointer input affect during card inspection? -> A: Pointer input rotates only the card; the camera remains fixed according to `002-camera-setup`.
- Q: What visual detail should the V0.1 card placeholder include? -> A: Plain white thin slab, no bevel, no art, no text, with only minimal visible lighting needed to see it.
- Q: Should the existing pointer tilt and smoothing targets remain? -> A: Keep maximum 20 degrees tilt from neutral on each axis and a 100 ms smoothing target.
- Q: Which crate owns card-specific POC behavior? -> A: `bevy/crates/game` owns card geometry, card resources, card pointer mapping, card smoothing, and future card/gameplay expansion. It may consume reusable window, camera, DebugHUD, inspector, and diagnostic input systems from `bevy/crates/shared`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Enter Direct Card POC (Priority: P1)

A reviewer launches the prototype and immediately sees a single white trading-card placeholder centered in the scene, without menus, gameplay prompts, gameplay overlays, or additional cards. When `003-debug-hud` is implemented, the approved DebugHUD appears by default and is the only HUD exception.

**Why this priority**: This is the minimum visible proof that the prototype is focused on card presentation rather than gameplay or navigation.

**Independent Test**: Launch the prototype and verify that the first visible state is one centered white card placeholder with no menu or interactive gameplay UI; if `003-debug-hud` is present, verify that only the approved DebugHUD is visible.

**Acceptance Scenarios**:

1. **Given** the prototype is not running, **When** the reviewer launches it, **Then** the first visible screen shows one centered card placeholder.
2. **Given** the prototype is running, **When** the reviewer observes the scene, **Then** no menus, gameplay text, counters, buttons, or additional cards are visible; the approved DebugHUD from `003-debug-hud` is allowed when that feature is implemented.

---

### User Story 2 - Present Real-World Poker Card Proportions (Priority: P1)

A reviewer sees a card placeholder whose proportions represent a standard poker-size paper card based on the provided reference: 63 mm by 88 mm, also shown as 2.5 inches by 3.5 inches.

**Why this priority**: The POC is specifically about a digital version of paper trading cards, so the placeholder must establish the correct physical card proportions before richer rendering is added.

**Independent Test**: Inspect the visible card dimensions and verify that the height-to-width ratio matches 88:63 within the accepted visual tolerance.

**Acceptance Scenarios**:

1. **Given** the prototype is running, **When** the card is viewed in its neutral centered state, **Then** it appears as a vertical rectangle using poker-card proportions of 63 mm by 88 mm.
2. **Given** the card dimensions are evaluated, **When** comparing height to width, **Then** the ratio matches 88:63 within 2% tolerance.

---

### User Story 3 - Smooth Mouse-Driven Card Rotation (Priority: P1)

A reviewer moves the mouse across the screen and the card smoothly rotates so its front-facing direction follows the mouse position within the visible viewing area.

**Why this priority**: The POC's primary interaction is the feeling of physically inspecting or tilting a single card with the pointer.

**Independent Test**: Move the mouse to the center, corners, and edges of the screen and verify that the card orientation smoothly follows the pointer without abrupt snapping while the camera remains stationary.

**Acceptance Scenarios**:

1. **Given** the mouse is near the center of the screen, **When** the reviewer holds the mouse still, **Then** the card returns toward a neutral front-facing orientation.
2. **Given** the mouse moves to the upper-right of the screen, **When** the card finishes its smoothing response, **Then** the card's front-facing direction points toward the upper-right of the visible viewing area.
3. **Given** the mouse moves to any other corner or edge of the screen, **When** the card finishes its smoothing response, **Then** the card's front-facing direction points toward the corresponding area of the visible viewing area.
4. **Given** the mouse moves quickly between positions, **When** the card updates orientation, **Then** the rotation transitions smoothly and reaches the new target orientation within 100 ms instead of instantly snapping.
5. **Given** the mouse moves across the screen, **When** the card responds to pointer input, **Then** only the card rotates and the camera position and orientation remain fixed according to `002-camera-setup`.

---

### User Story 4 - Keep V0.1 Rendering Minimal (Priority: P2)

A reviewer sees only a simple white card placeholder in V0.1, formed as a thin slab with a front face and slight thickness, with no bevel, card art, text, rich material effects, textures, border details, card backs, or gameplay elements. The scene may use only minimal lighting needed to make the white slab visible.

**Why this priority**: The POC must isolate card sizing, placement, camera-facing orientation, and smooth pointer response before later visual and gameplay work.

**Independent Test**: Inspect the card and scene visually and confirm the card is a plain white rectangle, with no decorative or gameplay content.

**Acceptance Scenarios**:

1. **Given** the prototype is running, **When** the reviewer observes the card, **Then** the card appears as a plain white rectangular thin slab with no bevel.
2. **Given** the prototype is running, **When** the reviewer interacts with the mouse, **Then** no gameplay actions, scoring, card selection, card movement, or multi-card behavior occurs.

---

### Edge Cases

- If the mouse leaves the visible application area, the card should hold or ease toward the last valid pointer-driven orientation without jittering.
- If the application window size changes, the card should remain centered and preserve poker-card proportions.
- If the mouse is at an exact corner or edge, the card should remain visible and limit tilt to a maximum of 20 degrees from neutral on each axis.
- If no mouse movement has occurred after launch, the card should remain centered in a neutral front-facing orientation.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The prototype MUST start directly in the card POC view without showing any menu, loading menu, gameplay overlay, or gameplay prompt; when `003-debug-hud` is implemented, its approved DebugHUD MUST be shown by default.
- **FR-002**: The prototype MUST display exactly one card placeholder at the center of the visible scene.
- **FR-003**: The card placeholder MUST use poker-size proportions based on the provided reference: 63 mm wide by 88 mm tall.
- **FR-004**: The card placeholder MUST appear as a plain white vertical rectangular thin slab in V0.1, with a front face, slight thickness, no bevel, no art, no text, and only minimal visible lighting needed to see it.
- **FR-005**: The card placeholder MUST initially face the viewer in a neutral centered orientation.
- **FR-006**: The card placeholder MUST update its orientation based on the mouse position within the visible application area.
- **FR-007**: Moving the mouse toward the upper-right, upper-left, lower-right, or lower-left of the visible area MUST orient the card's front-facing direction toward the corresponding visible area.
- **FR-008**: Mouse-driven card rotation MUST transition smoothly over time rather than snapping immediately to each new pointer position, while reaching the target orientation within 100 ms.
- **FR-009**: The card MUST remain centered during rotation and must not translate around the scene as part of the V0.1 interaction.
- **FR-010**: The card MUST remain visible and recognizable as a card throughout the full supported pointer range by limiting pointer-driven tilt to a maximum of 20 degrees from neutral on each axis.
- **FR-011**: V0.1 MUST NOT include gameplay, multiple cards, card art, card text content, card backs, rich material effects, textures, bevels, deck handling, selection, dragging, scoring, turns, rules, or HUD elements other than the approved `003-debug-hud` DebugHUD.
- **FR-012**: The prototype scope MUST preserve future room for V0.2 to expand single-card rendering and card setup while still showing one centered card.
- **FR-013**: The prototype scope MUST preserve future room for V0.3 to explore multiple cards and gameplay, without including those behaviors in V0.1.
- **FR-014**: The camera MUST remain stationary according to `002-camera-setup` during pointer-driven inspection; pointer input MUST rotate only the card placeholder, not the camera or scene framing.
- **FR-015**: The card POC MUST support both Windows desktop and browser WebGPU before completion; during implementation iterations, desktop-only builds are acceptable for fast feedback.
- **FR-016**: Card-specific placeholder geometry, card defaults, pointer-to-card rotation mapping, smoothing, and future card/gameplay expansion MUST live under `bevy/crates/game`; reusable system-level window, camera, DebugHUD, inspector, and diagnostic input behavior MUST remain under `bevy/crates/shared`.

### Key Entities

- **Card Placeholder**: The single visible digital card in V0.1; key attributes are centered position, poker-size proportions, thin-slab geometry with slight thickness and no bevel, plain white appearance, and pointer-responsive orientation.
- **Pointer Position**: The reviewer's current mouse position within the visible application area; used to define the card's target facing direction.
- **Viewing Area**: The visible application region used to map pointer positions to card orientation targets.
- **Fixed Camera**: The stationary view used for V0.1 inspection; it frames the centered card and does not move, orbit, pan, or zoom in response to pointer input.
- **POC Version Scope**: The agreed boundary for V0.1, V0.2, and V0.3 work so future rendering and gameplay expansion does not leak into the initial proof of concept.
- **Game Card Runtime**: Card-specific runtime behavior owned by `bevy/crates/game`, composed with shared window, camera, DebugHUD, inspector, and diagnostic input systems from `bevy/crates/shared`.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: On launch, a reviewer reaches the card POC view in under 2 seconds after the application window appears, with no required menu interaction or required HUD interaction.
- **SC-002**: In 100% of launch checks, exactly one card placeholder is visible and centered in the scene.
- **SC-003**: The card's visible height-to-width ratio matches 88:63 within 2% tolerance.
- **SC-004**: During pointer movement tests to all four corners and the center, the card visibly follows the pointer direction in all 5 tested positions.
- **SC-005**: In pointer movement tests, card orientation changes reach the target orientation within 100 ms, are perceived as smooth by at least 4 out of 5 reviewers, and show no abrupt snapping during ordinary mouse movement.
- **SC-006**: In V0.1 acceptance review, reviewers identify no gameplay HUD, card art, card text, textures, bevels, additional cards, or deck behavior in the scene; the approved `003-debug-hud` DebugHUD is allowed.
- **SC-007**: During pointer tests at each screen corner, measured card tilt does not exceed 20 degrees from neutral on either axis.
- **SC-008**: During pointer movement tests, reviewers observe no camera movement, orbiting, panning, zooming, or framing adjustment.
- **SC-009**: Final acceptance verification passes for the card POC on Windows desktop and browser WebGPU, or any blocked target is documented with the exact blocker.

## Assumptions

- The target user for V0.1 is an internal reviewer validating card presentation and pointer-driven feel, not an end player.
- The provided poker-size reference is authoritative for V0.1 dimensions: 63 mm by 88 mm, equivalent to 2.5 inches by 3.5 inches.
- A plain white card face is sufficient for V0.1; richer rendering, material effects, textures, and card setup are intentionally deferred to V0.2.
- Multiple cards and gameplay are intentionally deferred to V0.3 or later.
- Keyboard, touch, controller, and mobile-specific interactions are out of scope for V0.1.
- Accessibility and localization requirements are not evaluated in V0.1 because the only allowed text is reviewer-facing diagnostic HUD text from `003-debug-hud`, and no player-facing menu or gameplay flow is included.
- Desktop-only builds are acceptable while iterating, but final completion requires Windows desktop and browser WebGPU verification.
- `bevy/crates/game` owns card-specific POC behavior; `bevy/crates/shared` owns reusable system-level runtime behavior.
