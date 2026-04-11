# Codex Orchestrator Installer Script Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: Use bounded execution, file-backed progress, and explicit verification. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a one-click install script that installs the plugin, marketplace entry, bundled agents, and Codex plugin enablement into the user's local Codex environment.

**Architecture:** The installer is a repository-owned Bash script with a small amount of `node`-based JSON editing for marketplace registration and config-file block updates for Codex plugin enablement. It supports `link`, `copy`, and `dry-run` behavior and preserves existing agent files by backing them up before replacement.

**Tech Stack:** Bash, Node.js, JSON file editing, text-based TOML section updates, local filesystem operations.

---

## Context

- The plugin is implemented and file-backed, but installation is still manual.
- The installer needs to target local Codex conventions rather than repository-local scaffolding.
- Existing user agent files must be handled safely.
- Codex will not treat the plugin as enabled unless `~/.codex/config.toml` includes the plugin enablement block.

## Artifact Model

- Spec path: `docs/specs/2026-04-10-codex-orchestrator-installer-design.md`
- Completed plan path: `docs/plans/completed/2026-04-10-codex-orchestrator-installer-implementation.md`
- Script path: `scripts/install-codex-orchestrator.sh`
- Test path: `tests/install-script.test.ts`
- Config enablement target: `~/.codex/config.toml`

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| I1. Create installer spec and active plan | None | Establishes the execution contract |
| I2. Implement installer script | I1 | Script behavior depends on agreed targets and safety model |
| I3. Add config enablement handling | I2 | Codex config enablement is required for real activation |
| I4. Add installer tests and docs | I2, I3 | Tests and docs depend on the implemented script and config behavior |

## Parallel Execution Graph

Wave 1:
- I1. Create installer spec and active plan

Wave 2:
- I2. Implement installer script

Wave 3:
- I3. Add config enablement handling

Wave 4:
- I4. Add installer tests and docs

## Quality Gates

- Dry-run performs no writes.
- Marketplace registration is idempotent.
- Agent conflicts are backed up before replacement.
- Codex plugin enablement is written or updated idempotently.

## Execution Status

- Current wave: Completed
- Active task: none
- Blockers: None
- Last review result: I4 quality pass

## TODO List

- [x] I1. Create Installer Spec And Active Plan
- [x] I2. Implement Installer Script
- [x] I3. Add Config Enablement Handling
- [x] I4. Add Installer Tests And Docs

### Task I1: Create Installer Spec And Active Plan

**Files:**
- Create: `docs/specs/2026-04-10-codex-orchestrator-installer-design.md`
- Create: `docs/plans/completed/2026-04-10-codex-orchestrator-installer-implementation.md`
- Modify: `AGENTS.md`
- Modify: `docs/index.md`
- Modify: `task_plan.md`
- Modify: `progress.md`

**Category:** plan
**Owner Role:** harness-planner
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Create the installer design spec
- [x] Step 2: Create the installer active implementation plan
- [x] Step 3: Update routing docs to point at the installer plan
- [x] Step 4: Verify the installer plan is the execution anchor

### Task I2: Implement Installer Script

**Files:**
- Create: `scripts/install-codex-orchestrator.sh`
- Modify: `README.md`
- Modify: `docs/plans/completed/2026-04-10-codex-orchestrator-installer-implementation.md`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Implement install target and option parsing
- [x] Step 2: Implement plugin install and marketplace registration
- [x] Step 3: Implement agent backup and install behavior
- [x] Step 4: Sync the plan

### Task I3: Add Config Enablement Handling

**Files:**
- Modify: `scripts/install-codex-orchestrator.sh`
- Modify: `tests/install-script.test.ts`
- Modify: `docs/specs/2026-04-10-codex-orchestrator-installer-design.md`
- Modify: `docs/plans/completed/2026-04-10-codex-orchestrator-installer-implementation.md`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add config-path handling to the installer
- [x] Step 2: Write or update the plugin enablement block in Codex config
- [x] Step 3: Add tests for config enablement behavior
- [x] Step 4: Sync the plan

### Task I4: Add Installer Tests And Docs

**Files:**
- Create: `tests/install-script.test.ts`
- Modify: `README.md`
- Modify: `progress.md`
- Modify: `findings.md`
- Modify: `docs/plans/completed/2026-04-10-codex-orchestrator-installer-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add dry-run and real-install tests
- [x] Step 2: Run the installer tests
- [x] Step 3: Update docs and progress
- [x] Step 4: Mark final acceptance state

## Final Acceptance

- [x] Installer script exists
- [x] Dry-run is verified
- [x] Marketplace registration is verified
- [x] Agent backup and install behavior is verified
- [x] Codex config enablement is verified
