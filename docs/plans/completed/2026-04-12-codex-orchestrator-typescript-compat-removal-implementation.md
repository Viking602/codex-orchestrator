# Codex Orchestrator TypeScript Compatibility Removal Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: remove the legacy TypeScript runtime compatibility layer now that the Rust CLI is the sole supported MCP runtime. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Delete the old TypeScript runtime and its runtime-coupled plugin-local tests while preserving behavior coverage and simplifying the plugin surface around the Rust CLI.

**Architecture:** Rust integration tests become the behavior boundary for orchestration logic. Repo-level contract tests cover structural and metadata expectations. The active plugin surface no longer includes `plugins/codex-orchestrator/src/` or plugin-local TypeScript runtime test scaffolding.

**Tech Stack:** Rust CLI, Rust integration tests, Node-based repo contract tests, Markdown routing docs.

---

## Context

- The Rust CLI is already the supported MCP runtime for both source-checkout development and installed Codex sessions.
- The old TypeScript runtime is now dead code, but plugin-local tests and config still import it directly.
- Removal should preserve test coverage rather than trading dead runtime cleanup for blind spots.

## Artifact Model

- Spec path: `docs/specs/2026-04-12-codex-orchestrator-typescript-compat-removal-design.md`
- Active plan path: `docs/plans/active/2026-04-12-codex-orchestrator-typescript-compat-removal-implementation.md`
- Runtime crate:
  - `plugins/codex-orchestrator/rust-cli/`
- Deleted compatibility surface:
  - `plugins/codex-orchestrator/src/`
  - `plugins/codex-orchestrator/tsconfig.json`
  - plugin-local tests that import `../src/**`
- Repo-level contract tests:
  - `tests/*.test.ts`

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| T1. Write the removal spec and execution anchor | None | Establishes the approved deletion boundary |
| T2. Add regression coverage for the deleted compatibility surface | T1 | Tests should fail before files are removed |
| T3. Delete the TypeScript runtime compatibility layer | T2 | File deletion follows the new regression boundary |
| T4. Sync docs and validate the simplified surface | T3 | Routing and closeout happen after the surface changes |

## Quality Gates

- A repo-level regression test fails if the deleted TypeScript runtime surface reappears.
- Rust integration tests cover the behavior previously protected by deleted plugin-local runtime tests.
- The plugin package surface no longer advertises the deleted TypeScript runtime as active.
- Repository validation passes after the removal.

## Execution Status

- Current wave: Completed
- Active task: none
- Blockers: None
- Last review result: Rust-only surface validation pass

## TODO List

- [x] T1. Write The Removal Spec And Execution Anchor
- [x] T2. Add Regression Coverage For The Deleted Compatibility Surface
- [x] T3. Delete The TypeScript Runtime Compatibility Layer
- [x] T4. Sync Docs And Validate The Simplified Surface

### Task T1: Write The Removal Spec And Execution Anchor

**Files:**
- Create: `docs/specs/2026-04-12-codex-orchestrator-typescript-compat-removal-design.md`
- Create: `docs/plans/completed/2026-04-12-codex-orchestrator-typescript-compat-removal-implementation.md`
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

- [x] Step 1: Write the TypeScript compatibility-removal design spec
- [x] Step 2: Write the active implementation plan
- [x] Step 3: Switch repository routing docs to the new active plan

### Task T2: Add Regression Coverage For The Deleted Compatibility Surface

**Files:**
- Create: `tests/typescript-compat-removal.test.ts`
- Modify: `plugins/codex-orchestrator/rust-cli/tests/*.rs`
- Move or create: `tests/plugin-manifest.test.ts`
- Move or create: `tests/docs-relative-path-policy.test.ts`
- Move or create: `tests/agent-bundle.test.ts`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add a failing structural regression test for the deleted TypeScript surface
- [x] Step 2: Port or replace plugin-local runtime coverage with Rust and repo-level tests
- [x] Step 3: Re-run the red-green verification loop on the new coverage

### Task T3: Delete The TypeScript Runtime Compatibility Layer

**Files:**
- Delete: `plugins/codex-orchestrator/src/**`
- Delete: `plugins/codex-orchestrator/tests/*.test.ts`
- Delete: `plugins/codex-orchestrator/tsconfig.json`
- Modify: `plugins/codex-orchestrator/package.json`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Remove runtime-coupled TypeScript source and test files
- [x] Step 2: Simplify package scripts around the Rust runtime
- [x] Step 3: Ensure no active repository path still references the deleted compatibility surface

### Task T4: Sync Docs And Validate The Simplified Surface

**Files:**
- Modify: `README.md`
- Modify: `install.md`
- Modify: `docs/index.md`
- Modify: `AGENTS.md`
- Modify: `task_plan.md`
- Modify: `progress.md`
- Modify: `findings.md`
- Modify: `docs/plans/completed/2026-04-12-codex-orchestrator-typescript-compat-removal-implementation.md`

**Category:** docs
**Owner Role:** harness-doc-gardener
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Update docs to say the Rust CLI is the only supported runtime
- [x] Step 2: Run repository validation and record evidence
- [x] Step 3: Sync findings, progress, and final acceptance

## Final Acceptance

- [x] The legacy TypeScript runtime and its plugin-local runtime tests are removed
- [x] Structural regression tests fail if the removed compatibility surface comes back
- [x] Rust and repo-level tests preserve behavior and metadata coverage
- [x] Repository validation passes on the simplified Rust-only plugin surface
