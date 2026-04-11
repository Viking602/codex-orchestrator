# Codex Orchestrator Incremental Step Synchronization Design

## Context

The plugin already has `orchestrator_begin_step` and `orchestrator_complete_step`, but the control-plane contract is still too weak to produce reliably gradual progress updates in practice. Parent agents can still treat a whole top-level task as one long opaque execution segment, then batch several step updates at the end. That creates the user-visible failure mode where the first TODO item appears stuck for a long time and then multiple completions land at once.

The problem is not only instruction quality. The MCP surface does not currently tell the parent:

- which unchecked step should start next
- whether the current step pointer is missing or stale
- whether step progress was advanced after a completion
- whether the active task has drifted away from its step synchronization contract

As a result, gradual progress is optional behavior instead of the default machine-guided path.

## Goals

- Make incremental step progress the default control-plane behavior instead of a prompt-only suggestion.
- Expose machine-readable step guidance through `orchestrator_next_action`.
- Auto-seed the current step pointer when a task begins and auto-advance it when a step completes.
- Detect and surface step-sync drift when a task has open steps but no valid current-step pointer.
- Strengthen planning and routing guidance so future plans avoid overly coarse progress units.

## Non-goals

- Replacing top-level TODO acceptance with per-step top-level TODO completion.
- Adding a background daemon that mutates plans out of band.
- Letting child agents mark top-level task acceptance directly.
- Redesigning the whole task-state model beyond step guidance and synchronization.

## Design

### 1. Add Machine-Readable Step Progress Metadata

When a task still has unchecked steps, the plugin should expose:

- `current_step_label`
- `next_step_label`
- `next_step_text`
- `remaining_step_count`
- `step_sync_status`
- `step_sync_action`

Recommended `step_sync_status` values:

- `all_steps_complete`
- `step_in_progress`
- `needs_begin_step`
- `stale_current_step`

Recommended `step_sync_action` values:

- `none`
- `continue_current_step`
- `begin_next_step`
- `repair_current_step`

This metadata should be attached to `orchestrator_next_action` so the parent does not need to re-derive it from markdown parsing logic.

### 2. Auto-Seed The First Step On Task Start

`orchestrator_begin_task` should no longer leave `Current Step` at `none` when the task has unchecked steps. After the task enters execution:

- find the first unchecked step in the task block
- set both runtime `activeStepLabel` and plan `Current Step` to that step
- return the same step-progress metadata described above

This ensures a newly started task immediately shows visible progress context instead of waiting for a later ad hoc `begin_step` call.

### 3. Auto-Advance The Step Pointer On Step Completion

`orchestrator_complete_step` should:

- mark the completed step checked
- find the next unchecked step, if any
- update runtime `activeStepLabel` and plan `Current Step` to that next step
- otherwise clear `Current Step` to `none`
- return whether it auto-advanced and what the next step is

This keeps step progress moving forward immediately after each completion instead of leaving the parent to remember a follow-up sync call.

### 4. Detect Step-Sync Drift In Next Action And Watchdog Paths

If a task is running implementation or review work and still has unchecked steps, but the current-step pointer is missing, stale, or points at an already checked step, the plugin should expose that drift explicitly.

`orchestrator_next_action` should still return the task-level action, but it should also say whether the parent must first repair step synchronization.

`orchestrator_watchdog_tick` should prefer a step-sync-specific suggested action when progress drift is detected. This gives the parent a deterministic recovery path instead of silently continuing with misleading task state.

### 5. Planning Guidance Should Favor More Visible Progress

The plugin cannot completely fix coarse progress if plans themselves are too chunky. The bundled planning guidance should therefore say:

- top-level tasks should be small enough to produce visible progress increments during normal execution
- if a task would likely hide progress for a long stretch, split it into more top-level tasks instead of relying on one oversized task with many hidden steps

This is a guidance-layer assist, not a substitute for the MCP changes above.

## Success Criteria

- `orchestrator_begin_task` immediately seeds the first unchecked step.
- `orchestrator_complete_step` auto-advances to the next unchecked step when available.
- `orchestrator_next_action` returns machine-readable step guidance and drift status.
- `orchestrator_watchdog_tick` can surface step-sync drift explicitly.
- Tests cover step seeding, step auto-advance, and next-action step guidance.
