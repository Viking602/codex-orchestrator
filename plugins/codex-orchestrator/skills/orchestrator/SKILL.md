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
2. If requirements are incomplete or materially blocked, ask clarifying questions one at a time until purpose, constraints, and success criteria are clear.
3. If the direction is materially open or the user explicitly asked for options, propose 2-3 approaches with trade-offs and a recommendation before locking the design.
4. If the chosen design would materially reinterpret the user's request, present it in sections and get approval before writing the implementation plan.
5. If the user already supplied a workable direction and no hard blocker exists, summarize assumptions briefly and write the spec and implementation plan without asking a second confirmation question.
6. Ensure a design spec exists and reflects the approved or direction-clear design before implementation.
7. Perform a brief spec self-review for contradictions, ambiguity, missing scope boundaries, or placeholders.
8. Ensure an active implementation plan exists before execution.
9. Read the active implementation plan before dispatching work.
10. Resolve the task category through the plugin MCP server.
11. Read the category delegation signal before deciding whether work stays local or goes to a child agent.
12. Treat repository inspection, codebase checks, repo audits, and read-only repo-understanding requests as `research` work and dispatch `search-specialist` before the parent does first-pass evidence gathering.
13. If multiple top-level tasks are dependency-ready, category-compatible, and free of child-owned write-scope conflicts, dispatch them as one parallel child batch instead of serializing on the first task.
14. When `orchestrator_next_action` returns `parallel_task_ids` and `parallel_dispatches`, launch the whole returned cohort in one round and treat the first task id as the mirror anchor rather than the only task allowed to move.
15. If the top-level action is `acquire_parallel_write_leases`, acquire one lease per returned child dispatch scope and then launch that full batch.
16. Give each top-level task its own dedicated child session instead of letting the parent keep task-local execution context.
17. When `orchestrator_next_action` returns `task_session_mode`, `task_session_key`, and `continue_agent_id`, follow that routing literally: spawn a fresh task child when asked, resume the same task child when asked, and keep different top-level tasks on different child sessions.
18. When `orchestrator_next_action` returns `subagent_tool_action`, `subagent_agent_type`, and `subagent_dispatch_message`, treat them as executable child-dispatch instructions rather than advisory prose.
19. If `subagent_tool_action = spawn_agent`, call `spawn_agent` with `agent_type = subagent_agent_type` and pass `subagent_dispatch_message` as the bounded child brief.
20. If `subagent_tool_action = send_input`, call `send_input` against `continue_agent_id` and pass `subagent_dispatch_message` instead of reabsorbing that task work into the parent.
21. If `orchestrator_next_action` returns `blocking_control_plane_actions`, perform those writes before launching or resuming the child for that task.
22. Treat `child_execution_mode = current-step` as a hard boundary: the child owns only the current step on that resume and should return after that bounded step or a blocker.
23. Treat reviewer work as a separate child lane from implementation; reviewers must not overwrite or replace the task's dedicated implementer child.
24. Update plan status before and after every bounded execution step.
25. Treat `Current Step` as a live pointer, not a deferred summary: seed it when work starts and advance it immediately after each completed step.
26. When native `update_plan` is available, mirror the active implementation plan into that surface instead of inventing a separate chat todo.
27. Require review gates before top-level task acceptance.
28. When a terminal review pass closes the task, accept the top-level task in the same control-plane pass instead of leaving acceptance for a later sweep.
29. Keep `AGENTS.md`, `docs/index.md`, and architecture docs synchronized with the changed project surface.

## Core Rules

- `codex-orchestrator` subsumes the repository-useful parts of brainstorming; do not invoke `using-superpowers` or standalone `brainstorming` as the entry workflow for normal repository work.
- Treat clarification, approach comparison, design approval, and spec writing as part of orchestration rather than a reason to hand control to a different process stack.
- Do not reopen a settled user direction with extra options or a second `shall I proceed` question when no hard blocker exists.
- If you are tempted to ask for redundant start confirmation, treat it as `direction_confirmation`, record assumptions, and continue into spec, plan, and execution.
- The implementation plan file is the source of truth for progress.
- Runtime state supports orchestration but does not replace plan checkboxes.
- The parent is the control plane; ordinary plan, research, implementation, and review work should bias to child execution.
- Repository inspection, codebase-check, repo-audit, and read-only codebase-understanding requests should usually start by dispatching `search-specialist` instead of keeping the parent on the initial search pass.
- Dependency-ready top-level tasks that do not conflict on child-owned write scope should be dispatched together as one batch when the category contract permits parallelism.
- Treat `parallel_task_ids` and `parallel_dispatches` from `orchestrator_next_action` as executable scheduling data, not advisory prose.
- Treat `task_session_mode`, `task_session_key`, and `continue_agent_id` from `orchestrator_next_action` as the session-ownership contract for the task.
- Treat `subagent_tool_action`, `subagent_agent_type`, and `subagent_dispatch_message` from `orchestrator_next_action` as executable child-launch instructions.
- `subagent_tool_action = spawn_agent` means the parent should call `spawn_agent` immediately instead of doing the task work itself.
- `subagent_tool_action = send_input` means the parent should resume the owned child with `send_input` instead of reabsorbing that task-local context.
- Treat `blocking_control_plane_actions` from `orchestrator_next_action` as blocking control-plane actions, not optional hints.
- If `blocking_control_plane_actions` is non-empty, the parent must perform those writes before launching or resuming the child.
- Treat `child_execution_mode = current-step` as a current-step-only resume contract; do not let a child absorb multiple bounded plan steps in one uninterrupted run.
- One top-level task should map to one dedicated implementer child session; do not reuse one child across different top-level tasks just because the role is the same.
- If a task-owned implementer child already exists, repair and continuation should resume that child instead of spawning a new generic worker.
- Reviewer children are separate guardrails: spawn or resume them from the review lane, but do not let them erase the task's dedicated implementer ownership.
- Parent-owned coordination artifacts such as the active plan file, `task_plan.md`, `progress.md`, `findings.md`, `AGENTS.md`, and routing docs do not count as child write conflicts.
- If `orchestrator_next_action` returns `acquire_parallel_write_leases`, acquire each required lease first and then launch the whole returned batch instead of falling back to serial dispatch.
- Child implementers and reviewers do not mark top-level tasks complete.
- Parent acceptance must happen immediately once a terminal review pass closes a task; do not batch accepted-task checkbox updates at end of wave.
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
