# Codex Orchestrator Install And Update Guide

This file is for Codex itself.

If the user asks you to install, update, or verify `codex-orchestrator`, do the work yourself. Perform the file operations directly. Do not send the user to a repository-owned installer script, and do not ask the user to hand-edit Codex files unless direct local edits fail and you cannot recover.

## Goal

Install and update the plugin by reconciling the expected Codex files directly:

- plugin source
- installed cache copy
- installed native MCP binary
- personal marketplace entry
- plugin enabled state
- active global `AGENTS` block
- bundled agent files

Install and update use the same workflow. On first install you create missing targets. On update you overwrite stale plugin-owned targets and preserve unrelated user content.

Plugin enablement alone is not sufficient success. If the bundled skill is visible but `orchestrator_*` tools are missing from the callable tool registry, installation is still incomplete.

## Target Paths

Use these targets:

- plugin source: `~/.codex/plugins/codex-orchestrator`
- installed cache: `~/.codex/plugins/cache/local-plugins/codex-orchestrator/local`
- installed native MCP binary:
  - Unix: `~/.codex/plugins/cache/local-plugins/codex-orchestrator/local/.codex-orchestrator/bin/codex-orchestrator-mcp`
  - Windows: `~/.codex/plugins/cache/local-plugins/codex-orchestrator/local/.codex-orchestrator/bin/codex-orchestrator-mcp.exe`
- personal marketplace: `~/.agents/plugins/marketplace.json`
- plugin enablement: `~/.codex/config.toml`
- managed global MCP bootstrap: `~/.codex/config.toml`
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
7. Build or refresh the installed-cache native MCP binary from `~/.codex/plugins/cache/local-plugins/codex-orchestrator/local/rust-cli/Cargo.toml`.
8. Resolve the absolute installed-cache binary path under `~/.codex/plugins/cache/local-plugins/codex-orchestrator/local/.codex-orchestrator/bin/`.
9. Ensure `~/.codex/config.toml` contains the managed `mcp_servers.codex-orchestrator` block shown below, using that resolved absolute binary path.
10. Ensure the active global `AGENTS` file contains the managed `codex-orchestrator` default-workflow block shown below.
11. Copy or refresh bundled agent files from `plugins/codex-orchestrator/codex/agents/` into `~/.codex/agents/`.
12. Run verification yourself and report evidence.
13. If the desktop app was already open, tell the user a restart is required before a fresh thread will see the updated external state.

No repository-owned shell installer is the supported path anymore. Codex should perform this reconciliation directly.

The direct reconcile flow must close the MCP gap too: a fresh session should not stop at bundled skill discovery while leaving `orchestrator_*` unavailable.
The managed workflow guidance must also preserve immediate top-level acceptance so native todo progress does not stay pinned on the first task until a late sweep.

## Managed Global AGENTS Block

Keep this block present and up to date in the active global `AGENTS` file while preserving unrelated user text outside it.

