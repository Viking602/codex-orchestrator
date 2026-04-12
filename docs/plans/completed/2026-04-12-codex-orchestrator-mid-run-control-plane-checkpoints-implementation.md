# Codex Orchestrator Mid-Run Control-Plane Checkpoints Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: do not defer control-plane writes until the end of a long task run. Parent-owned MCP writes for task start and step synchronization must happen before child execution continues, and child execution should return at current-step boundaries instead of swallowing a whole top-level task.

**Goal:** Make MCP state updates happen during execution by turning task-start and step-sync writes into blocking pre-dispatch control-plane actions and by narrowing child execution to the current step.

**Architecture:** This is a runtime and workflow-contract refinement. The plan file stays authoritative, but `orchestrator_next_action` now emits blocking parent-owned pre-dispatch control-plane writes, and child dispatch metadata becomes current-step-scoped.

**Tech Stack:** Rust CLI, Rust integration tests, markdown workflow docs, bundled agent prompts.

---

## Context

- The current contract exposes step-sync hints, but not as blocking top-level actions.
- Dedicated task-owned child sessions exist, but their execution boundary is still too broad.
- The user expects MCP writes to happen during execution, not only as a terminal replay batch.

## Artifact Model

- Spec path: `docs/specs/2026-04-12-codex-orchestrator-mid-run-control-plane-checkpoints-design.md`
- Completed plan path: `docs/plans/completed/2026-04-12-codex-orchestrator-mid-run-control-plane-checkpoints-implementation.md`
- Completed plan path: `docs/plans/completed/2026-04-12-codex-orchestrator-mid-run-control-plane-checkpoints-implementation.md`
- Runtime files:
  - `plugins/codex-orchestrator/rust-cli/src/tools.rs`
- Test files:
  - `plugins/codex-orchestrator/rust-cli/tests/runtime_contracts.rs`
  - `plugins/codex-orchestrator/rust-cli/tests/repo_contracts.rs`
- Workflow files:
  - `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
  - `plugins/codex-orchestrator/codex/agents/backend-developer.toml`
  - `plugins/codex-orchestrator/codex/agents/search-specialist.toml`
  - `plugins/codex-orchestrator/codex/agents/harness-planner.toml`
  - `docs/architecture/mcp-tool-contract.md`
  - `docs/architecture/agent-contracts.md`
  - `docs/architecture/plan-sync-rules.md`
  - `AGENTS.md`
  - `install.md`

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| C1. Write the checkpointing design spec and active plan | None | Anchors the contract and execution surface |
| C2. Promote task-start and step-sync to blocking pre-dispatch control-plane actions | C1 | Runtime must block child execution until control-plane state is current |
| C3. Add single-step child-dispatch metadata and tighten bundled child prompts | C2 | Execution-scope contract depends on the updated runtime payload |
| C4. Add regression coverage and sync workflow docs | C3 | Tests and docs depend on the final payload and child-boundary rules |
| C5. Validate, archive, and refresh the local install | C4 | Closeout depends on passing behavior and synced install surface |

## Quality Gates

- Untouched tasks do not dispatch child work before `blocking_control_plane_actions` records task start.
- Step-desynchronized running tasks do not continue child work before `blocking_control_plane_actions` repair step sync.
- Child dispatch payloads explicitly say they are current-step-scoped.
- Bundled child prompts tell implementers and planners to return at current-step boundaries.
- Cargo validation passes.

## Execution Status

- Current wave: Wave Complete
- Active task: none
- Blockers: None
- Last review result: quality pass

## TODO List

- [x] C1. Write The Checkpointing Design Spec And Active Plan
- [x] C2. Promote Task-Start And Step-Sync To Blocking Pre-Dispatch Control-Plane Actions
- [x] C3. Add Single-Step Child-Dispatch Metadata And Tighten Bundled Child Prompts
- [x] C4. Add Regression Coverage And Sync Workflow Docs
- [x] C5. Validate, Archive, And Refresh The Local Install

### Task C1: Write The Checkpointing Design Spec And Active Plan

**Files:**
- Create: `docs/specs/2026-04-12-codex-orchestrator-mid-run-control-plane-checkpoints-design.md`
- Create: `docs/plans/completed/2026-04-12-codex-orchestrator-mid-run-control-plane-checkpoints-implementation.md`
- Modify: `docs/index.md`
- Modify: `task_plan.md`

**Category:** plan
**Owner Role:** harness-planner
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Write the mid-run control-plane checkpointing design spec
- [x] Step 2: Route the repository to the active implementation plan

### Task C2: Promote Task-Start And Step-Sync To Blocking Pre-Dispatch Control-Plane Actions

**Files:**
- Modify: `plugins/codex-orchestrator/rust-cli/src/tools.rs`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Emit blocking `orchestrator_begin_task` actions before first child dispatch on untouched tasks
- [x] Step 2: Emit blocking `orchestrator_begin_step` actions before continuing step-desynchronized running tasks

### Task C3: Add Single-Step Child-Dispatch Metadata And Tighten Bundled Child Prompts

**Files:**
- Modify: `plugins/codex-orchestrator/rust-cli/src/tools.rs`
- Modify: `plugins/codex-orchestrator/codex/agents/backend-developer.toml`
- Modify: `plugins/codex-orchestrator/codex/agents/search-specialist.toml`
- Modify: `plugins/codex-orchestrator/codex/agents/harness-planner.toml`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Emit current-step child dispatch metadata from `next_action` and `parallel_dispatches`
- [x] Step 2: Update bundled child prompts to return after the current step instead of the whole task

### Task C4: Add Regression Coverage And Sync Workflow Docs

**Files:**
- Modify: `plugins/codex-orchestrator/rust-cli/tests/runtime_contracts.rs`
- Modify: `plugins/codex-orchestrator/rust-cli/tests/repo_contracts.rs`
- Modify: `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
- Modify: `docs/architecture/mcp-tool-contract.md`
- Modify: `docs/architecture/agent-contracts.md`
- Modify: `docs/architecture/plan-sync-rules.md`
- Modify: `AGENTS.md`
- Modify: `install.md`
- Modify: `progress.md`
- Modify: `findings.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add runtime tests for pre-dispatch control-plane actions and single-step child metadata
- [x] Step 2: Update workflow and architecture docs to require mid-run MCP checkpointing

### Task C5: Validate, Archive, And Refresh The Local Install

**Files:**
- Modify: `docs/index.md`
- Modify: `task_plan.md`
- Modify: `progress.md`
- Modify: `docs/plans/completed/2026-04-12-codex-orchestrator-mid-run-control-plane-checkpoints-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Run Cargo validation and targeted fresh-process checks
- [x] Step 2: Archive the plan and refresh the installed plugin/runtime

## Final Acceptance

- [x] `orchestrator_next_action` blocks child dispatch behind `blocking_control_plane_actions` for task start and step sync
- [x] child dispatch payloads explicitly scope a resume to the current step
- [x] bundled child prompts return at current-step boundaries
- [x] Cargo validation passes
