# {{PROJECT_NAME}}

{{SUMMARY}} Single-player and offline only.

No game files are included. You need your own copies of {{HOST_GAME}} and {{GUEST_GAME}}.
Guest data is read from your install on your machine.

## What works
- Nothing yet. The project is scaffolded; nothing has been built or tested in game.

## What doesn't work yet
- Everything. Milestone 1 is in `docs/DESIGN.md`.

## Requirements
- Windows 11, {{HOST_GAME}}, {{GUEST_GAME}}
- [Rust](https://rustup.rs) stable with the MSVC toolchain, plus Visual Studio Build Tools 2022
- {{LOADER}}

## Build and run
```
cargo test -p guest-core
cargo build --release -p host-bridge
```
Then load `target/release/host_bridge.dll` with {{LOADER}}: see `loader/README.md`.

## Project files
- `AGENTS.md`: rules for the AI agent working on this project
- `docs/DESIGN.md`: how it works
- `docs/CONTRACT.md`: what the host owns and what the guest core owns
- `MODLOG.md`: what changed and how it was tested
- `playtests/`: in-game test reports
- `CREDITS.md`: where things came from
