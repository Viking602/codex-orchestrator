# Codex Orchestrator Default Workflow Routing Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: Use bounded execution, file-backed progress, and explicit verification. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make `codex-orchestrator` the default repository workflow by bootstrapping global and repo guidance, strengthening skill discovery, and verifying the installer-managed routing surfaces.

**Architecture:** Default routing is established through durable instruction surfaces rather than hoping plugin installation alone changes model behavior. The installer bootstraps a managed global `AGENTS` block, the repository sharpens its own routing rules, and the bundled skill/manifest metadata broaden implicit discovery for normal coding prompts.

**Tech Stack:** Markdown routing docs, Bash installer scripting, Node.js file editing, JSON plugin metadata, Node test runner.

---

## Context

- Marketplace/bootstrap installation is already complete and validated.
- Current Codex docs say plugin installation alone does not force implicit routing.
- Current Codex docs say `AGENTS.md` participates in the instruction chain and skill descriptions affect implicit discovery.
- The repository needs a stronger default route so users do not have to manually invoke `@codex-orchestrator` on every task.

## Artifact Model

- Spec path: `docs/specs/2026-04-11-codex-orchestrator-default-workflow-routing-design.md`
- Completed plan path: `docs/plans/completed/2026-04-11-codex-orchestrator-default-workflow-routing-implementation.md`
- Repo routing doc: `AGENTS.md`
- Docs index: `docs/index.md`
- Session summary: `task_plan.md`
- Installer path: `scripts/install-codex-orchestrator.sh`
- Plugin manifest path: `plugins/codex-orchestrator/.codex-plugin/plugin.json`
- Plugin skill path: `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
- Test path: `tests/install-script.test.ts`

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| R1. Create default-routing spec and active plan | None | Establishes the routing contract |
| R2. Add failing tests for global AGENTS bootstrap | R1 | Installer behavior must be specified before test changes |
| R3. Implement routing bootstrap and discovery updates | R2 | Production changes follow the failing tests |
| R4. Verify installer bootstrap and sync docs | R3 | Verification only matters after behavior exists |

## Quality Gates

- Installer writes or updates a managed default-workflow block in the active global `AGENTS` file.
- Installer preserves existing user guidance outside the managed block.
- Repo `AGENTS.md` routes repository tasks into `codex-orchestrator` before generic process skills.
- Bundled skill metadata names normal repository task triggers, not just orchestration-specific terms.
- Relevant tests pass after the routing changes.

## Execution Status

- Current wave: Completed
- Active task: none
- Blockers: None
- Last review result: local quality pass

## TODO List

- [x] R1. Create Default-Routing Spec And Active Plan
- [x] R2. Add Failing Tests For Global AGENTS Bootstrap
- [x] R3. Implement Routing Bootstrap And Discovery Updates
- [x] R4. Verify Installer Bootstrap And Sync Docs

### Task R1: Create Default-Routing Spec And Active Plan

**Files:**
- Create: `docs/specs/2026-04-11-codex-orchestrator-default-workflow-routing-design.md`
- Create: `docs/plans/completed/2026-04-11-codex-orchestrator-default-workflow-routing-implementation.md`
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

- [x] Step 1: Write the default-routing design spec
- [x] Step 2: Write the active implementation plan
- [x] Step 3: Switch routing docs to the new active plan
- [x] Step 4: Verify the new plan is the execution anchor

### Task R2: Add Failing Tests For Global AGENTS Bootstrap

**Files:**
- Modify: `tests/install-script.test.ts`
- Modify: `docs/plans/completed/2026-04-11-codex-orchestrator-default-workflow-routing-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add a failing installer test for global AGENTS bootstrap
- [x] Step 2: Add a failing installer test for preserving existing AGENTS content
- [x] Step 3: Run the installer test suite to verify the new checks fail
- [x] Step 4: Sync plan status after the red phase

### Task R3: Implement Routing Bootstrap And Discovery Updates

**Files:**
- Modify: `scripts/install-codex-orchestrator.sh`
- Modify: `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
- Modify: `plugins/codex-orchestrator/.codex-plugin/plugin.json`
- Modify: `AGENTS.md`
- Modify: `README.md`
- Modify: `docs/specs/2026-04-11-codex-orchestrator-default-workflow-routing-design.md`
- Modify: `docs/plans/completed/2026-04-11-codex-orchestrator-default-workflow-routing-implementation.md`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Implement installer-managed global AGENTS block updates
- [x] Step 2: Strengthen bundled skill and manifest discovery metadata
- [x] Step 3: Sharpen repo-local routing guidance and install docs
- [x] Step 4: Re-run the installer tests and sync the green state

### Task R4: Verify Installer Bootstrap And Sync Docs

**Files:**
- Modify: `progress.md`
- Modify: `findings.md`
- Modify: `task_plan.md`
- Modify: `docs/index.md`
- Modify: `AGENTS.md`
- Modify: `docs/plans/completed/2026-04-11-codex-orchestrator-default-workflow-routing-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Run fresh verification commands for routing bootstrap behavior
- [x] Step 2: Reinstall the plugin into the local Codex home so global guidance is updated
- [x] Step 3: Inspect the active global AGENTS file, config, and skill surfaces
- [x] Step 4: Mark acceptance state and sync repository status docs

## Final Acceptance

- [x] Global AGENTS bootstrap is verified
- [x] Repo routing guidance is verified
- [x] Bundled skill discovery wording is verified
- [x] Installer tests are green
- [x] Local Codex bootstrap has been refreshed
