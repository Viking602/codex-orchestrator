# Codex Orchestrator Inspection-First Delegation Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: make repository inspection and codebase-check requests classify into `research` so `search-specialist` is dispatched before the parent keeps that work locally. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Harden entry routing so codebase inspection requests trigger the research/search-specialist path and preserve the plugin's subagent-first execution model.

**Architecture:** This is a routing-contract change, not a new runtime subsystem. The category resolver remains the entry classifier, `research` remains the existing delegated category, and workflow docs reinforce that repository inspection is child-owned read-only work.

**Tech Stack:** Rust CLI, Rust integration tests, markdown workflow guidance.

---

## Context

- At plan start, `research` was already `subagent-required`, but repository-inspection requests could still miss the category resolver and fall into `backend-impl`.
- The user explicitly wants repository codebase inspection to trigger proactive subagent dispatch instead of parent-local recognition and analysis.
- The smallest durable fix is to harden the resolver and workflow guidance, then lock the behavior with regression tests.

## Artifact Model

- Spec path: `docs/specs/2026-04-12-codex-orchestrator-inspection-first-delegation-design.md`
- Completed plan path: `docs/plans/completed/2026-04-12-codex-orchestrator-inspection-first-delegation-implementation.md`
- Runtime files:
  - `plugins/codex-orchestrator/rust-cli/src/category_registry.rs`
- Workflow files:
  - `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
  - `AGENTS.md`
  - `install.md`
- Regression tests:
  - `plugins/codex-orchestrator/rust-cli/tests/runtime_contracts.rs`
  - `plugins/codex-orchestrator/rust-cli/tests/repo_contracts.rs`

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| I1. Write the inspection-first delegation spec and active plan | None | Establishes the change contract and execution anchor |
| I2. Harden resolver and workflow guidance | I1 | Runtime and guidance changes should follow the written contract |
| I3. Add regression coverage for inspection-first routing | I2 | Tests depend on the final behavior and wording |
| I4. Validate, sync routing docs, and archive the plan | I3 | Closeout depends on verified final behavior |

## Quality Gates

- Repository inspection prompts resolve to `research`.
- `research` next-action metadata still advertises subagent-required dispatch to `search-specialist`.
- Workflow guidance explicitly treats codebase inspection as child-owned research work.
- Cargo validation passes.

## Execution Status

- Current wave: Completed
- Active task: none
- Blockers: None
- Last review result: full cargo validation pass

## TODO List

- [x] I1. Write The Inspection-First Delegation Spec And Active Plan
- [x] I2. Harden Resolver And Workflow Guidance
- [x] I3. Add Regression Coverage For Inspection-First Routing
- [x] I4. Validate, Sync Routing Docs, And Archive The Plan

### Task I1: Write The Inspection-First Delegation Spec And Active Plan

**Files:**
- Create: `docs/specs/2026-04-12-codex-orchestrator-inspection-first-delegation-design.md`
- Create: `docs/plans/active/2026-04-12-codex-orchestrator-inspection-first-delegation-implementation.md`
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

- [x] Step 1: Write the inspection-first delegation design spec
- [x] Step 2: Write the active implementation plan and route the repo to it

### Task I2: Harden Resolver And Workflow Guidance

**Files:**
- Modify: `plugins/codex-orchestrator/rust-cli/src/category_registry.rs`
- Modify: `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
- Modify: `AGENTS.md`
 - Modify: `install.md`

**Category:** research
**Owner Role:** search-specialist
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Expand repository-inspection keyword matching for `research`
- [x] Step 2: Strengthen workflow guidance so codebase inspection routes to `search-specialist`

### Task I3: Add Regression Coverage For Inspection-First Routing

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

- [x] Step 1: Add resolver and dispatch regression tests for repository-inspection prompts
- [x] Step 2: Add repo-contract coverage for inspection-first workflow wording

### Task I4: Validate, Sync Routing Docs, And Archive The Plan

**Files:**
- Modify: `docs/index.md`
- Modify: `AGENTS.md`
- Modify: `task_plan.md`
- Modify: `progress.md`
- Modify: `docs/plans/completed/2026-04-12-codex-orchestrator-inspection-first-delegation-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Run Cargo validation and confirm inspection-first routing holds
- [x] Step 2: Sync final routing docs and archive the completed plan

## Final Acceptance

- [x] Repository inspection prompts resolve to `research` instead of falling through to `backend-impl`
- [x] `research` dispatch metadata still requires subagent execution and points at `search-specialist`
- [x] Workflow guidance explicitly treats codebase inspection as child-owned research work
- [x] Cargo validation passes
