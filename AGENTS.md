# Project03

## Purpose

This repository hosts the Codex orchestrator plugin that replaces the core engineering workflow previously delegated to `harness-engineering` and related superpowers process skills.

## Routing

- Read [docs/index.md](docs/index.md) first for the document map.
- Read [install.md](install.md) when the task is installing or verifying plugin installation.
- Read the active implementation plan in `docs/plans/active/` when one exists before changing behavior.
- Treat the implementation plan file as the execution source of truth.
- Update routing docs in the same pass when paths, commands, entrypoints, or document locations change.

## Default Workflow

- For repository work in this repo, `codex-orchestrator` is the default workflow.
- Invoke the installed `codex-orchestrator` bundled skill before generic process skills when doing feature work, bug fixes, refactors, debugging, docs changes, architecture work, or other multi-step repo tasks.
- Treat generic process skills as subordinate helpers after orchestration begins or when the plugin is unavailable.
- Start by reading the active plan when one exists, then keep plan status and routing docs synchronized throughout execution.

## Execution Rules

- Keep specification, implementation plan, and execution status in files, not only in chat.
- Do not treat runtime state as the final truth for completion. Plan checkboxes are authoritative.
- Parent orchestration owns task acceptance and plan checkbox updates.
- Child implementers and reviewers must not mark top-level plan tasks complete.
- During active work, `Current Step` must point at the actionable unchecked step instead of lingering at `none`.
- Step synchronization drift is a repository defect; repair missing or stale step pointers before continuing implementation.
- When native Codex `update_plan` is available, mirror the active implementation plan into that surface instead of maintaining a separate chat todo.
- Documentation in this repository must never contain machine-specific absolute filesystem paths.
- Repository docs must use repo-relative artifact links such as `docs/...`, `../AGENTS.md`, or `privacy-policy.md`.
- When an agent first touches the repository and finds legacy absolute-path documentation, it must repair those docs in the same pass without asking for confirmation.

## Current Artifact Model

- Design specs: `docs/specs/`
- Active implementation plans: `docs/plans/active/`
- Completed implementation plans: `docs/plans/completed/`
- Architecture notes: `docs/architecture/`
- Product notes: `docs/product/`
- Decisions: `docs/decisions/`
- Session planning files: `task_plan.md`, `findings.md`, `progress.md`

## Current Key Docs

- [Design spec](docs/specs/2026-04-08-codex-orchestrator-plugin-design.md)
- [Phase 2 design spec](docs/specs/2026-04-09-codex-orchestrator-phase-2-design.md)
- [Phase 3 design spec](docs/specs/2026-04-09-codex-orchestrator-phase-3-design.md)
- [Bundled agents design spec](docs/specs/2026-04-09-codex-orchestrator-bundled-agents-design.md)
- [Installer design spec](docs/specs/2026-04-10-codex-orchestrator-installer-design.md)
- [Marketplace install design spec](docs/specs/2026-04-11-codex-orchestrator-marketplace-install-design.md)
- [Default workflow routing design spec](docs/specs/2026-04-11-codex-orchestrator-default-workflow-routing-design.md)
- [Delegation-first dispatch design spec](docs/specs/2026-04-11-codex-orchestrator-delegation-first-dispatch-design.md)
- [Incremental step synchronization design spec](docs/specs/2026-04-11-codex-orchestrator-incremental-step-sync-design.md)
- [Native Codex todo mirroring design spec](docs/specs/2026-04-11-codex-orchestrator-native-codex-todo-mirroring-design.md)
- [Install guide design spec](docs/specs/2026-04-11-codex-orchestrator-install-guide-design.md)
- [Completed plan auto-archive design spec](docs/specs/2026-04-11-codex-orchestrator-plan-archive-design.md)
- [Relative doc-path policy design spec](docs/specs/2026-04-11-codex-orchestrator-doc-relative-path-policy-design.md)
- [Root install guide](install.md)
- [Phase 1 completed plan](docs/plans/completed/2026-04-08-codex-orchestrator-plugin-implementation.md)
- [Phase 2 completed plan](docs/plans/completed/2026-04-09-codex-orchestrator-phase-2-implementation.md)
- [Phase 3 completed plan](docs/plans/completed/2026-04-09-codex-orchestrator-phase-3-implementation.md)
- [Bundled agents completed plan](docs/plans/completed/2026-04-09-codex-orchestrator-bundled-agents-implementation.md)
- [Installer completed plan](docs/plans/completed/2026-04-10-codex-orchestrator-installer-implementation.md)
- [Marketplace install completed plan](docs/plans/completed/2026-04-11-codex-orchestrator-marketplace-install-implementation.md)
- [Default workflow routing completed plan](docs/plans/completed/2026-04-11-codex-orchestrator-default-workflow-routing-implementation.md)
- [Delegation-first dispatch completed plan](docs/plans/completed/2026-04-11-codex-orchestrator-delegation-first-dispatch-implementation.md)
- [Incremental step synchronization completed plan](docs/plans/completed/2026-04-11-codex-orchestrator-incremental-step-sync-implementation.md)
- [Native Codex todo mirroring completed plan](docs/plans/completed/2026-04-11-codex-orchestrator-native-codex-todo-mirroring-implementation.md)
- [Install guide completed plan](docs/plans/completed/2026-04-11-codex-orchestrator-install-guide-implementation.md)
- [Completed plan auto-archive implementation plan](docs/plans/completed/2026-04-11-codex-orchestrator-plan-archive-implementation.md)
- [Relative doc-path policy completed plan](docs/plans/completed/2026-04-11-codex-orchestrator-doc-relative-path-policy-implementation.md)
- [Category contract](docs/architecture/category-contract.md)
- [Runtime state schema](docs/architecture/runtime-state-schema.md)
- [Write lease protocol](docs/architecture/write-lease-protocol.md)
- [MCP tool contract](docs/architecture/mcp-tool-contract.md)
- [Agent contracts](docs/architecture/agent-contracts.md)
- [Bundled agent bundle](docs/architecture/bundled-agent-bundle.md)
- [Plan sync rules](docs/architecture/plan-sync-rules.md)
- [Question gate protocol](docs/architecture/question-gate-protocol.md)
- [Completion guard](docs/architecture/completion-guard.md)
- [Review repair loop](docs/architecture/review-repair-loop.md)
