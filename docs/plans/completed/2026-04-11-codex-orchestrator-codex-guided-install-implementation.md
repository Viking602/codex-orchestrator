# Codex Orchestrator Codex-Guided Install Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: keep the repository docs truthful, remove the shell-installer path from active guidance, and replace it with a Codex-run install/update guide. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Remove the shell-script installer from the current install flow and replace it with a direct Codex guide that tells an AI agent how to install and update the plugin itself.

**Architecture:** `install.md` becomes the install/update source of truth. Current docs route to that guide, the shell installer leaves the active repository surface, and regression coverage verifies the guide contract instead of wrapper-script behavior.

**Tech Stack:** Markdown docs, Node.js test runner, repository file operations described for Codex execution.

---

## Context

- The current install flow is still script-first.
- The desired flow is Codex-first: the guide should tell an AI agent what to copy, update, and verify directly.
- Current tests still assume the shell installer is the supported path.

## Artifact Model

- Spec path: `docs/specs/2026-04-11-codex-orchestrator-codex-guided-install-design.md`
- Completed plan path: `docs/plans/completed/2026-04-11-codex-orchestrator-codex-guided-install-implementation.md`
- Current install surfaces:
  - `install.md`
  - `README.md`
  - `AGENTS.md`
  - `docs/index.md`
- Test surfaces:
  - `tests/install-guide.test.ts`
- Removed surface:
  - `scripts/install-codex-orchestrator.sh`

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| P1. Create Codex-guided install spec and active plan | None | Establishes the execution anchor |
| P2. Add failing guide-contract regression coverage | P1 | Red-phase tests should target the new install contract |
| P3. Replace script-first docs with Codex-guided install and update instructions | P2 | Production docs follow failing tests |
| P4. Remove the shell installer from the active repository surface | P3 | Removal depends on the replacement flow being documented |
| P5. Verify the new install flow and close the plan | P4 | Verification depends on final docs and file layout |

## Quality Gates

- Current docs no longer present the shell installer as the supported install flow.
- `install.md` gives Codex direct install and update guidance.
- README routes to the direct Codex guide.
- The shell installer file is removed.
- Regression tests cover the new guide contract and pass.

## Execution Status

- Current wave: Completed
- Active task: none
- Blockers: None
- Last review result: full verification pass

## TODO List

- [x] P1. Create Codex-Guided Install Spec And Active Plan
- [x] P2. Add Failing Guide-Contract Regression Coverage
- [x] P3. Replace Script-First Docs With Codex-Guided Install And Update Instructions
- [x] P4. Remove The Shell Installer From The Active Repository Surface
- [x] P5. Verify The New Install Flow And Close The Plan

### Task P1: Create Codex-Guided Install Spec And Active Plan

**Files:**
- Create: `docs/specs/2026-04-11-codex-orchestrator-codex-guided-install-design.md`
- Create: `docs/plans/completed/2026-04-11-codex-orchestrator-codex-guided-install-implementation.md`
- Modify: `docs/index.md`
- Modify: `AGENTS.md`
- Modify: `task_plan.md`

**Category:** plan
**Owner Role:** harness-planner
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Write the Codex-guided install design spec
- [x] Step 2: Write the active implementation plan
- [x] Step 3: Switch routing docs to the new active execution anchor
- [x] Step 4: Verify the repository now points at the new plan

### Task P2: Add Failing Guide-Contract Regression Coverage

**Files:**
- Modify: `tests/install-guide.test.ts`
- Modify: `docs/plans/completed/2026-04-11-codex-orchestrator-codex-guided-install-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Replace installer-script tests with guide-contract expectations
- [x] Step 2: Run targeted tests and verify the red phase

### Task P3: Replace Script-First Docs With Codex-Guided Install And Update Instructions

**Files:**
- Modify: `install.md`
- Modify: `README.md`
- Modify: `AGENTS.md`
- Modify: `docs/index.md`
- Modify: `docs/plans/completed/2026-04-11-codex-orchestrator-codex-guided-install-implementation.md`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Rewrite the install guide around direct Codex install and update actions
- [x] Step 2: Rewrite README install routing to point at the direct guide
- [x] Step 3: Sync the plan after the docs turn green

### Task P4: Remove The Shell Installer From The Active Repository Surface

**Files:**
- Delete: `scripts/install-codex-orchestrator.sh`
- Modify: `tests/install-guide.test.ts`
- Modify: `docs/plans/completed/2026-04-11-codex-orchestrator-codex-guided-install-implementation.md`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Remove the shell installer file
- [x] Step 2: Ensure current tests and docs no longer depend on it

### Task P5: Verify The New Install Flow And Close The Plan

**Files:**
- Modify: `task_plan.md`
- Modify: `progress.md`
- Modify: `findings.md`
- Modify: `docs/index.md`
- Modify: `AGENTS.md`
- Modify: `docs/plans/completed/2026-04-11-codex-orchestrator-codex-guided-install-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Run the relevant test suite
- [x] Step 2: Verify current docs no longer route through the shell installer
- [x] Step 3: Sync findings, progress, and routing docs
- [x] Step 4: Mark final acceptance state

## Final Acceptance

- [x] Current install guidance no longer presents the shell installer as the supported path
- [x] `install.md` tells Codex how to install and update the plugin directly
- [x] README points to the direct Codex install guide
- [x] The shell installer file is removed from the active repository surface
- [x] Regression tests pass with the new guide contract
