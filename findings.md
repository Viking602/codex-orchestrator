# Findings

## 2026-04-08

- The target repository is currently empty, so the artifact model can be established cleanly without migration debt.
- The desired plugin should replace core orchestration flow now, not the entire superpowers ecosystem.
- The plugin must absorb the control-plane responsibilities currently provided by `harness-engineering`, `brainstorming`, `writing-plans`, subagent execution discipline, and code review gating.
- The plugin should still rely on Codex native subagents for execution instead of spawning external Codex CLI sessions.
- OpenAgent's strongest transferable patterns are category-based routing, file-backed plans, runtime state, review gates, and continuation-driven completion pressure.
- Category semantics need to encode workflow behavior, not only model preference.
- Runtime state should support orchestration and recovery, but plan markdown remains the final source of truth.
- Real-time plan synchronization needs explicit machine-updatable task fields to avoid batch completion drift.
- The empty repository made it safe to establish the full artifact model without migration complexity.
- The first working implementation uses a zero-third-party stdio MCP runtime on Node.js because network installs for the MCP SDK were unreliable in this environment.
- The implemented MCP surface is sufficient for category resolution, plan reads, task/step updates, review recording, acceptance gating, doc drift checks, and watchdog queries.
