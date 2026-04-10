# Codex Orchestrator Bundled Agents

This directory holds the Codex `.toml` agent bundle shipped with the plugin.

## Included Agents

- `harness-planner`
- `harness-dispatch-gate`
- `search-specialist`
- `backend-developer`
- `harness-evaluator`
- `harness-doc-gardener`

## Source Policy

- `harness-*` files are customized copies derived from the maintainer's local `harness-engineering` workspace.
- `backend-developer` and `search-specialist` are customized copies derived from `awesome-codex-subagents` (MIT licensed).
- The files in this directory are the source of truth for this repository. Edit these copies directly when the plugin workflow changes.
- The current copies are additionally tuned using the GPT-specific orchestration patterns observed in OpenAgent's GPT-5.4 Sisyphus and Sisyphus-Junior prompts: tighter intent gates, findings-first contracts, explicit verification/completion contracts, and stronger anti-drift scope discipline.

## Activation Note

The plugin advertises this directory through `plugins/codex-orchestrator/agents/openai.yaml`.

If a host Codex runtime does not automatically register plugin-bundled agent definitions, use the `.toml` files here as the install source for:

- `.codex/agents/`
- `~/.codex/agents/`
