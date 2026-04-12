# Codex Orchestrator Direction-First Execution Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: when the user already supplies a workable direction, do not ask a second confirmation question. Write the spec, create the active plan, and start execution unless a hard blocker exists. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Stop redundant re-confirmation after the user already pointed the direction, while preserving real blocker checks, file-backed planning, and delegated execution.

**Architecture:** This is a workflow-surface and control-policy refinement. The orchestrator skill, repository guidance, install-managed guidance, plugin metadata, and `orchestrator_question_gate` become consistent about when to continue without asking.

**Tech Stack:** Rust CLI, Rust integration tests, markdown workflow docs, plugin manifest metadata.

---

## Context

- At plan start, the workflow still said `get design approval before writing the implementation plan` as a broad rule.
- At plan start, `question_gate` blocked optional expansion but had no explicit non-question path for redundant `should I proceed` confirmation.
- The user explicitly wants direction-clear requests to go from planning into execution without another user turn.

## Artifact Model

- Spec path: `docs/specs/2026-04-12-codex-orchestrator-direction-first-execution-design.md`
- Completed plan path: `docs/plans/completed/2026-04-12-codex-orchestrator-direction-first-execution-implementation.md`
- Runtime files:
  - `plugins/codex-orchestrator/rust-cli/src/tools.rs`
  - `plugins/codex-orchestrator/rust-cli/tests/runtime_contracts.rs`
- Workflow files:
  - `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
  - `AGENTS.md`
  - `install.md`
  - `plugins/codex-orchestrator/.codex-plugin/plugin.json`
- Repo contract tests:
  - `plugins/codex-orchestrator/rust-cli/tests/repo_contracts.rs`

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| D1. Write the direction-first spec and active plan | None | Establishes the change contract and execution anchor |
| D2. Update workflow guidance, metadata, and question-gate behavior | D1 | Runtime and workflow changes should follow the written contract |
| D3. Add regression coverage for direction-first execution | D2 | Tests depend on the final workflow wording and gate semantics |
| D4. Validate, sync routing docs, and archive the plan | D3 | Closeout depends on validated behavior |

## Quality Gates

- Redundant direction confirmation is treated as a non-question.
- Workflow docs say clear-direction requests should proceed without another confirmation turn.
- Manifest prompts push the host toward plan-and-execute behavior once direction is clear.
- Cargo validation passes.

## Execution Status

- Current wave: Completed
- Active task: none
- Blockers: None
- Last review result: full cargo validation pass

## TODO List

- [x] D1. Write The Direction-First Spec And Active Plan
- [x] D2. Update Workflow Guidance, Metadata, And Question-Gate Behavior
- [x] D3. Add Regression Coverage For Direction-First Execution
- [x] D4. Validate, Sync Routing Docs, And Archive The Plan

### Task D1: Write The Direction-First Spec And Active Plan

**Files:**
- Create: `docs/specs/2026-04-12-codex-orchestrator-direction-first-execution-design.md`
- Create: `docs/plans/active/2026-04-12-codex-orchestrator-direction-first-execution-implementation.md`
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

- [x] Step 1: Write the direction-first execution design spec
- [x] Step 2: Write the active plan and route the repo to it

### Task D2: Update Workflow Guidance, Metadata, And Question-Gate Behavior

**Files:**
- Modify: `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
- Modify: `AGENTS.md`
- Modify: `install.md`
- Modify: `plugins/codex-orchestrator/.codex-plugin/plugin.json`
- Modify: `plugins/codex-orchestrator/rust-cli/src/tools.rs`
- Modify: `docs/architecture/question-gate-protocol.md`
- Modify: `docs/architecture/mcp-tool-contract.md`

**Category:** plan
**Owner Role:** harness-planner
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Narrow the comparison-and-approval wording to direction-open cases only
- [x] Step 2: Add a non-question `direction_confirmation` gate result and align metadata

### Task D3: Add Regression Coverage For Direction-First Execution

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

- [x] Step 1: Add runtime coverage for `direction_confirmation`
- [x] Step 2: Add repo-contract coverage for direction-first workflow wording and manifest prompts

### Task D4: Validate, Sync Routing Docs, And Archive The Plan

**Files:**
- Modify: `docs/index.md`
- Modify: `AGENTS.md`
- Modify: `task_plan.md`
- Modify: `progress.md`
- Modify: `docs/plans/completed/2026-04-12-codex-orchestrator-direction-first-execution-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Run Cargo validation and confirm direction-first behavior
- [x] Step 2: Sync final routing docs and archive the completed plan

## Final Acceptance

- [x] The workflow does not ask a second confirmation question when the user already supplied a workable direction
- [x] `orchestrator_question_gate` returns a plan-and-execute action for `direction_confirmation`
- [x] Skill, guidance, and manifest prompts all reflect direction-first execution
- [x] Cargo validation passes
