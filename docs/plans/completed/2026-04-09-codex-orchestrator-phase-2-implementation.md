# Codex Orchestrator Plugin Phase 2 Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: Use the Codex orchestrator control flow with bounded subagent tasks, real-time plan synchronization, mandatory review gates, and deterministic parent-action derivation. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Harden the orchestrator control plane with write lease enforcement, stronger watchdog logic, and deterministic parent-agent next-action guidance.

**Architecture:** Phase 2 extends the existing zero-third-party stdio MCP runtime. The plugin remains a control-plane and state-plane only system, but it adds write lease state and a more deterministic parent protocol over the same file-backed plan model.

**Tech Stack:** TypeScript, Node.js, zero-third-party stdio MCP runtime, SQLite runtime state, structured markdown plan synchronization, TOML-style category configuration, Codex native subagent dispatch.

---

## Context

- Phase 1 is complete and remains the foundation.
- `backend-impl` already declares `write_policy = lease-required`, but runtime enforcement is not implemented yet.
- `watchdog_tick` currently lists stale tasks but does not produce a deterministic parent protocol.
- Parent agents still need a direct `next_action` tool instead of deriving orchestration from prose alone.

## Artifact Model

- Phase 1 spec: `docs/specs/2026-04-08-codex-orchestrator-plugin-design.md`
- Phase 1 completed plan: `docs/plans/completed/2026-04-08-codex-orchestrator-plugin-implementation.md`
- Phase 2 spec: `docs/specs/2026-04-09-codex-orchestrator-phase-2-design.md`
- Completed plan path: `docs/plans/completed/2026-04-09-codex-orchestrator-phase-2-implementation.md`
- Routing docs:
  - `AGENTS.md`
  - `docs/index.md`
- Runtime state root:
  - `plugins/codex-orchestrator/.codex-orchestrator/state/`

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| P2-T1. Create phase 2 spec and active plan | None | Establishes the execution contract for phase 2 |
| P2-T2. Add write lease schema, storage, and docs | P2-T1 | Runtime lease work needs a phase 2 contract |
| P2-T3. Implement write lease MCP tools and enforcement | P2-T2 | Tool behavior depends on lease schema and rules |
| P2-T4. Strengthen watchdog recommendations | P2-T2 | Recovery policy should consider lease and review state |
| P2-T5. Implement parent-agent next_action tool | P2-T3, P2-T4 | Deterministic next action depends on lease and watchdog semantics |
| P2-T6. Add tests and sync docs | P2-T3, P2-T4, P2-T5 | Validation depends on implemented runtime behavior |

## Parallel Execution Graph

Wave 1:
- P2-T1. Create phase 2 spec and active plan

Wave 2:
- P2-T2. Add write lease schema, storage, and docs

Wave 3:
- P2-T3. Implement write lease MCP tools and enforcement
- P2-T4. Strengthen watchdog recommendations

Wave 4:
- P2-T5. Implement parent-agent next_action tool

Wave 5:
- P2-T6. Add tests and sync docs

Critical Path:
- P2-T1 -> P2-T2 -> P2-T3 -> P2-T5 -> P2-T6

## Quality Gates

- Lease-required categories must fail closed without a lease.
- New tools must have tests.
- New behavior must be reflected in architecture and routing docs.
- Parent next-action derivation must be deterministic from file-backed plan plus runtime state.

## Execution Status

- Current wave: Completed
- Active task: none
- Blockers: None
- Last review result: P2-T6 quality pass

## TODO List

- [x] P2-T1. Create Phase 2 Spec And Active Plan
- [x] P2-T2. Add Write Lease Schema, Storage, And Docs
- [x] P2-T3. Implement Write Lease MCP Tools And Enforcement
- [x] P2-T4. Strengthen Watchdog Recommendations
- [x] P2-T5. Implement Parent-Agent Next_Action Tool
- [x] P2-T6. Add Tests And Sync Docs

### Task P2-T1: Create Phase 2 Spec And Active Plan

**Files:**
- Create: `docs/specs/2026-04-09-codex-orchestrator-phase-2-design.md`
- Create: `docs/plans/completed/2026-04-09-codex-orchestrator-phase-2-implementation.md`
- Modify: `AGENTS.md`
- Modify: `docs/index.md`
- Modify: `task_plan.md`
- Modify: `progress.md`

**Category:** plan
**Owner Role:** harness-planner
**Required Skills:** file-backed planning discipline
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent
**Acceptance Criteria:**
- Phase 2 spec exists
- New active implementation plan exists
- Routing docs point to the new active plan

- [x] Step 1: Create the phase 2 design spec
- [x] Step 2: Create the phase 2 active implementation plan
- [x] Step 3: Update routing and planning files to point at phase 2
- [x] Step 4: Verify the new active plan is the execution anchor

**Verification:**
Run:
```bash
find docs/specs docs/plans/active -maxdepth 1 -type f | sort
```

Expected:
- Phase 2 spec and active plan exist alongside the completed phase 1 artifacts

**Review Gates:**
- Spec review required
- Quality review required

### Task P2-T2: Add Write Lease Schema, Storage, And Docs

**Files:**
- Modify: `plugins/codex-orchestrator/src/services/runtime-store.ts`
- Modify: `plugins/codex-orchestrator/src/types.ts`
- Modify: `docs/architecture/runtime-state-schema.md`
- Create: `docs/architecture/write-lease-protocol.md`
- Modify: `docs/plans/completed/2026-04-09-codex-orchestrator-phase-2-implementation.md`

