# MCP Tool Contract

## Purpose

The orchestrator MCP server exposes control-plane operations. It does not replace Codex native subagent execution.

## Implementation Note

Phase 1 uses a zero-third-party stdio MCP server implemented directly on Node.js with `--experimental-strip-types`. The server speaks a minimal JSON-RPC subset needed for initialization, tool listing, and tool calling.

## Tool Set

- `orchestrator_resolve_category`
- `orchestrator_read_plan_state`
- `orchestrator_begin_task`
- `orchestrator_begin_step`
- `orchestrator_complete_step`
- `orchestrator_record_subagent_run`
- `orchestrator_record_review`
- `orchestrator_accept_task`
- `orchestrator_check_doc_drift`
- `orchestrator_watchdog_tick`

## Enforcement

- Tools fail closed when required state is missing.
- Task acceptance is rejected if steps are unchecked or reviews are incomplete.
- Review recording rejects implementer-as-reviewer reuse.
- Plan synchronization always writes to markdown instead of relying on chat state.

