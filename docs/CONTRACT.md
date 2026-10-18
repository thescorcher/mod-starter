# Bridge contract

Template from `references/ai-modding-guides/templates/BRIDGE-CONTRACT.md`. Check every bridge change
against this file. Fill each UNDECIDED once it is confirmed from code or measurement.

## Route
{{ROUTE}}. {{ROUTE_NOTE}}

## Programs and exact versions
- Host: {{HOST_GAME}}, version UNDECIDED (record from `um scan`), loader {{LOADER}} version UNDECIDED
- Guest: {{GUEST_GAME}}, version UNDECIDED
- OS / translation layer: Windows 11, native

## Ownership
| Thing | Owner | How control is handed back, and how it returns |
|-------|-------|------------------------------------------------|
| Player position and physics | {{PLAYER_OWNER}} | UNDECIDED (menus, cutscenes, vehicles, ladders usually go back to the host) |
| Camera | UNDECIDED | |
| World geometry | {{HOST_GAME}} | |
| Collision (host → guest) | UNDECIDED | |
| Collision (guest → host) | UNDECIDED | |
| NPCs / enemies | UNDECIDED | |
| Damage and health | UNDECIDED | |
| Inventory | UNDECIDED | |
| Saves | UNDECIDED | |
| Menus, pause, loading screens | {{HOST_GAME}} | `guest-core` stops ticking; resumes on return |
| Cutscenes, vehicles, furniture, scripted events | {{HOST_GAME}} | Stock host behaviour while active |

## Units and axes
- Distance: UNDECIDED (measure the same character or object in both games)
- Axes: UNDECIDED (host up-axis, guest up-axis, handedness)
- Angles: radians in `guest-core`
- Health/damage: UNDECIDED
- Time: `guest-core` steps at a fixed rate (`TICK_HZ`), decoupled from the host frame rate

## Messages
| Channel | Kind (snapshot / queue / image) | Direction | Rate | What happens if full or late |
|---------|---------------------------------|-----------|------|------------------------------|
| `HostInput` | snapshot | host → core | every host frame | Core uses the last snapshot |
| `CoreOutput` | snapshot | core → host | every core tick | Host applies the latest; none → stock behaviour |

- Protocol: Rust structs in `guest-core` when in-process; for cross-process routes, UNDECIDED
  (shared memory with magic, version and generation counter is the usual choice)
- How each side detects the other restarting: not applicable in-process; otherwise UNDECIDED

## Frames (only if images cross)
- Not applicable unless the route is frame compositing.

## Lifecycle
- Start order: loader injects `host_bridge.dll` → DLL starts a worker thread → waits until the player exists
- On pause / menu: core stops ticking
- On loading a save or changing area: UNDECIDED
- On death: UNDECIDED
- On crash of the core: catch the panic, log it, return to stock behaviour; three panics within 5 s disables the core until restart

## Not covered (be honest)
- Online play (never). Everything else: UNDECIDED

## UNDECIDED
- Exact game and loader versions
- Unit scale and axes
- Where the host keeps player position, velocity and grounded state
- Every ownership row marked UNDECIDED above
