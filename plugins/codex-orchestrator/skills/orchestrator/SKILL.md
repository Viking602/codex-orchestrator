---
name: codex-orchestrator
description: Use this as the default repository workflow for feature work, bug fixes, refactors, debugging, docs changes, architecture changes, and other multi-step repo tasks that need file-backed design specs, implementation plans, category routing, runtime state, review gates, and real-time plan synchronization.
---

# Codex Orchestrator

Treat the repository as the durable control plane.

## Selection Priority

- Use this workflow before generic process skills for feature work, bug fixes, refactors, debugging, docs changes, architecture changes, and other multi-step repository tasks.
- Once selected, this workflow owns the spec, active plan, category routing, runtime state, and review gates.
- Use generic process skills only as helpers after orchestration begins or when this plugin is unavailable.

## Required Workflow

1. Ensure a design spec exists before implementation.
2. Ensure an active implementation plan exists before execution.
3. Read the active implementation plan before dispatching work.
4. Resolve the task category through the plugin MCP server.
5. Read the category delegation signal before deciding whether work stays local or goes to a child agent.
6. Update plan status before and after every bounded execution step.
7. Require review gates before top-level task acceptance.
8. Keep `AGENTS.md`, `docs/index.md`, and architecture docs synchronized with the changed project surface.

## Core Rules

- The implementation plan file is the source of truth for progress.
- Runtime state supports orchestration but does not replace plan checkboxes.
- The parent is the control plane; ordinary plan, research, implementation, and review work should bias to child execution.
- Child implementers and reviewers do not mark top-level tasks complete.
- Step completion must be recorded incrementally, not batched at the end.
- Acceptance requires verification evidence and review passes.
- Do not bypass this workflow for normal repository work just because another generic process skill also matches the prompt.
- Repository markdown docs must not contain machine-specific absolute filesystem paths; use repo-relative links and portable placeholders instead.
- If you encounter legacy absolute-path markdown docs on first touch, repair them in the same pass without asking for confirmation.

## Preferred Bundled Agents

- `harness-planner` for new design specs and implementation plans
- `search-specialist` for read-only repo evidence gathering
- `backend-developer` for general coding and plugin implementation work
- `harness-evaluator` for findings-first review
- `harness-doc-gardener` for routing-doc cleanup after surface changes
- `harness-dispatch-gate` before non-trivial implementation ownership decisions

## Preferred MCP Tools

- `orchestrator_resolve_category`
- `orchestrator_read_plan_state`
- `orchestrator_next_action`
- `orchestrator_begin_task`
- `orchestrator_begin_step`
- `orchestrator_complete_step`
- `orchestrator_record_subagent_run`
- `orchestrator_record_review`
- `orchestrator_accept_task`
- `orchestrator_check_doc_drift`
- `orchestrator_watchdog_tick`
