# Codex Orchestrator Plugin Phase 3 Implementation Plan

> **For agentic workers:** REQUIRED WORKFLOW: Use the Codex orchestrator control flow with bounded subagent tasks, real-time plan synchronization, review/repair loops, strict question gating, and completion guards. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ensure the parent agent owns final accountability for child output, never asks for optional expansion by default, and cannot declare completion before true 100% plan completion.

**Architecture:** Phase 3 extends the existing zero-third-party stdio MCP runtime with three new parent-control tools: strict question gate, subagent completion assessment, and completion guard. Review/repair loop behavior is derived deterministically from plan plus runtime state instead of inferred ad hoc.

**Tech Stack:** TypeScript, Node.js, zero-third-party stdio MCP runtime, SQLite runtime state, structured markdown plan synchronization, TOML-style category configuration, Codex native subagent dispatch.

---

## Context

- Phase 2 is complete and remains active baseline behavior.
- Parent orchestration can already derive `next_action`, but it still needs stronger enforcement around partial child output and premature completion.
- User-facing question behavior must now explicitly reject optional expansion questions.

## Artifact Model

- Phase 3 spec: `docs/specs/2026-04-09-codex-orchestrator-phase-3-design.md`
- Completed plan path: `docs/plans/completed/2026-04-09-codex-orchestrator-phase-3-implementation.md`
- Routing docs:
  - `AGENTS.md`
  - `docs/index.md`
- Runtime state root:
  - `plugins/codex-orchestrator/.codex-orchestrator/state/`

## Task Dependency Graph

| Task | Depends On | Reason |
|---|---|---|
| P3-T1. Create phase 3 spec and active plan | None | Establishes the phase 3 execution anchor |
| P3-T2. Add strict question gate docs and tool | P3-T1 | Parent behavior needs a clear contract first |
| P3-T3. Add subagent completion assessment | P3-T1 | Child-output enforcement depends on the phase 3 contract |
| P3-T4. Add automatic review/repair stage derivation | P3-T3 | Repair loop depends on completion assessment results |
| P3-T5. Add completion guard | P3-T3, P3-T4 | Final completion depends on both assessment and repair loop state |
| P3-T6. Add tests and sync docs | P3-T2, P3-T3, P3-T4, P3-T5 | Validation depends on implemented runtime behavior |

## Parallel Execution Graph

Wave 1:
- P3-T1. Create phase 3 spec and active plan

Wave 2:
- P3-T2. Add strict question gate docs and tool
- P3-T3. Add subagent completion assessment

Wave 3:
- P3-T4. Add automatic review/repair stage derivation
- P3-T5. Add completion guard

Wave 4:
- P3-T6. Add tests and sync docs

Critical Path:
- P3-T1 -> P3-T3 -> P3-T4 -> P3-T5 -> P3-T6

## Quality Gates

- Optional expansion should not produce `ask_user = true`.
- Child `DONE` must not imply `can_accept = true` without assessment.
- Repair loops must identify a next required role when review fails.
- Completion guard must fail closed when plan completion is below 100%.

## Execution Status

- Current wave: Completed
- Active task: none
- Blockers: None
- Last review result: P3-T6 quality pass

## TODO List

- [x] P3-T1. Create Phase 3 Spec And Active Plan
- [x] P3-T2. Add Strict Question Gate Docs And Tool
- [x] P3-T3. Add Subagent Completion Assessment
- [x] P3-T4. Add Automatic Review/Repair Stage Derivation
- [x] P3-T5. Add Completion Guard
- [x] P3-T6. Add Tests And Sync Docs

### Task P3-T1: Create Phase 3 Spec And Active Plan

**Files:**
- Create: `docs/specs/2026-04-09-codex-orchestrator-phase-3-design.md`
- Create: `docs/plans/completed/2026-04-09-codex-orchestrator-phase-3-implementation.md`
- Modify: `AGENTS.md`
- Modify: `docs/index.md`
- Modify: `task_plan.md`
- Modify: `progress.md`

**Category:** plan
**Owner Role:** harness-planner
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent
**Acceptance Criteria:**
- Phase 3 spec exists
- New active implementation plan exists
- Routing docs point to the phase 3 plan

- [x] Step 1: Create the phase 3 design spec
- [x] Step 2: Create the phase 3 active implementation plan
- [x] Step 3: Update routing and planning files to point at phase 3
- [x] Step 4: Verify the phase 3 plan is the execution anchor

**Verification:**
Run:
```bash
find docs/specs docs/plans/active -maxdepth 1 -type f | sort
```

Expected:
- Phase 3 spec and active plan exist and the new plan becomes the active execution anchor

**Review Gates:**
- Spec review required
- Quality review required

### Task P3-T2: Add Strict Question Gate Docs And Tool

