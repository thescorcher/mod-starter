# Loader: me3 (FromSoftware games)

me3 launches the game offline, without Easy Anti-Cheat, and loads `host_bridge.dll` plus any
file-replacement folder from this project. Nothing is copied into the game folder.

```
cargo build --release -p host-bridge
me3 launch --profile loader/{{PROJECT_SLUG}}.me3
```

- Never go online with this profile.
- Field names in the profile were written for me3 profile v1. Check them against the installed
  me3 version's docs (not tested).
- Prior art: universal-modder's FromSoft playbook and `knowledge/games/elden-ring/` field notes.
