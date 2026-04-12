---
name: codex-orchestrator
description: Use this as the default repository workflow for discovery, requirements clarification, design approval, feature work, bug fixes, refactors, debugging, docs changes, architecture changes, and other multi-step repo tasks that need file-backed specs, implementation plans, category routing, runtime state, review gates, and real-time plan synchronization.
---

# Codex Orchestrator

Treat the repository as the durable control plane.

## Selection Priority

- Use this workflow before generic process skills for discovery, requirements clarification, design work, feature work, bug fixes, refactors, debugging, docs changes, architecture changes, and other multi-step repository tasks.
- For repository tasks, do not enter through `using-superpowers` or standalone `brainstorming`; this workflow already absorbs the repository brainstorming stage.
- Once selected, this workflow owns the spec, active plan, category routing, runtime state, and review gates.
- Use generic process skills only as helpers after orchestration begins or when this plugin is unavailable.

## Required Workflow

1. Explore the current project context before proposing a design or plan.
2. If requirements are incomplete, ask clarifying questions one at a time until purpose, constraints, and success criteria are clear.
3. Propose 2-3 approaches with trade-offs and a recommendation before locking the design.
4. Present the design in sections and get user approval before writing the implementation plan.
5. Ensure a design spec exists and reflects the approved design before implementation.
6. Perform a brief spec self-review for contradictions, ambiguity, missing scope boundaries, or placeholders.
7. Ensure an active implementation plan exists before execution.
8. Read the active implementation plan before dispatching work.
9. Resolve the task category through the plugin MCP server.
10. Read the category delegation signal before deciding whether work stays local or goes to a child agent.
11. Update plan status before and after every bounded execution step.
12. Treat `Current Step` as a live pointer, not a deferred summary: seed it when work starts and advance it immediately after each completed step.
13. When native `update_plan` is available, mirror the active implementation plan into that surface instead of inventing a separate chat todo.
14. Require review gates before top-level task acceptance.
15. Keep `AGENTS.md`, `docs/index.md`, and architecture docs synchronized with the changed project surface.

## Core Rules

- `codex-orchestrator` subsumes the repository-useful parts of brainstorming; do not invoke `using-superpowers` or standalone `brainstorming` as the entry workflow for normal repository work.
- Treat clarification, approach comparison, design approval, and spec writing as part of orchestration rather than a reason to hand control to a different process stack.
- The implementation plan file is the source of truth for progress.
- Runtime state supports orchestration but does not replace plan checkboxes.
- The parent is the control plane; ordinary plan, research, implementation, and review work should bias to child execution.
- Child implementers and reviewers do not mark top-level tasks complete.
- Step completion must be recorded incrementally, not batched at the end.
- If `orchestrator_next_action` says step sync is missing or stale, repair that drift before continuing the task.
- Prefer plans whose top-level tasks are small enough to produce visible progress instead of hiding most progress inside one oversized task.
- Use `orchestrator_export_codex_todo` plus native `update_plan` to mirror plan progress when available; do not keep a separate prose todo for plan-tracked work.
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
- `orchestrator_export_codex_todo`
- `orchestrator_next_action`
- `orchestrator_begin_task`
- `orchestrator_begin_step`
- `orchestrator_complete_step`
- `orchestrator_record_subagent_run`
- `orchestrator_record_review`
- `orchestrator_accept_task`
- `orchestrator_check_doc_drift`
- `orchestrator_watchdog_tick`
