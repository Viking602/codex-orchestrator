# Codex Orchestrator Plugin Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: Use the Codex orchestrator control flow with bounded subagent tasks, real-time plan synchronization, and mandatory review gates. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Codex-native orchestrator plugin that replaces the core engineering workflow currently provided by process-oriented skills, while keeping execution anchored to file-backed plans and documents.

**Architecture:** The plugin acts as control-plane and state-plane only. Codex native subagents remain the execution engine. Durable truth lives in spec, plan, and repository documents, while runtime state accelerates orchestration and recovery.

**Tech Stack:** TypeScript, Node.js, zero-third-party stdio MCP runtime, SQLite runtime state, structured markdown plan synchronization, TOML-style category configuration, Codex native subagent dispatch.

---

## Context

- The repository is currently empty.
- The plugin must replace the core orchestration workflow, not all expert skills.
- The plugin must absorb the responsibilities of design capture, planning, category routing, review gates, plan synchronization, and document synchronization.
- The plugin must use file-backed artifacts to prevent execution drift.

## Artifact Model

- Spec path: `docs/specs/2026-04-08-codex-orchestrator-plugin-design.md`
- Active plan path: `docs/plans/active/2026-04-08-codex-orchestrator-plugin-implementation.md`
- Routing docs:
  - `AGENTS.md`
  - `docs/index.md`
- Long-lived docs:
  - `docs/architecture/`
  - `docs/product/`
  - `docs/decisions/`
- Runtime state root:
  - `.codex-orchestrator/state/`

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| T1. Define plugin artifact model and repo skeleton | None | Establishes the file-backed source of truth |
| T2. Define category contract and runtime state schema | T1 | Needs the artifact model and document locations |
| T3. Define MCP tool surface and state transitions | T2 | Tool contracts depend on category and runtime semantics |
| T4. Define parent, implementer, and reviewer contracts | T2, T3 | Role prompts must align with categories and MCP capabilities |
| T5. Define plan sync and document drift rules | T1, T3, T4 | Sync behavior depends on plan structure and execution lifecycle |
| T6. Scaffold plugin and MCP implementation | T3, T4, T5 | Code skeleton should reflect the final contracts |
| T7. Validate workflow against spec and update docs | T6 | Requires the first integrated implementation pass |

## Parallel Execution Graph

Wave 1:
- T1. Define plugin artifact model and repo skeleton

Wave 2:
- T2. Define category contract and runtime state schema

Wave 3:
- T3. Define MCP tool surface and state transitions
- T4. Define parent, implementer, and reviewer contracts

Wave 4:
- T5. Define plan sync and document drift rules

Wave 5:
- T6. Scaffold plugin and MCP implementation

Wave 6:
- T7. Validate workflow against spec and update docs

Critical Path:
- T1 -> T2 -> T3 -> T5 -> T6 -> T7

## Quality Gates

- Every top-level task must have explicit files, acceptance criteria, and verification.
- The plan file must be updated during execution, not only after execution.
- No task may reach accepted state without spec review and quality review pass.
- Routing docs must be updated in the same pass when the project surface changes.
- Final acceptance cannot pass while any top-level checkbox remains open.

## Execution Status

- Current wave: Completed
- Active task: none
- Blockers: None
- Last review result: T7 quality pass

## TODO List

- [x] T1. Define Plugin Artifact Model And Repo Skeleton
- [x] T2. Define Category Contract And Runtime State Schema
- [x] T3. Define MCP Tool Surface And State Transitions
- [x] T4. Define Parent, Implementer, And Reviewer Contracts
- [x] T5. Define Plan Sync And Document Drift Rules
- [x] T6. Scaffold Plugin And MCP Implementation
- [x] T7. Validate Workflow Against Spec And Update Docs

### Task T1: Define Plugin Artifact Model And Repo Skeleton

