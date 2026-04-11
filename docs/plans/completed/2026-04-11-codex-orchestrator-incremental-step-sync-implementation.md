# Codex Orchestrator Incremental Step Synchronization Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: Use bounded execution, file-backed progress, explicit verification, and record step progress incrementally instead of batching step completion at the end. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make gradual step progress the default plugin behavior by auto-seeding and auto-advancing `Current Step`, exposing machine-readable step guidance through `next_action`, and detecting step-sync drift.

**Architecture:** The change stays within the existing control plane. `register-tools.ts` becomes the step-guidance contract surface, while `PlanDocument` remains the markdown truth layer. Planning and routing docs reinforce smaller visible progress units so top-level TODO progression does not stay artificially opaque.

**Tech Stack:** TypeScript, Node.js test runner, markdown routing docs, bundled agent instructions.

---

## Context

- The plugin already exposes `orchestrator_begin_step` and `orchestrator_complete_step`.
- Parent agents can still skip explicit per-step synchronization and batch progress updates late.
- `orchestrator_next_action` currently returns task-level actions without step-level machine guidance.
- Users therefore see long stretches where the first TODO appears stuck before multiple completions land at once.

## Artifact Model

- Spec path: `docs/specs/2026-04-11-codex-orchestrator-incremental-step-sync-design.md`
- Completed plan path: `docs/plans/completed/2026-04-11-codex-orchestrator-incremental-step-sync-implementation.md`
- Implementation surfaces:
  - `plugins/codex-orchestrator/src/tools/register-tools.ts`
  - `plugins/codex-orchestrator/src/services/plan-document.ts`
- Test surfaces:
  - `plugins/codex-orchestrator/tests/tools.test.ts`
  - `plugins/codex-orchestrator/tests/plan-document.test.ts`
- Routing and guidance surfaces:
  - `AGENTS.md`
  - `docs/index.md`
  - `docs/architecture/mcp-tool-contract.md`
  - `docs/architecture/plan-sync-rules.md`
  - `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
  - `plugins/codex-orchestrator/codex/agents/harness-planner.toml`

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| P1. Create step-sync spec and active plan | None | Establishes the contract and execution anchor |
| P2. Add failing regression coverage for gradual step sync | P1 | Red-phase tests should target the approved contract |
| P3. Implement step guidance, seeding, and auto-advance | P2 | Production code follows failing tests |
| P4. Update routing and planning guidance | P3 | Documentation should describe the implemented behavior |
| P5. Verify the new sync path and close the plan | P4 | Verification depends on the final code and docs |

## Quality Gates

- Starting a task seeds `Current Step` to the first unchecked step.
- Completing a step auto-advances `Current Step` to the next unchecked step when available.
- `orchestrator_next_action` returns step guidance and drift metadata.
- Watchdog or next-action output can identify missing or stale step synchronization.
- Tests pass and docs state that step progress must update incrementally.

## Execution Status

- Current wave: Completed
- Active task: none
- Blockers: None
- Last review result: full verification pass

## TODO List

- [x] P1. Create Step-Sync Spec And Active Plan
- [x] P2. Add Failing Regression Coverage For Gradual Step Sync
- [x] P3. Implement Step Guidance, Seeding, And Auto-Advance
- [x] P4. Update Routing And Planning Guidance
- [x] P5. Verify The New Sync Path And Close The Plan

### Task P1: Create Step-Sync Spec And Active Plan

**Files:**
- Create: `docs/specs/2026-04-11-codex-orchestrator-incremental-step-sync-design.md`
- Create: `docs/plans/completed/2026-04-11-codex-orchestrator-incremental-step-sync-implementation.md`
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

- [x] Step 1: Write the incremental step synchronization design spec
- [x] Step 2: Write the active implementation plan
- [x] Step 3: Switch routing docs to the new active execution anchor
- [x] Step 4: Verify the repository now points at the new plan

### Task P2: Add Failing Regression Coverage For Gradual Step Sync

**Files:**
- Modify: `plugins/codex-orchestrator/tests/tools.test.ts`
- Modify: `docs/plans/completed/2026-04-11-codex-orchestrator-incremental-step-sync-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add failing tests for task-start step seeding and step auto-advance
- [x] Step 2: Add failing tests for next_action step guidance and sync drift detection
- [x] Step 3: Run targeted tests and verify the red phase

### Task P3: Implement Step Guidance, Seeding, And Auto-Advance

**Files:**
- Modify: `plugins/codex-orchestrator/src/tools/register-tools.ts`
- Modify: `plugins/codex-orchestrator/src/services/plan-document.ts`
- Modify: `docs/plans/completed/2026-04-11-codex-orchestrator-incremental-step-sync-implementation.md`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add shared helpers for next unchecked step and step-sync metadata
- [x] Step 2: Seed the first step in begin_task and auto-advance in complete_step
- [x] Step 3: Return step guidance and drift metadata from next_action and watchdog paths
- [x] Step 4: Sync the plan after implementation turns green

### Task P4: Update Routing And Planning Guidance

**Files:**
- Modify: `docs/architecture/plan-sync-rules.md`
- Modify: `docs/architecture/mcp-tool-contract.md`
- Modify: `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
- Modify: `plugins/codex-orchestrator/codex/agents/harness-planner.toml`
- Modify: `AGENTS.md`
- Modify: `docs/index.md`
- Modify: `docs/plans/completed/2026-04-11-codex-orchestrator-incremental-step-sync-implementation.md`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Document immediate step seeding and auto-advance rules
- [x] Step 2: Strengthen planner guidance toward smaller visible progress units
- [x] Step 3: Sync routing docs with the new step-sync contract

### Task P5: Verify The New Sync Path And Close The Plan

**Files:**
- Modify: `task_plan.md`
- Modify: `progress.md`
- Modify: `findings.md`
- Modify: `docs/index.md`
- Modify: `AGENTS.md`
- Modify: `plugins/codex-orchestrator/tests/plan-document.test.ts`
- Modify: `docs/plans/completed/2026-04-11-codex-orchestrator-incremental-step-sync-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Run the full plugin test suite
- [x] Step 2: Verify step guidance appears in tool outputs and docs
- [x] Step 3: Sync findings, progress, and routing docs
- [x] Step 4: Mark final acceptance state

## Final Acceptance

- [x] Task start seeds the first unchecked step automatically
- [x] Step completion auto-advances progress to the next unchecked step when available
- [x] `orchestrator_next_action` exposes machine-readable step guidance and sync drift
- [x] Planning guidance now biases toward smaller visible progress units
- [x] Plugin tests pass with the new step-sync coverage
