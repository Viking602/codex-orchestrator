# Docs Index

## Start Here

- [Repository map](../AGENTS.md)
- [Current design spec](specs/2026-04-12-codex-orchestrator-immediate-top-level-acceptance-design.md)
- [Phase 2 design spec](specs/2026-04-09-codex-orchestrator-phase-2-design.md)
- [Phase 3 design spec](specs/2026-04-09-codex-orchestrator-phase-3-design.md)
- [Installer design spec](specs/2026-04-10-codex-orchestrator-installer-design.md)
- [Marketplace install design spec](specs/2026-04-11-codex-orchestrator-marketplace-install-design.md)
- [Default workflow routing design spec](specs/2026-04-11-codex-orchestrator-default-workflow-routing-design.md)
- [Delegation-first dispatch design spec](specs/2026-04-11-codex-orchestrator-delegation-first-dispatch-design.md)
- [Incremental step synchronization design spec](specs/2026-04-11-codex-orchestrator-incremental-step-sync-design.md)
- [Native Codex todo mirroring design spec](specs/2026-04-11-codex-orchestrator-native-codex-todo-mirroring-design.md)
- [Codex-guided install design spec](specs/2026-04-11-codex-orchestrator-codex-guided-install-design.md)
- [MCP bootstrap install design spec](specs/2026-04-12-codex-orchestrator-mcp-bootstrap-install-design.md)
- [Rust MCP CLI design spec](specs/2026-04-12-codex-orchestrator-rust-mcp-cli-design.md)
- [TypeScript compatibility removal design spec](specs/2026-04-12-codex-orchestrator-typescript-compat-removal-design.md)
- [Full TypeScript removal design spec](specs/2026-04-12-codex-orchestrator-full-typescript-removal-design.md)
- [Brainstorming integration design spec](specs/2026-04-12-codex-orchestrator-brainstorming-integration-design.md)
- [Install guide design spec](specs/2026-04-11-codex-orchestrator-install-guide-design.md)
- [Immediate top-level acceptance design spec](specs/2026-04-12-codex-orchestrator-immediate-top-level-acceptance-design.md)
- [Completed plan auto-archive design spec](specs/2026-04-11-codex-orchestrator-plan-archive-design.md)
- [Relative doc-path policy design spec](specs/2026-04-11-codex-orchestrator-doc-relative-path-policy-design.md)
- [Bundled agents architecture note](architecture/bundled-agent-bundle.md)
- [Root install guide](../install.md)
- [Full TypeScript removal completed implementation plan](plans/completed/2026-04-12-codex-orchestrator-full-typescript-removal-implementation.md)
- [Brainstorming integration completed implementation plan](plans/completed/2026-04-12-codex-orchestrator-brainstorming-integration-implementation.md)

## Architecture

- [Architecture docs landing page](architecture/README.md)
- [Category contract](architecture/category-contract.md)
- [Runtime state schema](architecture/runtime-state-schema.md)
- [MCP tool contract](architecture/mcp-tool-contract.md)
- [Agent contracts](architecture/agent-contracts.md)
- [Bundled agent bundle](architecture/bundled-agent-bundle.md)
- [Plan synchronization rules](architecture/plan-sync-rules.md)
- [Write lease protocol](architecture/write-lease-protocol.md)
- [Question gate protocol](architecture/question-gate-protocol.md)
- [Completion guard](architecture/completion-guard.md)
- [Review repair loop](architecture/review-repair-loop.md)

## Product

- [Product docs landing page](product/README.md)
- [Privacy Policy](product/privacy-policy.md)
- [Terms of Service](product/terms-of-service.md)

## Decisions

- [Decision docs landing page](decisions/README.md)
- [File-backed execution truth](decisions/2026-04-08-file-backed-execution-truth.md)

## Plans

- `docs/plans/active/` holds plans currently being executed.
- `docs/plans/completed/` holds closed plans for historical reference.
- Active plans should advance `Current Step` incrementally so progress is visible during execution rather than batch-updated at the end.
- When native Codex `update_plan` is available, the parent should mirror the active plan into that surface instead of creating a separate chat todo.
- Completed plans auto-move out of `active/` once all top-level TODO items are checked and any `Final Acceptance` items are closed.
- `docs/plans/active/` is currently empty.
- The most recent completed implementation plan is [2026-04-12-codex-orchestrator-immediate-top-level-acceptance-implementation.md](plans/completed/2026-04-12-codex-orchestrator-immediate-top-level-acceptance-implementation.md).

## Specs

- `docs/specs/` holds design specifications approved before implementation.
