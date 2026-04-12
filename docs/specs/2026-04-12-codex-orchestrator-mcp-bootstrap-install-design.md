# Codex Orchestrator MCP Bootstrap Install Design

## Context

The current Codex-guided install flow copies the plugin, stages the installed cache, writes marketplace state, enables the plugin, and bootstraps global `AGENTS` guidance. In practice that is not sufficient to guarantee that the plugin's `orchestrator_*` MCP tools appear in a fresh Codex session.

The failure mode is reproducible:

- the bundled `codex-orchestrator` skill is discoverable and selectable
- the plugin is enabled in `~/.codex/config.toml`
- but `orchestrator_*` tools are absent from the callable tool registry

When the same server is registered explicitly as a global MCP stdio server, the `orchestrator_*` tools enter the tool registry. That means the install flow currently leaves the user in a half-installed state: plugin copy succeeds, but runtime MCP exposure is not guaranteed.

## Goals

- Make installation and updates bootstrap a stable MCP runtime entry for `codex-orchestrator`.
- Keep the plugin-bundled `.mcp.json` explicit about working directory and startup tolerance.
- Update the install guide and regression tests so the repository no longer treats plugin enablement alone as sufficient.
- Preserve idempotent reconciliation behavior and avoid clobbering unrelated user config.

## Non-goals

- Changing orchestrator tool schemas or tool semantics.
- Replacing the plugin packaging model with a pure global-MCP-only distribution.
- Solving unrelated duplicate bundled-agent warnings in the host runtime.

## Design

### 1. Treat MCP Exposure As Part Of Installation Correctness

The install contract should no longer stop at:

- plugin source copy
- installed cache copy
- marketplace entry
- plugin enabled state
- global `AGENTS` bootstrap

It must also ensure that a fresh Codex session can see `orchestrator_*` as callable tools. If that requires an explicit global MCP server registration, the install flow should do it.

### 2. Add A Stable Global MCP Bootstrap

The install guide should require a managed config block under `~/.codex/config.toml`:

```toml
[mcp_servers.codex-orchestrator]
command = "node"
args = ["--experimental-strip-types", "<absolute path to installed cache>/src/server.ts"]
startup_timeout_sec = 30
```

The path in `args` should be the resolved absolute path to the installed cache copy:

- `~/.codex/plugins/cache/local-plugins/codex-orchestrator/local/src/server.ts`

Using an absolute path avoids depending on the session cwd when Codex launches the stdio server.

### 3. Make The Plugin-Bundled MCP Config More Explicit

The plugin's `.mcp.json` should include:

- `cwd: "."`
- `startup_timeout_sec: 30`

This keeps the bundled MCP declaration self-descriptive and gives the host a better chance to start it correctly if plugin-native MCP loading begins to honor those fields more strictly.

### 4. Preserve User Config Outside Managed Plugin Surfaces

The install/update flow should rewrite only the plugin-owned sections:

- `[plugins."codex-orchestrator@local-plugins"]`
- `[mcp_servers.codex-orchestrator]`
- the managed global `AGENTS` block

Other user config and unrelated MCP servers must remain untouched.

### 5. Verify Tool Availability, Not Only File Presence

Verification should explicitly check both:

- file/config state
- fresh-session tool exposure

The practical acceptance bar is:

- a fresh `codex exec` session can see `orchestrator_resolve_category` as a callable tool

The install guide does not need to require every MCP tool call to complete for every arbitrary cwd, but it must prove that the tool registry contains the orchestrator tools after installation.

## Success Criteria

- The install guide requires global MCP bootstrap for `codex-orchestrator`.
- The plugin `.mcp.json` declares `cwd` and a startup timeout.
- Regression tests fail if MCP bootstrap guidance disappears from the install contract.
- A fresh Codex session after install exposes `orchestrator_*` tools instead of only the bundled skill text.
