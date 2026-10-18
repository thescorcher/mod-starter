# Mod starter

A starter for "one game inside another" mods built with an AI agent, set up the way
[AI Game Modding Guides](https://gitlab.com/trevaintdead/ai-modding-guides) recommends: one folder,
Git from day one, a rules file, a bridge contract, a running log and playtest reports.

## Make a project

```
python new_project.py
```

It asks for the host game, guest game, route (guide 11), loader and paths, then creates a new folder
next to this one with everything filled in. Or pass everything at once:

```
python new_project.py --name "Keyblade Ring" --host "Elden Ring" --host-path "E:\steam\steamapps\common\ELDEN RING" --guest "Kingdom Hearts II Final Mix" --route 7 --loader me3 --me3-game eldenring
```

## What you get

| Path | What it is |
|------|------------|
| `AGENTS.md`, `CLAUDE.md` | Rules the agent follows every session |
| `docs/CONTRACT.md` | Who owns what between host and guest, with UNDECIDED rows to fill |
| `docs/DESIGN.md` | How it works and Milestone 1 |
| `MODLOG.md`, `playtests/`, `CREDITS.md` | Log, test reports, attribution |
| `crates/guest-core` | Guest rules as plain Rust, unit-tested (starts with an example jump mechanic) |
| `crates/guest-data` | Reads the player's own guest install into `local/guest/` |
| `crates/host-bridge` | DLL the loader puts into the host game (logs only, to start) |
| `loader/` | Config and steps for the chosen loader: `me3` (FromSoft), `asi` (most native games), or `other` |
| `.gitignore` | Whitelist: only source files can be committed |
| `local/`, `references/` | Game-derived data and reference guides, never committed |

## Which routes and loaders fit

- The Rust crates fit **route 7** (rebuilt mechanic inside the host) and the host side of
  **route 1** (live passthrough) with native-DLL loaders (me3, Ultimate ASI Loader, ScriptHook).
- For Unity, Bethesda, Unreal or Minecraft hosts, pick `--loader other`: `loader/README.md` explains
  how to keep `guest-core` and swap the bridge for that engine's mod language.
- For route 5 (engine recreation) or 6 (asset conversion), the notes in `AGENTS.md` say which crate
  becomes the main one.
