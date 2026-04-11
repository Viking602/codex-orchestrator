# MCP Tool Contract

## Purpose

The orchestrator MCP server exposes control-plane operations. It does not replace Codex native subagent execution.

## Implementation Note

Phase 1 uses a zero-third-party stdio MCP server implemented directly on Node.js with `--experimental-strip-types`. The server speaks a minimal JSON-RPC subset needed for initialization, tool listing, and tool calling.

## Tool Set

- `orchestrator_resolve_category`
- `orchestrator_read_plan_state`
- `orchestrator_begin_task`
- `orchestrator_acquire_write_lease`
- `orchestrator_release_write_lease`
- `orchestrator_question_gate`
- `orchestrator_assess_subagent_completion`
- `orchestrator_begin_step`
- `orchestrator_complete_step`
- `orchestrator_record_subagent_run`
- `orchestrator_record_review`
- `orchestrator_accept_task`
- `orchestrator_check_doc_drift`
- `orchestrator_watchdog_tick`
- `orchestrator_next_action`
- `orchestrator_completion_guard`

## Enforcement

- Tools fail closed when required state is missing.
- Task acceptance is rejected if steps are unchecked or reviews are incomplete.
- Review recording rejects implementer-as-reviewer reuse.
- Plan synchronization always writes to markdown instead of relying on chat state.
- Lease-required categories fail closed without an active write lease.
- `resolve_category` returns both workflow category and the default delegation bias for that category.
- `next_action` derives a deterministic parent move from plan plus runtime state and now exposes whether child intervention is required.
- `question_gate` rejects optional expansion questions by default.
- `assess_subagent_completion` prevents child self-report from being treated as task completion.
- `completion_guard` fails closed before parent completion.

## Key Payload Fields

### `orchestrator_resolve_category`

- `delegation_preference`
- `requires_subagent_default`

### `orchestrator_next_action`

- `requires_subagent`
- `dispatch_role`
- `intervention_reason`
- `dispatch_mode`
