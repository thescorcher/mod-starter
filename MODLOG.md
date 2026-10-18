# MODLOG

Newest entries at the top. Entry format:

```markdown
## [Date] [short title]

**Changed:** what was changed, and in which files
**Why:** the problem or goal
**Tested how:** what you did to check it (played the game, read logs, ran a test)
**Result:** what happened, with numbers or log lines where possible
**Still broken / not tested:** be honest
**Next:** what to do next
```

## {{DATE}} Project scaffold

**Changed:** Created rules (`AGENTS.md`, `CLAUDE.md`), `docs/CONTRACT.md`, `docs/DESIGN.md`, whitelist
`.gitignore`, a Rust workspace (`guest-core`, `guest-data`, `host-bridge`) and loader config.
**Why:** Set up the project the way ai-modding-guides recommends before writing real code.
**Tested how:** Not tested in game. Run `cargo test -p guest-core` to check the core.
**Result:** Files only.
**Still broken / not tested:** Everything in game. Loader config not checked
against the installed loader version.
**Next:** Record exact game and loader versions, then start Milestone 1 (see `docs/DESIGN.md`).
