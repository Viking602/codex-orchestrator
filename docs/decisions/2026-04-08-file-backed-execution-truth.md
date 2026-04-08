# 2026-04-08 File-Backed Execution Truth

## Decision

The active implementation plan file is the primary execution truth for progress and completion. Runtime state exists to support orchestration but must not replace the plan file as the authoritative record.

## Why

- Chat context is not durable enough for long-running execution.
- Runtime state alone is too opaque for human review.
- Checkbox progress in the plan file creates a resumable, inspectable execution ledger.

## Consequences

- Parent orchestration must update the plan file throughout execution.
- Implementers and reviewers cannot self-declare top-level task completion.
- Acceptance requires synchronized runtime state and plan state.
