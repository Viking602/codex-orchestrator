# MCP Tool Contract

## Purpose

The orchestrator MCP server exposes control-plane operations. It does not replace Codex native subagent execution.

## Implementation Note

The current runtime uses a standalone Rust stdio MCP server that speaks the minimal JSON-RPC subset needed for initialization, tool listing, and tool calling. Source-checkout development launches it through `cargo run`, while installed runtime should point at the staged native binary.

## Tool Set

- `orchestrator_resolve_category`
- `orchestrator_read_plan_state`
- `orchestrator_export_codex_todo`
- `orchestrator_begin_task`
- `orchestrator_acquire_write_lease`
- `orchestrator_release_write_lease`
- `orchestrator_question_gate`
- `orchestrator_assess_subagent_completion`
- `orchestrator_begin_step`
- `orchestrator_complete_step`
- `orchestrator_record_subagent_run`
- `orchestrator_record_review`
- `orchestrator_accept_task`
- `orchestrator_check_doc_drift`
- `orchestrator_watchdog_tick`
- `orchestrator_next_action`
- `orchestrator_completion_guard`

## Enforcement

- Tools fail closed when required state is missing.
- Task acceptance is rejected if steps are unchecked or reviews are incomplete.
- Review recording rejects implementer-as-reviewer reuse.
- Plan synchronization always writes to markdown instead of relying on chat state.
- Lease-required categories fail closed without an active write lease.
- `resolve_category` returns both workflow category and the default delegation bias for that category.
- `next_action` derives a deterministic parent move from plan plus runtime state and now exposes whether child intervention is required.
- `begin_task` seeds the first unchecked step when active work starts.
- `complete_step` auto-advances the current-step pointer to the next unchecked step when available.
- `next_action` and `watchdog_tick` expose step-sync guidance so parents can repair drift instead of batching late step updates.
- `export_codex_todo` projects file-backed plan state into a mirror-ready Codex native todo snapshot instead of creating a second source of truth.
- `question_gate` rejects optional expansion questions and redundant direction-confirmation questions by default.
- `record_review` immediately accepts a terminal-ready task when the quality-pass write closes all gates and steps.
- `assess_subagent_completion` prevents child self-report from being treated as task completion.
- `completion_guard` fails closed before parent completion.

## Key Payload Fields

### `orchestrator_resolve_category`

- `delegation_preference`
- `requires_subagent_default`

### `orchestrator_next_action`

- `requires_subagent`
- `dispatch_role`
- `intervention_reason`
- `dispatch_mode`
- `current_step_label`
- `current_step_text`
- `next_step_label`
- `next_step_text`
- `remaining_step_count`
- `step_sync_status`
- `step_sync_action`

### `orchestrator_begin_task`

- seeds the first unchecked step into `Current Step` when the task enters active execution
- returns the same step-guidance fields used by `next_action`

### `orchestrator_complete_step`

- `auto_advanced`
- returns the updated step-guidance fields after the checked step is recorded

### `orchestrator_record_review`

- `accepted`
- `top_level_todo_checked`
- `next_active_task_id`

### `orchestrator_export_codex_todo`

- `items`
- `active_task_id`
- `active_task_title`
- `current_step_label`
- `current_step_text`
- `remaining_step_count`
- `step_sync_status`
- `step_sync_action`
- `open_acceptance_items`

Use this payload as the parent-facing bridge into Codex native `update_plan`. The implementation plan remains authoritative; the exported todo is only a mirror.
