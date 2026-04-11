# Codex Orchestrator Install And Update Guide

This file is for Codex itself.

If the user asks you to install, update, or verify `codex-orchestrator`, do the work yourself. Perform the file operations directly. Do not send the user to a repository-owned installer script, and do not ask the user to hand-edit Codex files unless direct local edits fail and you cannot recover.

## Goal

Install and update the plugin by reconciling the expected Codex files directly:

- plugin source
- installed cache copy
- personal marketplace entry
- plugin enabled state
- active global `AGENTS` block
- bundled agent files

Install and update use the same workflow. On first install you create missing targets. On update you overwrite stale plugin-owned targets and preserve unrelated user content.

## Target Paths

Use these targets:

- plugin source: `~/.codex/plugins/codex-orchestrator`
- installed cache: `~/.codex/plugins/cache/local-plugins/codex-orchestrator/local`
- personal marketplace: `~/.agents/plugins/marketplace.json`
- plugin enablement: `~/.codex/config.toml`
- active global guidance: `~/.codex/AGENTS.override.md` when it exists, otherwise `~/.codex/AGENTS.md`
- bundled agents: `~/.codex/agents`

Repository source inputs:

- plugin payload: `plugins/codex-orchestrator/`
- repo marketplace seed: `.agents/plugins/marketplace.json`

## Codex-Driven Reconcile Flow

1. Start from the repository root.
2. Determine the active Codex home and active global `AGENTS` file.
3. Copy or sync `plugins/codex-orchestrator/` into `~/.codex/plugins/codex-orchestrator`.
4. Copy or sync that installed plugin into `~/.codex/plugins/cache/local-plugins/codex-orchestrator/local`.
5. Ensure `~/.agents/plugins/marketplace.json` contains a `codex-orchestrator` local plugin entry.
6. Ensure `~/.codex/config.toml` contains `[plugins."codex-orchestrator@local-plugins"]` with `enabled = true`.
7. Ensure the active global `AGENTS` file contains the managed `codex-orchestrator` default-workflow block shown below.
8. Copy or refresh bundled agent files from `plugins/codex-orchestrator/codex/agents/` into `~/.codex/agents/`.
9. Run verification yourself and report evidence.
10. If the desktop app was already open, tell the user a restart is required before a fresh thread will see the updated external state.

No repository-owned shell installer is the supported path anymore. Codex should perform this reconciliation directly.

## Managed Global AGENTS Block

Keep this block present and up to date in the active global `AGENTS` file while preserving unrelated user text outside it.

```md
<!-- codex-orchestrator-default-workflow:begin -->
## Codex Orchestrator Default Workflow

- When `codex-orchestrator` is installed and enabled, use it as the default workflow for repository tasks unless the user explicitly asks for a different workflow.
- Start with the bundled `codex-orchestrator` skill for feature work, bug fixes, refactors, debugging, docs changes, architecture work, and other multi-step repository tasks.
- Use the plugin MCP tools to resolve category, read the active plan, export the native Codex todo mirror, record step progress, and enforce review and completion gates.
- When native `update_plan` is available, mirror the active implementation plan into that surface instead of maintaining a separate chat todo.
- Generic process skills are fallback helpers after `codex-orchestrator` takes control or when the plugin is unavailable.
- Follow stronger repository-local `AGENTS.md` guidance when a repository provides it.
<!-- codex-orchestrator-default-workflow:end -->
```

## Marketplace Entry

Ensure the personal marketplace includes a `codex-orchestrator` entry with:

- `name = "codex-orchestrator"`
- local source path pointing at `./.codex/plugins/codex-orchestrator`
- `policy.installation = "AVAILABLE"`
- `policy.authentication = "ON_INSTALL"`
- `category = "Coding"`

If the marketplace file does not exist yet, create it.

## Config Requirement

Ensure `~/.codex/config.toml` contains:

```toml
[plugins."codex-orchestrator@local-plugins"]
enabled = true
```

Preserve unrelated config content. Do not force unrelated feature flags.

## Bundled Agents Rule

Copy or refresh the bundled agent files from `plugins/codex-orchestrator/codex/agents/` into `~/.codex/agents/`.

If a conflicting file already exists and it is not plugin-owned, back it up before replacement instead of silently deleting user work.

## Verification Commands

Run verification yourself and report the output-backed result.

### PowerShell verification

```powershell
Test-Path "$env:USERPROFILE\.codex\plugins\codex-orchestrator\.codex-plugin\plugin.json"
Test-Path "$env:USERPROFILE\.codex\plugins\cache\local-plugins\codex-orchestrator\local\.codex-plugin\plugin.json"
Select-String -Path "$env:USERPROFILE\.codex\config.toml" -Pattern 'codex-orchestrator@local-plugins','enabled = true'
Select-String -Path "$env:USERPROFILE\.codex\AGENTS.md","$env:USERPROFILE\.codex\AGENTS.override.md" -Pattern 'codex-orchestrator-default-workflow'
Select-String -Path "$env:USERPROFILE\.agents\plugins\marketplace.json" -Pattern 'codex-orchestrator','./.codex/plugins/codex-orchestrator'
```

### Bash verification

```bash
test -f "${HOME}/.codex/plugins/codex-orchestrator/.codex-plugin/plugin.json"
test -f "${HOME}/.codex/plugins/cache/local-plugins/codex-orchestrator/local/.codex-plugin/plugin.json"
grep -n 'codex-orchestrator@local-plugins' "${HOME}/.codex/config.toml"
grep -n 'codex-orchestrator-default-workflow' "${HOME}/.codex/AGENTS.md" "${HOME}/.codex/AGENTS.override.md" 2>/dev/null
grep -n 'codex-orchestrator' "${HOME}/.agents/plugins/marketplace.json"
```

### Guide regression suite

```bash
node --experimental-strip-types --test tests/install-guide.test.ts
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

## Update Rule

Treat updates as the same reconcile flow as install:

- refresh the plugin source directory from the repository copy
- refresh the installed cache copy
- rewrite the marketplace/config/managed-AGENTS surfaces if they drifted
- refresh bundled agents
- rerun verification

Do not ask the user to uninstall first unless a verification failure shows a real conflict you cannot repair locally.

## MCP Validation

If the user wants proof that MCP is usable instead of just installed, ask Codex to do a read-only orchestration step and report which `orchestrator_*` tools were called.

Example prompt:

```text
Use codex-orchestrator only for read-only validation: resolve category, read plan state, and then list the orchestrator_* tools you called.
```

## Restart Rule

If installation or update changed files under `~/.codex/` while the desktop app was already running, a restart is still required before expecting a fresh thread in the app to reflect the new external plugin state and global guidance.

## Repo-Local Override Rule

The plugin can be installed and healthy while another repository still follows its own local workflow. Repo-local `AGENTS.md`, nested project guidance, or repo-specific harness docs can override the global default workflow. If installation looks correct but the plugin is not chosen in another repo, inspect that repo's local guidance next.
