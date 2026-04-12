# Bundled Agent Bundle

## Purpose

This document defines the Codex agent bundle shipped with `plugins/codex-orchestrator/`.

The goal is not to mirror every possible custom agent. The goal is to make the plugin's preferred workflow roles travel with the plugin so the default planning, research, coding implementation, review, dispatch, and routing-doc flows do not depend on an undocumented host setup.

## Activation Model

- Bundled Codex agent files live in `plugins/codex-orchestrator/codex/agents/`.
- `plugins/codex-orchestrator/agents/openai.yaml` advertises that bundle as the plugin's codex agent surface.
- The plugin still relies on Codex native subagent execution.
- Some hosts may still require copying the bundled `.toml` files into `.codex/agents/` or `~/.codex/agents/` before ordinary conversations can dispatch them by name. The plugin package includes the files either way.

## Bundled Roles

| Agent | Bundled Path | Source | Why Bundled | Modifiable |
|---|---|---|---|---|
| `harness-planner` | `plugins/codex-orchestrator/codex/agents/harness-planner.toml` | `DevSkills/harness-engineering` local source | Preferred planning role for file-backed specs and plans | Yes |
| `harness-dispatch-gate` | `plugins/codex-orchestrator/codex/agents/harness-dispatch-gate.toml` | `DevSkills/harness-engineering` local source | Workflow gate before stack-owned edits | Yes |
| `harness-evaluator` | `plugins/codex-orchestrator/codex/agents/harness-evaluator.toml` | `DevSkills/harness-engineering` local source | Preferred review role for findings-first evaluation | Yes |
| `harness-doc-gardener` | `plugins/codex-orchestrator/codex/agents/harness-doc-gardener.toml` | `DevSkills/harness-engineering` local source | Routing-doc cleanup after project-surface changes | Yes |
| `backend-developer` | `plugins/codex-orchestrator/codex/agents/backend-developer.toml` | `awesome-codex-subagents` | Preferred generic coding implementation role for this plugin | Yes |
| `search-specialist` | `plugins/codex-orchestrator/codex/agents/search-specialist.toml` | `awesome-codex-subagents` | Preferred research role for fast repo evidence gathering | Yes |

## Selection Rules

The bundle intentionally matches the plugin's current preferred roles in `plugins/codex-orchestrator/config/categories.toml`:

- `plan` -> `harness-planner`
- `research` -> `search-specialist`
- `backend-impl` -> `backend-developer`
- `review` -> `harness-evaluator`

Two extra workflow roles are bundled because they materially improve parent behavior:

- `harness-dispatch-gate`
- `harness-doc-gardener`

## Deliberate Exclusions

- `mcp-developer` is not bundled because this is a coding-direction plugin and should not make a niche MCP specialist its default implementation persona.
- Language-specific implementation personas are not the default bundled path because the plugin should stay generic instead of binding the happy path to one stack-specialist role.
- `harness-generator` is not bundled by default because the plugin already has a stronger generic implementation owner: `backend-developer`.
- `code-reviewer` is not bundled by default because the review happy path already lands on `harness-evaluator`. Adding both as bundled defaults would create overlapping review surfaces without improving determinism.

## Customization Policy

These bundled files are plugin-owned local derivatives.

- Edit the copies in `plugins/codex-orchestrator/codex/agents/`.
- Do not treat upstream source directories as the runtime source of truth for this repository.
- Keep source attribution comments at the top of the vendored files.
- When a category preferred role changes, update the bundled set and the regression test in the same pass.

## Source And Attribution Notes

- `awesome-codex-subagents` is MIT licensed; keep attribution when vendoring derived files.
- `harness-engineering` sources here were copied from the maintainer's local workspace and are maintained as plugin-specific derivatives in this repository.
