# Codex Orchestrator TypeScript Compatibility Removal Design

## Context

The Rust CLI migration is complete and the active MCP runtime now launches through:

- `cargo run --manifest-path ./rust-cli/Cargo.toml` for source-checkout development
- a staged native binary for installed Codex sessions

The legacy TypeScript runtime under `plugins/codex-orchestrator/src/` is no longer on any supported execution path. It remains only as migration residue together with plugin-local tests that import those TypeScript modules directly.

Keeping that dead compatibility layer in-tree creates three problems:

- it suggests there are still two supported runtimes when there is only one
- plugin-local tests still couple the repository to removed runtime internals instead of the Rust contract
- package/test surface remains noisier than necessary after the Rust cutover

## Goals

- Remove the legacy TypeScript runtime implementation under `plugins/codex-orchestrator/src/`.
- Remove plugin-local test and config files that only exist to exercise the deleted TypeScript runtime.
- Preserve equivalent behavior coverage through Rust integration tests and repo-level contract tests.
- Simplify the plugin package surface so repository-local runtime commands point at Rust only.

## Non-goals

- Removing every TypeScript file from the repository.
- Rewriting historical completed plans so they no longer mention the prior TypeScript runtime.
- Changing the `orchestrator_*` tool contract or runtime semantics.
- Replacing repo-level Node-based contract tests that do not depend on the deleted runtime implementation.

## Design

### 1. Remove The Dead Runtime Tree

Delete:

- `plugins/codex-orchestrator/src/`

After the Rust cutover, no supported development or installed-runtime path should refer back to those files.

### 2. Replace Runtime-Coupled Plugin Tests

Delete the plugin-local test files that import `../src/**` directly and move the durable coverage boundary to:

- Rust integration tests under `plugins/codex-orchestrator/rust-cli/tests/`
- repo-level contract tests under `tests/`

Coverage should still include:

- category resolution contract
- plan read/archive behavior
- next-action and step-sync behavior
- todo export behavior
- completion and review gates
- agent-bundle and manifest contract checks

### 3. Remove Plugin-Local TypeScript Test Scaffolding

Delete:

- `plugins/codex-orchestrator/tsconfig.json`

The plugin package should no longer need a TypeScript project boundary once the runtime-coupled tests are gone.

### 4. Simplify Package Scripts Around Rust

Keep `plugins/codex-orchestrator/package.json` only as a lightweight developer command surface if it still adds value. Its scripts should point at Rust-only actions:

- `cargo test --manifest-path rust-cli/Cargo.toml`
- `cargo run --quiet --manifest-path rust-cli/Cargo.toml`

It should not present Node-based plugin-runtime tests as the primary validation path after the TypeScript runtime is removed.

### 5. Add A Structural Regression Guard

Add a repo-level regression test that fails if:

- `plugins/codex-orchestrator/src/` still exists
- `plugins/codex-orchestrator/tsconfig.json` still exists
- the plugin package scripts still advertise the deleted TypeScript runtime path as an active execution surface

This keeps the removal durable instead of relying on one-time cleanup.

## Verification Strategy

Verification should cover:

1. Rust integration coverage for the behaviors previously guarded by plugin-local TypeScript runtime tests.
2. Repo-level contract coverage for the removed TypeScript compatibility surface and for retained plugin metadata/docs behavior.
3. Standard repository validation:
   - `cargo test --manifest-path plugins/codex-orchestrator/rust-cli/Cargo.toml`
   - `node --experimental-strip-types --test tests/*.test.ts`

## Success Criteria

- `plugins/codex-orchestrator/src/` is removed from the active repository surface.
- Plugin-local TypeScript runtime tests and `tsconfig.json` are removed.
- Rust and repo-level tests cover the deleted runtime surface sufficiently to keep behavior regressions visible.
- Plugin developer scripts no longer imply that the deleted TypeScript runtime is supported.
