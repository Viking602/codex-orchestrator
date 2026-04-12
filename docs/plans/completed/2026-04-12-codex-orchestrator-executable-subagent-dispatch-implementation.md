# Codex Orchestrator Executable Subagent Dispatch Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: when `orchestrator_next_action` returns child-launch instructions, treat them as executable tool-routing data. Do not replace a returned `spawn_agent` or `send_input` contract with parent-local execution.

**Goal:** Make subagent delegation visible and enforceable by adding executable child-launch fields to runtime payloads and syncing workflow contracts around them.

**Architecture:** MCP remains the control plane. Codex native subagents remain the execution plane. This change closes the last contract gap between “delegation is recommended” and “the parent actually launches or resumes a child.”

**Tech Stack:** Rust CLI, Rust contract tests, markdown workflow docs.

---

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| E1. Define the executable subagent-dispatch contract | None | Anchors the payload and workflow change |
| E2. Add literal child-launch fields to runtime payloads | E1 | Runtime behavior depends on the contract |
| E3. Sync tests and workflow docs to the executable dispatch contract | E2 | Validation and docs depend on the final payload |
| E4. Validate and refresh routing records | E3 | Closeout depends on passing tests and synced docs |

## Execution Status

- Current wave: Wave Complete
- Active task: none
- Blockers: None
- Last review result: quality pass

## TODO List

- [x] E1. Define The Executable Subagent-Dispatch Contract
- [x] E2. Add Literal Child-Launch Fields To Runtime Payloads
- [x] E3. Sync Tests And Workflow Docs To The Executable Dispatch Contract
- [x] E4. Validate And Refresh Routing Records

## Completed Work

- Added `subagent_tool_action`, `subagent_agent_type`, and `subagent_dispatch_message` to top-level and parallel dispatch payloads
- Derived deterministic `spawn_agent` vs `send_input` routing from task-session policy
- Added runtime assertions for spawn, resume, and parallel child-launch behavior
- Updated skill, repo guidance, install guidance, and architecture docs so returned child-launch fields require literal tool use
- Verified `cargo test --manifest-path plugins/codex-orchestrator/rust-cli/Cargo.toml` passes

## Final Acceptance

- [x] Spawn paths return `subagent_tool_action = spawn_agent`
- [x] Resume paths return `subagent_tool_action = send_input`
- [x] Parallel dispatch entries expose executable child-launch fields
- [x] Workflow docs require literal `spawn_agent` / `send_input`
- [x] Rust validation passes
