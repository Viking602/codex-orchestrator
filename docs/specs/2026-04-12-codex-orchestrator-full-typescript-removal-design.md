# Codex Orchestrator Full TypeScript Removal Design

## Context

The Rust CLI migration and the earlier TypeScript compatibility removal deleted the plugin-local runtime, but the repository still keeps a root-level TypeScript contract-test surface under `tests/*.test.ts`.

That residual TypeScript layer now causes three problems:

- the repository still requires Node to validate active surfaces that are otherwise Rust-only
- the remaining `.ts` files contradict the intended "Rust-only active surface" message
- current install and validation docs still point at `node --experimental-strip-types --test tests/*.test.ts`

The user explicitly wants the remaining TypeScript code removed. The durable way to do that is to migrate the surviving repo-contract assertions into Rust integration tests and then delete the root TypeScript test files.

## Goals

- Remove every remaining `.ts` and `.tsx` file from the active repository surface.
- Preserve the current contract coverage for manifest metadata, bundled agents, brainstorming integration, and markdown path hygiene.
- Make Cargo the only active repository validation entrypoint.
- Repair active docs and bundled-agent guidance so they no longer point at deleted TypeScript paths or Node-based validation commands.

## Non-goals

- Rewriting historical completed plans or specs that correctly document the prior TypeScript implementation.
- Removing non-TypeScript developer metadata such as `package.json` when it still serves as a lightweight Rust command surface.
- Changing the Rust MCP behavior or `orchestrator_*` tool contract.

## Design

### 1. Replace Root TypeScript Contract Tests With Rust Repo-Contract Tests

Move the remaining repository-structure assertions from `tests/*.test.ts` into Rust integration tests under:

- `plugins/codex-orchestrator/rust-cli/tests/`

The Rust repo-contract suite should cover:

- bundled agent inventory and preferred-role alignment
- brainstorming integration routing and manifest metadata
- markdown path hygiene for active markdown docs
- plugin manifest ownership metadata
- absence of remaining `.ts` and `.tsx` files in the repository

### 2. Delete The Remaining TypeScript Files

Delete:

- `tests/agent-bundle.test.ts`
- `tests/brainstorming-integration.test.ts`
- `tests/docs-relative-path-policy.test.ts`
- `tests/plugin-manifest.test.ts`
- `tests/typescript-compat-removal.test.ts`

After the Rust replacement lands, there should be no `.ts` or `.tsx` files left in the active repository surface.

### 3. Update Active Guidance To A Cargo-Only Validation Surface

Repair active surfaces that still describe Node-based repository validation:

- `README.md`
- `install.md`
- `AGENTS.md`
- `docs/index.md`
- bundled agent instructions that still point to deleted `plugins/codex-orchestrator/src/**` or `plugins/codex-orchestrator/tests/**`

Current guidance should point at:

- `plugins/codex-orchestrator/rust-cli/`
- `plugins/codex-orchestrator/rust-cli/tests/`
- `cargo test --manifest-path plugins/codex-orchestrator/rust-cli/Cargo.toml`

### 4. Add A Stronger Structural Guard

The Rust repo-contract suite should fail if:

- any `.ts` or `.tsx` file remains under the repository root
- the plugin package scripts or active docs still advertise the deleted TypeScript validation path
- active bundled-agent guidance still points at the deleted plugin-local TypeScript runtime tree

## Verification Strategy

Verification should cover:

1. Rust integration coverage for orchestration behavior:
   - `cargo test --manifest-path plugins/codex-orchestrator/rust-cli/Cargo.toml`
2. Structural confirmation that no TypeScript files remain:
   - `rg --files -g "*.ts" -g "*.tsx"`

## Success Criteria

- No `.ts` or `.tsx` file remains in the active repository surface.
- The surviving repo-contract coverage runs from Rust only.
- Active docs and bundled-agent guidance no longer point at deleted TypeScript files or Node-based validation commands.
- Cargo validation passes after the removal.