**Files:**
- Modify: `plugins/codex-orchestrator/src/tools/register-tools.ts`
- Modify: `docs/architecture/agent-contracts.md`
- Create: `docs/architecture/question-gate-protocol.md`
- Modify: `docs/architecture/mcp-tool-contract.md`
- Modify: `docs/plans/completed/2026-04-09-codex-orchestrator-phase-3-implementation.md`

**Category:** backend-impl
**Owner Role:** mcp-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent
**Acceptance Criteria:**
- Question gate tool exists
- Optional expansion no longer yields `ask_user = true`

- [x] Step 1: Define the question gate protocol
- [x] Step 2: Add the MCP question gate tool
- [x] Step 3: Document parent question rules
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
- `orchestrator_question_gate` appears in the tool list

**Review Gates:**
- Spec review required
- Quality review required

### Task P3-T3: Add Subagent Completion Assessment

**Files:**
- Modify: `plugins/codex-orchestrator/src/tools/register-tools.ts`
- Modify: `plugins/codex-orchestrator/src/services/plan-document.ts`
- Modify: `docs/architecture/agent-contracts.md`
- Modify: `docs/architecture/mcp-tool-contract.md`
- Modify: `docs/plans/completed/2026-04-09-codex-orchestrator-phase-3-implementation.md`

**Category:** backend-impl
**Owner Role:** mcp-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent
**Acceptance Criteria:**
- Parent can assess child completion against plan steps and evidence
- Child `DONE` is not enough to accept a task

- [x] Step 1: Add completion assessment logic
- [x] Step 2: Expose the assessment tool
- [x] Step 3: Document parent accountability rules
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
- `orchestrator_assess_subagent_completion` appears in the tool list

**Review Gates:**
- Spec review required
- Quality review required

### Task P3-T4: Add Automatic Review/Repair Stage Derivation

**Files:**
- Modify: `plugins/codex-orchestrator/src/tools/register-tools.ts`
- Modify: `docs/architecture/agent-contracts.md`
- Modify: `docs/architecture/mcp-tool-contract.md`
- Modify: `docs/plans/completed/2026-04-09-codex-orchestrator-phase-3-implementation.md`

**Category:** backend-impl
**Owner Role:** mcp-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent
**Acceptance Criteria:**
- Parent can derive whether to review, repair, redispatch, or accept
- Failed reviews recommend a repair role instead of premature acceptance

- [x] Step 1: Add review/repair loop derivation
- [x] Step 2: Fold it into parent-facing tool outputs
- [x] Step 3: Sync docs and the plan

**Verification:**
Run:
```bash
rg -n "repair|review loop|next_required_stage|repair_role" plugins/codex-orchestrator/src docs/architecture
```

Expected:
- Runtime logic and docs mention review and repair loop derivation

**Review Gates:**
- Spec review required
- Quality review required

### Task P3-T5: Add Completion Guard

**Files:**
- Modify: `plugins/codex-orchestrator/src/tools/register-tools.ts`
- Modify: `docs/architecture/mcp-tool-contract.md`
- Create: `docs/architecture/completion-guard.md`
- Modify: `docs/plans/completed/2026-04-09-codex-orchestrator-phase-3-implementation.md`

**Category:** backend-impl
**Owner Role:** mcp-developer
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent
**Acceptance Criteria:**
- Completion guard tool exists
- It fails closed whenever work is incomplete

- [x] Step 1: Add completion guard logic
- [x] Step 2: Expose the completion guard MCP tool
- [x] Step 3: Document close-plan rules
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
- `orchestrator_completion_guard` appears in the tool list

**Review Gates:**
- Spec review required
- Quality review required

### Task P3-T6: Add Tests And Sync Docs

**Files:**
- Modify: `plugins/codex-orchestrator/tests/*.test.ts`
- Modify: `docs/index.md`
- Modify: `progress.md`
- Modify: `findings.md`
- Modify: `task_plan.md`
- Modify: `docs/plans/completed/2026-04-09-codex-orchestrator-phase-3-implementation.md`

**Category:** review
**Owner Role:** harness-evaluator
**Task Status:** accepted
**Current Step:** none
**Spec Review Status:** pass
**Quality Review Status:** pass
**Assigned Agent:** local-parent
**Acceptance Criteria:**
- Phase 3 behavior is covered by tests
- Docs reflect the new parent-accountability guarantees

- [x] Step 1: Add tests for question gate, completion assessment, repair loop derivation, and completion guard
- [x] Step 2: Re-run tests and MCP smoke checks
- [x] Step 3: Sync docs, findings, and progress
- [x] Step 4: Mark final acceptance state

**Verification:**
Run:
```bash
node --experimental-strip-types --test tests/*.test.ts
```

Expected:
- All tests pass with phase 3 coverage

**Review Gates:**
- Spec review required
- Quality review required

## Final Acceptance

- [x] All phase 3 top-level task checkboxes completed
- [x] Optional expansion defaults no longer produce user questions
- [x] Parent can assess child completion before review or acceptance
- [x] Review and repair loops are derived deterministically
- [x] Completion guard fails closed below 100%
