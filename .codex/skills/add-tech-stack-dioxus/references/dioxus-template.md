# Dioxus Template Reference

Reference repo: `https://github.com/SamuelAsherRivello/dioxus-project-template`

## Signals To Import

| Area | Reference Signal |
| ---- | ---------------- |
| Root purpose | Starting point for Dioxus full-stack projects with hot reload and Rust coding standards. |
| Codex/Specify | `.agents`, `.codex`, `.specify`, and `.specs` are first-class planning and agent surfaces. |
| App layout | `packages/ui`, `packages/web`, and `packages/desktop` split shared UI from platform entrypoints. |
| Scripts | `Scripts/Common/InstallDependencies.ps1`, `RunAppWeb.ps1`, and other web/desktop/test server helpers. |
| Dioxus rules | Use Dioxus 0.7 patterns: components return `Element`, state uses signals/resources/memos/context, routes use `Routable`. |
| README discipline | Keep Dioxus feature/component matrices, routes, cache behavior, platform support, and images aligned with implementation. |

## Adaptation Rules

- Add Dioxus only when the generated project is intentionally a Dioxus app.
- Keep shared app behavior in a shared UI layer and keep web/desktop entrypoints thin when both platforms are selected.
- Preserve visible feedback for async loading, cache reads/writes, and setup flows.
- Keep first-time database or storage setup explicit and do not make normal reads recreate or reseed data.
