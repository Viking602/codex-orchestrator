# Codex Orchestrator Parallel Top-Level Dispatch Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: when multiple top-level tasks are dependency-ready and their child-owned write scopes do not conflict, dispatch them together as one parallel child batch instead of forcing strict serial dispatch. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Enable automatic parallel subagent dispatch for independent top-level tasks in the implementation plan.

**Architecture:** This is a control-plane scheduling refinement. The plan file remains authoritative, but the runtime now parses task dependencies and file ownership metadata so `orchestrator_next_action` can emit a conflict-safe parallel dispatch cohort.

**Tech Stack:** Rust CLI, Rust integration tests, markdown workflow docs.

---

## Context

- The current runtime only dispatches the first unchecked top-level task.
- Existing plan files already declare dependency order and task-owned files.
- The user wants independent, non-conflicting top-level tasks to launch in parallel automatically.

## Artifact Model

- Spec path: `docs/specs/2026-04-12-codex-orchestrator-parallel-top-level-dispatch-design.md`
- Active plan path: `docs/plans/active/2026-04-12-codex-orchestrator-parallel-top-level-dispatch-implementation.md`
- Completed plan path: `docs/plans/completed/2026-04-12-codex-orchestrator-parallel-top-level-dispatch-implementation.md`
- Runtime files:
  - `plugins/codex-orchestrator/rust-cli/src/types.rs`
  - `plugins/codex-orchestrator/rust-cli/src/plan_document.rs`
  - `plugins/codex-orchestrator/rust-cli/src/tools.rs`
- Test files:
  - `plugins/codex-orchestrator/rust-cli/tests/runtime_contracts.rs`
  - `plugins/codex-orchestrator/rust-cli/tests/repo_contracts.rs`
- Workflow files:
  - `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
  - `AGENTS.md`
  - `install.md`
  - `docs/architecture/category-contract.md`
  - `docs/architecture/agent-contracts.md`
  - `docs/architecture/mcp-tool-contract.md`

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| P1. Write the parallel-dispatch spec and active plan | None | Establishes the contract and execution anchor |
| P2. Extend plan parsing with dependency and file metadata | P1 | Scheduler behavior depends on machine-readable plan data |
| P3. Implement parallel dispatch cohort selection in `next_action` | P2 | Batch dispatch depends on parsed dependencies and scopes |
| P4. Add regression coverage and update workflow docs | P3 | Tests and docs depend on the final scheduling behavior |
| P5. Validate, archive the plan, and refresh the installed plugin | P4 | Closeout depends on green behavior and synced docs/install |

## Quality Gates

- The runtime parses dependency and file metadata from the current plan format.
- `orchestrator_next_action` emits a parallel child batch when tasks are dependency-ready and conflict-free.
- Conflicting or dependency-blocked tasks are excluded from the batch.
- Cargo validation passes.

## Execution Status

- Current wave: Wave Complete
- Active task: none
- Blockers: None
- Last review result: quality pass

## TODO List

- [x] P1. Write The Parallel-Dispatch Spec And Active Plan
- [x] P2. Extend Plan Parsing With Dependency And File Metadata
- [x] P3. Implement Parallel Dispatch Cohort Selection In `next_action`
- [x] P4. Add Regression Coverage And Update Workflow Docs
- [x] P5. Validate, Archive The Plan, And Refresh The Installed Plugin

### Task P1: Write The Parallel-Dispatch Spec And Active Plan

**Files:**
- Create: `docs/specs/2026-04-12-codex-orchestrator-parallel-top-level-dispatch-design.md`
- Create: `docs/plans/active/2026-04-12-codex-orchestrator-parallel-top-level-dispatch-implementation.md`
- Modify: `docs/index.md`
- Modify: `task_plan.md`

**Category:** plan
**Owner Role:** harness-planner
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Write the parallel top-level dispatch design spec
- [x] Step 2: Create the active implementation plan and route docs to it

### Task P2: Extend Plan Parsing With Dependency And File Metadata

**Files:**
- Modify: `plugins/codex-orchestrator/rust-cli/src/types.rs`
- Modify: `plugins/codex-orchestrator/rust-cli/src/plan_document.rs`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Parse task dependency edges from `Task Dependency Graph`
- [x] Step 2: Parse per-task declared files from `Files:` blocks and expose them in plan state

### Task P3: Implement Parallel Dispatch Cohort Selection In `next_action`

**Files:**
- Modify: `plugins/codex-orchestrator/rust-cli/src/tools.rs`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Derive child-owned dispatch scope and conservative conflict detection
- [x] Step 2: Return a parallel dispatch cohort for dependency-ready, conflict-free tasks

### Task P4: Add Regression Coverage And Update Workflow Docs

**Files:**
- Modify: `plugins/codex-orchestrator/rust-cli/tests/runtime_contracts.rs`
- Modify: `plugins/codex-orchestrator/rust-cli/tests/repo_contracts.rs`
- Modify: `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
- Modify: `AGENTS.md`
- Modify: `install.md`
- Modify: `docs/architecture/category-contract.md`
- Modify: `docs/architecture/agent-contracts.md`
- Modify: `docs/architecture/mcp-tool-contract.md`
- Modify: `findings.md`
- Modify: `progress.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add runtime coverage for dependency-aware parallel batch selection
- [x] Step 2: Update workflow and architecture docs for parallel child dispatch

### Task P5: Validate, Archive The Plan, And Refresh The Installed Plugin

**Files:**
- Modify: `docs/index.md`
- Modify: `task_plan.md`
- Modify: `progress.md`
- Modify: `docs/plans/completed/2026-04-12-codex-orchestrator-parallel-top-level-dispatch-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Run Cargo validation and confirm the parallel dispatch payload
- [x] Step 2: Archive the plan and refresh the local installed plugin/runtime

## Final Acceptance

- [x] `orchestrator_next_action` emits a parallel dispatch batch for independent top-level tasks
- [x] blocked or conflicting tasks stay out of the returned batch
- [x] workflow docs tell the parent to launch the whole returned batch
- [x] Cargo validation passes
