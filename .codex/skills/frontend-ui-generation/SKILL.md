---
name: frontend-ui-generation
description: Generate and refine frontend UI for local-first applications. Use when Codex is asked to build or polish Dioxus 0.7, React, TypeScript, CSS, layout, interaction states, browser-visible behavior, dashboards, tools, forms, or Unity-adjacent web UI, with responsive verification and practical design choices.
---

# Frontend UI Generation

## Workflow

Build the actual usable interface first, not a marketing placeholder. Match existing design systems and code style before introducing new components.

1. Inspect current routes, components, CSS, assets, localization, and app scripts.
2. Define the primary workflow and the data/state surfaces the user needs.
3. Implement dense, usable controls with loading, empty, error, disabled, hover/focus, and success states.
4. For Dioxus, use 0.7 APIs: `#[component]`, `Element`, `use_signal`, `use_memo`, `use_resource`, `Router`, and `asset!`; do not use `cx`, `Scope`, or `use_state`.
5. For React/TypeScript, prefer typed components, controlled state, accessible labels, and existing UI libraries.
6. Preserve local-first behavior; do not add cloud dependencies unless requested.
7. Verify in a real browser or runtime when practical; hand off to `qa-workflows` for deeper smoke tests.

## Visual Rules

Use restrained, production UI for tools, dashboards, and engineering apps. Avoid decorative filler, fragile responsive sizing, text overflow, and single-hue palettes. Use icons for recognizable actions when a library is already present.

## PowerShell Verification

```powershell
.\Scripts\Common\RunAppWeb.ps1
npm run dev
npm run build
cargo check
```

If a browser-visible change is made, include the served URL or explain why runtime verification was not run.
