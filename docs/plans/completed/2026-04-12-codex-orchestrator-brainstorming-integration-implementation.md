# Codex Orchestrator Brainstorming Integration Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: integrate the useful brainstorming-stage behaviors into `codex-orchestrator` so repository tasks enter through the plugin instead of global superpowers process skills. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make `codex-orchestrator` absorb the repository brainstorming phase, block superpowers from taking the entry route, and keep the resulting discovery/design flow durable through tests and install-managed guidance.

**Architecture:** The change is a workflow-surface integration, not an MCP contract change. The bundled skill, repository `AGENTS.md`, installer-managed global `AGENTS` block, and plugin metadata become the control points for discovery/design routing. Existing `orchestrator_*` tools continue to own the control plane after selection.

**Tech Stack:** Markdown workflow docs, plugin manifest metadata, repo-level Node contract tests, existing Rust MCP runtime.

---

## Context

- At plan start, `using-superpowers` was still broad enough to claim the start of a conversation before repository-local workflow selection.
- At plan start, `codex-orchestrator` already owned spec/plan/execution control, but it did not yet explicitly subsume the brainstorming loop that users expected.
- The fix should prioritize entry-routing clarity instead of adding unnecessary new MCP tools.

## Artifact Model

- Spec path: `docs/specs/2026-04-12-codex-orchestrator-brainstorming-integration-design.md`
- Completed plan path: `docs/plans/completed/2026-04-12-codex-orchestrator-brainstorming-integration-implementation.md`
- Workflow surfaces:
  - `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
  - `plugins/codex-orchestrator/.codex-plugin/plugin.json`
  - `AGENTS.md`
  - `install.md`
  - `README.md`
- Repo-level regression tests:
  - `tests/brainstorming-integration.test.ts`
- Local installed guidance to refresh:
  - `~/.codex/AGENTS.md`
  - `~/.codex/plugins/cache/local-plugins/codex-orchestrator/local/skills/orchestrator/SKILL.md`

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| B1. Write the brainstorming-integration spec and active plan | None | Establishes the approved workflow contract |
| B2. Integrate brainstorming behavior into orchestrator surfaces | B1 | Workflow text should follow the approved design |
| B3. Add regression coverage and sync installed local guidance | B2 | Tests and local machine sync depend on final workflow wording |
| B4. Close the plan and verify the integrated workflow | B3 | Closeout depends on the finished surface and validation evidence |

## Quality Gates

- The bundled skill explicitly includes brainstorming-stage discovery and design behaviors.
- Repository and install-managed guidance make `codex-orchestrator` the entry workflow ahead of superpowers.
- Repo-level regression tests cover the integrated workflow contract.
- Existing repository validation still passes after the routing changes.

## Execution Status

- Current wave: Completed
- Active task: none
- Blockers: None
- Last review result: integrated workflow validation pass

## TODO List

- [x] B1. Write The Brainstorming-Integration Spec And Active Plan
- [x] B2. Integrate Brainstorming Behavior Into Orchestrator Surfaces
- [x] B3. Add Regression Coverage And Sync Installed Local Guidance
- [x] B4. Close The Plan And Verify The Integrated Workflow

### Task B1: Write The Brainstorming-Integration Spec And Active Plan

**Files:**
- Create: `docs/specs/2026-04-12-codex-orchestrator-brainstorming-integration-design.md`
- Create: `docs/plans/completed/2026-04-12-codex-orchestrator-brainstorming-integration-implementation.md`
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

- [x] Step 1: Write the brainstorming-integration design spec
- [x] Step 2: Write the active implementation plan
- [x] Step 3: Switch repository routing docs to the new active plan

### Task B2: Integrate Brainstorming Behavior Into Orchestrator Surfaces

**Files:**
- Modify: `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
- Modify: `plugins/codex-orchestrator/.codex-plugin/plugin.json`
- Modify: `AGENTS.md`
- Modify: `install.md`
- Modify: `README.md`

**Category:** docs
**Owner Role:** harness-doc-gardener
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Update the bundled skill to absorb brainstorming-stage behavior
- [x] Step 2: Update repository and install-managed guidance to block superpowers as the entry workflow
- [x] Step 3: Strengthen plugin metadata around discovery, design, and approval

### Task B3: Add Regression Coverage And Sync Installed Local Guidance

**Files:**
- Create: `tests/brainstorming-integration.test.ts`
- Modify: `task_plan.md`
- Modify: `progress.md`
- Modify: `findings.md`
- Sync external files:
  - `~/.codex/AGENTS.md`
  - `~/.codex/plugins/cache/local-plugins/codex-orchestrator/local/skills/orchestrator/SKILL.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add regression coverage for the integrated brainstorming contract
- [x] Step 2: Sync the installed local guidance surfaces on this machine
- [x] Step 3: Re-run the relevant validation suites

### Task B4: Close The Plan And Verify The Integrated Workflow

**Files:**
- Modify: `docs/index.md`
- Modify: `AGENTS.md`
- Modify: `docs/plans/completed/2026-04-12-codex-orchestrator-brainstorming-integration-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Confirm the repo-level tests pass
- [x] Step 2: Confirm the runtime and install suites still pass
- [x] Step 3: Sync final acceptance and archive the completed plan

## Final Acceptance

- [x] `codex-orchestrator` explicitly owns repository brainstorming/discovery before planning and execution
- [x] Superpowers entry skills are demoted to subordinate helpers for normal repository tasks
- [x] Plugin metadata, skill text, and guidance surfaces all describe the same integrated workflow
- [x] Repo-level regression tests and existing validation suites pass
