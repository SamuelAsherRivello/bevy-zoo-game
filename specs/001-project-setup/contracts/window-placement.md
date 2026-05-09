# Contract: Window Placement

| Contract Item | Requirement |
| ------------- | ----------- |
| Source ownership | Window defaulting, placement loading, placement validation, and placement saving live in `bevy/crates/shared`; `bevy/crates/game` composes the shared behavior only |
| Default size | The desktop window opens at 1024x768 when no valid placement exists |
| Save timing | Placement is written only on normal window close |
| Saved fields | Saved placement includes x/y position, size, screen name when available, screen position, screen size, and window-relative-to-screen position |
| Same-screen restore | When the saved screen is available and the placement is visible, the app restores x/y and size within 20 physical pixels |
| Two-screen support | Restoring must work when the saved screen is either screen in a two-screen setup |
| Invalid/off-screen fallback | The app opens centered on the primary screen at 1024x768 |
| Local state | Placement file lives under ignored generated runtime state and is not a source artifact |
| Browser behavior | Browser startup is not blocked by desktop placement persistence |
