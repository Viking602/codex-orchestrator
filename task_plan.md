# Task Plan

## Goal

Make `codex-orchestrator` emit executable child-launch instructions so parent agents actually call native subagent tools instead of keeping delegated task work local.

## Phases

| Phase | Status | Notes |
|---|---|---|
| 1. Define the executable dispatch contract | completed | Added a design spec and completed implementation record for literal child-launch routing |
| 2. Add runtime child-launch fields | completed | `orchestrator_next_action` and `parallel_dispatches` now emit tool action, agent type, and child brief |
| 3. Sync workflow contracts | completed | Skill, repo guidance, install guidance, and architecture docs now require literal `spawn_agent` / `send_input` execution |
| 4. Validate and close out | completed | Rust contract tests passed and routing docs now point at the new completed plan |

## Current Decisions

- Keep MCP as the control plane and Codex native subagents as the execution plane.
- Encode child delegation as literal `spawn_agent` / `send_input` instructions instead of only abstract routing metadata.
- Pass a bounded handoff brief through `subagent_dispatch_message` so the parent does not have to improvise the child prompt.

## Open Questions

- Whether a future version should also emit an explicit wait policy for each child lane.
- Whether reviewer lanes should eventually get a dedicated reviewer brief format distinct from the generic bounded dispatch message.

## Completed This Session

- Wrote the executable subagent-dispatch design spec
- Added `subagent_tool_action`, `subagent_agent_type`, and `subagent_dispatch_message` to runtime payloads
- Added runtime assertions for spawn, resume, and parallel child-launch cases
- Updated workflow docs to require literal `spawn_agent` / `send_input` calls when the runtime returns those fields
- Verified `cargo test --manifest-path plugins/codex-orchestrator/rust-cli/Cargo.toml` passes
