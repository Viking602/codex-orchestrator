# Task Plan

## Goal

Remove the remaining root TypeScript contract-test surface so the active `codex-orchestrator` repository becomes fully Rust-only.

## Phases

| Phase | Status | Notes |
|---|---|---|
| 1. Define the full-removal contract and execution anchor | completed | The design spec and implementation plan were written and routed through the repo docs |
| 2. Port the remaining repo-contract checks to Rust | completed | Bundle, manifest, brainstorming, markdown-path, and no-TypeScript guards now run under `rust-cli/tests/` |
| 3. Delete the remaining TypeScript files and stale guidance | completed | Root `tests/*.test.ts` files are gone and active docs/agent scopes now point at the Rust-only surface |
| 4. Validate and archive the removal | completed | `cargo test` passes and the repo contains no `.ts` or `.tsx` files |

## Current Decisions

- The active repository runtime and contract-test surface should be Rust-only.
- Repository contract checks belong in `plugins/codex-orchestrator/rust-cli/tests/`, not in a separate root TypeScript layer.
- Active validation guidance should point at `cargo test --manifest-path plugins/codex-orchestrator/rust-cli/Cargo.toml`.
- Bundled agent scopes and search priorities must track the live Rust source tree instead of deleted TypeScript paths.
- Historical specs and completed plans may continue to document prior TypeScript phases; the active repository surface should not.

## Open Questions

- Bundled fallback agent installation still creates duplicate-role warnings when the host already has the same role names installed globally.
- The Codex app still needs a restart after external marketplace/install changes before its plugin browser picks up the new local source and enabled state.
- The active global `AGENTS` file may be `~/.codex/AGENTS.override.md` or `~/.codex/AGENTS.md`, so installer routing bootstrap must target the active one.
- Plugin-native MCP loading is still not reliable enough to treat bundled skill discovery as proof that `orchestrator_*` tools are callable.

## Completed This Session

- Wrote the full TypeScript removal design spec and implementation plan
- Routed `docs/index.md` and `AGENTS.md` to the new execution anchor
- Added `plugins/codex-orchestrator/rust-cli/tests/repo_contracts.rs` to replace the remaining root TypeScript contract tests
- Ported bundle, manifest, brainstorming, markdown-path, and no-TypeScript assertions into the Rust test surface
- Deleted `tests/agent-bundle.test.ts`
- Deleted `tests/brainstorming-integration.test.ts`
- Deleted `tests/docs-relative-path-policy.test.ts`
- Deleted `tests/plugin-manifest.test.ts`
- Deleted `tests/typescript-compat-removal.test.ts`
- Updated `README.md` and `install.md` to a Cargo-only validation path
- Updated bundled agent scopes and search priorities to point at `rust-cli/src/` and `rust-cli/tests/`
- Removed the stale `typescript-pro` role from the active `backend-impl` allow-list
- Verified `cargo test --manifest-path plugins/codex-orchestrator/rust-cli/Cargo.toml` passes
- Verified `rg --files -g "*.ts" -g "*.tsx"` returns no files
