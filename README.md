# codex-orchestrator

Codex Orchestrator is a local Codex plugin that turns specs, implementation plans, runtime orchestration state, review gates, and completion rules into repository-backed artifacts.

## Repo Marketplace

This repository now ships a repo-local marketplace at [`.agents/plugins/marketplace.json`](./.agents/plugins/marketplace.json).

When this repository is open in Codex, the Plugins directory can discover:

- source: `Local Plugins`
- plugin: `codex-orchestrator`

If the source does not appear immediately, restart Codex after opening the repository.

## Bootstrap Install

Run the installer from the repository root:

```bash
bash scripts/install-codex-orchestrator.sh --copy
```

What it installs:

- plugin source files into `~/.codex/plugins/codex-orchestrator`
- a personal marketplace entry in `~/.agents/plugins/marketplace.json`
- an installed plugin cache copy in `~/.codex/plugins/cache/local-plugins/codex-orchestrator/local`
- bundled agent files into `~/.codex/agents`
- a plugin enablement block in `~/.codex/config.toml`

On Windows, the script prefers `%USERPROFILE%` when it is available so the install lands in the Windows Codex home instead of a WSL-only home.

Useful options:

```bash
# Preview actions without writing files
bash scripts/install-codex-orchestrator.sh --dry-run

# Use symlinks instead of copied files for local development
bash scripts/install-codex-orchestrator.sh --link

# Override install paths for testing or custom environments
bash scripts/install-codex-orchestrator.sh \
  --plugin-home /custom/.codex/plugins \
  --marketplace-path /custom/.agents/plugins/marketplace.json \
  --agent-dir /custom/.codex/agents
```

The installer backs up conflicting agent files before replacing them.
It also writes or updates `[plugins."codex-orchestrator@local-plugins"]` in the Codex config and stages the installed cache copy that Codex loads.

After running the installer, restart Codex so the personal marketplace and installed plugin state are reloaded.