**Category:** backend-impl
**Owner Role:** mcp-developer
**Required Skills:** runtime state design
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent
**Acceptance Criteria:**
- Lease schema exists in docs and runtime state
- Runtime store can persist and release leases

- [x] Step 1: Define write lease record type and schema
- [x] Step 2: Add lease storage to the runtime store
- [x] Step 3: Document the write lease protocol
- [x] Step 4: Sync the phase 2 plan

**Verification:**
Run:
```bash
rg -n "write lease|lease_id|holder_agent_id" docs plugins/codex-orchestrator/src
```

Expected:
- Lease schema appears in runtime code and architecture docs

**Review Gates:**
- Spec review required
- Quality review required

### Task P2-T3: Implement Write Lease MCP Tools And Enforcement

**Files:**
- Modify: `plugins/codex-orchestrator/src/tools/register-tools.ts`
- Modify: `plugins/codex-orchestrator/src/services/runtime-store.ts`
- Modify: `plugins/codex-orchestrator/src/services/category-registry.ts`
- Modify: `docs/architecture/mcp-tool-contract.md`
- Modify: `docs/plans/completed/2026-04-09-codex-orchestrator-phase-2-implementation.md`

**Category:** backend-impl
**Owner Role:** mcp-developer
**Required Skills:** MCP implementation
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent
**Acceptance Criteria:**
- Acquire and release lease tools exist
- Lease-required tasks fail closed without a lease

- [x] Step 1: Add acquire lease tool
- [x] Step 2: Add release lease tool
- [x] Step 3: Enforce lease-required categories before implementation start
- [x] Step 4: Sync docs and plan

**Verification:**
Run:
```bash
node --experimental-strip-types src/server.ts <<'EOF'
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"probe","version":"0.0.0"}}}
{"jsonrpc":"2.0","id":2,"method":"tools/list"}
EOF
```

Expected:
- Lease tools appear in the tool list

**Review Gates:**
- Spec review required
- Quality review required

### Task P2-T4: Strengthen Watchdog Recommendations

**Files:**
- Modify: `plugins/codex-orchestrator/src/tools/register-tools.ts`
- Modify: `docs/architecture/mcp-tool-contract.md`
- Modify: `docs/architecture/plan-sync-rules.md`
- Modify: `docs/plans/completed/2026-04-09-codex-orchestrator-phase-2-implementation.md`

**Category:** backend-impl
**Owner Role:** mcp-developer
**Required Skills:** orchestration control
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent
**Acceptance Criteria:**
- Watchdog emits action-oriented recommendations
- Recommendation logic includes review failures and lease status

- [x] Step 1: Expand watchdog recommendation logic
- [x] Step 2: Document the new recommendation semantics
- [x] Step 3: Sync the plan

**Verification:**
Run:
```bash
rg -n "watchdog|suggested_action|acquire_write_lease|re-run_review" plugins/codex-orchestrator/src docs/architecture
```

Expected:
- Watchdog logic and docs mention actionable outcomes

**Review Gates:**
- Spec review required
- Quality review required

### Task P2-T5: Implement Parent-Agent Next_Action Tool

**Files:**
- Modify: `plugins/codex-orchestrator/src/tools/register-tools.ts`
- Modify: `plugins/codex-orchestrator/src/services/plan-document.ts`
- Modify: `docs/architecture/agent-contracts.md`
- Modify: `docs/architecture/mcp-tool-contract.md`
- Modify: `docs/plans/completed/2026-04-09-codex-orchestrator-phase-2-implementation.md`

**Category:** backend-impl
**Owner Role:** mcp-developer
**Required Skills:** parent-agent protocol design
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent
**Acceptance Criteria:**
- `orchestrator_next_action` exists
- Parent next-action result is deterministic from plan and runtime state

- [x] Step 1: Add next_action derivation logic
- [x] Step 2: Expose the new MCP tool
- [x] Step 3: Document parent protocol takeover semantics
- [x] Step 4: Sync the plan

**Verification:**
Run:
```bash
node --experimental-strip-types src/server.ts <<'EOF'
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"probe","version":"0.0.0"}}}
{"jsonrpc":"2.0","id":2,"method":"tools/list"}
EOF
```

Expected:
- `orchestrator_next_action` appears in the tool list

**Review Gates:**
- Spec review required
- Quality review required

### Task P2-T6: Add Tests And Sync Docs

**Files:**
- Modify: `plugins/codex-orchestrator/tests/*.test.ts`
- Modify: `docs/index.md`
- Modify: `progress.md`
- Modify: `findings.md`
- Modify: `task_plan.md`
- Modify: `docs/plans/completed/2026-04-09-codex-orchestrator-phase-2-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Required Skills:** workflow validation
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent
**Acceptance Criteria:**
- New lease and next_action behavior are covered by tests
- Routing docs and progress logs reflect phase 2

- [x] Step 1: Add tests for leases, watchdog recommendations, and next_action
- [x] Step 2: Re-run tests and MCP smoke checks
- [x] Step 3: Sync docs, findings, and progress
- [x] Step 4: Mark final acceptance state

**Verification:**
Run:
```bash
node --experimental-strip-types --test tests/*.test.ts
```

Expected:
- All tests pass with the new phase 2 coverage

**Review Gates:**
- Spec review required
- Quality review required

## Final Acceptance

- [x] All phase 2 top-level task checkboxes completed
- [x] All phase 2 review gates passed
- [x] Lease-required categories are enforced
- [x] `next_action` can drive parent orchestration deterministically
- [x] Docs and runtime state stay aligned
