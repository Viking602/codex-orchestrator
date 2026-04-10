# Write Lease Protocol

## Purpose

Write leases prevent lease-required categories from entering implementation without explicit ownership over a write scope.

## Required Inputs

- `plan_id`
- `task_id`
- `holder_agent_id`
- `scope`

## Lease Fields

- `lease_id`
- `plan_id`
- `task_id`
- `holder_agent_id`
- `scope_json`
- `status`
- `created_at`
- `released_at`

## Rules

1. Only one active lease may exist per task.
2. `lease-required` categories cannot start implementation until an active lease exists.
3. Releasing a lease clears the active lease pointer from task state.
4. Lease state is runtime support only. Acceptance still depends on the plan file and review gates.

## Parent Expectations

- Acquire lease before dispatching an implementation run for a lease-required task.
- Release lease when the parent ends or abandons the implementation pass.
- Use the lease scope as the write boundary for the current run.

