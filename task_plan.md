# Task Plan

## Goal

Bundle the plugin's default Codex agents with the repository so planning, research, MCP implementation, review, dispatch, and routing-doc maintenance roles travel with the plugin instead of depending on implicit host setup.

## Phases

| Phase | Status | Notes |
|---|---|---|
| 1. Create bundled-agent spec and execution plan | complete | New active plan and design spec created, routing switched |
| 2. Vendor and customize bundled agents | complete | Selected six roles, added plugin-owned Codex `.toml` copies, documented provenance |
| 3. Wire plugin metadata and add drift checks | complete | Updated `openai.yaml`, skill docs, tests, and verification flow |

## Current Decisions

- The implementation plan file is the execution source of truth.
- Runtime state is auxiliary and must not replace plan checkbox truth.
- The plugin should replace the core orchestration workflow, not every domain skill.
- The repository now has an explicit file-backed artifact model and routing entrypoints.
- Phase 1 is complete; phase 2 is now the active execution track.
- Phase 2 implementation is now complete and validated.
- Phase 3 is now the active execution track.
- Phase 3 implementation is now complete and validated.
- The plugin should ship only the happy-path default roles, not a full third-party agent catalog.
- Bundled agent files under `plugins/codex-orchestrator/codex/agents/` are plugin-owned local derivatives.
- The default implementation role should stay generic to coding work and avoid both niche MCP specialization and overfitting to one language persona.

## Open Questions

- Host support for automatic registration of plugin-bundled `.toml` agents still depends on the surrounding Codex runtime, so the bundle docs must keep a fallback install path explicit.

## Completed This Session

- Created routing docs and planning files
- Wrote the design specification
- Wrote the active implementation plan
- Added artifact-model directory entrypoints for architecture, product, decisions, and completed plans
- Implemented the plugin shell, zero-third-party stdio MCP server, and test suite
- Completed the active implementation plan and synchronized final acceptance state
- Created the phase 2 design spec and phase 2 active implementation plan
- Completed phase 2 lease enforcement, watchdog hardening, and parent next-action protocol
- Created the phase 3 design spec and phase 3 active implementation plan
- Completed phase 3 question gating, completion assessment, review/repair derivation, and completion guard
- Created the bundled-agent design spec and active implementation plan
- Bundled six Codex agent definitions under `plugins/codex-orchestrator/codex/agents/`
- Added plugin-specific agent-surface wiring and drift tests for category preferred roles
