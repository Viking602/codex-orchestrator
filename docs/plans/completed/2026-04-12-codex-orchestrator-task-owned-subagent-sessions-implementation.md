# Codex Orchestrator Task-Owned Subagent Sessions Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: each top-level task should map to its own dedicated child session. The parent owns the control plane; task-local execution, continuation, and repair should stay inside the task-owned child whenever the runtime has enough state to resume it.

**Goal:** Make each top-level plan task execute through its own dedicated child session so the parent does not accumulate task-local context.

**Architecture:** This is a runtime control-plane change. The plan file remains the source of truth, but runtime state and `next_action` now preserve task-owned child identities and expose explicit spawn/resume policy to the parent.

**Tech Stack:** Rust CLI, Rust integration tests, markdown workflow docs.

---

## Context

- The current runtime already prefers child execution and can batch dependency-ready top-level tasks.
- Session policy is still implicit, so parents can keep task-local reasoning or lose the original implementer child once review begins.
- The desired model is one dedicated child per top-level task, with reviewer children remaining separate guardrails.

## Artifact Model

- Spec path: `docs/specs/2026-04-12-codex-orchestrator-task-owned-subagent-sessions-design.md`
- Active plan path: `docs/plans/active/2026-04-12-codex-orchestrator-task-owned-subagent-sessions-implementation.md`
- Completed plan path: `docs/plans/completed/2026-04-12-codex-orchestrator-task-owned-subagent-sessions-implementation.md`
- Runtime files:
  - `plugins/codex-orchestrator/rust-cli/src/types.rs`
  - `plugins/codex-orchestrator/rust-cli/src/runtime_store.rs`
  - `plugins/codex-orchestrator/rust-cli/src/tools.rs`
- Test files:
  - `plugins/codex-orchestrator/rust-cli/tests/runtime_contracts.rs`
  - `plugins/codex-orchestrator/rust-cli/tests/repo_contracts.rs`
- Workflow files:
  - `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
  - `docs/architecture/runtime-state-schema.md`
  - `docs/architecture/agent-contracts.md`
  - `docs/architecture/mcp-tool-contract.md`
  - `AGENTS.md`
  - `install.md`

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| S1. Write the task-owned-session spec and active plan | None | Establishes the new control-plane contract |
| S2. Extend runtime state for dedicated implementer/reviewer ownership | S1 | Stable child ownership depends on persisted runtime metadata |
| S3. Emit explicit session-routing policy from `orchestrator_next_action` | S2 | Parent dispatch behavior depends on runtime ownership metadata |
| S4. Add regression coverage and workflow docs | S3 | Tests and docs depend on the final session contract |
| S5. Validate, archive, and refresh the installed plugin | S4 | Closeout depends on green behavior and synced local install |

## Quality Gates

- Runtime state preserves implementer and reviewer ownership separately.
- `orchestrator_next_action` exposes spawn/resume policy for task-owned child sessions.
- Parallel dispatch entries expose per-task session-routing instructions.
- Cargo validation passes.

## Execution Status

- Current wave: Wave Complete
- Active task: none
- Blockers: None
- Last review result: quality pass

## TODO List

- [x] S1. Write The Task-Owned-Session Spec And Active Plan
- [x] S2. Extend Runtime State For Dedicated Implementer/Reviewer Ownership
- [x] S3. Emit Explicit Session-Routing Policy From `orchestrator_next_action`
- [x] S4. Add Regression Coverage And Workflow Docs
- [x] S5. Validate, Archive, And Refresh The Installed Plugin

### Task S1: Write The Task-Owned-Session Spec And Active Plan

**Files:**
- Create: `docs/specs/2026-04-12-codex-orchestrator-task-owned-subagent-sessions-design.md`
- Create: `docs/plans/active/2026-04-12-codex-orchestrator-task-owned-subagent-sessions-implementation.md`
- Modify: `docs/index.md`
- Modify: `task_plan.md`

**Category:** plan
**Owner Role:** harness-planner
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Write the task-owned subagent session design spec
- [x] Step 2: Create the active implementation plan and route docs to it

### Task S2: Extend Runtime State For Dedicated Implementer/Reviewer Ownership

**Files:**
- Modify: `plugins/codex-orchestrator/rust-cli/src/types.rs`
- Modify: `plugins/codex-orchestrator/rust-cli/src/runtime_store.rs`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Persist dedicated implementer and reviewer child ids in runtime state
- [x] Step 2: Preserve implementer ownership across review activity and reviewer updates

### Task S3: Emit Explicit Session-Routing Policy From `orchestrator_next_action`

**Files:**
- Modify: `plugins/codex-orchestrator/rust-cli/src/tools.rs`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add task-level spawn/resume session metadata to next-action payloads
- [x] Step 2: Add per-entry session metadata to parallel dispatch payloads

### Task S4: Add Regression Coverage And Workflow Docs

**Files:**
- Modify: `plugins/codex-orchestrator/rust-cli/tests/runtime_contracts.rs`
- Modify: `plugins/codex-orchestrator/rust-cli/tests/repo_contracts.rs`
- Modify: `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
- Modify: `docs/architecture/runtime-state-schema.md`
- Modify: `docs/architecture/agent-contracts.md`
- Modify: `docs/architecture/mcp-tool-contract.md`
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

- [x] Step 1: Add runtime coverage for dedicated task-owned child sessions
- [x] Step 2: Update workflow and architecture docs for task-level child ownership

### Task S5: Validate, Archive, And Refresh The Installed Plugin

**Files:**
- Modify: `docs/index.md`
- Modify: `task_plan.md`
- Modify: `progress.md`
- Modify: `docs/plans/completed/2026-04-12-codex-orchestrator-task-owned-subagent-sessions-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Run Cargo validation and fresh-process probes for task-owned child routing
- [x] Step 2: Archive the plan and refresh the local installed plugin/runtime

## Final Acceptance

- [x] runtime state preserves dedicated implementer and reviewer child ownership
- [x] `orchestrator_next_action` tells the parent when to spawn or resume a dedicated child session for a top-level task
- [x] parallel dispatch entries expose one child-session instruction per top-level task
- [x] Cargo validation passes
