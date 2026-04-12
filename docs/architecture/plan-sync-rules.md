# Plan Synchronization Rules

## Purpose

The implementation plan must stay aligned with execution in real time so the run can resume from files alone.

## Machine-Updatable Fields

Each task block must include these mutable fields:

- `Task Status`
- `Current Step`
- `Spec Review Status`
- `Quality Review Status`
- `Assigned Agent`

The execution status section must include:

- current wave
- active task
- blockers
- last review result

## Synchronization Rules

1. When a task starts, update execution status and task metadata.
2. When a task starts and unchecked steps remain, seed `Current Step` to the first unchecked step immediately instead of leaving it at `none`.
3. When a step starts explicitly, update the task current step.
4. When a step completes, check the matching checkbox and immediately advance `Current Step` to the next unchecked step when one exists.
4. When a review completes, update task review fields and top-level execution status.
5. When a terminal review pass closes a task, accept it in the same control-plane pass, check the top-level TODO checkbox immediately, and advance `Active task`.
6. When a task is accepted outside the review path, check the top-level task checkbox in the TODO list immediately.
7. When a blocker appears, update both execution status and task state.
8. When a write lease is acquired or released, update task status context so the parent can resume from files plus runtime state.
9. When every top-level TODO item is checked and any `Final Acceptance` items are also checked, move the plan from `docs/plans/active/` to `docs/plans/completed/` in the same pass.

## Step Sync Rule

The parent should not treat a whole task as one opaque execution segment when bounded steps exist.

- `Current Step` should name the actionable unchecked step, not remain at `none` during active work.
- If unchecked steps remain but `Current Step` is missing or stale, that is synchronization drift and should be repaired before pretending progress is current.
- Plans should prefer smaller visible progress units so top-level TODO movement is not hidden behind one oversized task.

## Truth Rule

Runtime state may say a task is ready for acceptance, but the task is not truly complete until the plan file reflects that state.
If the plan is complete, the durable truth location is the archived file under `docs/plans/completed/`.
