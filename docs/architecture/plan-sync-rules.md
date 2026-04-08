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
2. When a step starts, update the task current step.
3. When a step completes, check the matching checkbox.
4. When a review completes, update task review fields and top-level execution status.
5. When a task is accepted, check the top-level task checkbox in the TODO list.
6. When a blocker appears, update both execution status and task state.

## Truth Rule

Runtime state may say a task is ready for acceptance, but the task is not truly complete until the plan file reflects that state.

