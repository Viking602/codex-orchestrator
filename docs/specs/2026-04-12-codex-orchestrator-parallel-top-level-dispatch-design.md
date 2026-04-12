# Codex Orchestrator Parallel Top-Level Dispatch Design

## Context

`codex-orchestrator` already classifies work, enforces delegation bias, and emits whether the next step should stay in the parent or go to a child agent.

However, the current control plane still advances through top-level tasks one at a time. `orchestrator_next_action` picks the first unchecked top-level task and only returns a single dispatch target. That leaves throughput on the table when the implementation plan already says several tasks are independent and their child-owned write scopes do not overlap.

The user wants a stronger default:

- if multiple top-level tasks have no unmet dependencies
- and their child-owned write scopes do not conflict
- and they are in the same dispatch stage

then the parent should receive a parallel dispatch batch instead of being forced into strictly serial subagent handoff.

## Goals

- Reuse the existing implementation-plan format instead of inventing a new dispatch DSL.
- Detect dependency-ready top-level tasks from the plan's `Task Dependency Graph`.
- Detect child-owned write conflicts from each task's declared `Files:` list.
- Let `orchestrator_next_action` return a parallel dispatch cohort when multiple top-level tasks are safe to launch together.
- Preserve the parent as the orchestration layer and keep file-backed plans authoritative.

## Non-goals

- Having the MCP server call Codex subagent APIs directly.
- Representing multiple `in_progress` items in the native Codex todo mirror.
- Replacing the single `Execution Status -> Active task` field with a new multi-active plan schema in this pass.
- Treating parent-owned routing/status files as child write-scope conflicts.

## Design

### 1. Parse Dependency And File Metadata From The Existing Plan

The current plan template already contains what the scheduler needs:

- `## Task Dependency Graph`
- per-task `**Files:**`

The runtime should parse both and attach them to each `PlanTask`:

- `depends_on`
- `declared_files`

Dependency rules:

- a task is dependency-ready only when every `depends_on` task already has its top-level TODO checked
- absence of dependencies means the task is eligible from a dependency perspective

### 2. Derive Child-Owned Dispatch Scope Conservatively

The child-owned dispatch scope should be derived from `declared_files`, but should exclude parent-owned coordination artifacts such as:

- `task_plan.md`
- `progress.md`
- `findings.md`
- `AGENTS.md`
- `docs/index.md`
- `docs/plans/active/...`
- `docs/plans/completed/...`

These files are orchestration/routing surfaces. They are often listed in task files, but they should not falsely block parallel child execution when the parent is the only writer for those artifacts.

The remaining normalized file list is the task's child dispatch scope.

### 3. Extend `orchestrator_next_action` With A Parallel Dispatch Cohort

`orchestrator_next_action` should continue returning the primary task/action, but when the primary task is a child-owned dispatch stage it should also search for additional runnable tasks that satisfy all of these conditions:

- unchecked top-level task
- dependency-ready
- same child-owned dispatch stage as the primary task
- `requires_subagent = true`
- no child-owned write-scope conflict with already selected batch members

When two or more tasks are selected, `orchestrator_next_action` should return:

- `action = "dispatch_parallel_tasks"`
- `parallel_task_ids`
- `parallel_dispatches`

Each `parallel_dispatches` entry should include:

- `task_id`
- `title`
- `dispatch_role`
- `dispatch_mode`
- `requires_write_lease`
- `dispatch_scope`
- `depends_on`
- `reason`

### 4. Parallelism Rules Stay Category-Aware

Category semantics still apply:

- `research` and `review` may batch in `parallel-subagents`
- `backend-impl` may batch only when child-owned write scopes are disjoint, exposed as `write-scope-subagent`
- `plan` remains effectively serial because its category parallelism is `single`

This preserves the existing category contract rather than layering a separate scheduler with inconsistent rules.

### 5. Keep One Primary Active Task For Mirror Compatibility

The plan file and native todo mirror currently model a single primary active task. This design keeps that contract:

- the primary selected task remains the `task_id` and active-plan anchor
- additional parallel tasks are exposed through `parallel_dispatches`
- parent/workflow guidance should dispatch the whole batch, but the mirror remains anchored on the primary task until a future multi-active plan design exists

### 6. Tighten Workflow Guidance

Workflow docs should say:

- when `orchestrator_next_action` returns a parallel dispatch cohort, the parent should launch those child agents in the same round
- the parent must still keep plan synchronization, review gating, and acceptance centralized
- parallel batching is allowed only for dependency-ready, conflict-free top-level tasks

## Verification Strategy

Verification should cover:

1. Rust integration tests for dependency parsing and conflict-aware parallel dispatch batches
2. Rust repo-contract tests for workflow wording around parallel batches
3. a full Cargo test run

## Success Criteria

- `Task Dependency Graph` and `Files:` metadata are machine-readable in the runtime.
- `orchestrator_next_action` returns a multi-task parallel batch when tasks are independent and conflict-free.
- conflicting or dependency-blocked tasks stay out of the batch.
- workflow docs instruct the parent to dispatch the returned batch in one round.
