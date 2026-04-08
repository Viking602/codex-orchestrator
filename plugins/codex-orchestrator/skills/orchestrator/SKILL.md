---
name: codex-orchestrator
description: Use this as the default repository workflow when you need file-backed design specs, implementation plans, category routing, runtime state, review gates, and real-time plan synchronization.
---

# Codex Orchestrator

Treat the repository as the durable control plane.

## Required Workflow

1. Ensure a design spec exists before implementation.
2. Ensure an active implementation plan exists before execution.
3. Read the active implementation plan before dispatching work.
4. Resolve the task category through the plugin MCP server.
5. Update plan status before and after every bounded execution step.
6. Require review gates before top-level task acceptance.
7. Keep `AGENTS.md`, `docs/index.md`, and architecture docs synchronized with the changed project surface.

## Core Rules

- The implementation plan file is the source of truth for progress.
- Runtime state supports orchestration but does not replace plan checkboxes.
- Child implementers and reviewers do not mark top-level tasks complete.
- Step completion must be recorded incrementally, not batched at the end.
- Acceptance requires verification evidence and review passes.

## Preferred MCP Tools

- `orchestrator_resolve_category`
- `orchestrator_read_plan_state`
- `orchestrator_begin_task`
- `orchestrator_begin_step`
- `orchestrator_complete_step`
- `orchestrator_record_subagent_run`
- `orchestrator_record_review`
- `orchestrator_accept_task`
- `orchestrator_check_doc_drift`
- `orchestrator_watchdog_tick`

