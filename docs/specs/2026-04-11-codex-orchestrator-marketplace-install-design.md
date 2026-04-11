# Codex Orchestrator Marketplace Install Design

## Context

The repository already ships a valid Codex plugin manifest, bundled skills, an MCP server, and a bootstrap installer script. However, the current installer was designed around direct filesystem placement plus config-file enablement. Current Codex documentation uses marketplaces as the supported discovery and installation surface for custom plugins, with plugin installation landing in the Codex plugin cache and `config.toml` storing plugin on/off state.

The repository needs to align with that model so the plugin is both discoverable in Codex and installable into a user's local Codex environment without manual JSON editing.

## Goals

- Add a repository marketplace so Codex can discover the plugin when the repository is open.
- Update the installer to bootstrap a personal marketplace and personal plugin source path that match current Codex conventions.
- Support a direct local install mode that stages the plugin into the personal plugin cache and enables it in `~/.codex/config.toml`.
- Preserve bundled agent installation only as a compatibility fallback instead of the primary installation contract.
- Keep installation idempotent and verifiable.

## Non-goals

- Publishing to the official OpenAI-curated Plugin Directory.
- Treating plugin-bundled agent manifests as a stable public install surface.
- Requiring users to edit marketplace or config files by hand.

## Design

### Installation Surfaces

- Repo marketplace: `$REPO_ROOT/.agents/plugins/marketplace.json`
- Personal marketplace: `~/.agents/plugins/marketplace.json`
- Personal plugin source root: `~/.codex/plugins/codex-orchestrator`
- Installed plugin cache path: `~/.codex/plugins/cache/local-plugins/codex-orchestrator/local`
- Plugin enablement entry: `[plugins."codex-orchestrator@local-plugins"]`

### Repository Marketplace

The repository should ship `.agents/plugins/marketplace.json` with a single local plugin entry:

- marketplace name: `local-plugins`
- marketplace display name: `Local Plugins`
- plugin name: `codex-orchestrator`
- plugin source path: `./plugins/codex-orchestrator`
- installation policy: `AVAILABLE`
- authentication policy: `ON_INSTALL`

This makes the plugin discoverable when the repository is opened in Codex.

### Personal Bootstrap Installer

The installer should be treated as a bootstrap tool, not as a substitute for the Codex plugin model.

Default behavior:

- copy or link the plugin into `~/.codex/plugins/codex-orchestrator`
- write or update `~/.agents/plugins/marketplace.json`
- stage a local installed copy in `~/.codex/plugins/cache/local-plugins/codex-orchestrator/local`
- write or update `[plugins."codex-orchestrator@local-plugins"] enabled = true`

Compatibility fallback:

- install bundled `.toml` agents into `~/.codex/agents`
- back up conflicting agent files before replacement

### Config Handling

The installer should no longer force `[features] apps = true` as part of plugin installation. That flag is for ChatGPT Apps/connectors support and is not required for a plugin that only bundles skills and MCP config.

The installer should only manage the plugin's own enabled state in `config.toml`.

### Install Modes

- `link` mode remains the default for local development
- `copy` mode remains available for self-contained installs
- `dry-run` remains available and must perform no writes

### Success Criteria

- Opening this repository in Codex exposes a `Local Plugins` marketplace source.
- Running the installer once bootstraps the personal marketplace, plugin source directory, installed cache copy, and enabled plugin state.
- The installer remains idempotent.
- The installer tests cover repo marketplace metadata, personal bootstrap paths, and config enablement behavior.
