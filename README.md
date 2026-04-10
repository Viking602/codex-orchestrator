# codex-orchestrator

Codex Orchestrator is a local Codex plugin that turns specs, implementation plans, runtime orchestration state, review gates, and completion rules into repository-backed artifacts.

## One-Click Install

Run the installer from the repository root:

```bash
bash scripts/install-codex-orchestrator.sh
```

What it installs:

- plugin files into `~/plugins/codex-orchestrator`
- a marketplace entry in `~/.agents/plugins/marketplace.json`
- bundled agent files into `~/.codex/agents`
- a plugin enablement block in `~/.codex/config.toml`

Useful options:

```bash
# Preview actions without writing files
bash scripts/install-codex-orchestrator.sh --dry-run

# Copy files instead of creating a symlinked plugin install
bash scripts/install-codex-orchestrator.sh --copy

# Override install paths for testing or custom environments
bash scripts/install-codex-orchestrator.sh \
  --plugin-home /custom/plugins \
  --marketplace-path /custom/.agents/plugins/marketplace.json \
  --agent-dir /custom/.codex/agents
```

The installer backs up conflicting agent files before replacing them.
It also writes or updates `[plugins."codex-orchestrator@local-plugins"]` in the Codex config so the plugin is actually enabled after installation.
