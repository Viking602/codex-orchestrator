# Codex Orchestrator Immediate Top-Level Acceptance Design

## Context

`codex-orchestrator` already mirrors file-backed plans into Codex native `update_plan`, but the mirror only changes top-level todo state when the plan's top-level checkbox changes.

That means a common parent-agent failure mode still leaks through:

- a task finishes all bounded steps
- review gates pass
- the parent does not run top-level acceptance immediately
- the native todo mirror keeps the first task `in_progress`
- several top-level tasks then jump to `completed` together when acceptance is batch-applied later

The user-visible effect is exactly the wrong progress shape: the first item appears stuck for a long time, then the whole list suddenly completes.

## Goals

- Advance the top-level todo mirror at the real task-completion boundary instead of waiting for a later acceptance sweep.
- Keep the implementation plan markdown as the source of truth.
- Preserve parent-owned acceptance semantics while removing the unnecessary gap between terminal review pass and top-level acceptance.
- Keep execution status aligned so `Active task` no longer lingers on an already accepted task.

## Non-goals

- Replacing top-level task completion with per-step top-level completion.
- Letting child implementers or reviewers mark top-level tasks complete directly.
- Moving plan truth into runtime state or Codex native todo state.
- Redesigning the broader review/repair loop.

## Design

### 1. Accept Immediately When The Terminal Review Closes The Task

`orchestrator_record_review` should immediately accept the task when all of these are true after the review write lands:

- all plan steps are checked
- spec review status is `pass`
- quality review status is `pass`

This is still parent-owned control-plane work because the parent is the caller of `orchestrator_record_review`. The plugin is only removing the fragile follow-up gap where the parent had to remember a second `accept_task` call after the terminal quality review pass.

### 2. Share Acceptance Logic Between Explicit And Immediate Acceptance Paths

The runtime should use one helper for top-level acceptance so both paths stay identical:

- explicit `orchestrator_accept_task`
- immediate acceptance triggered by `orchestrator_record_review`

That shared helper should:

- set task runtime status to `accepted`
- clear the task step pointer
- set plan task status to `accepted`
- clear plan `Current Step`
- check the task's top-level TODO checkbox
- advance `Execution Status -> Active task` to the next unchecked top-level task, or `none`

### 3. Surface The Acceptance Result In The Review Payload

`orchestrator_record_review` should return whether the task was accepted in the same control-plane pass.

Useful payload fields:

- `accepted`
- `top_level_todo_checked`
- `next_active_task_id`
- final `task_status`

This lets the parent refresh native `update_plan` immediately instead of inferring whether acceptance still needs a second call.

### 4. Tighten Workflow Guidance Against Late Acceptance

Workflow docs should say:

- when the terminal review pass closes a task, top-level acceptance should happen in the same pass
- parent agents must not defer accepted-task checkbox updates to an end-of-wave sweep
- native todo refresh should follow immediate acceptance so the next top-level task becomes visible right away

### 5. Add Regression Coverage For The User-Visible Failure Mode

Regression tests should cover:

- `orchestrator_record_review` auto-accepting a terminal-ready task
- `orchestrator_export_codex_todo` showing the next top-level task as `in_progress` immediately after that review pass
- execution status `Active task` advancing away from the accepted task

## Verification Strategy

Verification should cover:

1. Rust integration tests for immediate acceptance after terminal quality review pass
2. Rust repo-contract tests for workflow wording about immediate acceptance
3. a full Cargo test run

## Success Criteria

- A task no longer waits for a later explicit sweep once its terminal review pass closes all gates.
- The first native todo item does not remain `in_progress` after its task is already acceptance-ready.
- The next top-level task becomes visible immediately in the mirror.
- Docs and runtime behavior align on immediate top-level acceptance.
