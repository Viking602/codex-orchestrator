# Codex Orchestrator Install Guide Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: Use bounded execution, file-backed progress, and explicit verification. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a root-level `install.md` that lets a shell-capable AI agent install and verify the `codex-orchestrator` plugin without asking the user to edit files manually.

**Architecture:** The installer remains the sole supported installation mechanism. The repository gains an AI-oriented root install guide, a doc regression test that keeps the guide aligned with the current installer contract, and routing-doc links so agents can discover the guide quickly.

**Tech Stack:** Markdown documentation, Node test runner, existing installer test suite.

---

## Context

- The installer and README already describe the supported bootstrap flow.
- The repository lacks a dedicated AI-facing installation entrypoint.
- Routing docs should point at the new guide so it is discoverable.

## Artifact Model

- Spec path: `docs/specs/2026-04-11-codex-orchestrator-install-guide-design.md`
- Active plan path: `docs/plans/active/2026-04-11-codex-orchestrator-install-guide-implementation.md`
- Root install guide: `install.md`
- Installer regression test: `tests/install-script.test.ts`
- Routing docs:
  - `README.md`
  - `AGENTS.md`
  - `docs/index.md`
  - `task_plan.md`

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| I1. Create install-guide spec and active plan | None | Establishes the contract |
| I2. Add failing install-guide regression test | I1 | Red-phase test should target the documented contract |
| I3. Write install.md and route to it | I2 | Production docs follow the failing test |
| I4. Verify tests and sync repository status docs | I3 | Verification depends on green docs and tests |

## Quality Gates

- `install.md` exists at the repository root.
- The guide tells an AI agent to run the installer and verification commands itself.
- Routing docs reference the guide.
- The installer test suite passes after the guide is added.

## Execution Status

- Current wave: Completed
- Active task: none
- Blockers: None
- Last review result: local quality pass

## TODO List

- [x] I1. Create Install-Guide Spec And Active Plan
- [x] I2. Add Failing Install-Guide Regression Test
- [x] I3. Write Install.md And Route To It
- [x] I4. Verify Tests And Sync Repository Status Docs

### Task I1: Create Install-Guide Spec And Active Plan

**Files:**
- Create: `docs/specs/2026-04-11-codex-orchestrator-install-guide-design.md`
- Create: `docs/plans/active/2026-04-11-codex-orchestrator-install-guide-implementation.md`
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

- [x] Step 1: Write the install-guide design spec
- [x] Step 2: Write the install-guide active implementation plan
- [x] Step 3: Switch routing docs to the new execution anchor
- [x] Step 4: Verify the new plan is the active execution source

### Task I2: Add Failing Install-Guide Regression Test

**Files:**
- Modify: `tests/install-script.test.ts`
- Modify: `docs/plans/active/2026-04-11-codex-orchestrator-install-guide-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add a failing test requiring `install.md`
- [x] Step 2: Add assertions for the supported install command and verification references
- [x] Step 3: Run the installer test suite to verify the red phase
- [x] Step 4: Sync the plan after the failing test run

### Task I3: Write Install.md And Route To It

**Files:**
- Create: `install.md`
- Modify: `README.md`
- Modify: `AGENTS.md`
- Modify: `docs/index.md`
- Modify: `task_plan.md`
- Modify: `docs/plans/active/2026-04-11-codex-orchestrator-install-guide-implementation.md`

**Category:** docs
**Owner Role:** harness-doc-gardener
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Write the root install guide for AI-driven installation
- [x] Step 2: Link the guide from README and routing docs
- [x] Step 3: Re-read the guide for install-contract accuracy
- [x] Step 4: Sync the plan after the green doc pass

### Task I4: Verify Tests And Sync Repository Status Docs

**Files:**
- Modify: `progress.md`
- Modify: `findings.md`
- Modify: `task_plan.md`
- Modify: `docs/index.md`
- Modify: `AGENTS.md`
- Modify: `docs/plans/active/2026-04-11-codex-orchestrator-install-guide-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Run fresh verification commands for the guide and installer tests
- [x] Step 2: Inspect the final guide content and routing links
- [x] Step 3: Sync progress, findings, and task summary
- [x] Step 4: Mark final acceptance state

## Final Acceptance

- [x] `install.md` exists
- [x] The guide is AI-oriented and installer-backed
- [x] Routing docs reference the guide
- [x] Installer tests are green
