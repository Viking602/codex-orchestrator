# Codex Orchestrator MCP Bootstrap Install Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: fix the installation contract so a fresh Codex session gets callable `orchestrator_*` tools instead of only the bundled skill text. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extend the install contract so Codex-guided installation also bootstraps a stable MCP server registration for `codex-orchestrator`.

**Architecture:** The plugin keeps its bundled `.mcp.json`, but installation now also reconciles a managed `[mcp_servers.codex-orchestrator]` block in user config using the installed cache `src/server.ts` absolute path. The guide and tests treat callable tool exposure as part of installation correctness.

**Tech Stack:** Markdown docs, plugin JSON config, Node test runner, Codex config bootstrap.

---

## Context

- The bundled skill can be discovered while `orchestrator_*` tools remain absent from the runtime tool registry.
- Explicit global MCP registration makes the tools appear in fresh sessions.
- The current install guide and tests do not treat that as required install state.

## Artifact Model

- Spec path: `docs/specs/2026-04-12-codex-orchestrator-mcp-bootstrap-install-design.md`
- Completed plan path: `docs/plans/completed/2026-04-12-codex-orchestrator-mcp-bootstrap-install-implementation.md`
- Plugin MCP config: `plugins/codex-orchestrator/.mcp.json`
- Install guide: `install.md`
- Routing docs:
  - `README.md`
  - `AGENTS.md`
  - `docs/index.md`
  - `task_plan.md`
- Regression test:
  - `tests/install-guide.test.ts`

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| B1. Create spec and execution anchor | None | Establishes the contract |
| B2. Patch plugin and install docs | B1 | Docs and plugin config should reflect the new bootstrap model |
| B3. Extend regression coverage | B2 | Tests should lock the updated install contract |
| B4. Apply local bootstrap and verify | B3 | Verification depends on the final contract and implementation |

## Quality Gates

- `install.md` documents the managed global MCP bootstrap.
- `plugins/codex-orchestrator/.mcp.json` declares `cwd` and startup timeout.
- Regression tests cover the MCP bootstrap contract and pass.
- A fresh local Codex session exposes `orchestrator_*` tools after bootstrap.

## Execution Status

- Current wave: Completed
- Active task: none
- Blockers: None
- Last review result: fresh-session verification pass

## TODO List

- [x] B1. Create Spec And Execution Anchor
- [x] B2. Patch Plugin And Install Docs
- [x] B3. Extend Regression Coverage
- [x] B4. Apply Local Bootstrap And Verify

### Task B1: Create Spec And Execution Anchor

**Files:**
- Create: `docs/specs/2026-04-12-codex-orchestrator-mcp-bootstrap-install-design.md`
- Create: `docs/plans/active/2026-04-12-codex-orchestrator-mcp-bootstrap-install-implementation.md`
- Modify: `docs/index.md`
- Modify: `AGENTS.md`
- Modify: `task_plan.md`

**Category:** plan
**Owner Role:** harness-planner
**Task Status:** accepted
**Current Step:** Step 4
**Spec Review Status:** pass
**Quality Review Status:** pending
**Assigned Agent:** local-parent

- [x] Step 1: Reproduce the missing-tool failure
- [x] Step 2: Verify explicit global MCP registration makes tools appear
- [x] Step 3: Write the design spec
- [x] Step 4: Write the active implementation plan and switch routing docs

### Task B2: Patch Plugin And Install Docs

**Files:**
- Modify: `plugins/codex-orchestrator/.mcp.json`
- Modify: `install.md`
- Modify: `README.md`
- Modify: `docs/index.md`
- Modify: `AGENTS.md`
- Modify: `task_plan.md`

**Category:** docs
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Make the bundled MCP config explicit about cwd and timeout
- [x] Step 2: Add managed global MCP bootstrap guidance to `install.md`
- [x] Step 3: Update routing docs to point at the new install contract

### Task B3: Extend Regression Coverage

**Files:**
- Modify: `tests/install-guide.test.ts`
- Modify: `docs/plans/completed/2026-04-12-codex-orchestrator-mcp-bootstrap-install-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add assertions for MCP bootstrap guidance in the install guide
- [x] Step 2: Add assertions for bundled `.mcp.json` cwd and timeout
- [x] Step 3: Run the guide-contract suite

### Task B4: Apply Local Bootstrap And Verify

**Files:**
- Modify: `progress.md`
- Modify: `findings.md`
- Modify: `docs/plans/completed/2026-04-12-codex-orchestrator-mcp-bootstrap-install-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Reconcile the managed `mcp_servers.codex-orchestrator` block in local config
- [x] Step 2: Verify a fresh session sees `orchestrator_*` in the tool registry
- [x] Step 3: Sync progress and findings

## Final Acceptance

- [x] The install guide treats MCP visibility as part of installation correctness
- [x] The bundled MCP config is explicit about cwd and startup timeout
- [x] The guide-contract suite passes
- [x] Local fresh-session verification confirms orchestrator tools are exposed
