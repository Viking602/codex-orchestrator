# Task Plan

## Goal

Make `codex-orchestrator` advance top-level task acceptance immediately when terminal review passes close a task, so native todo progress does not stay pinned on the first task and then jump all at once.

## Phases

| Phase | Status | Notes |
|---|---|---|
| 1. Define the immediate-acceptance contract and execution anchor | completed | The design spec and active plan now pin the change to the top-level acceptance timing problem |
| 2. Update runtime acceptance behavior | completed | The Rust control plane now closes terminal-ready tasks through the shared acceptance path |
| 3. Add regression coverage | completed | Runtime and repo-contract tests now cover immediate acceptance and mirror advancement |
| 4. Validate and archive the change | completed | Cargo validation passed and the completed plan has been archived |

## Current Decisions

- Top-level todo movement must happen when a task truly finishes, not in a delayed end-of-wave sweep.
- The terminal quality-review pass is the right control-plane point to close top-level acceptance when all steps and gates are already satisfied.
- Explicit `accept_task` and immediate acceptance should share one runtime path so state updates stay identical.
- `Active task` should advance away from accepted work as part of the same acceptance write.

## Open Questions

- Bundled fallback agent installation still creates duplicate-role warnings when the host already has the same role names installed globally.
- The desktop app still needs a restart after external plugin file changes before a fresh thread sees the updated installed runtime.
- The immediate-acceptance path now covers terminal review recording; future parent-owned gate closures should reuse the same helper instead of forking behavior.

## Completed This Session

- Wrote the immediate top-level acceptance design spec
- Created and completed the implementation plan at `docs/plans/completed/2026-04-12-codex-orchestrator-immediate-top-level-acceptance-implementation.md`
- Routed `docs/index.md` to the new execution anchor
- Narrowed the root cause to late top-level acceptance rather than a native todo UI bug
- Chose a runtime fix that closes top-level acceptance during the terminal review pass instead of waiting for a later sweep
- Verified `cargo test --manifest-path plugins/codex-orchestrator/rust-cli/Cargo.toml` passes
