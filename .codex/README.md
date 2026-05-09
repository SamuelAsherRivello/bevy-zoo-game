# Codex Project Template

Use this folder for Codex-specific project context that should travel with this repository.

## Project

`codex-project-template` is a project-neutral starter for Codex and Specify workflows.

Use `.codex/project-identity.md` as the first checklist when changing this template into a new named project.

| Path | Purpose |
| ---- | ------- |
| `.codex/rules` | Repo-local rules that generated projects can extend. |
| `.codex/skills` | Reusable Codex workflow skills. |
| `.codex/memory` | Durable notes about project conventions and decisions. |
| `.agents/skills` | Specify workflow skills. |
| `.specify` | Specify configuration, templates, scripts, and constitution. |
| `specs` | Active feature specifications. |
| `bevy/crates/game/assets` | Runtime game assets and fixtures. |
| `scripts` | Repeatable local project scripts. |
| `documentation/images` | README-visible image assets. |

## AI Maintenance Notes

| Topic | Update Path |
| ----- | ----------- |
| Desktop window default size | Edit `DEFAULT_WINDOW_WIDTH` and `DEFAULT_WINDOW_HEIGHT` in `bevy/crates/shared/src/window.rs`, then update `specs/001-project-setup/spec.md` if the approved launch/fallback size changes. |

## Default Workflow

1. Confirm project identity and scope.
2. Write or update the relevant spec.
3. Create a plan and task list.
4. Implement in the generated project's chosen source layout.
5. Verify behavior with the generated project's documented commands.
6. Refresh README and image assets when visible structure or workflow changes.

## Technology Stack Skills

| Skill | Purpose |
| ----- | ------- |
| `add-tech-stack` | Shared setup philosophy for adding a project-specific stack without losing the generic Codex/Specify foundation. |
| `add-tech-stack-rust` | Rust overlay based on `https://github.com/SamuelAsherRivello/rust-project-template`. |
| `add-tech-stack-bevy` | Bevy overlay based on `https://github.com/SamuelAsherRivello/bevy-project-template`. |
| `add-tech-stack-dioxus` | Dioxus overlay based on `https://github.com/SamuelAsherRivello/dioxus-project-template`. |
