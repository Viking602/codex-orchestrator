# Project03

## Purpose

This repository hosts the Codex orchestrator plugin that replaces the core engineering workflow previously delegated to `harness-engineering` and related superpowers process skills.

## Routing

- Read [docs/index.md](docs/index.md) first for the document map.
- Read [install.md](install.md) when the task is installing or verifying plugin installation.
- Read the active implementation plan in `docs/plans/active/` when one exists before changing behavior.
- Treat the implementation plan file as the execution source of truth.
- Update routing docs in the same pass when paths, commands, entrypoints, or document locations change.

## Default Workflow

- For repository work in this repo, `codex-orchestrator` is the default workflow.
- `codex-orchestrator` absorbs the repository brainstorming stage, so normal repo tasks must not enter through `using-superpowers` or standalone `brainstorming`.
- Invoke the installed `codex-orchestrator` bundled skill before generic process skills when doing feature work, bug fixes, refactors, debugging, docs changes, architecture work, or other multi-step repo tasks.
- Treat generic process skills as subordinate helpers after orchestration begins or when the plugin is unavailable.
- During discovery and design, explore context first, ask clarifying questions one at a time only when something material is missing, compare 2-3 approaches only when the direction is still open, and ask for approval before the implementation plan only when the proposed direction materially changes the request.
- If the user already supplied a workable direction and no hard blocker exists, do not ask a second confirmation question; summarize assumptions, write the spec and plan, and continue.
- Start by reading the active plan when one exists, then keep plan status and routing docs synchronized throughout execution.
- Treat repository inspection, codebase-check, repo-audit, and read-only codebase-understanding requests as `research` work that should dispatch `search-specialist` before the parent keeps that work local.
- Do not let the parent absorb first-pass repo inspection when `search-specialist` can gather the evidence read-only.
- When multiple top-level tasks are dependency-ready, category-compatible, and free of child-owned write-scope conflicts, dispatch them together as one parallel child batch instead of serializing on the first task.
- If `orchestrator_next_action` returns `parallel_task_ids` and `parallel_dispatches`, the parent should launch the whole returned cohort in one round and use the first task id only as the native todo mirror anchor.
- If the top-level action is `acquire_parallel_write_leases`, acquire one lease per returned child dispatch scope and then launch the full batch.
- Give each top-level task its own dedicated child session; the parent should stay control-plane-only instead of keeping task-local execution context.
- If `orchestrator_next_action` returns `task_session_mode`, `task_session_key`, and `continue_agent_id`, the parent should follow that routing literally and keep different top-level tasks on different child sessions.
- If `orchestrator_next_action` returns `subagent_tool_action`, `subagent_agent_type`, and `subagent_dispatch_message`, the parent should execute that child-dispatch contract literally with `spawn_agent` or `send_input` instead of doing the task work locally.
- If `orchestrator_next_action` returns `blocking_control_plane_actions`, the parent must perform those writes before launching or resuming the child.
- If `child_execution_mode` is `current-step`, the child owns only the current step for that task resume and should return after that bounded step or a blocker.
- Reviewer work stays on a separate child lane and must not overwrite the task's dedicated implementer ownership.
- Parent-owned coordination artifacts such as the active plan file, `task_plan.md`, `progress.md`, `findings.md`, `AGENTS.md`, and routing docs do not count as child write conflicts.

## Execution Rules

- Keep specification, implementation plan, and execution status in files, not only in chat.
- Do not treat runtime state as the final truth for completion. Plan checkboxes are authoritative.
- Parent orchestration owns task acceptance and plan checkbox updates.
- Child-dispatch instructions from `orchestrator_next_action` must be executed as real `spawn_agent` or `send_input` tool calls, not translated into parent-local implementation.
- Parent orchestration must not defer begin-task, begin-step, or step-repair writes into a terminal replay batch when `orchestrator_next_action` already identified them.
- When a terminal review pass closes a task, parent acceptance must happen in the same control-plane pass instead of a later end-of-wave sweep.
- Child implementers and reviewers must not mark top-level plan tasks complete.
- During active work, `Current Step` must point at the actionable unchecked step instead of lingering at `none`.
- Step synchronization drift is a repository defect; repair missing or stale step pointers before continuing implementation.
- When native Codex `update_plan` is available, mirror the active implementation plan into that surface instead of maintaining a separate chat todo.
- Documentation in this repository must never contain machine-specific absolute filesystem paths.
- Repository docs must use repo-relative artifact links such as `docs/...`, `../AGENTS.md`, or `privacy-policy.md`.
- When an agent first touches the repository and finds legacy absolute-path documentation, it must repair those docs in the same pass without asking for confirmation.

