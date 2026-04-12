# Codex Orchestrator Task-Owned Subagent Sessions Design

## Summary

Strengthen orchestration so each top-level task gets its own dedicated child session instead of letting the parent absorb task-local execution context. The runtime should emit explicit session-routing metadata telling the parent when to spawn a fresh task child, when to resume the same task child, and when a separate reviewer child is required.

## Problem

The current control plane already prefers child execution and can batch dependency-ready tasks, but it still leaves too much session policy implicit:

- `orchestrator_next_action` tells the parent what role to dispatch, but not whether the task should own a fresh child session or resume an existing one.
- `task_state.agent_id` is overwritten by later task runs, so the original implementation child is not durably preserved once review begins.
- Parents can still keep too much top-level task reasoning local because there is no hard contract that one top-level task maps to one dedicated execution child.

That wastes parent context and makes repair paths weaker than they should be.

## Goals

- Make each top-level task own a dedicated execution child session.
- Keep the parent limited to control-plane work: plan sync, lease management, review gating, and acceptance.
- Preserve the original implementer child so repair can return to the same task owner after review failure.
- Make parallel task dispatch return one dedicated child-session instruction per top-level task.

## Non-Goals

- Do not create a new child for every single step inside a task.
- Do not merge reviewer identity into the implementer child; review remains a separate guardrail.
- Do not add a new planning DSL beyond the existing plan/task metadata.

## Design

### Runtime Ownership Model

Extend task runtime state to preserve task-owned child identities separately:

- `agent_id`: current active child associated with the task stage
- `implementation_agent_id`: dedicated implementer child for the task
- `review_agent_id`: current in-flight reviewer child for the task

This keeps implementation ownership durable even after review activity updates the active child.

### Parent-Facing Session Policy

Extend `orchestrator_next_action` to expose explicit session-routing metadata:

- `task_session_mode`
- `task_session_key`
- `continue_agent_id`

Expected modes:

- `parent-local`
- `spawn-dedicated-task-subagent`
- `resume-dedicated-task-subagent`
- `spawn-dedicated-reviewer-subagent`
- `resume-dedicated-reviewer-subagent`

`task_session_key` should be deterministic per plan/task/workstream:

- implementer lane: `task::<plan_id>::<task_id>::implementer`
- reviewer lane: `task::<plan_id>::<task_id>::review`

### Parallel Batch Contract

Each `parallel_dispatches` entry should carry the same session metadata so the parent can launch one child per top-level task instead of serializing or sharing a child across tasks.

### Ownership Rules

- First implementation dispatch for a top-level task spawns a dedicated implementer child.
- Later implementation continuation or repair resumes that same implementer child when available.
- First review dispatch spawns a dedicated reviewer child.
- In-progress review resumes the same reviewer child.
- Reviewer identity must never equal `implementation_agent_id`.
- Parent must not reuse one child session across different top-level tasks.

## Files

- `plugins/codex-orchestrator/rust-cli/src/types.rs`
- `plugins/codex-orchestrator/rust-cli/src/runtime_store.rs`
- `plugins/codex-orchestrator/rust-cli/src/tools.rs`
- `plugins/codex-orchestrator/rust-cli/tests/runtime_contracts.rs`
- `plugins/codex-orchestrator/rust-cli/tests/repo_contracts.rs`
- `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
- `docs/architecture/runtime-state-schema.md`
- `docs/architecture/agent-contracts.md`
- `docs/architecture/mcp-tool-contract.md`
- `AGENTS.md`
- `install.md`

## Acceptance Criteria

- `orchestrator_next_action` explicitly tells the parent whether to spawn or resume a dedicated child session for a top-level task.
- Runtime state preserves the implementer child separately from the reviewer child.
- Parallel dispatch payloads expose one dedicated child-session instruction per top-level task.
- Review rejection paths can resume the original implementer child instead of losing task ownership.
