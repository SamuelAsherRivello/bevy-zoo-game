# Bevy Zoo Game Agent Instructions

This repository is a Bevy ECS zoo game built from the Codex Project Template. Keep Codex and Specify infrastructure at the root, and keep game implementation under the Bevy workspace.

## Git Safety

- Never run destructive Git operations.
- Git use is limited to inspection and additive work.
- Do not delete repositories, branches, tags, commits, or history.
- Do not rewrite history, squash commits, amend commits, rebase, or force-push.
- Do not run commands that discard local file changes.
- Allowed Git operations are `git status`, `git diff`, `git log`, `git show`, `git fetch`, `git add`, `git commit`, and normal non-force `git push`.
- If a task appears to require destructive Git, stop and explain that the operation is not permitted.

## Secret And Credential Safety

- Never ask the user to paste passwords, private keys, seed phrases, API keys, cookies, session tokens, or database credentials into chat.
- Never print secrets from local files, remote files, environment variables, service settings, or config files.
- If a task needs a secret, instruct the user to enter it directly into the target app, terminal, secret manager, or hosting provider.
- Do not copy secrets into generated files, logs, commits, pull requests, screenshots, or Markdown notes.

## Workspace Scope

- Stay inside the current repository unless the user explicitly names another path or repository in the current request.
- Keep generated files, scratch files, downloaded assets, caches, and temporary outputs inside this repository.
- Do not change global editor, shell, Git, SSH, service, or machine configuration unless the user explicitly asks for that class of change.
- Do not expose local-only services, admin tools, databases, or app internals to the public internet without explicit approval.

## Project Workflow

- Inspect current files before editing; keep changes scoped to the requested behavior.
- Prefer specs before implementation when behavior is not already defined.
- Keep active specs under `specs/`.
- Keep Specify configuration and templates under `.specify/`.
- Keep Codex guidance, reusable skills, rules, and memory under `.codex/`.
- Follow `.codex/rules/bevy-runtime-structure.md` for Bevy crate ownership, ECS runtime structure, plugin wiring, hot-reload rules, tests, assets, and path-casing guardrails.
- Keep reusable non-game project assets under `bevy/crates/game/assets/`.
- Keep Bevy runtime assets under `bevy/crates/game/assets/`.
- Keep repeatable project scripts under `scripts/`.
- Keep README-visible images under `documentation/images/`.
- Keep Bevy source organized by ECS role under `bevy/crates/game/src/runtime/`.
- Keep desktop window defaults in `bevy/crates/shared/src/window.rs`; update `DEFAULT_WINDOW_WIDTH` and `DEFAULT_WINDOW_HEIGHT` there when changing the project-approved launch/fallback size.
- Load runtime assets lazily by default. Do not block scene startup, scene switches, or interaction on model/asset readiness unless the behavior technically requires it, and do not add preloaders for assets that can appear after the scene is usable.
- For the GameScene, spawn model scene handles immediately and let Bevy load them asynchronously without waiting. For the Model Browser, request model loads in grid order starting from the upper-left cell; completion may pop in asynchronously in any order.
- Use `scripts/main/InstallDependencies.ps1` once per machine to verify Rust setup, then use `scripts/other/RunTests.ps1`, `scripts/other/RunAppDesktop.ps1`, `scripts/other/RunAppWeb.ps1`, and `scripts/other/StopApp.ps1` for repeatable local workflows.

<!-- SPECKIT START -->
Active implementation plan: `specs/004-card-inspection-poc/plan.md`
<!-- SPECKIT END -->

## Markdown Format Rules

| Rule | Requirement |
| ---- | ----------- |
| Listed information | When Markdown content lists multiple related items, prefer a table over bullets or numbered lists unless the content is procedural code guidance or a short nested explanation. |
| Boolean table cells | When a table cell would otherwise say `Yes` or `No`, use ✅ for yes and ❌ for no. |
