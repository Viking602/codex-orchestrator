# Codex Orchestrator Native Codex Todo Mirroring Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: keep the implementation plan authoritative, mirror it into Codex native `update_plan` when available, and do not create a separate prose todo. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make Codex's native todo UI a projection of the active implementation plan by exposing a mirror-ready MCP snapshot and requiring the parent to use that snapshot instead of inventing a separate todo list.

**Architecture:** The markdown plan remains the source of truth. `register-tools.ts` exports a native-todo projection, and workflow docs make the parent responsible for feeding that projection into native `update_plan`.

**Tech Stack:** TypeScript, Node.js test runner, markdown routing docs, Codex native `update_plan`.

---

## Context

- The plugin already tracks top-level task completion and current-step progress in the plan and runtime store.
- Codex native todo updates are parent-owned, not callable from inside the plugin.
- Without a mirror-ready contract, parents can drift into separate chat todos even when the built-in Codex todo exists.

## Artifact Model

- Spec path: `docs/specs/2026-04-11-codex-orchestrator-native-codex-todo-mirroring-design.md`
- Completed plan path: `docs/plans/completed/2026-04-11-codex-orchestrator-native-codex-todo-mirroring-implementation.md`
- Implementation surfaces:
  - `plugins/codex-orchestrator/src/tools/register-tools.ts`
- Test surfaces:
  - `plugins/codex-orchestrator/tests/tools.test.ts`
- Routing and guidance surfaces:
  - `AGENTS.md`
  - `docs/index.md`
  - `docs/architecture/mcp-tool-contract.md`
  - `docs/architecture/agent-contracts.md`
  - `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
  - `scripts/install-codex-orchestrator.sh`

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| P1. Create native-todo mirroring spec and active plan | None | Establishes the execution anchor |
| P2. Add failing mirror-export regression coverage | P1 | Red-phase tests should target the approved contract |
| P3. Implement mirror-ready Codex todo export | P2 | Production code follows failing tests |
| P4. Update workflow guidance to require native todo mirroring | P3 | Parent behavior should follow the implemented contract |
| P5. Verify the mirror path and close the plan | P4 | Verification depends on final code and docs |

## Quality Gates

- The plugin exposes a mirror-ready snapshot for native Codex todo updates.
- Exactly one task is `in_progress` when work remains.
- The mirrored active todo includes current-step detail when available.
- Final acceptance appears when it is the only remaining work.
- Workflow docs require native todo mirroring instead of separate chat todos.

## Execution Status

- Current wave: Completed
- Active task: none
- Blockers: None
- Last review result: full verification pass

## TODO List

- [x] P1. Create Native-Todo Mirroring Spec And Active Plan
- [x] P2. Add Failing Mirror-Export Regression Coverage
- [x] P3. Implement Mirror-Ready Codex Todo Export
- [x] P4. Update Workflow Guidance To Require Native Todo Mirroring
- [x] P5. Verify The Mirror Path And Close The Plan

### Task P1: Create Native-Todo Mirroring Spec And Active Plan

**Files:**
- Create: `docs/specs/2026-04-11-codex-orchestrator-native-codex-todo-mirroring-design.md`
- Create: `docs/plans/completed/2026-04-11-codex-orchestrator-native-codex-todo-mirroring-implementation.md`
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

- [x] Step 1: Write the native-todo mirroring design spec
- [x] Step 2: Write the active implementation plan
- [x] Step 3: Switch routing docs to the new active execution anchor
- [x] Step 4: Verify the repository now points at the new plan

### Task P2: Add Failing Mirror-Export Regression Coverage

**Files:**
- Modify: `plugins/codex-orchestrator/tests/tools.test.ts`
- Modify: `docs/plans/completed/2026-04-11-codex-orchestrator-native-codex-todo-mirroring-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add failing tests for native todo export of active task progress
- [x] Step 2: Add failing tests for final-acceptance mirroring
- [x] Step 3: Run targeted tests and verify the red phase

### Task P3: Implement Mirror-Ready Codex Todo Export

**Files:**
- Modify: `plugins/codex-orchestrator/src/tools/register-tools.ts`
- Modify: `docs/plans/completed/2026-04-11-codex-orchestrator-native-codex-todo-mirroring-implementation.md`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add a mirror builder for Codex native todo items
- [x] Step 2: Expose the new MCP export tool and include current-step detail
- [x] Step 3: Sync the plan after implementation turns green

### Task P4: Update Workflow Guidance To Require Native Todo Mirroring

**Files:**
- Modify: `docs/architecture/mcp-tool-contract.md`
- Modify: `docs/architecture/agent-contracts.md`
- Modify: `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
- Modify: `AGENTS.md`
- Modify: `docs/index.md`
- Modify: `scripts/install-codex-orchestrator.sh`
- Modify: `docs/plans/completed/2026-04-11-codex-orchestrator-native-codex-todo-mirroring-implementation.md`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Document the native-todo export contract
- [x] Step 2: Require native `update_plan` mirroring when available
- [x] Step 3: Explicitly forbid separate prose todo lists for plan-tracked work

### Task P5: Verify The Mirror Path And Close The Plan

**Files:**
- Modify: `task_plan.md`
- Modify: `progress.md`
- Modify: `findings.md`
- Modify: `docs/index.md`
- Modify: `AGENTS.md`
- Modify: `docs/plans/completed/2026-04-11-codex-orchestrator-native-codex-todo-mirroring-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Run the full plugin test suite
- [x] Step 2: Verify native-todo mirror outputs through tests
- [x] Step 3: Sync findings, progress, and routing docs
- [x] Step 4: Mark final acceptance state

## Final Acceptance

- [x] The plugin exports a mirror-ready native Codex todo snapshot
- [x] The mirrored active item includes current-step detail when available
- [x] Final acceptance appears when it is the only remaining work
- [x] Workflow docs require native `update_plan` mirroring instead of separate chat todos
- [x] Plugin tests pass with native-todo mirroring coverage
