# Design

Plain-language notes on how the project works. Update when the architecture changes.

## Shape
Three pieces, so the guest rules can be built and tested without launching a game:

1. **`guest-core`** holds {{GUEST_GAME}}'s rules as ordinary Rust: a fixed tick that takes the
   player's input and surroundings (`HostInput`) and returns what the player should do (`CoreOutput`).
   It knows nothing about {{HOST_GAME}} or Windows, so `cargo test` checks it in seconds.
2. **`guest-data`** reads numbers from the player's own {{GUEST_GAME}} install and writes them to
   `local/guest/`. Nothing it produces goes into Git.
3. **`host-bridge`** is the DLL that {{LOADER}} loads into {{HOST_GAME}}. Each frame it reads the
   player's state from game memory, feeds `guest-core`, and writes the result back. If the core keeps
   failing, it hands control back to the stock game.

## Status
- `guest-core` contains one example mechanic (jump physics) with placeholder constants marked `UNVERIFIED`.
- `host-bridge` only writes a log line on load. It does not touch game memory yet.
- `guest-data` only checks the install path and lists files.

## Next
Milestone 1: one {{GUEST_GAME}} mechanic working in {{HOST_GAME}}.
1. Pick the mechanic that most defines how {{GUEST_GAME}} feels. Measure it from recorded footage and,
   if found, from the game's code. Write the numbers and their source into `guest-core`.
2. Find where {{HOST_GAME}} keeps the player's position, velocity and grounded state
   (field notes, community tables, `um`).
3. Drive that one value from `guest-core`; log it every frame.
4. Playtest and compare against {{GUEST_GAME}} footage.
