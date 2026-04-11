# Codex Orchestrator Marketplace Install Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: Use bounded execution, file-backed progress, and explicit verification. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Align codex-orchestrator with Codex's marketplace-driven plugin installation model and bootstrap a real local install into the user's Codex environment.

**Architecture:** The repository gains a repo-local marketplace for discovery, while the installer becomes a personal bootstrap tool that writes the personal marketplace, personal plugin source tree, installed cache copy, and enabled plugin state. Bundled agent copying remains as a compatibility fallback rather than the primary contract.

**Tech Stack:** JSON marketplace metadata, Bash installer scripting, Node.js file editing, text-based TOML updates, Node test runner.

---

## Context

- The current installer plan completed against an older direct-install mental model.
- Current Codex docs use marketplaces as the supported plugin discovery/install surface.
- The plugin must become discoverable in the Codex app and directly installable into the local Codex environment without manual user edits.

## Artifact Model

- Spec path: `docs/specs/2026-04-11-codex-orchestrator-marketplace-install-design.md`
- Active plan path: `docs/plans/active/2026-04-11-codex-orchestrator-marketplace-install-implementation.md`
- Repo marketplace path: `.agents/plugins/marketplace.json`
- Installer path: `scripts/install-codex-orchestrator.sh`
- Test path: `tests/install-script.test.ts`
- Config target: `~/.codex/config.toml`

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| M1. Create marketplace-install spec and active plan | None | Defines the new execution contract |
| M2. Add repo marketplace and update installer paths | M1 | Installation behavior depends on the documented contract |
| M3. Add tests and docs for marketplace/bootstrap install | M2 | Tests and docs depend on the new installer behavior |
| M4. Verify and perform local install | M2, M3 | Local installation should happen only after the code and docs are aligned |

## Quality Gates

- Repo marketplace metadata resolves to `./plugins/codex-orchestrator`.
- Installer writes no files in dry-run mode.
- Installer bootstraps personal marketplace, personal plugin source, installed cache copy, and enabled plugin state.
- Installer no longer forces `features.apps = true`.
- Conflicting bundled agent files are still backed up before replacement.

## Execution Status

- Current wave: Completed
- Active task: none
- Blockers: None
- Last review result: local quality pass

## TODO List

- [x] M1. Create Marketplace-Install Spec And Active Plan
- [x] M2. Add Repo Marketplace And Update Installer Paths
- [x] M3. Add Tests And Docs For Marketplace/Bootstrap Install
- [x] M4. Verify And Perform Local Install

### Task M1: Create Marketplace-Install Spec And Active Plan

**Files:**
- Create: `docs/specs/2026-04-11-codex-orchestrator-marketplace-install-design.md`
- Create: `docs/plans/active/2026-04-11-codex-orchestrator-marketplace-install-implementation.md`
- Modify: `AGENTS.md`
- Modify: `docs/index.md`
- Modify: `task_plan.md`

**Category:** plan
**Owner Role:** harness-planner
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Create the marketplace-install design spec
- [x] Step 2: Create the marketplace-install active implementation plan
- [x] Step 3: Update routing docs to point at the marketplace-install plan
- [x] Step 4: Verify the marketplace-install plan is the execution anchor

### Task M2: Add Repo Marketplace And Update Installer Paths

**Files:**
- Create: `.agents/plugins/marketplace.json`
- Modify: `scripts/install-codex-orchestrator.sh`
- Modify: `README.md`
- Modify: `docs/specs/2026-04-11-codex-orchestrator-marketplace-install-design.md`
- Modify: `docs/plans/active/2026-04-11-codex-orchestrator-marketplace-install-implementation.md`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add a failing test for repo marketplace and personal bootstrap install behavior
- [x] Step 2: Run the install-script tests to verify the new checks fail
- [x] Step 3: Add the repo marketplace file and update installer defaults/behavior
- [x] Step 4: Re-run the install-script tests and sync the plan

### Task M3: Add Tests And Docs For Marketplace/Bootstrap Install

**Files:**
- Modify: `tests/install-script.test.ts`
- Modify: `README.md`
- Modify: `findings.md`
- Modify: `progress.md`
- Modify: `task_plan.md`
- Modify: `docs/index.md`
- Modify: `AGENTS.md`
- Modify: `docs/plans/active/2026-04-11-codex-orchestrator-marketplace-install-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Expand installer tests for config and cache behavior
- [x] Step 2: Update repository docs to describe repo marketplace discovery and bootstrap install
- [x] Step 3: Run the relevant tests and sync progress/findings docs
- [x] Step 4: Mark documentation and verification acceptance state

### Task M4: Verify And Perform Local Install

**Files:**
- Modify: `docs/plans/active/2026-04-11-codex-orchestrator-marketplace-install-implementation.md`
- Modify: `progress.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Run the final installer verification commands
- [x] Step 2: Run the installer against the local Codex home
- [x] Step 3: Inspect marketplace, cache, and config results
- [x] Step 4: Mark final acceptance state

## Final Acceptance

- [x] Repo marketplace exists
- [x] Personal marketplace bootstrap is verified
- [x] Installed cache copy is verified
- [x] Plugin enabled state is verified
- [x] Bundled agent fallback install is verified
