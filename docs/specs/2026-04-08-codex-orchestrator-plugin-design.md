# Codex Orchestrator Plugin Design

## Context

The goal is to build a Codex-native orchestrator plugin that makes file-backed planning, bounded subagent execution, review gates, and document synchronization the default engineering workflow. Once enabled, this plugin should be able to replace the core workflow responsibilities currently provided by `harness-engineering` and the related process-oriented superpowers.

The plugin must not rebuild its own execution harness around external Codex CLI processes. It should use Codex native subagent dispatch and treat the plugin as the control-plane and state-plane.

## Problem Statement

The current desired workflow has three failure modes:

1. Plans can drift because they live partly in chat and partly in files.
2. Task progress can drift because Todo updates are often batched at the end instead of updated step-by-step.
3. Execution can drift because implementers, reviewers, and routing decisions are not enforced by a shared runtime contract.

## Goals

- Make spec and implementation plan files mandatory artifacts.
- Treat the active implementation plan as the authoritative execution ledger.
- Require incremental step completion updates instead of end-of-task batch updates.
- Route work through categories that encode workflow semantics, not just model preference.
- Enforce review gates before top-level task acceptance.
- Synchronize routing and architecture docs when work changes the project surface.
- Keep runtime state durable enough to resume execution without relying on chat context.

## Non-goals

- Replace all domain-specific skills and plugins in phase 1.
- Rebuild a tmux-based or CLI-based execution runtime.
- Recreate every OpenAgent hook and background daemon behavior in phase 1.
- Solve every model selection nuance before the control-plane workflow is stable.

## Design Principles

### 1. File-First Truth

The plugin should persist all durable artifacts to files:

- design spec
- active implementation plan
- routing docs
- architecture and product documents

Chat is not an authoritative source of execution state.

### 2. Plan Is The Control Surface

The implementation plan must function as:

- execution queue
- dependency graph
- progress ledger
- review gate ledger
- final acceptance checklist

The runtime state may accelerate orchestration, but completion truth lives in the plan file.

### 3. Parent Owns Acceptance

Child agents may implement and review, but only the parent orchestration layer may:

- set active task
- mark steps complete in the plan
- advance review state
- accept top-level tasks
- close the implementation plan

### 4. Category Means Workflow Semantics

Each category must define:

- allowed roles
- write policy
- review requirements
- plan requirement
- parallelism limits
- session reuse rules
- completion contract

Category must not be reduced to a direct model alias.

### 5. Review Before Acceptance

An implementer returning `DONE` is not sufficient. A task reaches accepted state only after:

- spec review passes
- quality review passes
- required verification evidence is recorded
- the parent updates the plan file

## Artifact Model

```text
AGENTS.md
docs/index.md
docs/specs/YYYY-MM-DD-<topic>-design.md
docs/plans/active/YYYY-MM-DD-<topic>-implementation.md
docs/plans/completed/
docs/architecture/
docs/product/
docs/decisions/
.codex-orchestrator/state/
task_plan.md
findings.md
progress.md
```

## Plugin Responsibilities

### Design Controller

Replaces the design-side control flow previously spread across brainstorming and related process skills.

Responsibilities:

- capture user intent
- record assumptions and non-goals
- produce a design spec file
- prevent implementation before design and plan artifacts exist

### Plan Controller

Produces an OpenAgent-style implementation plan with:

- context
- dependency graph
- parallel execution graph
- bounded task blocks
- checkbox steps
- review gates
- final acceptance section

### Category Router

Maps each top-level task to:

- category
- allowed role set
- preferred role
- model tier policy
- review policy
- session reuse policy

### Runtime State Manager

Stores execution metadata such as:

- plan id
- active task
- assigned agent id
- current execution stage
- retry count
- blocker
- review status

This state is supportive, not authoritative for completion.

### Plan Sync Engine

Writes runtime progress back to the implementation plan in real time.

Responsibilities:

- set active task markers
- mark step checkboxes as they complete
- write review outcomes
- record blockers
- advance final acceptance items

The synchronization mechanism should target explicit mutable fields and checkboxes in a structured markdown plan format instead of relying on chat state.

### Review Controller

Enforces:

- implementer -> spec review -> quality review
- failed review returns task to implementer
- no top-level task acceptance without both review passes

### Knowledge Sync

Ensures repository knowledge stays aligned with execution:

- update `AGENTS.md`
- update `docs/index.md`
- update architecture/product/decision docs when behavior or boundaries change

## Required Plan Shape

Every implementation plan should contain:

- `Context`
- `Artifact Model`
- `Task Dependency Graph`
- `Parallel Execution Graph`
- `Quality Gates`
- `Execution Status`
- task blocks with checkbox steps
- `Final Acceptance`

## Enforcement Strategy

The plugin should not rely on polite instructions alone. It should reject invalid progression:

- no category contract -> do not dispatch
- no active plan file -> do not implement
- no incremental step update -> do not accept task
- no review pass -> do not accept task
- unfinished plan checkboxes -> do not close plan

## Phase 1 Scope

Phase 1 should replace the core workflow responsibilities now handled by:

- `harness-engineering`
- `brainstorming`
- `writing-plans`
- `subagent-driven-development`
- `requesting-code-review`
- `verification-before-completion`

Phase 1 should not replace domain-specific expert skills.

## Success Criteria

- A design spec exists before implementation begins.
- An implementation plan exists and is the active source of truth during execution.
- Step checkboxes are updated incrementally, not batched at the end.
- Task acceptance always happens after review gates.
- Repository routing and architecture docs remain synchronized with execution changes.
- A session can resume from files without relying on prior chat context.

## Phase 1 Implementation Note

Phase 1 uses a zero-third-party stdio MCP server on top of Node.js `--experimental-strip-types` instead of the previously assumed MCP SDK dependency. This keeps the local plugin runnable without network package installation while preserving the same tool contract and file-backed workflow semantics.
