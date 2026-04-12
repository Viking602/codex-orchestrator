# Codex Orchestrator Mid-Run Control-Plane Checkpoints Design

## Goal

Make `codex-orchestrator` drive MCP control-plane writes during execution instead of letting the parent defer most `begin_task`, `begin_step`, `complete_step`, and review-adjacent state updates until the end of a long child run.

## Problem

The current workflow still allows this failure mode:

- `orchestrator_next_action` returns a child-execution action immediately for an untouched or step-desynchronized task.
- The parent launches or resumes a child and lets it run a whole task or several steps.
- The parent then replays many MCP writes in one late batch after the code work is already done.

That behavior is non-durable and non-visible:

- `Current Step` is stale during real execution.
- Native todo mirroring lags behind actual progress.
- The parent can appear idle from the control-plane perspective until the end.

## Root Cause

There are two contract gaps:

1. `orchestrator_next_action` exposes `step_sync_status` and `step_sync_action`, but they are advisory side fields rather than top-level actions. The parent can skip them and dispatch child work anyway.
2. The dedicated child-session contract is task-owned, but not step-bounded. Bundled implementer prompts still allow a child to finish a broad coding scope before returning, which hides intermediate MCP writes.

## Design

### 1. Promote task-start and step-sync repair into blocking pre-dispatch actions

`orchestrator_next_action` should expose parent-owned control-plane writes as blocking pre-dispatch actions before child execution when state is not ready:

- `orchestrator_begin_task` when a dependency-ready task has no runtime task state yet
- `orchestrator_begin_step` when a running task has no live `Current Step`
- `orchestrator_begin_step` with repair intent when runtime and plan step pointers drift

These actions should appear in a machine-readable `blocking_control_plane_actions` payload so the parent performs them before child dispatch or resume.

### 2. Make child execution explicitly single-step scoped

For child-owned implementation and research work, the payload should say the child owns only the current step on that resume:

- add machine-readable child step scope metadata such as `child_execution_mode` to `orchestrator_next_action`
- add the same metadata plus per-entry blocking control-plane actions to `parallel_dispatches`
- keep task-owned session routing, but narrow each resume to the current step instead of the whole task

The intended loop becomes:

1. Parent gets the current `next_action`
2. Parent performs any required `blocking_control_plane_actions` first
3. Parent dispatches or resumes the dedicated child for only the current step
4. Child returns after that bounded step or a blocker
5. Parent records `complete_step` or blocker state immediately
6. Parent refreshes `next_action` and repeats

### 3. Tighten child-agent instructions

Bundled child executors should be told:

- do not consume an entire top-level task in one uninterrupted run unless the task has only one step
- treat the current step as the execution boundary for the current resume
- return after the current step is finished, blocked, or proven invalid

The parent still owns task acceptance and review closure. Child agents must not mark top-level tasks complete.

## Implementation Areas

- `plugins/codex-orchestrator/rust-cli/src/tools.rs`
- `plugins/codex-orchestrator/rust-cli/tests/runtime_contracts.rs`
- `plugins/codex-orchestrator/rust-cli/tests/repo_contracts.rs`
- `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
- `plugins/codex-orchestrator/codex/agents/backend-developer.toml`
- `plugins/codex-orchestrator/codex/agents/search-specialist.toml`
- `plugins/codex-orchestrator/codex/agents/harness-planner.toml`
- `docs/architecture/mcp-tool-contract.md`
- `docs/architecture/agent-contracts.md`
- `docs/architecture/plan-sync-rules.md`
- `AGENTS.md`
- `install.md`

## Acceptance Criteria

- `orchestrator_next_action` returns `blocking_control_plane_actions` containing `orchestrator_begin_task` before first child dispatch for an untouched task.
- `orchestrator_next_action` returns `blocking_control_plane_actions` containing `orchestrator_begin_step` before continuing a step-desynchronized running task.
- Child dispatch payloads explicitly identify single-current-step execution scope.
- Bundled child-agent instructions require step-bounded returns instead of whole-task batching.
- Regression coverage proves the parent now sees mid-run control-plane checkpoints as first-class actions.
