# Codex Orchestrator Immediate Top-Level Acceptance Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: once a task reaches all steps checked plus passing review gates, accept the top-level task in the same control-plane pass. Do not leave top-level acceptance for an end-of-wave sweep. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Remove the user-visible native todo lag where the first task spins for too long and several top-level tasks suddenly complete together.

**Architecture:** This is a control-plane refinement around task acceptance timing. The durable plan remains authoritative, but terminal review recording now closes the top-level task immediately instead of waiting for a later parent-only sweep.

**Tech Stack:** Rust CLI, Rust integration tests, markdown workflow docs.

---

## Context

- The native todo mirror only marks a top-level task complete when the plan TODO checkbox is checked.
- The terminal review path currently leaves a gap between `quality pass` and `accepted`, so the first task can stay `in_progress` even after the real work is done.
- The user wants visible progress to move when each top-level task truly finishes, not in one late batch.

## Artifact Model

- Spec path: `docs/specs/2026-04-12-codex-orchestrator-immediate-top-level-acceptance-design.md`
- Active plan path: `docs/plans/active/2026-04-12-codex-orchestrator-immediate-top-level-acceptance-implementation.md`
- Completed plan path: `docs/plans/completed/2026-04-12-codex-orchestrator-immediate-top-level-acceptance-implementation.md`
- Runtime files:
  - `plugins/codex-orchestrator/rust-cli/src/tools.rs`
- Test files:
  - `plugins/codex-orchestrator/rust-cli/tests/runtime_contracts.rs`
  - `plugins/codex-orchestrator/rust-cli/tests/repo_contracts.rs`
- Workflow files:
  - `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
  - `AGENTS.md`
  - `install.md`
  - `docs/architecture/agent-contracts.md`
  - `docs/architecture/mcp-tool-contract.md`
  - `docs/architecture/plan-sync-rules.md`

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| I1. Write the immediate-acceptance spec and active plan | None | Establishes the contract and execution anchor |
| I2. Implement immediate top-level acceptance in the Rust MCP runtime | I1 | Runtime changes should follow the written contract |
| I3. Add regression coverage for immediate acceptance and mirror advancement | I2 | Tests depend on the final runtime behavior |
| I4. Update workflow docs, validate, and archive the plan | I3 | Closeout depends on finished behavior and verification |

## Quality Gates

- Terminal review pass closes the top-level task in the same control-plane pass.
- The exported native todo mirror advances to the next top-level task immediately.
- Execution status `Active task` does not linger on an accepted task.
- Cargo validation passes.

## Execution Status

- Current wave: Completed
- Active task: none
- Blockers: None
- Last review result: cargo test pass

## TODO List

- [x] I1. Write The Immediate-Acceptance Spec And Active Plan
- [x] I2. Implement Immediate Top-Level Acceptance In The Rust MCP Runtime
- [x] I3. Add Regression Coverage For Immediate Acceptance And Mirror Advancement
- [x] I4. Update Workflow Docs, Validate, And Archive The Plan

### Task I1: Write The Immediate-Acceptance Spec And Active Plan

**Files:**
- Create: `docs/specs/2026-04-12-codex-orchestrator-immediate-top-level-acceptance-design.md`
- Create: `docs/plans/active/2026-04-12-codex-orchestrator-immediate-top-level-acceptance-implementation.md`
- Modify: `docs/index.md`
- Modify: `task_plan.md`

**Category:** plan
**Owner Role:** harness-planner
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Write the immediate-acceptance design spec
- [x] Step 2: Create the active implementation plan and route docs to it

### Task I2: Implement Immediate Top-Level Acceptance In The Rust MCP Runtime

**Files:**
- Modify: `plugins/codex-orchestrator/rust-cli/src/tools.rs`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Refactor shared task-acceptance logic so explicit and immediate acceptance use the same path
- [x] Step 2: Auto-accept terminal-ready tasks from `orchestrator_record_review` and advance `Active task`

### Task I3: Add Regression Coverage For Immediate Acceptance And Mirror Advancement

**Files:**
- Modify: `plugins/codex-orchestrator/rust-cli/tests/runtime_contracts.rs`
- Modify: `plugins/codex-orchestrator/rust-cli/tests/repo_contracts.rs`
- Modify: `findings.md`
- Modify: `progress.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add runtime coverage for immediate acceptance after terminal review pass
- [x] Step 2: Add repo-contract coverage for immediate-acceptance workflow wording

### Task I4: Update Workflow Docs, Validate, And Archive The Plan

**Files:**
- Modify: `AGENTS.md`
- Modify: `install.md`
- Modify: `docs/architecture/agent-contracts.md`
- Modify: `docs/architecture/mcp-tool-contract.md`
- Modify: `docs/architecture/plan-sync-rules.md`
- Modify: `docs/index.md`
- Modify: `task_plan.md`
- Modify: `progress.md`
- Modify: `docs/plans/completed/2026-04-12-codex-orchestrator-immediate-top-level-acceptance-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Update workflow and architecture docs for immediate acceptance
- [x] Step 2: Run Cargo validation, sync routing docs, and archive the completed plan

## Final Acceptance

- [x] Terminal review pass closes top-level acceptance in the same control-plane pass
- [x] Native todo mirror advances to the next top-level task without a late batch jump
- [x] Execution status `Active task` no longer points at an already accepted task
- [x] Cargo validation passes
