# Codex Orchestrator Completed Plan Auto-Archive Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: Use bounded execution, file-backed progress, explicit verification, and repair stale completed plans in `docs/plans/active/` on first touch. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Automatically move fully completed implementation plans from `docs/plans/active/` to `docs/plans/completed/`, while repairing legacy completed plans left in `active/` and synchronizing all routing surfaces.

**Architecture:** `PlanDocument` becomes the path-reconciliation layer for implementation plans. It detects completed active plans, archives them into the completed-plan root, and preserves compatibility for stale callers. Tests and routing docs enforce the archive lifecycle.

**Tech Stack:** TypeScript, Node.js test runner, markdown routing docs.

---

## Context

- The repository already has separate `active/` and `completed/` plan directories, but completed plans still remain under `active/`.
- `PlanDocument` currently writes back to a fixed `planPath` and has no archive behavior.
- Historical completed-plan links in routing docs still point into `docs/plans/active/`.
- First-touch hygiene repair should happen automatically without asking the user.

## Artifact Model

- Spec path: `docs/specs/2026-04-11-codex-orchestrator-plan-archive-design.md`
- Completed plan path: `docs/plans/completed/2026-04-11-codex-orchestrator-plan-archive-implementation.md`
- Implementation surfaces:
  - `plugins/codex-orchestrator/src/services/plan-document.ts`
  - `plugins/codex-orchestrator/src/tools/register-tools.ts`
- Test surfaces:
  - `plugins/codex-orchestrator/tests/plan-document.test.ts`
  - `plugins/codex-orchestrator/tests/tools.test.ts`
- Routing and architecture docs:
  - `AGENTS.md`
  - `docs/index.md`
  - `docs/architecture/plan-sync-rules.md`
  - `docs/plans/completed/README.md`
  - `plugins/codex-orchestrator/codex/agents/harness-planner.toml`
  - `plugins/codex-orchestrator/codex/agents/search-specialist.toml`
  - `plugins/codex-orchestrator/codex/agents/harness-evaluator.toml`

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| P1. Create archive spec and execution plan | None | Establishes the contract and execution anchor |
| P2. Add failing archive regression coverage | P1 | Red-phase tests should target the approved archive rules |
| P3. Implement plan auto-archive and stale-path reconciliation | P2 | Production code follows failing tests |
| P4. Repair historical completed plans and routing references | P3 | Legacy cleanup depends on the archive behavior and final file locations |
| P5. Verify archive behavior and sync status docs | P4 | Verification depends on both code and repository cleanup |

## Quality Gates

- A completed plan under `docs/plans/active/` auto-moves to `docs/plans/completed/`.
- Reading a legacy completed active plan repairs it immediately.
- Stale active-path callers can still open the archived plan through `PlanDocument`.
- Routing docs describe `active/` as live work and `completed/` as historical reference.
- Repository tests pass after the archive cleanup.

## Execution Status

- Current wave: Completed
- Active task: none
- Blockers: None
- Last review result: full verification pass

## TODO List

- [x] P1. Create Archive Spec And Execution Plan
- [x] P2. Add Failing Archive Regression Coverage
- [x] P3. Implement Plan Auto-Archive And Stale-Path Reconciliation
- [x] P4. Repair Historical Completed Plans And Routing References
- [x] P5. Verify Archive Behavior And Sync Status Docs

### Task P1: Create Archive Spec And Execution Plan

**Files:**
- Create: `docs/specs/2026-04-11-codex-orchestrator-plan-archive-design.md`
- Create: `docs/plans/completed/2026-04-11-codex-orchestrator-plan-archive-implementation.md`
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

- [x] Step 1: Write the completed-plan auto-archive design spec
- [x] Step 2: Write the active implementation plan
- [x] Step 3: Switch routing docs to the new execution anchor
- [x] Step 4: Verify the new plan is the current execution source

### Task P2: Add Failing Archive Regression Coverage

**Files:**
- Modify: `plugins/codex-orchestrator/tests/plan-document.test.ts`
- Modify: `plugins/codex-orchestrator/tests/tools.test.ts`
- Modify: `docs/plans/completed/2026-04-11-codex-orchestrator-plan-archive-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add archive-on-read and archive-on-completion regression tests
- [x] Step 2: Run targeted tests and verify the red phase
- [x] Step 3: Sync the plan after the failing run

### Task P3: Implement Plan Auto-Archive And Stale-Path Reconciliation

**Files:**
- Modify: `plugins/codex-orchestrator/src/services/plan-document.ts`
- Modify: `plugins/codex-orchestrator/src/tools/register-tools.ts`
- Modify: `docs/architecture/plan-sync-rules.md`
- Modify: `docs/plans/completed/README.md`
- Modify: `docs/plans/completed/2026-04-11-codex-orchestrator-plan-archive-implementation.md`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add plan-path reconciliation and archive-on-completion logic to `PlanDocument`
- [x] Step 2: Keep tool-facing plan-path persistence aligned with the effective archived location
- [x] Step 3: Document the archive trigger in architecture docs
- [x] Step 4: Sync the plan after implementation turns green

### Task P4: Repair Historical Completed Plans And Routing References

**Files:**
- Modify: `AGENTS.md`
- Modify: `docs/index.md`
- Modify: `docs/specs/2026-04-08-codex-orchestrator-plugin-design.md`
- Modify: `docs/specs/2026-04-09-codex-orchestrator-bundled-agents-design.md`
- Modify: `plugins/codex-orchestrator/codex/agents/harness-planner.toml`
- Modify: `plugins/codex-orchestrator/codex/agents/search-specialist.toml`
- Modify: `plugins/codex-orchestrator/codex/agents/harness-evaluator.toml`
- Modify: `docs/plans/completed/2026-04-11-codex-orchestrator-plan-archive-implementation.md`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Archive the legacy completed plans currently left under `docs/plans/active/`
- [x] Step 2: Repair routing-doc links and plan-root descriptions to match the archive model
- [x] Step 3: Repair bundled-agent guidance to mention active and completed plan roots correctly
- [x] Step 4: Sync the plan after repository cleanup

### Task P5: Verify Archive Behavior And Sync Status Docs

**Files:**
- Modify: `task_plan.md`
- Modify: `progress.md`
- Modify: `findings.md`
- Modify: `docs/index.md`
- Modify: `AGENTS.md`
- Modify: `docs/plans/completed/2026-04-11-codex-orchestrator-plan-archive-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Run the full plugin test suite
- [x] Step 2: Verify completed plans no longer remain in `docs/plans/active/`
- [x] Step 3: Sync findings, progress, and routing docs
- [x] Step 4: Mark final acceptance state

## Final Acceptance

- [x] Completed active plans auto-move into `docs/plans/completed/`
- [x] First-touch reads repair stale completed plans left in `docs/plans/active/`
- [x] Routing docs no longer link completed plans through `docs/plans/active/`
- [x] Plugin tests pass with archive coverage
