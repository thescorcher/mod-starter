# Loader: {{LOADER}}

Fill in how this loader loads code, where its files live, and the exact build-and-launch steps.

The Rust `host-bridge` DLL fits loaders that load native DLLs. Some engines want another language:

| Engine / loader | Mod language | What to do with `host-bridge` |
|-----------------|--------------|-------------------------------|
| Unity + BepInEx/MelonLoader | C# | Write a C# plugin; call `guest-core` through a C ABI (`cdylib`) or port it |
| Bethesda + SKSE/F4SE | C++ (CommonLibSSE) | Thin C++ plugin that calls `guest-core` through a C ABI |
| Unreal + UE4SS | C++ or Lua | C++ mod calling `guest-core` through a C ABI |
| Minecraft + Fabric | Java | Fabric mod; usually port the rules or run `guest-core` out-of-process |
| .NET / XNA games (tModLoader, SMAPI) | C# | As Unity |

`guest-core` stays the same either way. Record the decision in `docs/DESIGN.md`.
