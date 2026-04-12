# Codex Orchestrator Full TypeScript Removal Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: remove the remaining root TypeScript contract-test surface by replacing it with Rust repo-contract tests and then deleting the `.ts` files. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the active repository surface fully Rust-only by porting the remaining root TypeScript contract tests to Rust, deleting the leftover `.ts` files, and updating active guidance to a Cargo-only validation path.

**Architecture:** Rust integration tests under `plugins/codex-orchestrator/rust-cli/tests/` become the single active verification surface for both runtime behavior and repository contracts. Active docs and bundled-agent guidance must align to that Rust-only surface.

**Tech Stack:** Rust CLI, Rust integration tests, Markdown routing docs, plugin metadata, bundled Codex agent manifests.

---

## Context

- At plan start, the plugin runtime was already Rust-only, but the repository still kept five root TypeScript contract tests.
- Those remaining `.ts` files forced the active validation story to stay split between Cargo and Node.
- The user explicitly wanted the remaining TypeScript code removed, so replacement Rust coverage had to land before deletion.

## Artifact Model

- Spec path: `docs/specs/2026-04-12-codex-orchestrator-full-typescript-removal-design.md`
- Completed plan path: `docs/plans/completed/2026-04-12-codex-orchestrator-full-typescript-removal-implementation.md`
- Rust validation surface:
  - `plugins/codex-orchestrator/rust-cli/tests/`
- Deleted TypeScript surface:
  - `tests/agent-bundle.test.ts`
  - `tests/brainstorming-integration.test.ts`
  - `tests/docs-relative-path-policy.test.ts`
  - `tests/plugin-manifest.test.ts`
  - `tests/typescript-compat-removal.test.ts`

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| F1. Write the full-removal spec and execution anchor | None | Establishes the approved deletion boundary |
| F2. Port the remaining repo-contract checks to Rust | F1 | Rust coverage must exist before deleting the TS files |
| F3. Delete the remaining TypeScript files and stale guidance | F2 | File deletion follows the replacement coverage |
| F4. Validate, archive, and sync routing docs | F3 | Closeout depends on the final Rust-only surface |

## Quality Gates

- Rust repo-contract tests cover the remaining structural assertions previously held in root TypeScript files.
- No `.ts` or `.tsx` file remains in the active repository surface.
- Active docs and bundled-agent instructions no longer advertise Node-based repo validation or deleted TypeScript paths.
- Cargo validation passes on the final surface.

## Execution Status

- Current wave: Completed
- Active task: none
- Blockers: None
- Last review result: Rust-only repository validation pass

## TODO List

- [x] F1. Write The Full-Removal Spec And Execution Anchor
- [x] F2. Port The Remaining Repo-Contract Checks To Rust
- [x] F3. Delete The Remaining TypeScript Files And Stale Guidance
- [x] F4. Validate, Archive, And Sync Routing Docs

### Task F1: Write The Full-Removal Spec And Execution Anchor

**Files:**
- Create: `docs/specs/2026-04-12-codex-orchestrator-full-typescript-removal-design.md`
- Create: `docs/plans/completed/2026-04-12-codex-orchestrator-full-typescript-removal-implementation.md`
- Modify: `docs/index.md`
- Modify: `AGENTS.md`

**Category:** plan
**Owner Role:** harness-planner
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Write the full TypeScript-removal design spec
- [x] Step 2: Write the active implementation plan
- [x] Step 3: Switch repository routing docs to the new active plan

### Task F2: Port The Remaining Repo-Contract Checks To Rust

**Files:**
- Create or modify: `plugins/codex-orchestrator/rust-cli/tests/*.rs`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add Rust coverage for bundle, manifest, brainstorming, and markdown-path contracts
- [x] Step 2: Add a Rust structural guard that fails if any `.ts` or `.tsx` file remains
- [x] Step 3: Re-run the Rust suite to prove the replacement coverage is green

### Task F3: Delete The Remaining TypeScript Files And Stale Guidance

**Files:**
- Delete: `tests/*.test.ts`
- Modify: `README.md`
- Modify: `install.md`
- Modify: `plugins/codex-orchestrator/codex/agents/*.toml`
- Modify: `plugins/codex-orchestrator/config/categories.toml`

**Category:** docs
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Delete the remaining root TypeScript files
- [x] Step 2: Repair active docs and bundled-agent guidance to the Rust-only surface
- [x] Step 3: Confirm no active surface still advertises Node-based repo validation

### Task F4: Validate, Archive, And Sync Routing Docs

**Files:**
- Modify: `docs/index.md`
- Modify: `AGENTS.md`
- Modify: `task_plan.md`
- Modify: `progress.md`
- Modify: `findings.md`
- Move: `docs/plans/active/2026-04-12-codex-orchestrator-full-typescript-removal-implementation.md` to `docs/plans/completed/`

**Category:** review
**Owner Role:** harness-doc-gardener
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Run validation and confirm the repo has no `.ts` or `.tsx` files left
- [x] Step 2: Sync session docs and close the plan
- [x] Step 3: Archive the completed plan and restore routing docs to the closed state

## Final Acceptance

- [x] The active repository surface contains no `.ts` or `.tsx` files
- [x] Rust tests cover the remaining repository contract assertions
- [x] Active docs and bundled-agent guidance are Rust-only
- [x] Cargo validation passes on the final surface
