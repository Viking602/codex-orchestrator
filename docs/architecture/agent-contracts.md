# Agent Contracts

## Parent Agent Contract

The parent orchestration layer owns:

- category resolution
- delegation decision interpretation
- native Codex `update_plan` mirroring when that tool is available
- write lease acquisition and release
- role dispatch
- runtime state transitions
- plan file synchronization
- review gate progression
- final task acceptance

## Implementer Contract

The implementer owns:

- bounded execution for a single top-level task
- creation of atomic execution substeps
- incremental progress reporting
- local verification for owned changes

The implementer must not:

- mark the top-level task complete
- self-approve review gates
- widen scope beyond assigned files and acceptance criteria

## Reviewer Contract

The reviewer owns:

- spec compliance review or quality review
- evidence-based findings
- explicit pass/fail output

The reviewer must not:

- reuse the implementer identity for the same task
- modify production code in the same pass
- mark plan tasks complete

## Incremental Todo Rule

- Child execution substeps must be reported one-by-one.
- Parent plan synchronization must happen after each completed step.
- End-of-task batch completion is invalid and should fail review.
- When a terminal review pass closes a task, the parent must accept that top-level task in the same control-plane pass instead of deferring checkbox movement.
- When native `update_plan` is available, the parent should mirror the active implementation plan into that surface instead of creating a separate prose todo list.

## Native Todo Mirror Rule

- The implementation plan file is the authoritative ledger.
- Codex native todo is a projection of that ledger, not an independent plan.
- Parents should refresh the native todo mirror after task start, step completion, task acceptance, and final-acceptance changes.
- `orchestrator_export_codex_todo` is the canonical projection surface for native todo mirroring.

## Parent Next-Action Rule

The parent should prefer `orchestrator_next_action` over informal reasoning when deciding:

- whether to acquire a write lease
- whether to dispatch a task
- whether to continue the same implementer
- whether to re-run review
- whether a task is ready for acceptance

The parent should also treat the following `orchestrator_next_action` fields as authoritative:

- `requires_subagent`
- `dispatch_role`
- `intervention_reason`
- `dispatch_mode`
- `parallel_task_ids`
- `parallel_dispatches`
- `task_session_mode`
- `task_session_key`
- `continue_agent_id`
- `subagent_tool_action`
- `subagent_agent_type`
- `subagent_dispatch_message`
- `blocking_control_plane_actions`
- `child_execution_mode`
- `child_execution_label`
- `child_execution_text`

Interpretation rule:

- `requires_subagent = true` means the parent should dispatch or continue a child agent instead of performing the task work locally.
- `requires_subagent = false` means the current step is parent-owned control-plane work.
- `parallel_task_ids` and `parallel_dispatches` mean the parent should launch the whole returned cohort in one round instead of serializing on the anchor task.
- If the top-level action is `acquire_parallel_write_leases`, the parent acquires one lease per returned child dispatch scope before launching the batch.
- `task_session_mode` tells the parent whether to spawn or resume a dedicated child for that top-level task.
- `task_session_key` is the deterministic ownership handle for that task lane; different top-level tasks must not share the same child session.
- `continue_agent_id` identifies the child that should be resumed when the task already has a durable owner.
- `subagent_tool_action = spawn_agent` means the parent should call `spawn_agent` immediately for that task lane.
- `subagent_tool_action = send_input` means the parent should resume the existing child with `send_input` instead of reabsorbing the task locally.
- `subagent_agent_type` is the concrete child role to pass into `spawn_agent`.
- `subagent_dispatch_message` is the bounded handoff brief the parent should pass into `spawn_agent` or `send_input` without rewriting the task as parent-local execution.
- `blocking_control_plane_actions` are blocking control-plane writes that must be performed before the child launches or resumes.
- `child_execution_mode = current-step` means the child should execute only the current step for that task resume and then return control to the parent.

## Task-Owned Session Rule

- One top-level task should map to one dedicated implementer child session.
- The parent should keep control-plane state and avoid holding task-local execution context once a dedicated child exists.
- The parent should not let the child skip task-start or step-sync checkpoints that the runtime already identified as blocking control-plane writes.
- Reviewer children are separate guardrail lanes and must not erase the task's dedicated implementer ownership.
- Repair and continuation should resume the existing task-owned implementer child whenever the runtime provides one.

## Parent Completion Accountability

The parent is responsible for the child outcome. Child status alone does not close a task.

Required parent checks:

- run `orchestrator_assess_subagent_completion`
- route into review or repair when needed
- run `orchestrator_completion_guard` before declaring work complete
- never accept optional expansion questions by default
