# Loader: Ultimate ASI Loader (most native C/C++ games)

Ultimate ASI Loader (ThirteenAG) is a proxy DLL (`dinput8.dll`, `version.dll`, `winmm.dll`, ...)
that loads `*.asi` plugins. An `.asi` is just a DLL renamed.

1. `cargo build --release -p host-bridge`
2. Copy `target/release/host_bridge.dll` to `<scripts or game folder>/host_bridge.asi`.

This step writes into the game folder, so the agent must ask first (AGENTS.md rule 4). Keep a list of
every file added to the game folder in `loader/INSTALLED.md` so it can be removed cleanly.

- Check the proxy DLL name the game actually loads (`um scan` and the loader's README).
- 32-bit games need `rustup target add i686-pc-windows-msvc` and `--target i686-pc-windows-msvc`.
- GTA V and RDR2 use ScriptHookV/ScriptHookRDR2 with the same `.asi` model.
