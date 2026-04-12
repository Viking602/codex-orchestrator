# Codex Orchestrator Rust MCP CLI Design

## Context

The current MCP runtime is implemented as a TypeScript stdio server launched with:

- `node --experimental-strip-types ./src/server.ts`

That works for local development, but it keeps the runtime tied to a JavaScript toolchain. The earlier install-contract repair made the server callable in fresh Codex sessions by bootstrapping a managed global MCP entry, but the runtime itself is still bound to:

- `node`
- TypeScript source files in the installed cache
- runtime parsing via `--experimental-strip-types`

For long-term distribution, that is the wrong boundary. The MCP runtime should be a standalone CLI that can be built, shipped, and launched directly as a native executable.

## Goals

- Replace the active MCP runtime with a Rust CLI that speaks the same stdio JSON-RPC protocol and exposes the same `orchestrator_*` tool surface.
- Keep the existing tool schemas and orchestration behavior stable so skill prompts and parent workflows do not need semantic changes.
- Update plugin configuration and install guidance so Codex-guided install builds or refreshes a native binary and points managed MCP bootstrap at that binary instead of `src/server.ts`.
- Preserve the installed plugin package structure and current marketplace/plugin installation model.

## Non-goals

- Replacing the file-backed plan model or runtime SQLite model.
- Changing category semantics, review gates, or todo-mirroring contracts.
- Deleting every TypeScript source file immediately after the first Rust cutover.
- Shipping a full cross-platform binary release pipeline in this same pass.

## Design

### 1. Introduce A Dedicated Rust MCP Crate

Add a Rust crate under the plugin payload:

- `plugins/codex-orchestrator/rust-cli/`

The crate builds a native binary named:

- `codex-orchestrator-mcp`

This binary becomes the authoritative MCP runtime.

### 2. Keep Protocol And Tool Surface Stable

The Rust server should implement the same line-delimited stdio JSON-RPC contract the current TypeScript server uses:

- `initialize`
- `notifications/initialized`
- `ping`
- `tools/list`
- `tools/call`

The exposed tools must keep the same names and payload shapes, including:

- category resolution
- plan reads
- native Codex todo export
- task and step tracking
- write lease management
- review recording
- next-action derivation
- question gate
- completion assessment
- completion guard

This migration is a runtime replacement, not a tool-contract redesign.

### 3. Port Core Services Into Rust

The Rust crate should own native equivalents of the current TypeScript services:

- category registry loaded from `config/categories.toml`
- runtime store backed by SQLite
- plan document parser/editor/archive helper
- documentation drift helper
- tool registry and action derivation

The Rust implementation should continue to treat repository markdown plans as the source of truth and runtime state as auxiliary orchestration state.

### 4. Switch Install Bootstrap To A Native Binary Path

Codex-guided install should no longer bootstrap the managed global MCP entry against:

- `<installed-cache>/src/server.ts`

Instead it should:

1. build the Rust crate from the installed cache copy
2. stage the built binary under a plugin-owned path in the installed cache
3. point `[mcp_servers.codex-orchestrator]` at that native executable

The stable installed-cache target should be:

- `~/.codex/plugins/cache/local-plugins/codex-orchestrator/local/.codex-orchestrator/bin/codex-orchestrator-mcp`
- Windows variant: `codex-orchestrator-mcp.exe`

### 5. Use Cargo For Source-Checkout Development, Native Binary For Installed Runtime

There are two different operating modes:

- source-checkout development
- installed plugin runtime

For source-checkout development, the bundled plugin `.mcp.json` can launch the Rust crate through `cargo run` so the repository remains runnable without a prebuilt committed binary.

For installed runtime, the managed global MCP bootstrap must point to the built native binary, not to `cargo run` and not to `node`.

That split keeps the repository developer-friendly while making the installed runtime independent of Node and `npx`.

### 6. Treat TypeScript As Compatibility Reference During The Migration Window

The existing TypeScript implementation can remain in-tree for now as:

- behavior reference
- test oracle
- migration fallback while the Rust runtime reaches parity

But the active install and runtime path must shift to Rust once verification passes.

## Verification Strategy

Verification should cover three layers:

1. Rust unit and integration coverage for the new runtime services and stdio tool flow.
2. Repository contract tests for install guidance and plugin MCP declarations.
3. Fresh-session end-to-end validation that a Codex process can see `orchestrator_*` tools when launched against the native binary bootstrap.

## Success Criteria

- A Rust MCP CLI exists under the plugin payload and builds successfully.
- The plugin can expose the orchestration tool surface without using `node` or `npx` at installed-runtime execution time.
- `install.md` and related routing docs describe native-binary bootstrap instead of `src/server.ts`.
- Fresh-session validation confirms the managed MCP bootstrap points to the native binary and `orchestrator_*` tools are callable.
