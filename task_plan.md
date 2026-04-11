# Task Plan

## Goal

Remove the shell-installer path from the current install flow and replace it with a direct Codex guide for plugin installation and updates.

## Phases

| Phase | Status | Notes |
|---|---|---|
| 1. Create Codex-guided install spec and execution plan | completed | The new spec and active plan are now the execution anchor for direct install/update guidance |
| 2. Add regression coverage for the guide contract | completed | The guide-contract tests replaced the old installer-script suite and captured the red phase |
| 3. Replace script-first install flow with direct Codex guidance | completed | The docs, tests, and active repository surface now route through the direct guide and the closeout plan is archived |

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
- Default workflow routing should be driven by `AGENTS.md` plus bundled skill discovery, not by plugin install state alone.
- The installer should manage the active global `AGENTS` file so default routing survives outside this repository's local chat history.
- Delegation preference should be part of the category contract instead of being implicit in role names or parent prose.
- Parent orchestration should remain the control plane while ordinary plan, research, implementation, and review work bias to child execution.
- AI-facing installation guidance should instruct Codex to reconcile plugin files directly instead of routing through a repository-owned installer wrapper.
- The root install guide should be the fastest path for agent-driven install and verification work.
- Install and update should share one idempotent Codex-driven reconcile flow across plugin files, marketplace state, config, AGENTS, and bundled agents.
- Repository markdown docs must use repo-relative artifact links and portable placeholders instead of machine-specific absolute filesystem paths.
- Legacy absolute-path docs are hygiene debt and should be repaired on first touch without escalating to the user.
- Completed implementation plans should not remain under `docs/plans/active/`; stale plan placement is hygiene debt and should be repaired automatically.
- Gradual progress requires plugin-side step guidance, not only prompt-level advice, because parents otherwise batch step updates too easily.
- Incremental step synchronization should be contract-driven: task start seeds the first open step, step completion advances the pointer, and next-action/watchdog outputs expose drift explicitly.
- Plan-path reconciliation must handle repo-relative `docs/plans/active/...` inputs as well as absolute filesystem paths, because the routing docs intentionally expose relative artifact links.
- The implementation plan remains the progress source of truth; Codex native todo should be a mirror of that plan, not an independent planning surface.
- The correct Codex integration point is a mirror-ready export tool plus parent-owned `update_plan`, not an attempt to make the plugin itself mutate native UI state.
- Current install/update guidance should target Codex directly; repository-owned wrapper scripts are no longer the desired primary interface.

## Open Questions

- Host support for automatic registration of plugin-bundled `.toml` agents still depends on the surrounding Codex runtime, so the bundle docs must keep a fallback install path explicit.
- The Codex app still needs a restart after external marketplace/install changes before its plugin browser picks up the new local source and enabled state.
- The active global `AGENTS` file may be `~/.codex/AGENTS.override.md` or `~/.codex/AGENTS.md`, so installer routing bootstrap must target the active one.
- Bundled fallback agent installation still creates duplicate-role warnings when the host already has the same role names installed globally.

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
- Began default-workflow routing redesign so Codex prefers codex-orchestrator without requiring manual `@` invocation
- Added installer-managed global `AGENTS` bootstrap, strengthened plugin routing metadata, refreshed the local install, and verified a fresh Codex run now defaults to `codex-orchestrator` for repository work
- Began delegation-first dispatch redesign so the parent classifies work first and then makes a machine-readable subagent intervention decision
- Implemented explicit delegation preference in the category contract and exposed `requires_subagent`, `dispatch_role`, `intervention_reason`, and `dispatch_mode` through `orchestrator_next_action`
- Tightened delegation behavior so write-lease acquisition stays parent-owned, in-progress review work continues the same child agent, and terminal next-action payloads keep the same contract shape
- Began install-guide work so installation and verification can be handled directly by an AI agent through a single root-level document
- Added `install.md`, linked it from the routing surfaces, and added a regression test that keeps the guide aligned with the supported installer path
- Began relative doc-path policy work so repo docs stop embedding maintainer-specific filesystem locations
- Repaired legacy absolute-path markdown links in routing and product docs, replaced hard-coded machine-path examples with portable forms, and added a regression scan that fails when absolute filesystem paths reappear
- Began completed-plan auto-archive work so fully accepted plans leave `docs/plans/active/` automatically and historical stale placement is repaired on first touch
- Added completed-plan auto-archive to `PlanDocument`, including stale active-path resolution, CRLF-safe parsing, and legacy-plan support when older plans omit a `Final Acceptance` section
- Archived the repository's historical completed plans out of `docs/plans/active/` and repaired routing/agent guidance to treat `active/` as live work and `completed/` as history
- Began incremental step synchronization work so parent agents receive explicit step guidance instead of relying on late batch step updates
- Completed incremental step synchronization so task start seeds the first unchecked step, step completion auto-advances, and next-action/watchdog outputs surface step drift explicitly
- Fixed completed-plan auto-archive for repo-relative plan paths so completed work still migrates cleanly when agents follow repository-relative routing links
- Began native Codex todo mirroring work so parents can drive the built-in `update_plan` from the active implementation plan instead of inventing a separate todo list
- Completed native Codex todo mirroring so the plugin exports a mirror-ready todo snapshot and the parent workflow now treats native `update_plan` as the canonical UI surface for plan progress
- Began Codex-guided install work so installation and updates stop routing through the repository-owned shell wrapper
- Replaced the shell-installer-first install surface with a direct Codex install/update guide, deleted the repository-owned installer script, and renamed the regression suite to `tests/install-guide.test.ts`
- Verified the guide-contract suite and full repository test suite, then archived the completed Codex-guided install plan out of `docs/plans/active/`