## Current Artifact Model

- Design specs: `docs/specs/`
- Active implementation plans: `docs/plans/active/`
- Completed implementation plans: `docs/plans/completed/`
- Architecture notes: `docs/architecture/`
- Product notes: `docs/product/`
- Decisions: `docs/decisions/`
- Session planning files: `task_plan.md`, `findings.md`, `progress.md`

## Current Key Docs

- [Design spec](docs/specs/2026-04-08-codex-orchestrator-plugin-design.md)
- [Phase 2 design spec](docs/specs/2026-04-09-codex-orchestrator-phase-2-design.md)
- [Phase 3 design spec](docs/specs/2026-04-09-codex-orchestrator-phase-3-design.md)
- [Bundled agents design spec](docs/specs/2026-04-09-codex-orchestrator-bundled-agents-design.md)
- [Installer design spec](docs/specs/2026-04-10-codex-orchestrator-installer-design.md)
- [Marketplace install design spec](docs/specs/2026-04-11-codex-orchestrator-marketplace-install-design.md)
- [Default workflow routing design spec](docs/specs/2026-04-11-codex-orchestrator-default-workflow-routing-design.md)
- [Delegation-first dispatch design spec](docs/specs/2026-04-11-codex-orchestrator-delegation-first-dispatch-design.md)
- [Incremental step synchronization design spec](docs/specs/2026-04-11-codex-orchestrator-incremental-step-sync-design.md)
- [Native Codex todo mirroring design spec](docs/specs/2026-04-11-codex-orchestrator-native-codex-todo-mirroring-design.md)
- [Codex-guided install design spec](docs/specs/2026-04-11-codex-orchestrator-codex-guided-install-design.md)
- [MCP bootstrap install design spec](docs/specs/2026-04-12-codex-orchestrator-mcp-bootstrap-install-design.md)
- [Rust MCP CLI design spec](docs/specs/2026-04-12-codex-orchestrator-rust-mcp-cli-design.md)
- [TypeScript compatibility removal design spec](docs/specs/2026-04-12-codex-orchestrator-typescript-compat-removal-design.md)
- [Full TypeScript removal design spec](docs/specs/2026-04-12-codex-orchestrator-full-typescript-removal-design.md)
- [Brainstorming integration design spec](docs/specs/2026-04-12-codex-orchestrator-brainstorming-integration-design.md)
- [Inspection-first delegation design spec](docs/specs/2026-04-12-codex-orchestrator-inspection-first-delegation-design.md)
- [Direction-first execution design spec](docs/specs/2026-04-12-codex-orchestrator-direction-first-execution-design.md)
- [Immediate top-level acceptance design spec](docs/specs/2026-04-12-codex-orchestrator-immediate-top-level-acceptance-design.md)
- [Parallel top-level dispatch design spec](docs/specs/2026-04-12-codex-orchestrator-parallel-top-level-dispatch-design.md)
- [Task-owned subagent sessions design spec](docs/specs/2026-04-12-codex-orchestrator-task-owned-subagent-sessions-design.md)
- [Mid-run control-plane checkpoints design spec](docs/specs/2026-04-12-codex-orchestrator-mid-run-control-plane-checkpoints-design.md)
- [Executable subagent dispatch design spec](docs/specs/2026-04-12-codex-orchestrator-executable-subagent-dispatch-design.md)
- [Install guide design spec](docs/specs/2026-04-11-codex-orchestrator-install-guide-design.md)
- [Completed plan auto-archive design spec](docs/specs/2026-04-11-codex-orchestrator-plan-archive-design.md)
- [Relative doc-path policy design spec](docs/specs/2026-04-11-codex-orchestrator-doc-relative-path-policy-design.md)
- [Root install guide](install.md)
- [Phase 1 completed plan](docs/plans/completed/2026-04-08-codex-orchestrator-plugin-implementation.md)
- [Phase 2 completed plan](docs/plans/completed/2026-04-09-codex-orchestrator-phase-2-implementation.md)
- [Phase 3 completed plan](docs/plans/completed/2026-04-09-codex-orchestrator-phase-3-implementation.md)
- [Bundled agents completed plan](docs/plans/completed/2026-04-09-codex-orchestrator-bundled-agents-implementation.md)
- [Installer completed plan](docs/plans/completed/2026-04-10-codex-orchestrator-installer-implementation.md)
- [Marketplace install completed plan](docs/plans/completed/2026-04-11-codex-orchestrator-marketplace-install-implementation.md)
- [Default workflow routing completed plan](docs/plans/completed/2026-04-11-codex-orchestrator-default-workflow-routing-implementation.md)
- [Delegation-first dispatch completed plan](docs/plans/completed/2026-04-11-codex-orchestrator-delegation-first-dispatch-implementation.md)
- [Incremental step synchronization completed plan](docs/plans/completed/2026-04-11-codex-orchestrator-incremental-step-sync-implementation.md)
- [Native Codex todo mirroring completed plan](docs/plans/completed/2026-04-11-codex-orchestrator-native-codex-todo-mirroring-implementation.md)
- [Codex-guided install completed plan](docs/plans/completed/2026-04-11-codex-orchestrator-codex-guided-install-implementation.md)
- [MCP bootstrap completed plan](docs/plans/completed/2026-04-12-codex-orchestrator-mcp-bootstrap-install-implementation.md)
- [Rust MCP CLI completed plan](docs/plans/completed/2026-04-12-codex-orchestrator-rust-mcp-cli-implementation.md)
- [TypeScript compatibility removal completed plan](docs/plans/completed/2026-04-12-codex-orchestrator-typescript-compat-removal-implementation.md)
- [Full TypeScript removal completed plan](docs/plans/completed/2026-04-12-codex-orchestrator-full-typescript-removal-implementation.md)
- [Brainstorming integration completed plan](docs/plans/completed/2026-04-12-codex-orchestrator-brainstorming-integration-implementation.md)
- [Inspection-first delegation completed plan](docs/plans/completed/2026-04-12-codex-orchestrator-inspection-first-delegation-implementation.md)
- [Direction-first execution completed plan](docs/plans/completed/2026-04-12-codex-orchestrator-direction-first-execution-implementation.md)
- [Immediate top-level acceptance completed plan](docs/plans/completed/2026-04-12-codex-orchestrator-immediate-top-level-acceptance-implementation.md)
- [Parallel top-level dispatch completed plan](docs/plans/completed/2026-04-12-codex-orchestrator-parallel-top-level-dispatch-implementation.md)
- [Task-owned subagent sessions completed plan](docs/plans/completed/2026-04-12-codex-orchestrator-task-owned-subagent-sessions-implementation.md)
- [Mid-run control-plane checkpoints completed plan](docs/plans/completed/2026-04-12-codex-orchestrator-mid-run-control-plane-checkpoints-implementation.md)
- [Executable subagent dispatch completed plan](docs/plans/completed/2026-04-12-codex-orchestrator-executable-subagent-dispatch-implementation.md)
- [Install guide completed plan](docs/plans/completed/2026-04-11-codex-orchestrator-install-guide-implementation.md)
- [Completed plan auto-archive implementation plan](docs/plans/completed/2026-04-11-codex-orchestrator-plan-archive-implementation.md)
- [Relative doc-path policy completed plan](docs/plans/completed/2026-04-11-codex-orchestrator-doc-relative-path-policy-implementation.md)
- [Category contract](docs/architecture/category-contract.md)
- [Runtime state schema](docs/architecture/runtime-state-schema.md)
- [Write lease protocol](docs/architecture/write-lease-protocol.md)
- [MCP tool contract](docs/architecture/mcp-tool-contract.md)
- [Agent contracts](docs/architecture/agent-contracts.md)
- [Bundled agent bundle](docs/architecture/bundled-agent-bundle.md)
- [Plan sync rules](docs/architecture/plan-sync-rules.md)
- [Question gate protocol](docs/architecture/question-gate-protocol.md)
- [Completion guard](docs/architecture/completion-guard.md)
- [Review repair loop](docs/architecture/review-repair-loop.md)
