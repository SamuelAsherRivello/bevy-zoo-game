---
name: dioxus-project-template
description: Work on the Dioxus Project Template Rust workspace. Use when Codex is asked to modify, debug, verify, or explain this repository, especially Dioxus 0.7 UI, routing, assets, localization, browser localStorage snapshots, native SQLite template data, or project scripts.
---

# Dioxus Project Template

Use this skill for repository-specific execution context. Follow `AGENTS.md` and `.codex/rules/dioxus-0.7-workflow.md` first for Dioxus 0.7 rules.

## Start Here

1. Confirm the current directory is `D:\Documents\Projects\VC\Rust\dioxus-project-template`.
2. Read `.codex/rules/dioxus-0.7-workflow.md` for Dioxus implementation work.
3. Read the files directly involved in the request before editing.
4. Keep web and desktop support intact unless the request is explicitly platform-specific.
5. Prefer the existing `packages/ui`, `packages/web`, and `packages/desktop` boundaries.
6. Identify whether the request targets the Template Project or Generated Project payload before editing root docs, `.specs/template/`, `.specs/generated/`, or specs.
7. Use the project scripts before inventing new commands.

## Workspace Map

| Path | Use |
| ---- | --- |
| `packages/ui/src/client/app.rs` | Shared app shell entry component. |
| `packages/ui/src/client/components` | Shared Dioxus components. |
| `packages/ui/src/client/pages` | Routed template pages. |
| `packages/ui/src/client/services` | Client template data, storage, localization, and database services. |
| `packages/ui/assets` | Shared CSS and localization/flag assets. |
| `packages/web/src/main.rs` | Web entrypoint. |
| `packages/desktop/src/main.rs` | Desktop entrypoint. |
| `README.md` | Root documentation, including the Dioxus Features matrix that should stay current as development continues. |
| `Documentation/Images` | README-visible image assets that must be refreshed after this skill is run. |
| `.specs/template` | Template Project guidance and specs for maintaining this reusable template. |
| `.specs/generated` | Generated Project root replacements and starter specs. |
| `Scripts` | Windows PowerShell setup and run workflows. |

## Dioxus 0.7 Constraints

- Use Dioxus `0.7` patterns from the checked-in workspace dependency.
- Do not use `cx`, `Scope`, or `use_state`.
- Use `#[component] fn Name(...) -> Element`.
- Use `use_signal`, `use_memo`, `use_resource`, and signal `.read()`, `.write()`, `.with_mut()`, or call syntax.
- Use `Router::<Route> {}` and Dioxus router links for routing.
- Use `asset!("/path/from/project/root")` for local assets.
- Keep props owned, `Clone`, and `PartialEq`.

## Cache And Loading Behavior

- Preserve visible loading affordances during cache reads, database creation, and refreshes.
- Browser builds use localStorage snapshots instead of browser SQLite.
- Non-wasm builds use native SQLite under local `data/`.
- Put first-time native database/schema/seed setup in `create_database_if_missing()` and keep normal reads non-destructive.
- Treat stale dev servers as a common source of false browser results.

## Verification

Use the narrowest check that proves the change:

```powershell
cargo check -p ui --target wasm32-unknown-unknown
cargo check -p web --target wasm32-unknown-unknown
cargo check -p desktop
.\Scripts\Common\RunAppWeb.ps1
.\Scripts\Other\RunAppDesktop.ps1
.\Scripts\Other\RunTests.ps1
```

For browser UI, routing, asset, or cache changes, serve the web app and inspect the actual page when practical.

## Image Refresh Requirement

After running this skill, update the README image assets in `Documentation/Images`:

- `Documentation/Images/Infographic01.png` must be a new custom ChatGPT Images 2.0 or better infographic covering the current repo.
- `Documentation/Images/Screenshot01.png` must be a screenshot of the actual running web app, not a mockup or generated UI image.

Refresh these files in place so the root README image links remain valid. Serve the real web app before capturing `Screenshot01.png`, and verify both images exist before finishing.
