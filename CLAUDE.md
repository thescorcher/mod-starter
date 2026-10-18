@AGENTS.md

## Claude Code specifics
- The universal-modder plugin is installed. At the start, run `um scan` on both games, then read
  the plugin's engine playbook for the host (`/universal-modder:mod-any-game`) and search its
  knowledge base for field notes on both games (`um kb search "<game>"`). For two-game work, also
  read `/universal-modder:mashup-mods`.
- Use `um scan`, `um backup` and `um win` rather than ad-hoc scripts where they fit.
- For long stretches, the plugin's `mod-orchestrator` agent can run a game worker and an offline
  worker. It costs several times the tokens; ask me first.
- `references/ai-modding-guides/` is the community guide set. Guides 11 (routes), 12 (case
  studies), 13 (ownership, sync) and 14 (decompile system map) apply most.
