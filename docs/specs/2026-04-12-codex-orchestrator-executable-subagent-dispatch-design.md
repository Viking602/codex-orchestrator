# Codex Orchestrator Executable Subagent Dispatch Design

## Problem

`codex-orchestrator` already knows when work belongs on a child lane, but it still exposes that decision as soft routing metadata. That makes it too easy for the parent to keep task work local even when `requires_subagent`, `dispatch_role`, and `task_session_mode` say otherwise.

## Goal

Turn child dispatch into an executable runtime contract so delegated work visibly flows through Codex native subagent tools.

## Decision

Add three literal child-launch fields to `orchestrator_next_action` and each `parallel_dispatches` entry:

- `subagent_tool_action`
- `subagent_agent_type`
- `subagent_dispatch_message`

Interpretation:

- `subagent_tool_action = spawn_agent` means the parent must call `spawn_agent` with `agent_type = subagent_agent_type`.
- `subagent_tool_action = send_input` means the parent must call `send_input` against `continue_agent_id`.
- `subagent_dispatch_message` is the bounded handoff brief the parent should pass through directly instead of rewriting it into parent-local execution.

## Scope

- `plugins/codex-orchestrator/rust-cli/src/tools.rs`
- `plugins/codex-orchestrator/rust-cli/tests/runtime_contracts.rs`
- `plugins/codex-orchestrator/rust-cli/tests/repo_contracts.rs`
- workflow and architecture docs

## Acceptance Criteria

- Spawn paths emit `subagent_tool_action = spawn_agent`
- Resume paths emit `subagent_tool_action = send_input`
- Parallel dispatch entries emit their own executable child-launch fields
- Workflow docs explicitly require literal `spawn_agent` / `send_input` use
- Rust validation passes
