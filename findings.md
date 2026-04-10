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
- Phase 2 adds lease-aware enforcement and deterministic parent action derivation instead of relying on parent free-form judgment.
- `orchestrator_next_action` now makes the plan plus runtime state machine-readable for the parent agent.
- Phase 2 write leases can be acquired and released independently of plan truth, which keeps runtime control separate from acceptance truth.
- Watchdog and next_action now use the same category-aware decision basis, reducing parent-side improvisation.
- Phase 3 closes the gap between child self-report and parent acceptance by introducing explicit completion assessment.
- Completion guard gives the parent a fail-closed answer before it can claim 100 percent completion.
- Review and repair loop semantics are now documented explicitly instead of being implicit in parent behavior.
- The plugin's preferred-role path was still dependent on external host agent inventory even after phase 3 completed.
- The smallest coherent bundled set should prefer a generic coding role such as `backend-developer`, not a niche MCP-specialist implementation role.
- `harness-generator` is not a good default bundled implementer here because the plugin already has a stronger generic implementation role.
- Bundled agent files should be treated as plugin-owned derivatives with source attribution, not as runtime pointers back to the source repos.
- Category preferred-role drift is cheap to test and should fail in CI-local verification before bundle docs silently go stale.
