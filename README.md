# codex-orchestrator

Codex Orchestrator is a local Codex plugin that turns specs, implementation plans, runtime orchestration state, review gates, and completion rules into repository-backed artifacts.
It now also absorbs the repository-useful brainstorming stage: context exploration, one-question-at-a-time clarification, approach comparison, and design approval before planning and execution.

For the supported Codex-guided install and update flow, read [`install.md`](./install.md).

## Repo Marketplace

This repository now ships a repo-local marketplace at [`.agents/plugins/marketplace.json`](./.agents/plugins/marketplace.json).

When this repository is open in Codex, the Plugins directory can discover:

- source: `Local Plugins`
- plugin: `codex-orchestrator`

If the source does not appear immediately, restart Codex after opening the repository.

## Codex-Guided Install

Use [`install.md`](./install.md) as the source of truth. Codex should perform the reconciliation directly instead of routing through a repository-owned shell installer.

What the guide tells Codex to install or update:

- plugin source files into `~/.codex/plugins/codex-orchestrator`
- a personal marketplace entry in `~/.agents/plugins/marketplace.json`
- an installed plugin cache copy in `~/.codex/plugins/cache/local-plugins/codex-orchestrator/local`
- a built native MCP binary under `~/.codex/plugins/cache/local-plugins/codex-orchestrator/local/.codex-orchestrator/bin/`
- a managed global MCP bootstrap in `~/.codex/config.toml` that points at that native binary
- a managed default-workflow block in the active global `AGENTS` file under `~/.codex/`
- bundled agent files into `~/.codex/agents`
- a plugin enablement block in `~/.codex/config.toml`

The direct guide also covers updates, native-binary staging, global `AGENTS` reconciliation, bundled-agent refresh, MCP bootstrap verification, and fresh-process validation.

After Codex finishes the install or update steps, restart Codex so the personal marketplace, installed plugin state, and global guidance are reloaded. New threads after restart should default to the `codex-orchestrator` workflow for normal repository tasks without requiring manual `@codex-orchestrator` invocation.

For source-checkout development in this repository, the bundled plugin manifest launches the Rust MCP crate through Cargo.
The active runtime and contract-test surface now live entirely under `plugins/codex-orchestrator/rust-cli/`; no TypeScript source or test files remain in the active repository surface.
