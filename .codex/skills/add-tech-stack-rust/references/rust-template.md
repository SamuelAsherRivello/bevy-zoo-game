# Rust Template Reference

Reference repo: `https://github.com/SamuelAsherRivello/rust-project-template`

## Signals To Import

| Area | Reference Signal |
| ---- | ---------------- |
| Root purpose | Starting point for Rust projects with hot reload and Rust coding standards. |
| Root files | `.aiignore`, `.gitignore`, `AGENTS.md`, `Cargo.toml`, `Cargo.lock`, `README.md`. |
| Source layout | `rust/crates/game`, `rust/crates/game_api`, and `rust/crates/game_shell` show split crates and API/shell separation. |
| Scripts | `scripts/InstallProject.ps1`, `BuildProject.ps1`, `RunProject.ps1`, `RunProjectWithHotReload.ps1`, `StopProject.ps1`. |
| Philosophy | Keep setup repeatable, hot reload available, and crate responsibilities explicit. |

## Adaptation Rules

- Use the reference repo's workspace and script philosophy, but rename folders to match the generated project's purpose.
- Prefer root `scripts` for this repository unless the generated Rust project intentionally adopts a different convention.
- Keep Rust rules in `AGENTS.md` concise: formatting, ownership, errors, tests, and avoiding broad refactors.
- Add Cargo files only when the user explicitly wants implementation, not for planning-only tasks.
