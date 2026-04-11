# Codex Orchestrator Delegation-First Dispatch Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: Use bounded execution, file-backed progress, explicit verification, and delegation-first parent behavior. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make `codex-orchestrator` classify work first, then explicitly decide whether subagent intervention is required, with a stronger bias toward child execution for normal repository tasks.

**Architecture:** The plugin keeps the current control-plane path of category resolution plus `next_action`, but category definitions now encode delegation preference and parent-facing tool outputs expose whether work should be executed by a child agent or stay in the local parent.

**Tech Stack:** TypeScript, Node.js, zero-third-party stdio MCP runtime, TOML category configuration, SQLite runtime state, structured markdown plans, Codex native subagent dispatch.

---

## Context

- The current plugin already resolves category, preferred role, write policy, and `next_action`.
- Delegation intent is still implicit, which allows the local parent to do work that should be child-owned.
- The desired default is delegation-first: parent owns orchestration, children own ordinary planning, research, implementation, and review execution.

## Artifact Model

- Spec path: `docs/specs/2026-04-11-codex-orchestrator-delegation-first-dispatch-design.md`
- Active plan path: `docs/plans/active/2026-04-11-codex-orchestrator-delegation-first-dispatch-implementation.md`
- Category config: `plugins/codex-orchestrator/config/categories.toml`
- Tool implementation: `plugins/codex-orchestrator/src/tools/register-tools.ts`
- Category resolver: `plugins/codex-orchestrator/src/services/category-registry.ts`
- Type definitions: `plugins/codex-orchestrator/src/types.ts`
- Tests:
  - `plugins/codex-orchestrator/tests/category-registry.test.ts`
  - `plugins/codex-orchestrator/tests/tools.test.ts`
- Routing docs:
  - `AGENTS.md`
  - `docs/index.md`
  - `task_plan.md`

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| D1. Create delegation-first spec and active plan | None | Establishes the execution contract |
| D2. Add failing delegation tests | D1 | Red-phase tests should target the approved contract |
| D3. Implement category delegation semantics and parent-facing outputs | D2 | Production changes follow failing tests |
| D4. Verify delegation-first behavior and sync docs | D3 | Verification depends on green implementation |

## Quality Gates

- Category definitions expose delegation preference.
- `orchestrator_resolve_category` returns default delegation bias.
- `orchestrator_next_action` returns explicit child-intervention metadata.
- `research`, `backend-impl`, and `review` bias to child execution.
- Relevant tests pass after the change.

## Execution Status

- Current wave: Completed
- Active task: none
- Blockers: None
- Last review result: local quality pass

## TODO List

- [x] D1. Create Delegation-First Spec And Active Plan
- [x] D2. Add Failing Delegation Tests
- [x] D3. Implement Category Delegation Semantics And Parent-Facing Outputs
- [x] D4. Verify Delegation-First Behavior And Sync Docs

### Task D1: Create Delegation-First Spec And Active Plan

**Files:**
- Create: `docs/specs/2026-04-11-codex-orchestrator-delegation-first-dispatch-design.md`
- Create: `docs/plans/active/2026-04-11-codex-orchestrator-delegation-first-dispatch-implementation.md`
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

- [x] Step 1: Write the delegation-first design spec
- [x] Step 2: Write the active implementation plan
- [x] Step 3: Switch routing docs to the new execution anchor
- [x] Step 4: Verify the new plan is the active execution source

### Task D2: Add Failing Delegation Tests

**Files:**
- Modify: `plugins/codex-orchestrator/tests/category-registry.test.ts`
- Modify: `plugins/codex-orchestrator/tests/tools.test.ts`
- Modify: `docs/plans/active/2026-04-11-codex-orchestrator-delegation-first-dispatch-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add a failing category-registry test for delegation preference
- [x] Step 2: Add failing next-action tests for explicit subagent intervention metadata
- [x] Step 3: Run the targeted test suite to verify the red phase
- [x] Step 4: Sync the plan after the failing test run

### Task D3: Implement Category Delegation Semantics And Parent-Facing Outputs

**Files:**
- Modify: `plugins/codex-orchestrator/config/categories.toml`
- Modify: `plugins/codex-orchestrator/src/types.ts`
- Modify: `plugins/codex-orchestrator/src/services/category-registry.ts`
- Modify: `plugins/codex-orchestrator/src/tools/register-tools.ts`
- Modify: `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
- Modify: `docs/architecture/category-contract.md`
- Modify: `docs/architecture/agent-contracts.md`
- Modify: `docs/architecture/mcp-tool-contract.md`
- Modify: `docs/plans/active/2026-04-11-codex-orchestrator-delegation-first-dispatch-implementation.md`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add delegation preference to the category contract and registry
- [x] Step 2: Expose delegation defaults in `orchestrator_resolve_category`
- [x] Step 3: Expose explicit child-intervention metadata in `orchestrator_next_action`
- [x] Step 4: Update orchestration docs and skill guidance
- [x] Step 5: Sync the plan after implementation turns green

### Task D4: Verify Delegation-First Behavior And Sync Docs

**Files:**
- Modify: `progress.md`
- Modify: `findings.md`
- Modify: `task_plan.md`
- Modify: `docs/index.md`
- Modify: `AGENTS.md`
- Modify: `docs/plans/active/2026-04-11-codex-orchestrator-delegation-first-dispatch-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Run the category and tools test suites
- [x] Step 2: Verify the new MCP payload fields through the test outputs
- [x] Step 3: Sync findings, progress, and routing docs
- [x] Step 4: Mark final acceptance state

## Final Acceptance

- [x] Category delegation preference is explicit
- [x] Category resolution exposes default child-execution bias
- [x] Next action exposes explicit subagent-intervention metadata
- [x] Parent behavior is documented as delegation-first
- [x] Delegation-focused tests are green