**Files:**
- Create: `AGENTS.md`
- Create: `docs/index.md`
- Create: `task_plan.md`
- Create: `findings.md`
- Create: `progress.md`
- Create: `docs/specs/2026-04-08-codex-orchestrator-plugin-design.md`
- Modify: `docs/plans/active/2026-04-08-codex-orchestrator-plugin-implementation.md`

**Category:** plan
**Owner Role:** harness-planner
**Required Skills:** file-backed planning discipline
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent
**Acceptance Criteria:**
- Repository has a clear artifact model and routing docs
- Design spec exists
- Active implementation plan exists
- Session planning files exist

- [x] Step 1: Create routing docs and planning files
- [x] Step 2: Create the initial design specification
- [x] Step 3: Create the active implementation plan
- [x] Step 4: Review artifact model for completeness

**Verification:**
Run:
```bash
find . -maxdepth 3 -type f | sort
```

Expected:
- Routing docs, spec, plan, and planning files exist in the expected locations

**Review Gates:**
- Spec review required
- Quality review required

### Task T2: Define Category Contract And Runtime State Schema

**Files:**
- Create: `docs/architecture/category-contract.md`
- Create: `docs/architecture/runtime-state-schema.md`
- Modify: `docs/plans/active/2026-04-08-codex-orchestrator-plugin-implementation.md`
- Modify: `findings.md`

**Category:** plan
**Owner Role:** harness-planner
**Required Skills:** orchestration design
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent
**Acceptance Criteria:**
- Category registry fields are defined
- Runtime state authority boundaries are defined
- Accepted task states and transitions are explicit

- [x] Step 1: Define category semantics and required fields
- [x] Step 2: Define runtime state schema and authority boundaries
- [x] Step 3: Define task status transitions and acceptance rules
- [x] Step 4: Sync plan and findings documents

**Verification:**
Run:
```bash
rg -n "Category|Runtime state|status" docs/architecture docs/plans
```

Expected:
- Category and runtime state rules are documented and referenced by the active plan

**Review Gates:**
- Spec review required
- Quality review required

### Task T3: Define MCP Tool Surface And State Transitions

**Files:**
- Create: `docs/architecture/mcp-tool-contract.md`
- Modify: `docs/architecture/runtime-state-schema.md`
- Modify: `docs/plans/active/2026-04-08-codex-orchestrator-plugin-implementation.md`
- Modify: `findings.md`

**Category:** plan
**Owner Role:** harness-planner
**Required Skills:** MCP design
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent
**Acceptance Criteria:**
- MCP tool set is explicitly listed
- Each tool has a clear responsibility
- State transition points are mapped to tools

- [x] Step 1: List minimum MCP tools required for orchestration
- [x] Step 2: Define each tool's role in state progression
- [x] Step 3: Record how plan sync interacts with tool outputs
- [x] Step 4: Sync plan and findings documents

**Verification:**
Run:
```bash
rg -n "orchestrator_" docs/architecture
```

Expected:
- MCP tool contract document defines the planned tool surface

**Review Gates:**
- Spec review required
- Quality review required

### Task T4: Define Parent, Implementer, And Reviewer Contracts

**Files:**
- Create: `docs/architecture/agent-contracts.md`
- Modify: `docs/architecture/category-contract.md`
- Modify: `docs/plans/active/2026-04-08-codex-orchestrator-plugin-implementation.md`

**Category:** plan
**Owner Role:** harness-planner
**Required Skills:** subagent orchestration
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent
**Acceptance Criteria:**
- Parent, implementer, and reviewer responsibilities are explicit
- Incremental todo update rules are explicit
- Review-before-acceptance rule is explicit

- [x] Step 1: Define parent-agent authority and duties
- [x] Step 2: Define implementer progress and return contract
- [x] Step 3: Define reviewer contract and acceptance gate
- [x] Step 4: Sync plan document

**Verification:**
Run:
```bash
rg -n "Parent|Implementer|Reviewer|accept" docs/architecture/agent-contracts.md
```

Expected:
- Contracts document clearly separates execution, review, and acceptance authority

**Review Gates:**
- Spec review required
- Quality review required

