# Project03

## Purpose

This repository hosts the Codex orchestrator plugin that replaces the core engineering workflow previously delegated to `harness-engineering` and related superpowers process skills.

## Routing

- Read [docs/index.md](/Users/viking/agents_dev/project03/docs/index.md) first for the document map.
- Read the active implementation plan in `docs/plans/active/` before changing behavior.
- Treat the implementation plan file as the execution source of truth.
- Update routing docs in the same pass when paths, commands, entrypoints, or document locations change.

## Execution Rules

- Keep specification, implementation plan, and execution status in files, not only in chat.
- Do not treat runtime state as the final truth for completion. Plan checkboxes are authoritative.
- Parent orchestration owns task acceptance and plan checkbox updates.
- Child implementers and reviewers must not mark top-level plan tasks complete.

## Current Artifact Model

- Design specs: `docs/specs/`
- Active implementation plans: `docs/plans/active/`
- Completed implementation plans: `docs/plans/completed/`
- Architecture notes: `docs/architecture/`
- Product notes: `docs/product/`
- Decisions: `docs/decisions/`
- Session planning files: `task_plan.md`, `findings.md`, `progress.md`

## Current Key Docs

- [Design spec](/Users/viking/agents_dev/project03/docs/specs/2026-04-08-codex-orchestrator-plugin-design.md)
- [Active implementation plan](/Users/viking/agents_dev/project03/docs/plans/active/2026-04-08-codex-orchestrator-plugin-implementation.md)
- [Category contract](/Users/viking/agents_dev/project03/docs/architecture/category-contract.md)
- [Runtime state schema](/Users/viking/agents_dev/project03/docs/architecture/runtime-state-schema.md)
- [MCP tool contract](/Users/viking/agents_dev/project03/docs/architecture/mcp-tool-contract.md)
- [Agent contracts](/Users/viking/agents_dev/project03/docs/architecture/agent-contracts.md)
- [Plan sync rules](/Users/viking/agents_dev/project03/docs/architecture/plan-sync-rules.md)
