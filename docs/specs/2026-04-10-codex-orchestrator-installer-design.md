# Codex Orchestrator Installer Script Design

## Context

The repository now contains a runnable Codex plugin, bundled agents, and local plugin metadata, but installation still requires manual copying and registration. A one-click installer should put the plugin, marketplace entry, and bundled agents into the expected local Codex paths without requiring manual file operations.

## Goals

- Install the plugin into a user-local plugin path with a single command.
- Register the plugin in `~/.agents/plugins/marketplace.json` when missing.
- Install bundled agents into `~/.codex/agents/` safely.
- Enable the plugin in `~/.codex/config.toml` so Codex actually loads it after installation.
- Default to a safe install mode that does not silently destroy existing agent files.
- Support `--dry-run` for verification and testing.

## Non-goals

- Cross-platform Windows PowerShell support in this phase.
- Remote installation or package publishing.
- Automatically restarting Codex after install.

## Design

### Install Targets

- Plugin target root: `~/plugins/codex-orchestrator`
- Marketplace file: `~/.agents/plugins/marketplace.json`
- Agent install directory: `~/.codex/agents`
- Codex config file: `~/.codex/config.toml`

### Install Modes

- `link` mode: symlink the plugin directory into the target plugin root
- `copy` mode: copy the plugin directory into the target plugin root

Default mode should be `link`, because the repository is the source of truth and local development changes should flow through immediately.

### Safety Model

- Existing plugin target is replaced by the selected install mode.
- Existing agent files with conflicting contents are backed up before replacement.
- Existing plugin enablement in Codex config is updated in place instead of duplicated.
- The script supports `--dry-run` to print actions without mutating the filesystem.

### Implementation Shape

- Bash entrypoint in `scripts/install-codex-orchestrator.sh`
- Use `node` for JSON marketplace editing
- Use `node` for TOML-like config block editing in `~/.codex/config.toml`
- Use shell copy/link logic for plugin and agent files
- Exclude runtime state artifacts from copied installs

## Success Criteria

- Running the script once creates or updates the plugin target, marketplace entry, agent files, and Codex plugin enablement config.
- Dry-run performs no writes.
- Existing conflicting agent files are backed up before replacement.
