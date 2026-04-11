# Codex Orchestrator Install Guide

This file is for an AI agent with shell access.

If the user asks you to install or verify `codex-orchestrator`, do the work yourself. Do not ask the user to edit files by hand unless the installer or verification command actually fails and you cannot recover locally.

## Goal

Install the plugin into the local Codex environment, verify that the supported files were written, and report evidence.

## Prerequisites

- Run from the repository root.
- `node` must be available.
- `bash` must be available.
- The supported install path is the repository-owned installer:

```bash
bash scripts/install-codex-orchestrator.sh --copy
```

Use `--link` only when the goal is a live development install from this working tree.

## Standard AI-Driven Install Flow

1. Change to the repository root.
2. Run the installer yourself.
3. Verify cache, config, and global guidance files yourself.
4. If the desktop app was already open, tell the user a restart is required for a fresh thread to pick up the new external install state.

### Standard install

```bash
bash scripts/install-codex-orchestrator.sh --copy
```

### Development install

```bash
bash scripts/install-codex-orchestrator.sh --link
```

### Dry run

```bash
bash scripts/install-codex-orchestrator.sh --dry-run
```

## Windows And WSL Note

On Windows, the installer prefers `%USERPROFILE%` when it is available so the install lands in the Windows Codex home instead of a WSL-only home. If you are running through WSL and need to target the Windows desktop app, export `USERPROFILE` first.

Example:

```bash
export USERPROFILE="$(powershell.exe -NoProfile -Command '$env:USERPROFILE' | tr -d '\r')"
bash scripts/install-codex-orchestrator.sh --copy
```

## What The Installer Writes

The installer is the source of truth for local installation. It writes or updates:

- plugin source: `~/.codex/plugins/codex-orchestrator`
- installed cache: `~/.codex/plugins/cache/local-plugins/codex-orchestrator/local`
- personal marketplace: `~/.agents/plugins/marketplace.json`
- plugin enablement: `~/.codex/config.toml`
- global default-workflow guidance: `~/.codex/AGENTS.md` or the active `AGENTS.override.md`
- bundled fallback agents: `~/.codex/agents`

Do not replace this with manual file edits as the default workflow.

## Verification Commands

Run verification yourself and report the output-backed result.

### PowerShell verification

```powershell
Test-Path "$env:USERPROFILE\.codex\plugins\cache\local-plugins\codex-orchestrator\local\.codex-plugin\plugin.json"
Select-String -Path "$env:USERPROFILE\.codex\config.toml" -Pattern 'codex-orchestrator@local-plugins','enabled = true'
Select-String -Path "$env:USERPROFILE\.codex\AGENTS.md" -Pattern 'codex-orchestrator-default-workflow'
```

### Bash verification

```bash
test -f "${HOME}/.codex/plugins/cache/local-plugins/codex-orchestrator/local/.codex-plugin/plugin.json"
grep -n 'codex-orchestrator@local-plugins' "${HOME}/.codex/config.toml"
grep -n 'codex-orchestrator-default-workflow' "${HOME}/.codex/AGENTS.md"
```

### Installer regression suite

```bash
node --experimental-strip-types --test tests/install-script.test.ts
```

## Fresh-Process Validation

Use a fresh Codex process when you need to verify the default workflow without relying on the current chat state.

Example:

```powershell
$out = Join-Path $env:TEMP 'codex-orchestrator-default-workflow-check.txt'
if (Test-Path $out) { Remove-Item $out -Force }
codex exec --skip-git-repo-check -C . -o $out "You are starting repository work here. In one sentence only, name the default workflow you must use for repository tasks in this repo."
Get-Content $out
```

Expected result: the output names `codex-orchestrator` as the default workflow.

## MCP Validation

If the user wants proof that MCP is usable instead of just installed, ask Codex to do a read-only orchestration step and report which `orchestrator_*` tools were called.

Example prompt:

```text
Use codex-orchestrator only for read-only validation: resolve category, read plan state, and then list the orchestrator_* tools you called.
```

## Restart Rule

If installation changed files under `~/.codex/` while the desktop app was already running, a restart is still required before expecting a fresh thread in the app to reflect the new external plugin state and global guidance.

## Repo-Local Override Rule

The plugin can be installed and healthy while another repository still follows its own local workflow. Repo-local `AGENTS.md`, nested project guidance, or repo-specific harness docs can override the global default workflow. If installation looks correct but the plugin is not chosen in another repo, inspect that repo's local guidance next.
