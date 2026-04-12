# Codex Orchestrator Rust MCP CLI Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: migrate the active MCP runtime from TypeScript-on-Node to a Rust CLI while preserving the existing orchestration contract. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the active MCP runtime with a Rust CLI and switch install/bootstrap guidance so installed Codex sessions launch the native binary instead of `src/server.ts`.

**Architecture:** A new Rust crate under `plugins/codex-orchestrator/rust-cli/` becomes the authoritative stdio MCP server. The bundled plugin config uses Cargo for source-checkout development, while Codex-guided install builds a native binary into the installed cache and points the managed `[mcp_servers.codex-orchestrator]` entry at that executable.

**Tech Stack:** Rust CLI, SQLite via Rust, TOML + JSON-RPC over stdio, Markdown routing docs, Node-based contract tests for repository guidance.

---

## Context

- At plan start, the install contract guaranteed MCP visibility, but the actual runtime still depended on `node --experimental-strip-types`.
- Distribution should not depend on `npx`, Node-only startup, or TypeScript source parsing at installed-runtime execution time.
- Tool semantics should remain stable for parent workflows and bundled skill behavior.

## Artifact Model

- Spec path: `docs/specs/2026-04-12-codex-orchestrator-rust-mcp-cli-design.md`
- Completed plan path: `docs/plans/completed/2026-04-12-codex-orchestrator-rust-mcp-cli-implementation.md`
- New runtime crate:
  - `plugins/codex-orchestrator/rust-cli/`
- Plugin runtime config:
  - `plugins/codex-orchestrator/.mcp.json`
- Install and routing docs:
  - `install.md`
  - `README.md`
  - `docs/index.md`
  - `AGENTS.md`
  - `task_plan.md`
- Contract tests:
  - `tests/install-guide.test.ts`
- Runtime verification:
  - fresh `codex exec` validation against native MCP bootstrap

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| R1. Create migration spec and execution anchor | None | Establishes the contract |
| R2. Implement the Rust MCP CLI core | R1 | Runtime code follows the approved design |
| R3. Switch config and install guidance to native bootstrap | R2 | Docs must reflect the implemented runtime path |
| R4. Add coverage and validate native runtime behavior | R3 | Verification depends on final runtime and install contract |

## Quality Gates

- The Rust CLI builds successfully from the repository.
- The plugin `.mcp.json` no longer uses `node` for source-checkout execution.
- The install guide documents native-binary bootstrap for installed runtime.
- Contract tests pass after the runtime-path changes.
- Fresh-session validation confirms `orchestrator_*` tools are callable through the Rust CLI bootstrap.

## Execution Status

- Current wave: Completed
- Active task: none
- Blockers: None
- Last review result: native bootstrap validation pass

## TODO List

- [x] R1. Create Migration Spec And Execution Anchor
- [x] R2. Implement The Rust MCP CLI Core
- [x] R3. Switch Config And Install Guidance To Native Bootstrap
- [x] R4. Add Coverage And Validate Native Runtime Behavior

### Task R1: Create Migration Spec And Execution Anchor

**Files:**
- Create: `docs/specs/2026-04-12-codex-orchestrator-rust-mcp-cli-design.md`
- Create: `docs/plans/completed/2026-04-12-codex-orchestrator-rust-mcp-cli-implementation.md`
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

- [x] Step 1: Write the Rust MCP CLI design spec
- [x] Step 2: Write the active implementation plan
- [x] Step 3: Switch repository routing docs to the new active plan

### Task R2: Implement The Rust MCP CLI Core

**Files:**
- Create: `plugins/codex-orchestrator/rust-cli/Cargo.toml`
- Create: `plugins/codex-orchestrator/rust-cli/src/main.rs`
- Create: `plugins/codex-orchestrator/rust-cli/src/*.rs`
- Modify: `plugins/codex-orchestrator/.mcp.json`
- Modify: `plugins/codex-orchestrator/package.json`

**Category:** backend-impl
**Owner Role:** backend-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Create the Rust crate and protocol skeleton
- [x] Step 2: Port category, plan, runtime-store, and doc-drift services
- [x] Step 3: Port the `orchestrator_*` tool registry and handlers
- [x] Step 4: Switch bundled development MCP config away from `node`

### Task R3: Switch Config And Install Guidance To Native Bootstrap

**Files:**
- Modify: `install.md`
- Modify: `README.md`
- Modify: `docs/index.md`
- Modify: `AGENTS.md`
- Modify: `task_plan.md`

**Category:** docs
**Owner Role:** harness-doc-gardener
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Document build-and-stage flow for the native binary
- [x] Step 2: Update managed MCP bootstrap examples to point at the binary
- [x] Step 3: Sync routing docs and task-plan state

### Task R4: Add Coverage And Validate Native Runtime Behavior

**Files:**
- Modify: `tests/install-guide.test.ts`
- Create: `plugins/codex-orchestrator/rust-cli/tests/*.rs`
- Modify: `progress.md`
- Modify: `findings.md`
- Modify: `docs/plans/completed/2026-04-12-codex-orchestrator-rust-mcp-cli-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent

- [x] Step 1: Add repository contract coverage for native bootstrap guidance
- [x] Step 2: Add Rust runtime tests and run them
- [x] Step 3: Validate a fresh Codex session sees `orchestrator_*` through the native binary
- [x] Step 4: Sync findings, progress, and final acceptance

## Final Acceptance

- [x] The Rust MCP CLI builds successfully and exposes the orchestration tool surface
- [x] The plugin no longer depends on `node` for installed-runtime MCP execution
- [x] Install guidance bootstraps a native binary instead of `src/server.ts`
- [x] Repository contract tests and Rust runtime tests pass
- [x] Fresh-session validation confirms the Rust CLI bootstrap exposes callable `orchestrator_*` tools
