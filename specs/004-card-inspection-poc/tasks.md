# Tasks: Card Inspection POC

**Input**: Design documents from `specs/004-card-inspection-poc/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/card-inspection.md, quickstart.md

## Phase 1: Setup

**Purpose**: Confirm the existing Bevy runtime shape before card behavior is added.

- [X] T001 Verify current runtime module boundaries in `bevy/crates/game/src/runtime/`
- [X] T002 Verify the available test runner path in `scripts/other/RunTests.ps1`

---

## Phase 2: Foundational

**Purpose**: Add game-specific ECS data needed by all card inspection stories.

- [X] T003 Add card placeholder and pointer inspection components in `bevy/crates/game/src/runtime/components/mod.rs`
- [X] T004 Add card dimensions, tilt limits, smoothing, and pointer target resources in `bevy/crates/game/src/runtime/resources/mod.rs`

---

## Phase 3: User Story 1 - Enter Direct Card POC (Priority: P1)

**Goal**: Launch directly into a one-card scene with the approved DebugHUD as the only HUD.

**Independent Test**: Startup query finds exactly one card placeholder, one primary camera, and the existing DebugHUD.

- [X] T005 [US1] Add startup tests for one-card POC scene composition in `bevy/crates/game/src/runtime/plugins/mod.rs`
- [X] T006 [US1] Spawn one named card placeholder entity with mesh and material handles in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T007 [US1] Wire card startup resources and systems into `bevy/crates/game/src/runtime/plugins/mod.rs`

---

## Phase 4: User Story 2 - Present Real-World Poker Card Proportions (Priority: P1)

**Goal**: Make the placeholder match `63 mm x 88 mm` poker-card proportions.

**Independent Test**: Automated test confirms the source dimensions produce an `88:63` ratio within 2%.

- [X] T008 [US2] Add card proportion tests in `bevy/crates/game/src/runtime/resources/mod.rs`
- [X] T009 [US2] Use the card defaults to build a thin no-bevel cuboid mesh in `bevy/crates/game/src/runtime/systems/mod.rs`

---

## Phase 5: User Story 3 - Smooth Mouse-Driven Card Rotation (Priority: P1)

**Goal**: Rotate only the card toward normalized pointer positions with smooth 100 ms response and 20 degree limits.

**Independent Test**: Automated tests confirm pointer target mapping, smoothing behavior, and fixed camera stability.

- [X] T010 [US3] Add pointer mapping and smoothing tests in `bevy/crates/game/src/runtime/plugins/mod.rs`
- [X] T011 [US3] Implement cursor-to-target-rotation tracking in `bevy/crates/game/src/runtime/systems/mod.rs`
- [X] T012 [US3] Implement smoothed card transform rotation in `bevy/crates/game/src/runtime/systems/mod.rs`

---

## Phase 6: User Story 4 - Keep V0.1 Rendering Minimal (Priority: P2)

**Goal**: Keep the card visually plain and avoid gameplay or extra rendering scope.

**Independent Test**: Source and startup tests show one white material, no asset texture, and no gameplay card systems.

- [X] T013 [US4] Add material and V0.1 scope assertions in `bevy/crates/game/src/runtime/plugins/mod.rs`
- [X] T014 [US4] Keep placeholder material plain white and untextured in `bevy/crates/game/src/runtime/systems/mod.rs`

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Validate implementation and update task status.

- [X] T015 Run `scripts/other/RunTests.ps1`
- [X] T016 Update completed task checkboxes in `specs/004-card-inspection-poc/tasks.md`

## Dependencies & Execution Order

| Phase | Depends On | Can Run In Parallel |
| ---- | ---- | ---- |
| Setup | None | T001 and T002 |
| Foundational | Setup | T003 and T004 after inspection |
| US1 | Foundational | Tests before implementation |
| US2 | Foundational | After card defaults exist |
| US3 | Foundational and US1 | Pointer tracking before smoothing |
| US4 | US1 and US2 | Material checks after spawn behavior |
| Polish | Desired stories complete | Verification only after code is complete |

## Implementation Strategy

| Step | Scope |
| ---- | ---- |
| MVP | Complete setup, foundational, US1, and US2 so launch shows one correctly proportioned card |
| Interaction | Complete US3 so pointer motion smoothly tilts only the card |
| Scope Guard | Complete US4 to keep V0.1 minimal |
| Validation | Run repository tests and document any desktop/browser verification gaps |
