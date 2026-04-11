# Task Plan

## Goal

Align codex-orchestrator with Codex's marketplace-driven installation model and bootstrap a real local install into the user's Codex environment.

## Phases

| Phase | Status | Notes |
|---|---|---|
| 1. Create marketplace-install spec and execution plan | complete | New design spec and active plan created, routing switched |
| 2. Add repo marketplace and update installer behavior | complete | Repo discovery, personal bootstrap install, and installed cache staging implemented |
| 3. Validate marketplace/bootstrap install and sync docs | complete | Tests passed, docs updated, and local Windows Codex install performed |

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
- Marketplace-install work is now complete and validated.

## Open Questions

- Host support for automatic registration of plugin-bundled `.toml` agents still depends on the surrounding Codex runtime, so the bundle docs must keep a fallback install path explicit.
- The Codex app still needs a restart after external marketplace/install changes before its plugin browser picks up the new local source and enabled state.

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
- Created the installer design spec and installer active implementation plan
- Implemented the one-click installer script and verified dry-run, copy install, marketplace registration, and agent backup behavior
- Began marketplace-install redesign so installation aligns with Codex's current plugin marketplace model
- Added a repo-local marketplace, updated the installer to bootstrap personal plugin source/cache/config state, and installed the plugin into the local Windows Codex home
