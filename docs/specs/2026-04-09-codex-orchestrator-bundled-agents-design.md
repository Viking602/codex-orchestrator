# Codex Orchestrator Bundled Agents Design

## Context

The plugin already ships its own MCP control plane, file-backed plans, and workflow contracts, but it still depends on whatever agent inventory happens to exist in the host Codex environment. That makes the preferred-role path in `plugins/codex-orchestrator/config/categories.toml` non-deterministic.

The next step is to bundle a small, opinionated set of Codex agents with the plugin so the default planning, research, implementation, review, and routing-doc paths travel with the plugin instead of being implicit external setup.

## Goals

- Bundle the minimum useful Codex agent set directly with the plugin.
- Align the bundled roles with the plugin's current preferred workflow categories.
- Treat bundled copies as plugin-owned local derivatives that can be tuned for this repository.
- Record provenance and modifiability so future edits do not depend on chat memory.
- Add regression checks so bundled agent inventory and category preferences cannot silently drift apart.

## Non-goals

- Replacing Codex native subagent execution with a plugin-side runtime.
- Mirroring the full `awesome-codex-subagents` catalog.
- Shipping a generic implementation agent when the plugin already has a stronger stack-aligned implementation role.
- Assuming every host automatically registers plugin-bundled `.toml` agents without an explicit path or install step.

## Design Principles

### 1. Bundle The Happy Path, Not The Whole World

Only bundle roles that directly support the current orchestrator flow:

- `harness-planner`
- `search-specialist`
- `backend-developer`
- `harness-evaluator`
- `harness-dispatch-gate`
- `harness-doc-gardener`

This covers the plugin's preferred path for planning, research, coding implementation, review, dispatch decisions, and routing-doc cleanup without creating overlapping review or implementation surfaces by default.

### 2. Prefer Stack-Aligned Implementation Over Narrow Specialization

The current codebase is a coding-direction plugin. The default implementation role should therefore be a generic coding agent instead of a narrow MCP-specialist persona or a language-locked role. `harness-generator` stays out of the default bundle because the plugin already has a clearer implementation owner.

### 3. Vendored Copies Become Plugin-Owned

Bundled agent files under `plugins/codex-orchestrator/codex/agents/` are the local source of truth for this repository. They should carry source attribution, but they are no longer treated as read-only upstream mirrors. Plugin-specific optimization is expected.

### 4. Activation Must Be Explicit

The plugin should advertise its bundled agents through `plugins/codex-orchestrator/agents/openai.yaml`. If a host does not automatically register plugin-local agent bundles, the plugin docs must say so plainly and point to the bundled `.toml` files as the install source.

### 5. Category Drift Must Fail In Tests

Regression tests should assert that the preferred roles in `categories.toml` are present in the plugin's bundled agent list. If the category contract changes, the bundle contract must be updated in the same pass.

## Selected Sources

### From `harness-engineering`

- `harness-planner`
- `harness-dispatch-gate`
- `harness-evaluator`
- `harness-doc-gardener`

These roles already encode the workflow semantics closest to the orchestrator plugin's planning, review, and doc-maintenance model.

### From `awesome-codex-subagents`

- `backend-developer`
- `search-specialist`

These roles match the plugin's preferred implementation and research categories without introducing a niche professional agent or overfitting the default coding path to a single language persona.

## Customization Strategy

Each bundled agent should be tightened around the current plugin:

- read `docs/index.md` first
- respect `docs/specs/`, `docs/plans/active/`, and `docs/plans/completed/` as canonical durable roots
- treat the parent agent as owner of orchestrator MCP tool calls and plan acceptance
- optimize instructions for `plugins/codex-orchestrator/**`, not for general-purpose repository work
- require routing-doc updates when plugin entrypoints or current docs move

## Success Criteria

- The plugin ships a `codex/agents/` bundle with the selected roles.
- `agents/openai.yaml` points Codex at the bundled `.toml` files.
- The repository documents source, purpose, and modifiability of the bundle.
- Tests fail if category preferred roles drift away from the bundled inventory.
