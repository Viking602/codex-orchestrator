# Category Contract

## Purpose

Categories define workflow semantics for orchestration. A category is not just a model alias. It encodes how work is routed, constrained, reviewed, and considered complete.

## Required Fields

Each category definition must include:

- `id`
- `intent`
- `allowed_roles`
- `preferred_role`
- `write_policy`
- `requires_plan`
- `requires_spec_review`
- `requires_quality_review`
- `parallelism`
- `reuse_policy`
- `completion_contract`

## Canonical Categories For Phase 1

### `plan`

- Intent: design, planning, decomposition, contract definition
- Write policy: docs-only
- Requires plan: no
- Spec review: yes
- Quality review: yes
- Parallelism: single active task
- Reuse policy: same task only

### `research`

- Intent: investigation, repo understanding, evidence gathering
- Write policy: read-only
- Requires plan: no
- Spec review: no
- Quality review: no
- Parallelism: multiple allowed when write scopes do not exist
- Reuse policy: no reuse by default

### `backend-impl`

- Intent: code implementation for backend or orchestration runtime
- Write policy: write lease required
- Requires plan: yes
- Spec review: yes
- Quality review: yes
- Parallelism: blocked by write-scope conflict
- Reuse policy: same task, same role, same write scope only

### `review`

- Intent: spec compliance review, code quality review, acceptance judgment
- Write policy: read-only unless explicitly converted to repair pass
- Requires plan: yes
- Parallelism: multiple allowed
- Reuse policy: never reuse implementer as reviewer

## Enforcement Rules

- If a task has no category contract, dispatch is rejected.
- If a category disallows write but changed files are recorded, the run fails review.
- If a category requires reviews and no review results exist, the task cannot be accepted.
- If a category disallows reuse, a prior session cannot be continued for that task.