```md
<!-- codex-orchestrator-default-workflow:begin -->
## Codex Orchestrator Default Workflow

- When `codex-orchestrator` is installed and enabled, use it as the default workflow for repository tasks unless the user explicitly asks for a different workflow.
- `codex-orchestrator` absorbs repository brainstorming and discovery, so normal repository tasks must not enter through `using-superpowers` or standalone `brainstorming`.
- Start with the bundled `codex-orchestrator` skill for discovery, requirements clarification, design work, feature work, bug fixes, refactors, debugging, docs changes, architecture work, and other multi-step repository tasks.
- Treat repository inspection, codebase-check, repo-audit, and read-only repo-understanding requests as `research` work that should dispatch `search-specialist` before the parent keeps that work local.
- Explore context first, ask clarifying questions one at a time only when something material is missing, compare 2-3 approaches only when the direction is still open, and ask for approval before the implementation plan only when the proposed direction materially changes the request.
- If the user already supplied a workable direction and no hard blocker exists, do not ask a second confirmation question; summarize assumptions, write the spec and plan, and continue.
- Use the plugin MCP tools to resolve category, read the active plan, export the native Codex todo mirror, record step progress, and enforce review and completion gates.
- When native `update_plan` is available, mirror the active implementation plan into that surface instead of maintaining a separate chat todo.
- When a terminal review pass closes a task, accept the top-level task in the same control-plane pass so the next top-level item becomes visible immediately.
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

Ensure `~/.codex/config.toml` contains the plugin enablement block:

```toml
[plugins."codex-orchestrator@local-plugins"]
enabled = true
```

Preserve unrelated config content. Do not force unrelated feature flags.

## Managed MCP Bootstrap

Ensure `~/.codex/config.toml` also contains a native executable bootstrap:

```toml
[mcp_servers.codex-orchestrator]
command = "<absolute installed-cache path>/.codex-orchestrator/bin/codex-orchestrator-mcp"
startup_timeout_sec = 30
```

Use the resolved absolute installed-cache path, not a relative path and not a literal `~` string. The intended target is the installed cache copy under:

- Windows: `~/.codex/plugins/cache/local-plugins/codex-orchestrator/local/.codex-orchestrator/bin/codex-orchestrator-mcp.exe`
- Unix: `~/.codex/plugins/cache/local-plugins/codex-orchestrator/local/.codex-orchestrator/bin/codex-orchestrator-mcp`

This bootstrap is required because plugin enablement can leave the bundled skill visible while the `orchestrator_*` MCP tools are still absent from the callable tool registry.

For source-checkout development inside this repository, the bundled plugin `.mcp.json` uses `cargo run --manifest-path ./rust-cli/Cargo.toml`. Installed runtime should not point back at deleted source-checkout paths or at `cargo run`.

## Native Binary Staging

When installing or updating, build from the installed cache copy and stage the release binary into the managed bin directory:

- build source: `~/.codex/plugins/cache/local-plugins/codex-orchestrator/local/rust-cli/Cargo.toml`
- stage directory: `~/.codex/plugins/cache/local-plugins/codex-orchestrator/local/.codex-orchestrator/bin/`

PowerShell example:

```powershell
$cache = Join-Path $env:USERPROFILE '.codex\plugins\cache\local-plugins\codex-orchestrator\local'
cargo build --release --manifest-path (Join-Path $cache 'rust-cli\Cargo.toml')
New-Item -ItemType Directory -Force -Path (Join-Path $cache '.codex-orchestrator\bin') | Out-Null
Copy-Item -Force (Join-Path $cache 'rust-cli\target\release\codex-orchestrator-mcp.exe') (Join-Path $cache '.codex-orchestrator\bin\codex-orchestrator-mcp.exe')
```

Bash example:

```bash
cache="${HOME}/.codex/plugins/cache/local-plugins/codex-orchestrator/local"
cargo build --release --manifest-path "${cache}/rust-cli/Cargo.toml"
mkdir -p "${cache}/.codex-orchestrator/bin"
cp "${cache}/rust-cli/target/release/codex-orchestrator-mcp" "${cache}/.codex-orchestrator/bin/codex-orchestrator-mcp"
```

## Bundled Agents Rule

Copy or refresh the bundled agent files from `plugins/codex-orchestrator/codex/agents/` into `~/.codex/agents/`.

If a conflicting file already exists and it is not plugin-owned, back it up before replacement instead of silently deleting user work.

## Verification Commands

Run verification yourself and report the output-backed result.

### PowerShell verification

```powershell
Test-Path "$env:USERPROFILE\.codex\plugins\codex-orchestrator\.codex-plugin\plugin.json"
Test-Path "$env:USERPROFILE\.codex\plugins\cache\local-plugins\codex-orchestrator\local\.codex-plugin\plugin.json"
Test-Path "$env:USERPROFILE\.codex\plugins\cache\local-plugins\codex-orchestrator\local\.codex-orchestrator\bin\codex-orchestrator-mcp.exe"
Select-String -Path "$env:USERPROFILE\.codex\config.toml" -Pattern 'codex-orchestrator@local-plugins','enabled = true'
Select-String -Path "$env:USERPROFILE\.codex\config.toml" -Pattern 'mcp_servers.codex-orchestrator','codex-orchestrator-mcp'
Select-String -Path "$env:USERPROFILE\.codex\AGENTS.md","$env:USERPROFILE\.codex\AGENTS.override.md" -Pattern 'codex-orchestrator-default-workflow'
Select-String -Path "$env:USERPROFILE\.agents\plugins\marketplace.json" -Pattern 'codex-orchestrator','./.codex/plugins/codex-orchestrator'
```

### Bash verification

```bash
test -f "${HOME}/.codex/plugins/codex-orchestrator/.codex-plugin/plugin.json"
test -f "${HOME}/.codex/plugins/cache/local-plugins/codex-orchestrator/local/.codex-plugin/plugin.json"
test -f "${HOME}/.codex/plugins/cache/local-plugins/codex-orchestrator/local/.codex-orchestrator/bin/codex-orchestrator-mcp"
grep -n 'codex-orchestrator@local-plugins' "${HOME}/.codex/config.toml"
grep -n 'mcp_servers.codex-orchestrator' "${HOME}/.codex/config.toml"
grep -n 'codex-orchestrator-mcp' "${HOME}/.codex/config.toml"
grep -n 'codex-orchestrator-default-workflow' "${HOME}/.codex/AGENTS.md" "${HOME}/.codex/AGENTS.override.md" 2>/dev/null
grep -n 'codex-orchestrator' "${HOME}/.agents/plugins/marketplace.json"
```

### Repository validation suite

```bash
cargo test --manifest-path plugins/codex-orchestrator/rust-cli/Cargo.toml
rg --files -g "*.ts" -g "*.tsx"
```

## Fresh-Process Validation

Use a fresh Codex process when you need to verify the default workflow and tool exposure without relying on the current chat state.

Example:

```powershell
$out = Join-Path $env:TEMP 'codex-orchestrator-default-workflow-check.txt'
if (Test-Path $out) { Remove-Item $out -Force }
codex exec --skip-git-repo-check -C . -o $out "You are starting repository work here. In one sentence only, name the default workflow you must use for repository tasks in this repo."
Get-Content $out
```

Expected result: the output names `codex-orchestrator` as the default workflow.

To validate the MCP bootstrap itself from a clean non-repo cwd, prefer a second fresh-process probe:

```powershell
$tmp = Join-Path $env:TEMP 'codex-orchestrator-nonrepo'
if (!(Test-Path $tmp)) { New-Item -ItemType Directory -Path $tmp | Out-Null }
$out = Join-Path $env:TEMP 'codex-orchestrator-tool-registry-check.txt'
if (Test-Path $out) { Remove-Item $out -Force }
codex exec --skip-git-repo-check -C $tmp -o $out "Use codex-orchestrator only. If orchestrator_resolve_category is available, say that it is present in the callable tool registry."
Get-Content $out
```

Expected result: the output confirms that `orchestrator_resolve_category` is callable instead of saying the tool is missing from the registry.

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
