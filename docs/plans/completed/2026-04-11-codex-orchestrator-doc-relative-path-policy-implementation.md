# Codex Orchestrator Relative Documentation Path Policy Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: Use bounded execution, file-backed progress, explicit verification, and fix legacy absolute-path docs on first touch. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Enforce a hard repository rule that markdown docs must use repo-relative artifact links and portable environment examples, while automatically repairing legacy absolute-path documentation when it is encountered.

**Architecture:** The change is documentation-contract and test enforcement work. Durable routing docs define the rule, markdown docs are repaired in place, and a regression test fails when absolute filesystem paths reappear in markdown documentation.

**Tech Stack:** Markdown docs, repository routing docs, Node.js test runner.

---

## Context

- The repository currently contains machine-specific absolute filesystem paths in markdown docs.
- These links are portability bugs, not subjective style.
- The repository workflow should fix such legacy doc debt on first intervention instead of escalating it into a user question.

## Artifact Model

- Spec path: `docs/specs/2026-04-11-codex-orchestrator-doc-relative-path-policy-design.md`
- Completed plan path: `docs/plans/completed/2026-04-11-codex-orchestrator-doc-relative-path-policy-implementation.md`
- Routing docs:
  - `AGENTS.md`
  - `docs/index.md`
  - `task_plan.md`
- Install guide:
  - `install.md`
- Product docs:
  - `docs/product/README.md`
- Orchestrator guidance:
  - `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
- Regression test:
  - `plugins/codex-orchestrator/tests/docs-relative-path-policy.test.ts`

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| P1. Create doc-path spec and active plan | None | Establishes the contract |
| P2. Add failing absolute-path regression test | P1 | Red-phase test should target the approved rule |
| P3. Enforce the rule and repair legacy markdown paths | P2 | Production changes follow failing coverage |
| P4. Verify enforcement and sync status docs | P3 | Verification depends on the repaired docs and green tests |

## Quality Gates

- Markdown docs do not contain machine-specific absolute filesystem paths.
- Repository artifact links are repo-relative.
- Install examples avoid hard-coded machine paths.
- `AGENTS.md` and orchestrator guidance define the rule as hard policy.
- Regression tests pass after the cleanup.

## Execution Status

- Current wave: Completed
- Active task: none
- Blockers: None
- Last review result: local quality pass

## TODO List

- [x] P1. Create Doc-Path Spec And Active Plan
- [x] P2. Add Failing Absolute-Path Regression Test
- [x] P3. Enforce The Rule And Repair Legacy Markdown Paths
- [x] P4. Verify Enforcement And Sync Status Docs

### Task P1: Create Doc-Path Spec And Active Plan

**Files:**
- Create: `docs/specs/2026-04-11-codex-orchestrator-doc-relative-path-policy-design.md`
- Create: `docs/plans/completed/2026-04-11-codex-orchestrator-doc-relative-path-policy-implementation.md`
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

- [x] Step 1: Write the relative-path policy design spec
- [x] Step 2: Write the active implementation plan
- [x] Step 3: Switch routing docs to the new execution anchor
- [x] Step 4: Verify the new plan is the active execution source

### Task P2: Add Failing Absolute-Path Regression Test

**Files:**
- Create: `plugins/codex-orchestrator/tests/docs-relative-path-policy.test.ts`
- Modify: `docs/plans/completed/2026-04-11-codex-orchestrator-doc-relative-path-policy-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add a failing scan for absolute filesystem paths in markdown docs
- [x] Step 2: Run the targeted regression test and verify the red phase
- [x] Step 3: Sync the plan after the failing run

### Task P3: Enforce The Rule And Repair Legacy Markdown Paths

**Files:**
- Modify: `AGENTS.md`
- Modify: `docs/index.md`
- Modify: `install.md`
- Modify: `docs/product/README.md`
- Modify: `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
- Modify: `docs/plans/completed/2026-04-11-codex-orchestrator-doc-relative-path-policy-implementation.md`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add hard repo rules against absolute documentation paths
- [x] Step 2: Repair legacy absolute markdown links in routing and product docs
- [x] Step 3: Replace hard-coded machine-path examples with portable forms
- [x] Step 4: Sync the plan after the cleanup turns green

### Task P4: Verify Enforcement And Sync Status Docs

**Files:**
- Modify: `progress.md`
- Modify: `findings.md`
- Modify: `task_plan.md`
- Modify: `docs/index.md`
- Modify: `AGENTS.md`
- Modify: `docs/plans/completed/2026-04-11-codex-orchestrator-doc-relative-path-policy-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Run the full plugin test suite
- [x] Step 2: Re-scan markdown docs for absolute filesystem paths
- [x] Step 3: Sync findings, progress, and routing docs
- [x] Step 4: Mark final acceptance state

## Final Acceptance

- [x] Markdown docs no longer contain machine-specific absolute filesystem paths
- [x] Repo artifact links are repo-relative
- [x] Hard repo rules require auto-repair of legacy absolute-path docs on first touch
- [x] Regression tests enforce the rule
