# Codex Orchestrator Brainstorming Integration Design

## Context

`codex-orchestrator` already owns the repository execution model after task selection, but discovery-time routing is still vulnerable to global superpowers entry skills.

In particular:

- `using-superpowers` declares itself applicable when starting any conversation.
- `brainstorming` requires a design-first conversational loop before implementation.
- `codex-orchestrator` currently says it should be used before generic process skills, but it does not explicitly absorb the brainstorming-stage behaviors that users actually want from that stack.

That leaves a routing gap: global superpowers process skills can claim the entry point first, and `codex-orchestrator` only takes over later or not at all.

## Goals

- Absorb the useful repository-oriented `brainstorming` behaviors into the `codex-orchestrator` workflow itself.
- Make `codex-orchestrator` the entry workflow for repository tasks even when superpowers skills are installed globally.
- Preserve the good parts of brainstorming:
  - explore project context first
  - ask clarifying questions one at a time
  - propose 2-3 approaches with a recommendation
  - present a design and get approval before implementation planning
  - write and review the design spec before execution
- Keep implementation control in the existing `codex-orchestrator` control plane instead of reviving a separate superpowers-owned planning path.

## Non-goals

- Re-implementing every brainstorming-specific extra such as the visual companion.
- Adding a new MCP tool if prompt and routing surfaces are sufficient.
- Reintroducing superpowers-specific artifact locations such as `docs/superpowers/specs/`.
- Replacing the existing spec, plan, review, or todo-mirroring contracts.

## Design

### 1. Make Brainstorming Part Of The Orchestrator Entry Contract

The bundled `codex-orchestrator` skill should explicitly own the design-discovery loop for repository tasks.

That loop should require:

1. explore current project context first
2. ask clarifying questions one at a time when requirements are incomplete
3. propose 2-3 approaches with trade-offs and a recommendation
4. present the design in sections and get approval before writing the implementation plan
5. write the spec and perform a brief self-review before execution

This makes the plugin the direct replacement for repository-useful `brainstorming` behavior instead of only a post-brainstorm execution wrapper.

### 2. Block Superpowers From Owning The Entry Point

Repository-local and installer-managed global `AGENTS` guidance should state that:

- repository tasks must enter through `codex-orchestrator`, not `using-superpowers`
- `brainstorming` and other generic process skills are subordinate helpers only after `codex-orchestrator` takes control
- `codex-orchestrator` already subsumes the repository brainstorming stage, so separate `using-superpowers` or `brainstorming` entry invocation is not required

This is the durable fix because `using-superpowers` itself defers to stronger `AGENTS.md` guidance.

### 3. Strengthen Plugin Metadata Around Discovery And Design

Plugin metadata and skill descriptions should describe the workflow as:

- requirements clarification
- design exploration
- spec creation
- planning
- execution and review

That makes the plugin more likely to match the user’s actual request language before generic process skills do.

### 4. Keep The MCP Contract Stable

No new MCP tool is required in this pass.

The brainstorming integration is a workflow and routing change:

- the spec and plan still live in files
- existing review and acceptance gates still apply
- existing `orchestrator_*` tools still manage the control plane

The plugin should continue to use:

- `orchestrator_resolve_category`
- `orchestrator_read_plan_state`
- `orchestrator_export_codex_todo`
- `orchestrator_next_action`
- review and acceptance tools

### 5. Add Regression Coverage

Add repo-level tests that fail if the integrated workflow drifts away from the intended contract.

The tests should verify that:

- the bundled skill explicitly contains the brainstorming-stage behaviors
- repository and install-managed guidance say `codex-orchestrator` absorbs brainstorming and blocks superpowers as the entry workflow
- plugin metadata describes design/discovery as part of the workflow

## Verification Strategy

Verification should cover:

1. repo-level tests for skill text, plugin metadata, and routing guidance
2. existing runtime and install contract suites to ensure no regression in the MCP/execution surface
3. local installed guidance sync so the current machine reflects the new entry behavior immediately

## Success Criteria

- `codex-orchestrator` explicitly owns repository brainstorming/discovery before planning and execution.
- Repository and global guidance no longer leave room for `using-superpowers` to claim the entry point for normal repository tasks.
- The bundled skill, plugin metadata, and install guidance all describe the same integrated workflow.
- Regression tests keep the brainstorming integration durable.