### Task T5: Define Plan Sync And Document Drift Rules

**Files:**
- Create: `docs/architecture/plan-sync-rules.md`
- Create: `docs/decisions/2026-04-08-file-backed-execution-truth.md`
- Modify: `AGENTS.md`
- Modify: `docs/index.md`
- Modify: `docs/plans/active/2026-04-08-codex-orchestrator-plugin-implementation.md`

**Category:** plan
**Owner Role:** harness-doc-gardener
**Required Skills:** routing-doc maintenance
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent
**Acceptance Criteria:**
- Real-time plan sync rules are explicit
- Drift guard rules are explicit
- Routing docs reflect the final artifact model

- [x] Step 1: Define real-time plan synchronization rules
- [x] Step 2: Define drift guard triggers for routing and architecture docs
- [x] Step 3: Record the file-backed truth decision
- [x] Step 4: Sync routing docs and active plan

**Verification:**
Run:
```bash
rg -n "plan sync|drift|truth" AGENTS.md docs/index.md docs/architecture docs/decisions
```

Expected:
- Routing docs and architecture docs consistently describe the same artifact model and sync rules

**Review Gates:**
- Spec review required
- Quality review required

### Task T6: Scaffold Plugin And MCP Implementation

**Files:**
- Create: `plugins/codex-orchestrator/.codex-plugin/plugin.json`
- Create: `plugins/codex-orchestrator/.mcp.json`
- Create: `plugins/codex-orchestrator/agents/openai.yaml`
- Create: `plugins/codex-orchestrator/skills/orchestrator/SKILL.md`
- Create: `plugins/codex-orchestrator/mcp/server.ts`
- Create: `plugins/codex-orchestrator/mcp/tools/`
- Create: `plugins/codex-orchestrator/mcp/db/`
- Modify: `docs/plans/active/2026-04-08-codex-orchestrator-plugin-implementation.md`

**Category:** backend-impl
**Owner Role:** mcp-developer
**Required Skills:** plugin creation, MCP implementation
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent
**Acceptance Criteria:**
- Plugin skeleton exists
- MCP server skeleton exists
- Tool layout matches documented MCP contract

- [x] Step 1: Scaffold plugin manifest and local MCP config
- [x] Step 2: Scaffold orchestrator skill and agent surface
- [x] Step 3: Scaffold MCP server entrypoint and module layout
- [x] Step 4: Sync active plan with created files

**Verification:**
Run:
```bash
find plugins/codex-orchestrator -maxdepth 4 -type f | sort
```

Expected:
- Plugin and MCP scaffold files exist in the expected layout

**Review Gates:**
- Spec review required
- Quality review required

### Task T7: Validate Workflow Against Spec And Update Docs

**Files:**
- Modify: `docs/specs/2026-04-08-codex-orchestrator-plugin-design.md`
- Modify: `docs/plans/active/2026-04-08-codex-orchestrator-plugin-implementation.md`
- Modify: `docs/index.md`
- Modify: `progress.md`

**Category:** review
**Owner Role:** harness-evaluator
**Required Skills:** workflow validation
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent
**Acceptance Criteria:**
- Spec and implementation plan are aligned
- Routing docs are current
- Remaining implementation tasks are ready for execution without chat-only context

- [x] Step 1: Compare implementation artifacts against the design spec
- [x] Step 2: Record any drift or missing contracts
- [x] Step 3: Update docs and progress log
- [x] Step 4: Mark final readiness state in the plan

**Verification:**
Run:
```bash
rg -n "Goal|Category|Review|Final Acceptance" docs/specs docs/plans AGENTS.md docs/index.md
```

Expected:
- Core workflow concepts appear consistently across spec, plan, and routing docs

**Review Gates:**
- Spec review required
- Quality review required

## Final Acceptance

- [x] All top-level task checkboxes completed
- [x] All review gates passed
- [x] Routing docs updated
- [x] Architecture and decision docs updated
- [x] Plugin scaffold created
- [x] Execution can resume from files without chat context
