# Codex Orchestrator Plugin Phase 2 Design

## Context

Phase 1 established the plugin shell, file-backed planning artifacts, runtime SQLite state, category routing, real-time plan synchronization, and review gates. Phase 2 extends the orchestrator so the parent agent can rely on stronger execution control instead of free-form judgment during long-running work.

The phase 2 focus is not broad feature expansion. It is control-plane hardening in three specific areas:

1. write lease enforcement for write-capable categories
2. stronger watchdog and stalled-task recovery hints
3. parent-agent protocol takeover via a deterministic `next_action` interface

## Goals

- Add a write lease protocol so `lease-required` categories cannot proceed without an explicit lease.
- Add runtime storage for leases and expose lease MCP tools.
- Upgrade the watchdog from a raw stale-task list to actionable recovery recommendations.
- Add a parent-facing `next_action` tool that derives the next deterministic orchestration step from plan plus runtime state.
- Keep the active implementation plan as the single execution truth while using the new tools to push state transitions.

## Non-goals

- Rebuild full OpenAgent continuation hooks.
- Add background daemons in phase 2.
- Add networked coordination or multi-host state.
- Replace Codex native subagent execution with a plugin-side runner.

## Design Principles

### 1. Write Must Be Lease-Gated

If a category declares `write_policy = lease-required`, the parent agent must acquire a write lease before the task can enter implementation. Lease state must record:

- lease id
- plan id
- task id
- scope
- holder agent id
- status
- timestamps

### 2. Watchdog Must Recommend Actions

`watchdog_tick` should stop being an observational tool only. It should derive recommended parent actions from task status and age, for example:

- `acquire_write_lease`
- `continue_same_agent`
- `return_to_implementer`
- `re-run_review`
- `mark_blocked`

### 3. Parent Protocol Must Be Machine-Readable

The parent should not infer next actions from prose. The MCP server should provide a `next_action` result containing:

- selected task id
- recommended action
- required role
- whether write lease is required
- whether review is next
- why this is the next valid move

### 4. Plan Remains Authoritative

The new tools may derive state, but task completion still depends on:

- checked plan steps
- passed review gates
- top-level todo completion
- final acceptance checklist

## New Runtime Entities

### `write_lease`

- `lease_id`
- `plan_id`
- `task_id`
- `holder_agent_id`
- `scope_json`
- `status`
- `created_at`
- `released_at`

### Parent Action Result

- `task_id`
- `action`
- `required_role`
- `requires_write_lease`
- `reason`
- `blocking_issue`

## New MCP Tools

- `orchestrator_acquire_write_lease`
- `orchestrator_release_write_lease`
- `orchestrator_next_action`

## Success Criteria

- The parent can no longer start `lease-required` work without a lease.
- Runtime state records lease ownership and release.
- Watchdog emits action-oriented recovery hints.
- `next_action` can deterministically choose the next valid move from current plan and runtime state.
- All new behavior is covered by tests and reflected in docs.

