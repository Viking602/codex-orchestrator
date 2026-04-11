# Codex Orchestrator Native Codex Todo Mirroring Design

## Context

The plugin already maintains file-backed implementation plans and step synchronization, but Codex's native todo UI is still outside the plugin contract. That leaves a gap: parent agents can read the plan and still choose to present progress through an ad hoc chat todo or a separate summary instead of keeping Codex's built-in `update_plan` surface aligned with the active plan.

The desired behavior is not a second source of truth. The implementation plan markdown should remain authoritative, and Codex's native todo UI should become a mirror of that plan. The plugin therefore needs to expose a mirror-ready projection of the active plan, while bundled workflow guidance tells the parent to update the native Codex todo instead of inventing a separate list.

## Goals

- Keep the implementation plan as the authoritative progress ledger.
- Expose a mirror-ready top-level todo snapshot that the parent can pass directly to Codex native `update_plan`.
- Represent the active task as the single `in_progress` native todo item and include current-step detail when available.
- Represent final-acceptance work in the native todo mirror when task execution is finished but acceptance is still open.
- Explicitly instruct parent agents not to create a separate prose todo when native `update_plan` is available.

## Non-goals

- Letting the plugin call native `update_plan` directly. That remains parent-owned.
- Replacing file-backed plan truth with Codex native todo state.
- Adding a second runtime ledger for todo mirroring.
- Reworking task acceptance semantics.

## Design

### 1. Add A Mirror-Ready MCP Tool

Add a new MCP tool:

- `orchestrator_export_codex_todo`

Inputs:

- `planPath`

Outputs:

- `plan_id`
- `plan_path`
- `items`
- `active_task_id`
- `active_task_title`
- `current_step_label`
- `current_step_text`
- `step_sync_status`
- `open_acceptance_items`

`items` should be shaped for direct translation into native `update_plan` entries:

- `step`
- `status`

Allowed statuses:

- `completed`
- `in_progress`
- `pending`

### 2. Mirror Top-Level Plan Tasks Deterministically

The mirror should derive native todo state from the plan, not from chat history.

Rules:

- checked top-level tasks become `completed`
- the active incomplete task becomes `in_progress`
- later incomplete tasks become `pending`
- if no active task is set, use the first incomplete top-level task as `in_progress`

The active item's `step` text should include current-step detail when available, for example by appending the active step label and text.

### 3. Mirror Final Acceptance As The Terminal Native Todo

If all top-level tasks are complete but final acceptance still has open items, append a final native todo item:

- `Final acceptance`

That item should be:

- `in_progress` while acceptance items remain open
- omitted when final acceptance is fully complete

The item text may include a short summary of open acceptance items.

### 4. Parent Workflow Must Mirror Native Todo, Not Invent Another One

Bundled workflow guidance should explicitly state:

- when native `update_plan` is available, mirror the active implementation plan into that surface
- do not create a separate prose todo list in chat
- the native todo is a projection of the file-backed plan, not a separate plan
- after task start, step completion, task acceptance, and final acceptance changes, refresh the native todo mirror

### 5. Truth Hierarchy Must Stay Explicit

The truth hierarchy should remain:

1. implementation plan markdown
2. plugin runtime state
3. native Codex todo mirror

This avoids any ambiguity about where progress is actually stored.

## Success Criteria

- The plugin exposes a mirror-ready native-todo snapshot through MCP.
- The active incomplete task maps to the single `in_progress` native todo entry.
- Current-step detail appears in the mirrored active todo entry when available.
- Final-acceptance work appears in the mirror when it is the only remaining work.
- Workflow docs explicitly forbid maintaining a separate chat todo when native `update_plan` is available.
