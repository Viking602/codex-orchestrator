# Runtime State Schema

## Purpose

Runtime state records short-lived orchestration metadata that supports recovery and control flow. It does not replace the implementation plan as the source of truth for completion.

## Authority Boundaries

### Plan File Is Authoritative For

- task step checkboxes
- top-level task completion checklist
- final acceptance checklist
- visible execution status for human review

### Runtime State Is Authoritative For

- current `agent_id`
- current execution stage
- retry counters
- blocker metadata
- active write lease
- last known review results before plan synchronization completes

## Core Entities

### `plan_state`

- `plan_id`
- `plan_path`
- `spec_path`
- `current_wave`
- `active_task_id`
- `last_review_result`
- `updated_at`

### `task_state`

- `plan_id`
- `task_id`
- `category_id`
- `status`
- `active_step_label`
- `assigned_role`
- `agent_id`
- `write_lease_id`
- `spec_review_status`
- `quality_review_status`
- `retry_count`
- `blocker_type`
- `blocker_message`
- `updated_at`

### `task_run`

- `plan_id`
- `task_id`
- `role`
- `agent_id`
- `status`
- `summary`
- `started_at`

### `verification_evidence`

- `plan_id`
- `task_id`
- `kind`
- `command`
- `result`
- `summary`
- `created_at`

### `write_lease`

- `lease_id`
- `plan_id`
- `task_id`
- `holder_agent_id`
- `scope_json`
- `status`
- `created_at`
- `released_at`

## Task Statuses

- `planned`
- `ready`
- `running_impl`
- `impl_done`
- `running_spec_review`
- `spec_failed`
- `running_quality_review`
- `quality_failed`
- `accepted`
- `blocked`
- `cancelled`

## Invariants

- Only the parent orchestration layer may advance a task to `accepted`.
- A task may not enter `accepted` unless all step checkboxes are checked in the plan file.
- A task may not enter `accepted` unless both reviews are `pass`.
- A reviewer must never reuse the implementer `agent_id`.
- A lease-required implementation task may not enter `running_impl` without an active lease.
- Runtime state must be recoverable without relying on chat history.
